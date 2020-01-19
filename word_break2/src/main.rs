// https://leetcode.com/problems/word-break-ii/
//TODO: Spawns Too many theads:
//"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
//["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"]

#[macro_use]
extern crate maplit;

use std::collections::HashSet;
use std::iter::FromIterator;
use std::sync::Arc;
use std::thread;

struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let word_set: HashSet<String> = HashSet::from_iter(word_dict);
        Solution::word_break_inner(s, Arc::new(word_set), Vec::new())
            .iter()
            .map(|x| String::from(x.join(" ")))
            .collect()
    }

    pub fn word_break_inner(
        cs: String,
        word_dict: Arc<HashSet<String>>,
        mut found_words: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut teststring: String = String::new();
        let chars_remaining: usize = cs.len();
        let mut my_threads: Vec<
            thread::JoinHandle<std::vec::Vec<std::vec::Vec<std::string::String>>>,
        > = Vec::new();
        let mut output: Vec<Vec<String>> = Vec::new();

        for (i, c) in cs.chars().enumerate() {
            teststring.push(c);
            if i < chars_remaining - 1 {
                if word_dict.contains(&teststring) {
                    let newstr: String = cs[i + 1..cs.len()].into();
                    let mut new_fw: Vec<String> = found_words.clone();
                    let shared_wd = word_dict.clone();
                    new_fw.push(teststring.clone());

                    my_threads.push(thread::spawn(move || {
                        Solution::word_break_inner(newstr, shared_wd, new_fw)
                    }));
                }
            } else {
                if word_dict.contains(&teststring) {
                    for thread in my_threads {
                        for v in thread.join().unwrap() {
                            output.push(v);
                        }
                    }
                    found_words.push(teststring);
                    output.push(found_words);
                    return output;
                }
            }
        }
        for thread in my_threads {
            for v in thread.join().unwrap() {
                output.push(v);
            }
        }
        output
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::word_break(String::from("test"), vec![String::from("test")])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_word() {
        assert_eq!(
            Solution::word_break(String::from("test"), vec![String::from("test")]),
            vec![String::from("test")]
        );
    }

    #[test]
    fn test1() {
        let vec_result = Solution::word_break(
            String::from("catsanddog"),
            vec![
                String::from("cat"),
                String::from("cats"),
                String::from("and"),
                String::from("sand"),
                String::from("dog"),
            ],
        );

        assert_eq!(
            HashSet::from_iter(vec_result),
            hashset![String::from("cats and dog"), String::from("cat sand dog")]
        );
    }

    #[test]
    fn test2() {
        let vec_result = Solution::word_break(
            String::from("pineapplepenapple"),
            vec![
                String::from("apple"),
                String::from("pen"),
                String::from("applepen"),
                String::from("pine"),
                String::from("pineapple"),
            ],
        );

        assert_eq!(
            HashSet::from_iter(vec_result),
            hashset![String::from("pine apple pen apple"), String::from("pineapple pen apple"), String::from("pine applepen apple")]
        );
    }
    #[test]
    fn test3() {
        let vec_result = Solution::word_break(
            String::from("catsandog"),
            vec![
                String::from("cat"),
                String::from("cats"),
                String::from("and"),
                String::from("sand"),
                String::from("dog"),
            ],
        );

        assert_eq!(
            HashSet::from_iter(vec_result),
            hashset![]
        );
    }

    #[test]
    fn empty() {
        let vec_result = Solution::word_break(
            String::from(""),
            vec![
            ],
        );

        assert_eq!(
            HashSet::from_iter(vec_result),
            hashset![]
        );
    }
}

