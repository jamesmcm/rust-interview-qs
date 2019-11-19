// Copyright (C) 2019 James McMurray
//
// This source code is subject to the terms of the GNU General Public License v3.0.
// You should have received a copy of the GNU General Public License
// along with this source code. If not, see <http://www.gnu.org/licenses/>.

// Recursive solution
struct Solution();

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let mut copy: Vec<Vec<i32>> = image;
        Solution::flood_fill_recurse(
            &mut copy,
            sr as usize,
            sc as usize,
            new_color,
        );
        copy
    }

    fn flood_fill_recurse(mut m: &mut Vec<Vec<i32>>, sr: usize, sc: usize, new_color: i32) {
        let origval: i32 = m[sr][sc];
        if new_color == origval {
            return;
        }
        m[sr][sc] = new_color;

        // Left
        match m[sr].get(sc.overflowing_sub(1).0) {
            Some(x) if *x == origval => {
                Solution::flood_fill_recurse(&mut m, sr, sc.overflowing_sub(1).0, new_color);
            }
            _ => (),
        }
        // Right
        match m[sr].get(sc + 1) {
            Some(x) if *x == origval => {
                Solution::flood_fill_recurse(&mut m, sr, sc + 1, new_color);
            }
            _ => (),
        }
        // Up
        match m.get(sr.overflowing_sub(1).0) {
            Some(x) if x[sc] == origval => {
                Solution::flood_fill_recurse(&mut m, sr.overflowing_sub(1).0, sc, new_color);
            }
            _ => (),
        }
        // Down
        match m.get(sr + 1) {
            Some(x) if x[sc] == origval => {
                Solution::flood_fill_recurse(&mut m, sr + 1, sc, new_color);
            }
            _ => (),
        }
    }
}

fn main() {
    let m: Vec<Vec<i32>> = vec![
        vec![0, 0, 1, 0],
        vec![0, 0, 1, 1],
        vec![0, 0, 1, 1],
        vec![0, 0, 0, 0],
    ];
    let out = Solution::flood_fill(m, 0, 0, 2);
    println!("{:#?}", out);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_leetcode_flood() {
        let m: Vec<Vec<i32>> = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let result: Vec<Vec<i32>> = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        let out = Solution::flood_fill(m, 0, 0, 2);
        assert_eq!(out, result);
    }

    #[test]
    fn test_flood1() {
        let m: Vec<Vec<i32>> = vec![
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 1],
            vec![0, 0, 1, 1],
            vec![0, 0, 0, 0],
        ];
        let result: Vec<Vec<i32>> = vec![
            vec![2, 2, 1, 0],
            vec![2, 2, 1, 1],
            vec![2, 2, 1, 1],
            vec![2, 2, 2, 2],
        ];
        let out = Solution::flood_fill(m, 0, 0, 2);
        assert_eq!(out, result);
    }

    #[test]
    fn test_flood2() {
        let m: Vec<Vec<i32>> = vec![
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 1],
            vec![0, 0, 1, 1],
            vec![0, 0, 0, 0],
        ];
        let result: Vec<Vec<i32>> = vec![
            vec![0, 0, 1, 2],
            vec![0, 0, 1, 1],
            vec![0, 0, 1, 1],
            vec![0, 0, 0, 0],
        ];
        let out = Solution::flood_fill(m, 0, 3, 2);
        assert_eq!(out, result);
    }

    #[test]
    fn test_flood3() {
        let m: Vec<Vec<i32>> = vec![
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 1],
            vec![0, 0, 1, 1],
            vec![0, 0, 0, 0],
        ];
        let result: Vec<Vec<i32>> = vec![
            vec![0, 0, 3, 0],
            vec![0, 0, 3, 3],
            vec![0, 0, 3, 3],
            vec![0, 0, 0, 0],
        ];
        let out = Solution::flood_fill(m, 2, 3, 3);
        assert_eq!(out, result);
    }
    #[test]
    fn test_flood4() {
        let m: Vec<Vec<i32>> = vec![
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 1],
            vec![0, 0, 1, 1],
            vec![0, 0, 0, 0],
        ];
        let result: Vec<Vec<i32>> = vec![
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 1],
            vec![0, 0, 1, 1],
            vec![0, 0, 0, 0],
        ];
        let out = Solution::flood_fill(m, 0, 0, 0);
        assert_eq!(out, result);
    }
}
