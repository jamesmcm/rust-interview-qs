// Copyright (C) 2019 James McMurray
//
// This source code is subject to the terms of the GNU General Public License v3.0.
// You should have received a copy of the GNU General Public License
// along with this source code. If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;


struct Solution {}

impl Solution {

  pub fn roman_to_int(s: String) -> i32 {
    let powers: HashMap<char, i32>  = [('I', 1), ('V', 5), ('X', 10), ('L', 50),
    ('C', 100), ('D', 500), ('M', 1000)].iter().cloned().collect();
    let mut sum: i32 = 0;

    let s: Vec<char> = s.chars().collect();
    for i in 0 .. s.len() {
      let first = s.get(i);
      let second = s.get(i+1);

      match (first, second) {
        (Some(x), Some(y)) if powers[x] >= powers[y] => sum += powers[x],
        (Some(x), Some(y)) if powers[x] < powers[y] => sum -= powers[x],
        (Some(x), None) => sum += powers[x],
         _ => (),
      }
    }
    println!("{:?}", sum);
    sum
  }

}


fn main() {
  Solution::roman_to_int(String::from("MCMXII"));
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_xii() {
        assert_eq!(Solution::roman_to_int(String::from("XII")), 12);
    }
    #[test]
    fn test_ix() {
        assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
    }
    #[test]
    fn test_mcmxii() {
        assert_eq!(Solution::roman_to_int(String::from("MCMXII")), 1912);
    }
    #[test]
    fn test_mcmxci() {
        assert_eq!(Solution::roman_to_int(String::from("MCMXCI")), 1991);
    }
    #[test]
    fn test_il() {
        assert_eq!(Solution::roman_to_int(String::from("IL")), 49);
    }
    #[test]
    fn test_iii() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    }
    #[test]
    fn test_iv() {
        assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
    }
    #[test]
    fn test_lviii() {
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
    }
    #[test]
    fn test_mcmxciv() {
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
