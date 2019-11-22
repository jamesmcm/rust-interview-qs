// Copyright (C) 2019 James McMurray
//
// This source code is subject to the terms of the GNU General Public License v3.0.
// You should have received a copy of the GNU General Public License
// along with this source code. If not, see <http://www.gnu.org/licenses/>.

// https://leetcode.com/problems/integer-to-english-words/

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

struct Solution {}

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        // Split in to triples, convert to number in hundreds then add thousand, million, billion
        let lowers: Vec<String> = vec_of_strings![
            "Zero",
            "One",
            "Two",
            "Three",
            "Four",
            "Five",
            "Six",
            "Seven",
            "Eight",
            "Nine",
            "Ten",
            "Eleven",
            "Twelve",
            "Thirteen",
            "Fourteen",
            "Fifteen",
            "Sixteen",
            "Seventeen",
            "Eighteen",
            "Nineteen"
        ];

        let tens: Vec<String> = vec_of_strings![
            "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"
        ];
        let joinwords: Vec<String> = vec_of_strings!["", "Thousand", "Million", "Billion"];
        let mut inner_num: i32 = num;
        let mut string_return: Vec<String> = vec![];
        let mut count: usize = 0;

        if num == 0 {
            return String::from("Zero");
        }

        while inner_num > 0 {
            if inner_num % 1000 > 0 {
                string_return.push(joinwords[count].clone());
                string_return.push(String::from(" "));
                string_return.push(Solution::handle_triple(inner_num % 1000, &lowers, &tens));
                string_return.push(String::from(" "));
            }

            inner_num /= 1000;
            count += 1;
        }

        string_return.reverse();
        string_return.concat().trim().to_string()
    }

    fn handle_triple(num: i32, lowers: &[String], tens: &[String]) -> String {
        if num == 0 {
            return String::from("");
        }
        // Handle case < 1000
        let mut string_return: Vec<String> = vec![];
        let mut inner_num: i32 = num;
        if inner_num > 99 {
            string_return.push(lowers[(inner_num / 100) as usize].clone());
            string_return.push(String::from(" Hundred"));
            inner_num %= 100;
        }
        if inner_num > 19 {
            if !string_return.is_empty() {
                string_return.push(String::from(" "));
            }
            string_return.push(tens[(inner_num / 10) as usize].clone());
            inner_num %= 10;
        }
        if inner_num != 0 {
            if !string_return.is_empty() {
                string_return.push(String::from(" "));
            }
            string_return.push(lowers[inner_num as usize].clone());
        }

        string_return.concat()
    }
}

fn main() {
    println!("{}", Solution::number_to_words(1156));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_123() {
        assert_eq!(Solution::number_to_words(123), "One Hundred Twenty Three");
    }

    #[test]
    fn test_12345() {
        assert_eq!(
            Solution::number_to_words(12345),
            "Twelve Thousand Three Hundred Forty Five"
        );
    }

    #[test]
    fn test_1234567() {
        assert_eq!(
            Solution::number_to_words(1234567),
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
        );
    }

    #[test]
    fn test_1234567891() {
        assert_eq!(Solution::number_to_words(1234567891), "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One");
    }

    #[test]
    fn test_0() {
        assert_eq!(Solution::number_to_words(0), "Zero");
    }

    #[test]
    fn test_1_000_000() {
        assert_eq!(Solution::number_to_words(1_000_000), "One Million");
    }
    #[test]
    fn test_10_000() {
        assert_eq!(Solution::number_to_words(10_000), "Ten Thousand");
    }
    #[test]
    fn test_100() {
        assert_eq!(Solution::number_to_words(100), "One Hundred");
    }
    #[test]
    fn test_100_000() {
        assert_eq!(Solution::number_to_words(100_000), "One Hundred Thousand");
    }
    #[test]
    fn test_1000() {
        assert_eq!(Solution::number_to_words(1000), "One Thousand");
    }
    #[test]
    fn test_max() {
        assert_eq!(Solution::number_to_words(2147483647), "Two Billion One Hundred Forty Seven Million Four Hundred Eighty Three Thousand Six Hundred Forty Seven");
    }
    #[test]
    fn test_1_000_010() {
        assert_eq!(Solution::number_to_words(1_000_010), "One Million Ten");
    }
    #[test]
    fn test_1_000_073() {
        assert_eq!(
            Solution::number_to_words(1_000_073),
            "One Million Seventy Three"
        );
    }
    #[test]
    fn test_1_000_373() {
        assert_eq!(
            Solution::number_to_words(1_000_373),
            "One Million Three Hundred Seventy Three"
        );
    }
    #[test]
    fn test_1_020_010() {
        assert_eq!(
            Solution::number_to_words(1_020_010),
            "One Million Twenty Thousand Ten"
        );
    }
}
