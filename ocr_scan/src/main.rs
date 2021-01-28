// An Optical Character Recognition (OCR) scanner scans an image of text
// and tries to produce a string. However, sometimes characters cannot be
// correctly identified are marked as unknown with a question mark: ?

// e.g. "Apple" might be scanned as "A??le" if the "p"s are not identified
// correctly.

// Our OCR scanner compresses the output, so that a run of N unidentified
// characters is written as the number N. i.e. the above output would
// actually be "A2le"

// Our text only contains uppercase and lowercase ASCII letters A-Z and a-z.

// Write a function that can take the output of two OCR scans and return
// true if they could be the same string or false if they could not be
// the same string.

// e.g. A2le and Ap2e could be from the same string: Apple

// But Am1le and Ap2e could not be.

// Note that the scans must have exactly the same length to be the
// same underlying string, i.e. A2le3 and A2le2 have different total lengths and so
// could not represent the same string.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Char(char),
    Number(u32),
}

pub fn get_next_token<T: std::iter::Iterator<Item = char>>(
    ic: &mut std::iter::Peekable<T>,
) -> Option<Token> {
    match ic.peek() {
        None => {
            ic.next();
            None
        }
        // if number then read parse whole number
        Some(c) if c.is_digit(10) => {
            let mut num = ic.next().unwrap().to_digit(10).unwrap();
            while let Some(x) = ic.peek() {
                if x.is_digit(10) {
                    num *= 10;
                    num += ic.next().unwrap().to_digit(10).unwrap();
                } else {
                    break;
                }
            }
            Some(Token::Number(num))
        }
        // else return char
        Some(_) => Some(Token::Char(ic.next().unwrap())),
    }
}

pub fn same_string(s1: &str, s2: &str) -> bool {
    let mut s1_chars = s1.chars().peekable();
    let mut s2_chars = s2.chars().peekable();

    let mut c1 = get_next_token(&mut s1_chars);
    let mut c2 = get_next_token(&mut s2_chars);

    loop {
        match (c1, c2) {
            (None, None) => break true,
            (None, _) => break false,
            (_, None) => break false,
            (Some(Token::Number(n)), Some(Token::Number(m))) => {
                if n > 1 {
                    c1 = Some(Token::Number(n - 1));
                } else {
                    c1 = get_next_token(&mut s1_chars);
                }

                if m > 1 {
                    c2 = Some(Token::Number(m - 1));
                } else {
                    c2 = get_next_token(&mut s2_chars);
                }
            }
            (Some(Token::Number(n)), Some(Token::Char(_))) => {
                if n > 1 {
                    c1 = Some(Token::Number(n - 1));
                } else {
                    c1 = get_next_token(&mut s1_chars);
                }
                c2 = get_next_token(&mut s2_chars);
            }
            (Some(Token::Char(_)), Some(Token::Number(m))) => {
                if m > 1 {
                    c2 = Some(Token::Number(m - 1));
                } else {
                    c2 = get_next_token(&mut s2_chars);
                }
                c1 = get_next_token(&mut s1_chars);
            }
            (Some(Token::Char(a)), Some(Token::Char(b))) => {
                if a != b {
                    break false;
                } else {
                    c1 = get_next_token(&mut s1_chars);
                    c2 = get_next_token(&mut s2_chars);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_true() {
        assert_eq!(same_string("A2le", "Ap2e"), true);
    }
    #[test]
    fn test_true_no_unknown() {
        assert_eq!(same_string("Apple", "Apple"), true);
    }

    #[test]
    fn test_false() {
        assert_eq!(same_string("Am1le", "Ap2e"), false);
    }
    #[test]
    fn test_false_length() {
        // Strings must have exact same length, we do not allow substring "matches"
        assert_eq!(same_string("A2le3", "A2le2"), false);
    }
    #[test]
    fn test_long_false() {
        assert_eq!(same_string("50000a49999", "50000b49999"), false);
    }
    #[test]
    fn test_long_true() {
        assert_eq!(same_string("49999a50000", "50000b49999"), true);
    }
    #[test]
    fn test_long_true_repeat() {
        assert_eq!(same_string("15a4de3", "14bad3d4"), true);
    }
    #[test]
    fn test_long_true_identical() {
        assert_eq!(same_string("15a5de3", "15a5de3"), true);
    }
    #[test]
    fn test_long_false_repeat() {
        assert_eq!(same_string("15a5de3", "14bcd3d4"), false);
    }
    #[test]
    fn short_test1() {
        assert_eq!(same_string("A2Le", "2pL1"), true);
    }
    #[test]
    fn short_test2() {
        assert_eq!(same_string("a10", "10a"), true);
    }
    #[test]
    fn short_test3() {
        assert_eq!(same_string("ba1", "1Ad"), false);
    }
    #[test]
    fn short_test4() {
        assert_eq!(same_string("3x2x", "8"), false);
    }
}

fn main() {
    println!("Hello, world!");
}
