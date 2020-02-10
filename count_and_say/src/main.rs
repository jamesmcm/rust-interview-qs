/*
 * The count-and-say sequence is the sequence of integers with the first five terms as following:

    1
    11
    21
    1211
    111221

1 is read off as "one 1" or 11.
11 is read off as "two 1s" or 21.
21 is read off as "one 2, then one 1" or 1211.
Given an integer n where 1 ≤ n ≤ 30, generate the nth term of the count-and-say sequence.
Note: Each term of the sequence of integers will be represented as a string.
Example 1:
Input: 1
Output: "1"
Example 2:
Input: 4
Output: "1211"
*/

struct Solution {}

struct CountAndSay {
    val: u32,
}

impl CountAndSay {
    fn count_and_say(init: u32) -> String {
        let mut lastc: char = 'z';
        let mut countc: u32 = 0;
        let mut output: String = String::new();

        for c in format!["{:?}", init].chars() {
            if c != lastc {
                if countc > 0 {
                    output.push_str(&format!["{:?}", countc]);
                    output.push(lastc);
                }
                countc = 0;
                lastc = c;
            }
            countc += 1;
        }
        output.push_str(&format!["{:?}", countc]);
        output.push(lastc);
        output
    }
}

impl Iterator for CountAndSay {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let origval = self.val;
        self.val = CountAndSay::count_and_say(self.val).parse().unwrap();
        Some(origval)
    }
}

impl Solution {
    fn count_and_say_n(n: usize) -> u32 {
        CountAndSay { val: 1 }.nth(n - 1).unwrap()
    }
}

fn main() {
    println!("{:?}", CountAndSay { val: 1 }.take(5).collect::<Vec<u32>>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation1() {
        assert_eq!(CountAndSay::count_and_say(1), String::from("11"));
    }
    #[test]
    fn test_generation2() {
        assert_eq!(CountAndSay::count_and_say(11), String::from("21"));
    }
    #[test]
    fn test_generation3() {
        assert_eq!(CountAndSay::count_and_say(21), String::from("1211"));
    }
    #[test]
    fn test_first_element() {
        assert_eq!(Solution::count_and_say_n(1), 1);
    }
    #[test]
    fn test_fourth_element() {
        assert_eq!(Solution::count_and_say_n(4), 1211);
    }
}
