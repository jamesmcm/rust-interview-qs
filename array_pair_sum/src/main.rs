/*
 * Given a list of integers, return indices of the two numbers such that they add up to a specific target.
 * You may assume that each input would have exactly one solution, and you may not use the same element twice.
 * Example:
 * Given nums = [2, 7, 11, 15], target = 9,
 *
 * Because nums[0] + nums[1] = 2 + 7 = 9,
*/

// Extensions:
// If we have repeat values we should decrement our indices over all values and not repeatedly test
// the same ones
// What if we wanted all possible pairs if many are possible?
// What if we allow negative numbers - initial binary search for high_index no longer works, would
// need to start from end of array

struct Solution {}

impl Solution {
    fn find_pairs(arr: &[i32], target: i32) -> Option<(usize, usize)> {
        let mut low_index: usize = 0;
        let mut high_index: usize = Solution::binary_search(&arr, target);

        while low_index < high_index {
            if arr[low_index] + arr[high_index] == target {
                return Some((low_index, high_index));
            } else if arr[low_index] + arr[high_index] > target {
                // Decrement high_index
                if high_index > 0 {
                    high_index -= 1;
                } else {
                    return None;
                }
            } else {
                // Increment low_index
                low_index += 1;
            }
        }
        None
    }

    fn binary_search(arr: &[i32], target: i32) -> usize {
        // Recurse with half-slices
        let midpoint: usize = arr.len() / 2;

        if arr.len() <= 1 {
            return 0;
        }

        match arr[midpoint] {
            x if x < target => {
                midpoint + Solution::binary_search(&arr[midpoint..arr.len()], target)
            }
            x if x > target => Solution::binary_search(&arr[0..midpoint], target),
            x if x == target => midpoint,
            _ => midpoint,
        }
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_pairs(&(vec![2 as i32, 7, 11, 15])[..], 9)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bs1() {
        assert_eq!(
            Solution::binary_search(&(vec![1 as i32, 2, 3, 4, 5])[..], 4),
            3
        );
    }

    #[test]
    fn test_bs2() {
        assert_eq!(
            Solution::binary_search(&(vec![1 as i32, 2, 3, 4, 5])[..], 1),
            0
        );
    }
    #[test]
    fn test_bs3() {
        assert_eq!(Solution::binary_search(&(vec![1 as i32, 2])[..], 2), 1);
    }
    #[test]
    fn test_bs4() {
        assert_eq!(Solution::binary_search(&(vec![1 as i32])[..], 1), 0);
    }
    #[test]
    fn test_bs5() {
        assert_eq!(Solution::binary_search(&(vec![1 as i32])[..], 5), 0);
    }
    #[test]
    fn test_bs6() {
        assert_eq!(Solution::binary_search(&(vec![1 as i32, 2])[..], 5), 1);
    }
    #[test]
    fn test_bs7() {
        assert_eq!(Solution::binary_search(&(vec![1 as i32, 2])[..], 0), 0);
    }

    #[test]
    fn test_pairs1() {
        assert_eq!(
            Solution::find_pairs(&(vec![2 as i32, 7, 11, 15])[..], 9),
            Some((0, 1))
        );
    }
    #[test]
    fn test_pairs2() {
        assert_eq!(
            Solution::find_pairs(&(vec![2 as i32, 7, 11, 15])[..], 10),
            None
        );
    }
}
