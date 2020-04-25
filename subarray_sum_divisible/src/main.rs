// https://leetcode.com/problems/subarray-sums-divisible-by-k/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn subarrays_div_by_k(a: Vec<i32>, k: i32) -> i32 {
        let mut seen_modulos: HashMap<i32, i32> = HashMap::with_capacity(k as usize);

        let mut result: i32 = 0;
        seen_modulos.insert(0, 1);
        let mut cumsum = 0;
        for num in a {
            cumsum += num;
            let modulo = cumsum.rem_euclid(k);
            result += *seen_modulos
                .entry(modulo)
                .and_modify(|e| *e += 1)
                .or_insert(1)
                - 1;
            // dbg!(&seen_modulos, &result);
        }
        result
    }
    pub fn subarrays_div_by_k_n2(a: Vec<i32>, k: i32) -> i32 {
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
        assert_eq!(
            Solution::subarrays_div_by_k(vec![0; 30000], 10000),
            450015000
        );
    }
    #[test]
    fn test_actual3() {
        assert_eq!(Solution::subarrays_div_by_k(vec![-1, 2, 9], 2), 2);
    }
}
