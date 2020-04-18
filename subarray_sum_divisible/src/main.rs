// https://leetcode.com/problems/maximum-subarray

struct Solution {}

impl Solution {
    pub fn subarrays_div_by_k(a: Vec<i32>, k: i32) -> i32 {
        Solution::gen_array(&a)
            .iter()
            .filter(|&x| (x % k == 0))
            .count() as i32
    }

    fn trinum(n: usize) -> usize {
        (n * (n + 1)) / 2
    }

    fn gen_array(nums: &Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut arr = Vec::with_capacity(Solution::trinum(n));

        for (i, val) in nums.iter().enumerate() {
            let tri = Solution::trinum(i);
            arr.push(*val);
            for j in 0..i {
                // println!("{}, {}", i, j);
                let newval = val + arr[j + tri - i];
                arr.push(newval);
            }
        }
        arr
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
        assert_eq!(Solution::gen_array(&vec![1, 2, 3]), vec![1, 2, 3, 3, 5, 6]);
    }

    #[test]
    fn test_actual() {
        assert_eq!(Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
    }
    #[test]
    fn test_actual2() {
        assert_eq!(Solution::subarrays_div_by_k(vec![0; 30000], 10000), 0);
    }
}
