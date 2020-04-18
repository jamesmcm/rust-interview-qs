// https://leetcode.com/problems/maximum-subarray

struct Solution {}

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let mut arr = Vec::with_capacity(Solution::trinum(n));

        for (i, val) in nums.iter().enumerate() {
            let tri = Solution::trinum(i);
            arr.push(*val);
            for j in 0..i {
                // println!("{}, {}", i, j);
                let newval = val + arr[j + tri - i];

                if (newval == 0) {
                    return true;
                }
                if (k != 0) && (newval % k == 0) && (newval >= k) {
                    return true;
                }
                arr.push(newval);
            }
        }
        false
    }

    fn trinum(n: usize) -> usize {
        (n * (n + 1)) / 2
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
    fn test_actual() {
        assert_eq!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6), true);
    }
    #[test]
    fn test_actual2() {
        assert_eq!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6), true);
    }
    #[test]
    fn test_actual3() {
        assert_eq!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 0), false);
    }
    #[test]
    fn test_actual4() {
        assert_eq!(Solution::check_subarray_sum(vec![0, 0], 0), true);
    }
    #[test]
    fn test_actual5() {
        assert_eq!(Solution::check_subarray_sum(vec![0; 6], 1), true);
    }
}
