// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
use std::collections::HashSet;

// Adapted from solution to rangesum:
// https://leetcode.com/problems/range-sum-query-immutable/
struct NumArray {
    lookup: Vec<usize>,
    nums: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<usize>) -> Self {
        let trisize: usize = nums.len() * (nums.len() + 1) / 2;
        let mut lookup: Vec<usize> = Vec::with_capacity(trisize);

        for (i, vi) in nums.iter().enumerate() {
            for (j, vj) in (&nums[i..]).iter().enumerate() {
                if j == 0 {
                    lookup.push(i);
                } else {
                    //println!("{}, {}, {}", i, j, (i * (i + 1) / 2) + j - 1);
                    let thistri = trisize - ((nums.len() - i) * (nums.len() + 1 - i) / 2);
                    let lastval: usize = nums[lookup[thistri + j - 1]];

                    // Min range
                    if lastval <= *vj {
                        lookup.push(lookup[thistri + j - 1]);
                    } else {
                        lookup.push(i + j);
                    }
                }
            }
        }

        NumArray {
            lookup,
            nums: nums.len(),
        }
    }

    fn min_range(&self, i: usize, j: usize) -> usize {
        let i = i as usize;
        let j = j as usize;
        let thistri = self.lookup.len() - ((self.nums - i) * (self.nums + 1 - i) / 2);
        self.lookup[thistri + (j - i)]
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(i, j);
 */

#[derive(Debug, Eq, PartialEq)]
struct TreeNode<T> {
    label: T,
    children: Vec<Option<usize>>,
}

type Tree<T> = Vec<Option<TreeNode<T>>>;

const fn num_bits<T>() -> usize {
    std::mem::size_of::<T>() * 8
}

fn log_2(x: usize) -> usize {
    if x == 0 {
        return 0;
    }
    num_bits::<usize>() as usize - x.leading_zeros() as usize - 1
}

fn build_binary_tree<T: Clone + std::fmt::Debug>(arr: Vec<Option<T>>) -> Tree<T> {
    let mut out = Vec::with_capacity(arr.len());
    for (i, el) in arr.iter().enumerate() {
        // 2**depth + 2**child
        let depth = log_2(i + 1);
        let child = i - ((2 as usize).pow(depth as u32) - 1);

        if let Some(elem) = el {
            let left_index =
                (2 as usize).pow(depth as u32) + ((2 as usize).pow(child as u32)) + i - 1;
            let left_child = match arr.get(left_index) {
                None => None,
                Some(None) => None,
                Some(Some(_)) => Some(left_index),
            };

            let right_child = match arr.get(left_index + 1) {
                None => None,
                Some(None) => None,
                Some(Some(_)) => Some(left_index + 1),
            };

            out.push(Some(TreeNode {
                label: (*elem).clone(),
                children: vec![left_child, right_child],
            }));
        } else {
            out.push(None);
        }
    }

    out
}

/// Complete euler walk of tree and return node visit order and depths
fn euler_walk<T>(tree: &Tree<T>) -> (Vec<usize>, Vec<usize>) {
    let mut stack: Vec<(&TreeNode<T>, usize, usize)> = Vec::with_capacity(tree.len());

    let mut visits = Vec::with_capacity(2 * tree.len());
    let mut depths = Vec::with_capacity(2 * tree.len());
    let mut seen = HashSet::with_capacity(tree.len());

    stack.push((tree[0].as_ref().unwrap(), 0, 0));

    while stack.len() > 0 {
        let task = stack.pop().unwrap();
        visits.push(task.2);
        depths.push(task.1);
        seen.insert(task.2);

        for child in &task.0.children {
            if let Some(x) = child {
                if !seen.contains(&x) {
                    stack.push((task.0, task.1, task.2));
                    stack.push((tree[*x].as_ref().unwrap(), task.1 + 1, *x));
                    break;
                }
            }
        }
    }

    (visits, depths)
}

fn lowest_common_ancestor<T>(tree: &Tree<T>, p: usize, q: usize) -> usize {
    let (visits, depths) = euler_walk(&tree);

    let firstp = visits.iter().position(|&x| x == p).unwrap();
    let firstq = visits.iter().position(|&x| x == q).unwrap();
    let numarray = NumArray::new(depths);
    let mindepth = numarray.min_range(firstp, firstq);
    visits[mindepth]
}

fn lowest_common_ancestor_from_labels<T: Eq>(tree: &Tree<T>, p: T, q: T) -> &T {
    let (visits, depths) = euler_walk(&tree);

    let indexp = tree
        .iter()
        .position(|x| x.is_some() && x.as_ref().unwrap().label == p)
        .unwrap();
    let indexq = tree
        .iter()
        .position(|x| x.is_some() && x.as_ref().unwrap().label == q)
        .unwrap();

    let mut firstp = visits.iter().position(|&x| x == indexp).unwrap();
    let mut firstq = visits.iter().position(|&x| x == indexq).unwrap();

    if firstp > firstq {
        std::mem::swap(&mut firstq, &mut firstp);
    }

    let numarray = NumArray::new(depths);
    let mindepth = numarray.min_range(firstp, firstq);
    &tree[visits[mindepth]].as_ref().unwrap().label
}

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */

fn main() {
    let tree = build_binary_tree(vec![
        Some(3),
        Some(5),
        Some(1),
        Some(6),
        Some(2),
        Some(0),
        Some(8),
        None,
        None,
        Some(7),
        Some(4),
    ]);

    for node in tree {
        println!("{:?}", node);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        let nums: Vec<usize> = vec![42, 0, 3, 5, 2, 1];
        let numarray = NumArray::new(nums);
        assert_eq!(numarray.min_range(0, 2), 1);
    }
    #[test]
    fn test2() {
        let nums: Vec<usize> = vec![42, 0, 3, 5, 2, 1];
        let numarray = NumArray::new(nums);
        assert_eq!(numarray.min_range(2, 5), 5);
    }
    #[test]
    fn test3() {
        let nums: Vec<usize> = vec![42, 0, 3, 5, 2, 1];
        let numarray = NumArray::new(nums);
        assert_eq!(numarray.min_range(0, 5), 1);
    }
    #[test]
    fn test_euler() {
        let tree = build_binary_tree(vec![Some(3), Some(5), Some(1)]);

        assert_eq!(
            tree,
            vec![
                Some(TreeNode {
                    label: 3,
                    children: vec![Some(1), Some(2)]
                }),
                Some(TreeNode {
                    label: 5,
                    children: vec![None, None]
                }),
                Some(TreeNode {
                    label: 1,
                    children: vec![None, None]
                })
            ]
        );

        let (visits, depths) = euler_walk(&tree);
        assert_eq!(visits, vec![0, 1, 0, 2, 0]);
        assert_eq!(depths, vec![0, 1, 0, 1, 0]);
    }

    #[test]
    fn test_small() {
        let tree = build_binary_tree(vec![Some(3), Some(5), Some(1)]);
        let (visits, depths) = euler_walk(&tree);
        let lca = lowest_common_ancestor(&tree, 1, 2);

        assert_eq!(lca, 0);
    }

    #[test]
    fn test_small_self() {
        let tree = build_binary_tree(vec![Some(3), Some(5), Some(1)]);
        let (visits, depths) = euler_walk(&tree);
        let lca = lowest_common_ancestor(&tree, 2, 2);

        assert_eq!(lca, 2);
    }

    #[test]
    fn test_large_1() {
        let tree = build_binary_tree(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let (visits, depths) = euler_walk(&tree);
        let lca = lowest_common_ancestor(&tree, 1, 2);

        assert_eq!(lca, 0);
    }
    #[test]
    fn test_large_2() {
        let tree = build_binary_tree(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let (visits, depths) = euler_walk(&tree);
        let lca = lowest_common_ancestor(&tree, 1, 10);

        assert_eq!(lca, 1);
    }
    #[test]
    fn test_large_3() {
        let tree = build_binary_tree(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let (visits, depths) = euler_walk(&tree);
        let lca = lowest_common_ancestor_from_labels(&tree, 4, 7);

        assert_eq!(*lca, 2);
    }
}
