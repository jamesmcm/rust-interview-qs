// https://leetcode.com/problems/add-strings/

struct Solution {}

impl Solution {
    pub fn add_strings(mut num1: String, mut num2: String) -> String {
        unsafe {
            let n1b = num1.as_bytes_mut();
            n1b.reverse();
            let n2b = num2.as_bytes_mut();
            n2b.reverse();

            let maxlen = n1b.len().max(n2b.len());
            let mut carry = false;
            let mut out_chars: Vec<u8> = Vec::with_capacity(maxlen);

            for i in 0..maxlen {
                let mut n = 0;
                if n1b.get(i).is_some() && n2b.get(i).is_some() {
                    n = (n1b[i] - 48) + (n2b[i] - 48);
                } else if n1b.get(i).is_some() {
                    n = n1b[i] - 48;
                } else if n2b.get(i).is_some() {
                    n = n2b[i] - 48;
                }

                if carry {
                    n += 1;
                    carry = false;
                }
                if n >= 10 {
                    carry = true;
                    n = n % 10;
                }
                out_chars.push(n + 48)
            }
            if carry {
                out_chars.push(49);
            }

            out_chars.reverse();
            String::from_utf8_unchecked(out_chars)
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::add_strings("9".to_string(), "99".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Solution::add_strings("1".to_string(), "1".to_string()), "2");
    }
    #[test]
    fn test_carry() {
        assert_eq!(
            Solution::add_strings("9".to_string(), "1".to_string()),
            "10"
        );
    }
    #[test]
    fn test_add2() {
        assert_eq!(
            Solution::add_strings("9".to_string(), "2".to_string()),
            "11"
        );
    }
    #[test]
    fn test_add3() {
        assert_eq!(
            Solution::add_strings("99".to_string(), "99".to_string()),
            "198"
        );
    }
    #[test]
    fn test_add4() {
        assert_eq!(
            Solution::add_strings("1000".to_string(), "1".to_string()),
            "1001"
        );
    }
    #[test]
    fn test_add5() {
        assert_eq!(
            Solution::add_strings("9".to_string(), "99".to_string()),
            "108"
        );
    }
}
