// https://leetcode.com/problems/course-schedule/
use std::collections::HashMap;
use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // Build HashMap
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        for pair in prerequisites {
            map.entry(pair[0])
                .and_modify(|v| v.push(pair[1]))
                .or_insert(vec![pair[1]]);
        }

        while !map.is_empty() {
            let preq: &i32 = map.keys().nth(0).unwrap();
            let mut innerset: Vec<(i32, HashSet<i32>)> = vec![(*preq, HashSet::new())];

            while !innerset.is_empty() {
                let ks = innerset.pop().unwrap();
                let k: i32 = ks.0;
                let mut seen: HashSet<i32> = ks.1;
                if seen.contains(&k) {
                    return false;
                }
                seen.insert(k);

                for el in map.remove(&k).iter() {
                    for iel in el {
                        innerset.push((*iel, seen.clone()));
                    }
                }
            }
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::can_finish(2, vec![vec![1, 0]]));
    }
    #[test]
    fn test2() {
        assert!(!Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    }
    #[test]
    fn test3() {
        assert!(Solution::can_finish(
            4,
            vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]
        ));
    }
}
