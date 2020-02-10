// https://leetcode.com/problems/happy-number/

use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        let mut ss: i32 = 0;
        let mut seen: HashSet<i32> = HashSet::new();

        loop {
            while n > 0 {
                ss += (n % 10).pow(2);
                n = n / 10;
            }
            if ss == 1 {
                return true;
            }
            if seen.contains(&ss) {
                return false;
            }
            seen.insert(ss);
            n = ss;
            // println!("{:?}", seen);
            ss = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_happy(19));
    }
}

fn main() {
    println!("{:?}", Solution::is_happy(2));
}
