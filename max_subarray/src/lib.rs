// https://leetcode.com/problems/maximum-subarray
pub struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // O(N)
        let mut cumsum: i32 = 0;
        let mut best: i32 = std::i32::MIN;

        for n in nums {
            cumsum += n;
            if cumsum > best {
                best = cumsum;
            }
            if cumsum < 0 {
                cumsum = 0;
            }
        }
        best
    }

    pub fn max_sub_array_n2(nums: Vec<i32>) -> i32 {
        // O(N**2)
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
            let tri = Solution::trinum(i);
            arr.push(*val);
            if *val > max {
                max = *val
            }
            for j in 0..i {
                let newval = val + arr[j + tri - i];
                arr.push(newval);
                if newval > max {
                    max = newval
                }
            }
        }
        (arr, max)
    }

    pub fn max_sub_array_n3(nums: Vec<i32>) -> i32 {
        // O(N**3)
        let mut best: i32 = std::i32::MIN;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                let sum = nums[i..=j].iter().sum();
                if sum > best {
                    best = sum;
                }
            }
        }
        best
    }
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

    #[test]
    fn test_actual_n2() {
        assert_eq!(
            Solution::max_sub_array_n2(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn test_actual2_n2() {
        assert_eq!(Solution::max_sub_array_n2(vec![-1]), -1);
    }

    #[test]
    fn test_actual_n3() {
        assert_eq!(
            Solution::max_sub_array_n3(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn test_actual2_n3() {
        assert_eq!(Solution::max_sub_array_n3(vec![-1]), -1);
    }
}
