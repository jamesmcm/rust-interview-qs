// Copyright (C) 2019 James McMurray
//
// This source code is subject to the terms of the GNU General Public License v3.0.
// You should have received a copy of the GNU General Public License
// along with this source code. If not, see <http://www.gnu.org/licenses/>.


// https://leetcode.com/problems/reverse-integer/

struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut nums: Vec<i32> = Solution::reverse_recurse(x);
        let mut out: i32 = 0;
        nums.reverse();

        for (i, num) in nums.iter().enumerate() {
            match num.overflowing_mul(10_i32.pow(i as u32)) {
                (x, false) => match out.overflowing_add(x) {
                    (_, false) => out += x,
                    (_, true) => {
                        out = 0;
                        break;
                    }
                },
                (_, true) => {
                    out = 0;
                    break;
                }
            }
        }

        out
    }

    pub fn reverse_recurse(x: i32) -> Vec<i32> {
        match x {
            0 => vec![],
            _ => [vec![(x % 10)], Solution::reverse_recurse(x / 10)].concat(),
        }
    }
}

fn main() {
    println!("{}", Solution::reverse(123));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recurse() {
        assert_eq!(Solution::reverse_recurse(123), vec![3, 2, 1]);
    }
    #[test]
    fn test_123() {
        assert_eq!(Solution::reverse(123), 321);
    }
    #[test]
    fn test_neg123() {
        assert_eq!(Solution::reverse(-123), -321);
    }
    #[test]
    fn test_120() {
        assert_eq!(Solution::reverse(120), 21);
    }
    #[test]
    fn test_0() {
        assert_eq!(Solution::reverse(0), 0);
    }
    #[test]
    fn test_1000() {
        assert_eq!(Solution::reverse(1000), 1);
    }
    #[test]
    fn test_2147483647() {
        assert_eq!(Solution::reverse(2147483647), 0); // Overflow
    }
    #[test]
    fn test_1563847412() {
        assert_eq!(Solution::reverse(1563847412), 0); // Overflow
    }
}
