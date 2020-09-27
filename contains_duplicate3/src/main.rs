struct Solution {}

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        if nums.len() < 2 {
            return false;
        }
        let mut startindex: usize = 0;
        let mut endindex: usize = (nums.len() - 1).min(startindex + k as usize);
        let mut sorted = nums[startindex..=endindex].to_vec();
        sorted.sort();
        if Self::check_sorted_diff(&sorted, t) {
            return true;
        }

        let mut to_add;
        let mut to_remove = nums[startindex];
        startindex += 1;
        endindex += 1;
        while endindex < nums.len() {
            to_add = nums[endindex];
            Self::update_sorted(&mut sorted, to_remove, to_add);
            if Self::check_sorted_diff(&sorted, t) {
                return true;
            }
            to_remove = nums[startindex];
            startindex += 1;
            endindex += 1;
        }

        false
    }

    fn check_sorted_diff(sorted: &[i32], t: i32) -> bool {
        // TODO: Avoud 64-bit case, can use unsafe?
        for window in sorted.windows(2) {
            if window[1] as i64 - window[0] as i64 <= t as i64 {
                return true;
            }
        }
        false
    }

    fn update_sorted(sorted: &mut Vec<i32>, to_remove: i32, to_add: i32) {
        // TODO: Remove double iteration here (can return enum of found type from closure?)
        let remove_index = sorted
            .iter()
            .enumerate()
            .find(|x| *x.1 == to_remove)
            .unwrap()
            .0;
        sorted.remove(remove_index);

        let add_index = sorted
            .iter()
            .enumerate()
            .find(|x| *x.1 >= to_add)
            .unwrap_or((sorted.len(), &(to_add - 1)))
            .0;
        sorted.insert(add_index, to_add);
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
        assert!(Solution::contains_nearby_almost_duplicate(
            vec![1, 2, 3, 1],
            3,
            0
        ));
    }
    #[test]
    fn test2() {
        assert!(Solution::contains_nearby_almost_duplicate(
            vec![1, 0, 1, 1],
            1,
            2
        ));
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3),
            false
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![-2147483648, 2147483647], 1, 1),
            false
        );
    }
    #[test]
    fn test5() {
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![2147483646, 2147483647], 3, 3),
            true
        );
    }
}
