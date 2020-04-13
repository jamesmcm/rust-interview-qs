// https://leetcode.com/problems/range-sum-query-immutable/

struct NumArray {
    lookup: Vec<i32>,
    nums: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let trisize: usize = nums.len() * (nums.len() + 1) / 2;
        let mut lookup: Vec<i32> = Vec::with_capacity(trisize);

        for (i, vi) in nums.iter().enumerate() {
            for (j, vj) in (&nums[i..]).iter().enumerate() {
                if j == 0 {
                    lookup.push(*vj);
                } else {
                    //println!("{}, {}, {}", i, j, (i * (i + 1) / 2) + j - 1);
                    let thistri = trisize - ((nums.len() - i) * (nums.len() + 1 - i) / 2);
                    let lastval: i32 = lookup[thistri + j - 1];

                    // Min range
                    // if lastval <= *vj {
                    //     lookup.push(lastval);
                    // } else {
                    //     lookup.push(*vj);
                    // }
                    // Sum range
                    lookup.push(lastval + *vj);
                }
            }
        }

        NumArray {
            lookup,
            nums: nums.len(),
        }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
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

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        let nums: Vec<i32> = vec![-2, 0, 3, -5, 2, -1];
        let numarray = NumArray::new(nums);
        assert_eq!(numarray.sum_range(0, 2), 1);
    }
    #[test]
    fn test2() {
        let nums: Vec<i32> = vec![-2, 0, 3, -5, 2, -1];
        let numarray = NumArray::new(nums);
        assert_eq!(numarray.sum_range(2, 5), -1);
    }
    #[test]
    fn test3() {
        let nums: Vec<i32> = vec![-2, 0, 3, -5, 2, -1];
        let numarray = NumArray::new(nums);
        assert_eq!(numarray.sum_range(0, 5), -3);
    }
}
