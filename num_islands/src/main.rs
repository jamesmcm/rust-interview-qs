// Copyright (C) 2019 James McMurray
//
// This source code is subject to the terms of the GNU General Public License v3.0.
// You should have received a copy of the GNU General Public License
// along with this source code. If not, see <http://www.gnu.org/licenses/>.

// https://leetcode.com/problems/number-of-islands/
// Recursive solution

use std::collections::HashSet;
struct Solution();

const LAND: char = '1';

type Position = (usize, usize);
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut seen = HashSet::<Position>::new();
        let mut num_i: i32 = 0;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == LAND && !seen.contains(&(y, x)) {
                    num_i = num_i + 1;
                    Solution::flood_fill((y, x), &grid, &mut seen);
                }
            }
        }
        num_i
    }

    fn flood_fill(pos: Position, grid: &Vec<Vec<char>>, seen: &mut HashSet<Position>) -> () {
        seen.insert(pos);

        // Left
        if !seen.contains(&(pos.0, pos.1.overflowing_sub(1).0)) {
            match grid[pos.0].get(pos.1.overflowing_sub(1).0) {
                Some(x) if *x == LAND => {
                    Solution::flood_fill((pos.0, pos.1.overflowing_sub(1).0), &grid, seen);
                }
                _ => (),
            }
        }
        // Right
        if !seen.contains(&(pos.0, pos.1 + 1)) {
            match grid[pos.0].get(pos.1 + 1) {
                Some(x) if *x == LAND => {
                    Solution::flood_fill((pos.0, pos.1 + 1), &grid, seen);
                }
                _ => (),
            }
        }
        // Up
        if !seen.contains(&(pos.0.overflowing_sub(1).0, pos.1)) {
            match grid.get(pos.0.overflowing_sub(1).0) {
                Some(x) if x[pos.1] == LAND => {
                    Solution::flood_fill((pos.0.overflowing_sub(1).0, pos.1), &grid, seen);
                }
                _ => (),
            }
        }
        // Down
        if !seen.contains(&(pos.0 + 1, pos.1)) {
            match grid.get(pos.0 + 1) {
                Some(x) if x[pos.1] == LAND => {
                    Solution::flood_fill((pos.0 + 1, pos.1), &grid, seen);
                }
                _ => (),
            }
        }
    }
}

fn main() {
    let grid: Vec<Vec<char>> = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    println!("{}", Solution::num_islands(grid));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_islands1() {
        let grid: Vec<Vec<char>> = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn test_islands2() {
        let grid: Vec<Vec<char>> = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(grid), 3);
    }

    #[test]
    fn test_flood_fill1() {
        let mut seen = HashSet::<Position>::new();
        let grid: Vec<Vec<char>> = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        Solution::flood_fill((0, 0), &grid, &mut seen);
        assert!(
            seen.contains(&(0, 0))
                && seen.contains(&(0, 1))
                && seen.contains(&(0, 2))
                && seen.contains(&(0, 3))
                && seen.contains(&(1, 0))
                && seen.contains(&(1, 1))
                && seen.contains(&(1, 3))
                && seen.contains(&(2, 0))
                && seen.contains(&(2, 1))
                && !seen.contains(&(0, 4))
        );
    }
}
