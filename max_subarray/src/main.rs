// https://leetcode.com/problems/maximum-subarray

struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        Solution::gen_array(&nums).1
    }

    fn trinum(n: usize) -> usize {
        (n * (n + 1)) / 2
    }

    fn gen_array(nums: &Vec<i32>) -> (Vec<i32>, i32) {
        let n = nums.len();
        let mut max: i32 = std::i32::MIN;
        let mut arr = Vec::with_capacity(Solution::trinum(n));

        for (i, val) in nums.iter().enumerate() {
            arr.push(*val);
            if *val > max {
                max = *val
            }
            for j in 0..i {
                // println!("{}, {}", i, j);
                let newval = val + arr[j + Solution::trinum(i) - i];
                arr.push(newval);
                if newval > max {
                    max = newval
                }
            }
        }
        (arr, max)
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
    fn test_trinum() {
        assert_eq!(Solution::trinum(5), 15);
    }
    #[test]
    fn test_gen_array() {
        assert_eq!(
            Solution::gen_array(&vec![1, 2, 3]),
            (vec![1, 2, 3, 3, 5, 6], 6)
        );
    }

    #[test]
    fn test_actual() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn test_actual2() {
        assert_eq!(Solution::max_sub_array(vec![-1]), -1);
    }
}
