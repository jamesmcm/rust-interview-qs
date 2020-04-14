// https://leetcode.com/problems/verifying-an-alien-dictionary/

use std::mem::MaybeUninit;
struct Solution {}

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let order_arr = Solution::get_order_arr(order);

        words
            .as_slice()
            .windows(2)
            .map(|x| Solution::check_pair(x[0].as_bytes(), x[1].as_bytes(), &order_arr))
            .fold(true, |acc, x| acc && x)
    }

    fn get_order_arr(s: String) -> [u8; 26] {
        let mut order_arr: [MaybeUninit<u8>; 26] = unsafe { MaybeUninit::uninit().assume_init() }; //[0; 26];
        for (i, byte) in s.as_bytes().iter().enumerate() {
            // Could skip check here
            unsafe {
                *order_arr.get_unchecked_mut((*byte - 97) as usize) = MaybeUninit::new(i as u8);
            }
        }

        unsafe { std::mem::transmute::<_, [u8; 26]>(order_arr) }

        // order_arr
    }
    fn check_pair(word1: &[u8], word2: &[u8], order: &[u8]) -> bool {
        for (i, c1) in word1.iter().enumerate() {
            let c2: u8;
            if let Some(x) = word2.get(i) {
                c2 = *x - 97;
            } else {
                return false;
            }

            if order[(*c1 - 97) as usize] < order[c2 as usize] {
                return true;
            }
            if order[(*c1 - 97) as usize] > order[c2 as usize] {
                return false;
            }
        }
        true
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
        assert!(Solution::is_alien_sorted(
            vec![String::from("hello"), String::from("leetcode")],
            String::from("hlabcdefgijkmnopqrstuvwxyz")
        ))
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::is_alien_sorted(
                vec![
                    String::from("words"),
                    String::from("world"),
                    String::from("row")
                ],
                String::from("worldabcefghijkmnpqstuvxyz")
            ),
            false
        )
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::is_alien_sorted(
                vec![String::from("apple"), String::from("app")],
                String::from("abcdefghijklmnopqrstuvwxyz")
            ),
            false
        )
    }
    #[test]
    fn test2_sub() {
        assert_eq!(
            Solution::check_pair(
                String::from("words").as_bytes(),
                String::from("world").as_bytes(),
                &Solution::get_order_arr(String::from("worldabcefghijkmnpqstuvxyz"))
            ),
            false
        )
    }
}
