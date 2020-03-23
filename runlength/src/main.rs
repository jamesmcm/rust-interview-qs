// https://leetcode.com/problems/string-compression

struct Solution {}

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut ptr = chars.as_mut_ptr();
        let mut vec_length = chars.len();

        let mut cur_char: Option<char> = None;
        let mut cur_count: u32 = 0;
        let mut new_length: usize = 0;

        for i in 0..vec_length {
            unsafe {
                match cur_char {
                    None => {
                        cur_char = Some(*ptr);
                        cur_count = 1;
                    }
                    Some(c) if c == *(ptr.offset(i as isize)) => {
                        cur_count += 1;
                    }
                    Some(c) if c != *(ptr.offset(i as isize)) => {
                        // Write

                        *(ptr.offset(new_length as isize)) = cur_char.unwrap();
                        cur_char = Some(*(ptr.offset(i as isize)));
                        new_length += 1;

                        if cur_count > 1 {
                            // Write cur_count
                            // *(ptr + new_length + 1) = cur_count;

                            let mut num_digits: usize = 0;

                            for (ix, power) in [1000, 100, 10, 1].iter().enumerate() {
                                if cur_count / power > 0 {
                                    *(ptr.offset((new_length + num_digits) as isize)) =
                                        std::char::from_digit(cur_count / power, 10).unwrap();
                                    num_digits += 1;
                                    cur_count = cur_count % power;
                                    if cur_count == 0 {
                                        for _j in 0..(3 - ix) {
                                            *(ptr.offset((new_length + num_digits) as isize)) = '0';
                                            num_digits += 1;
                                        }
                                        break;
                                    }
                                }
                            }
                            new_length += num_digits;
                        }

                        cur_count = 1;
                    }
                    _ => {
                        panic!("bad case");
                    }
                }
            }
        }

        // Write last piece
        unsafe {
            *(ptr.offset(new_length as isize)) = cur_char.unwrap();
            new_length += 1;

            if cur_count > 1 {
                // Write cur_count
                // *(ptr + new_length + 1) = cur_count;

                let mut num_digits: usize = 0;

                for (ix, power) in [1000, 100, 10, 1].iter().enumerate() {
                    if cur_count / power > 0 {
                        *(ptr.offset((new_length + num_digits) as isize)) =
                            std::char::from_digit(cur_count / power, 10).unwrap();
                        num_digits += 1;
                        cur_count = cur_count % power;
                        if cur_count == 0 {
                            for _j in 0..(3 - ix) {
                                *(ptr.offset((new_length + num_digits) as isize)) = '0';
                                num_digits += 1;
                            }
                            break;
                        }
                    }
                }
                new_length += num_digits;
            }
        }

        (new_length) as i32
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
    fn test1() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        let out = Solution::compress(&mut chars);
        assert_eq!(out, 6);
        assert_eq!(&chars[0..6], &['a', '2', 'b', '2', 'c', '3']);
    }

    #[test]
    fn test2() {
        let mut chars = vec!['a'];
        let out = Solution::compress(&mut chars);
        assert_eq!(out, 1);
        assert_eq!(&chars[0..1], &['a']);
    }

    #[test]
    fn test3() {
        let mut chars = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        let out = Solution::compress(&mut chars);
        assert_eq!(out, 4);
        assert_eq!(&chars[0..4], &['a', 'b', '1', '2']);
    }
}
