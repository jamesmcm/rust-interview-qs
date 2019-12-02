struct Solution {}

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let powers: Vec<(i32, char)> = vec![
            (1000, 'M'),
            (500, 'D'),
            (100, 'C'),
            (50, 'L'),
            (10, 'X'),
            (5, 'V'),
            (1, 'I'),
        ];
        // Loop through powers find first divisible
        // Check if we are within one rank lower of the rank higher, i.e. within 10 of 100 if >= 50
        // If so we use subtraction form (subtract greatest power): XCIII - 93
        // Else just add this rank and recurse on subtraction i.e. L + int_to_roman(6) - 56
        if num == 0 {
            return String::from("");
        }

        for (i, item) in powers.iter().enumerate() {
            if num / item.0 == 0 {
                continue;
            }
            // Subtraction
            if let Some(_x) = powers.get(i + (i % 2)) {
                if i > 0 {
                    let y = powers[i - 1];
                    if let Some(z) =
                        Solution::greatest_power(y.0, num, &powers[i..(i + 1 + (i % 2))])
                    {
                        return z.1.to_string()
                            + &y.1.to_string()
                            + &Solution::int_to_roman(num - (y.0 - z.0));
                    }
                }
            }
            // Addition
            return item.1.to_string() + &Solution::int_to_roman(num - item.0);
        }
        String::from("ERROR")
    }

    fn greatest_power(limit: i32, n: i32, powers: &[(i32, char)]) -> Option<(i32, char)> {
        // let powers: Vec<(i32, char)> = vec![(1000, 'M'), (500, 'D'), (100, 'C'), (50, 'L'), (10, 'X'), (5, 'V'), (1, 'I')];
        let vals: HashSet<i32> = HashSet::from_iter(powers.iter().map(|x| x.0));
        for p in powers {
            if p.0 < limit
                && (limit - p.0) <= n
                && (p.0 < limit / 2)
                && (!(vals.contains(&(limit - n))))
                || (p.0 == (limit - n) && p.0 != n)
            {
                return Some(*p);
            }
        }
        None
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
    fn test_xlv() {
        assert_eq!(Solution::int_to_roman(45), String::from("XLV"));
    }
    #[test]
    fn test_v() {
        assert_eq!(Solution::int_to_roman(5), String::from("V"));
    }
    #[test]
    fn test_i() {
        assert_eq!(Solution::int_to_roman(1), String::from("I"));
    }
    #[test]
    fn test_mmm() {
        assert_eq!(Solution::int_to_roman(3000), String::from("MMM"));
    }
    #[test]
    fn test_ic() {
        assert_eq!(Solution::int_to_roman(99), String::from("XCIX"));
    }
    #[test]
    fn test_xm() {
        assert_eq!(Solution::int_to_roman(990), String::from("CMXC"));
    }
    #[test]
    fn test_xlviii() {
        assert_eq!(Solution::int_to_roman(48), String::from("XLVIII"));
    }
    #[test]
    fn test_xl() {
        assert_eq!(Solution::int_to_roman(40), String::from("XL"));
    }
    #[test]
    fn test_xii() {
        assert_eq!(Solution::int_to_roman(12), String::from("XII"));
    }
    #[test]
    fn test_ix() {
        assert_eq!(Solution::int_to_roman(9), String::from("IX"));
    }
    #[test]
    fn test_mcmxii() {
        assert_eq!(Solution::int_to_roman(1912), String::from("MCMXII"));
    }
    #[test]
    fn test_cmxci() {
        assert_eq!(Solution::int_to_roman(991), String::from("CMXCI"));
    }
    #[test]
    fn test_mcmxci() {
        assert_eq!(Solution::int_to_roman(1991), String::from("MCMXCI"));
    }
    #[test]
    fn test_il() {
        assert_eq!(Solution::int_to_roman(49), String::from("XLIX"));
    }
    #[test]
    fn test_iii() {
        assert_eq!(Solution::int_to_roman(3), String::from("III"));
    }
    #[test]
    fn test_iv() {
        assert_eq!(Solution::int_to_roman(4), String::from("IV"));
    }
    #[test]
    fn test_lviii() {
        assert_eq!(Solution::int_to_roman(58), String::from("LVIII"));
    }
    #[test]
    fn test_mcmxciv() {
        assert_eq!(Solution::int_to_roman(1994), String::from("MCMXCIV"));
    }
}
