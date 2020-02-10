// https://leetcode.com/problems/pacific-atlantic-water-flow/

#[macro_use]
extern crate maplit;

use std::collections::HashSet;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

struct Solution {}

#[derive(Debug)]
enum Ocean {
    Pacific,
    Atlantic,
}

type Position = (usize, usize);

impl Solution {
    pub fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if matrix.len() == 0 {
           return vec![];
        }
        let s = Solution::pacific_atlantic_inner(matrix);
        s.iter().cloned().map(|x: (usize, usize)| vec![x.0 as i32, x.1 as i32]).collect()
    }

    fn check_matrix(
        matrix_shared: Arc<Vec<Vec<i32>>>,
        thread_set: &mut HashSet<Position>,
        x: &Position,
        to_check: &mut Vec<Position>,
    ) -> () {
        // UP
        if let Some(sub) = (x.0).checked_sub(1) {
            if let Some(row) = matrix_shared.get(sub as usize) {
                if let Some(col) = row.get(x.1 as usize) {
                    if *col >= matrix_shared[x.0 as usize][x.1 as usize]
                        && !thread_set.contains(&(sub, x.1))
                    {
                        to_check.push((sub, x.1));
                    }
                }
            }
        }
        // DOWN
        if let Some(row) = matrix_shared.get((x.0 + 1) as usize) {
            if let Some(col) = row.get(x.1 as usize) {
                if *col >= matrix_shared[x.0 as usize][x.1 as usize]
                    && !thread_set.contains(&(x.0 + 1, x.1))
                {
                    to_check.push((x.0 + 1, x.1));
                }
            }
        }
        // LEFT
        if let Some(row) = matrix_shared.get(x.0 as usize) {
            if let Some(sub) = (x.1).checked_sub(1) {
                if let Some(col) = row.get((sub) as usize) {
                    if *col >= matrix_shared[x.0 as usize][x.1 as usize]
                        && !thread_set.contains(&(x.0, sub))
                    {
                        to_check.push((x.0, sub));
                    }
                }
            }
        }
        // RIGHT
        if let Some(row) = matrix_shared.get(x.0 as usize) {
            if let Some(col) = row.get((x.1 + 1) as usize) {
                if *col >= matrix_shared[x.0 as usize][x.1 as usize]
                    && !thread_set.contains(&(x.0, x.1 + 1))
                {
                    to_check.push((x.0, x.1 + 1));
                }
            }
        }
    }

    fn pacific_atlantic_inner(matrix: Vec<Vec<i32>>) -> HashSet<Position> {
        let (tx, rx) = channel();
        let tx2 = tx.clone();

        let matrix = Arc::new(matrix);
        let matrix_shared1 = matrix.clone();
        // DFS
        let pacific = thread::spawn(move || {
            tx.send({
                let mut thread_set: HashSet<Position> = HashSet::new();
                let mut to_check: Vec<Position> = vec![];
                for y in 0..matrix_shared1.len() {
                    to_check.push((y, 0));
                }
                for x in 1..matrix_shared1[0].len() {
                    to_check.push((0, x));
                }

                while let Some(x) = to_check.pop() {
                    thread_set.insert(x);
                    Solution::check_matrix(
                        matrix_shared1.clone(),
                        &mut thread_set,
                        &x,
                        &mut to_check,
                    );
                }
                // println!("{:?}", (Ocean::Pacific, &thread_set));
                (Ocean::Pacific, thread_set)
            })
            .unwrap();
        });

        let matrix_shared2 = matrix.clone();
        let atlantic = thread::spawn(move || {
            tx2.send({
                let mut thread_set: HashSet<Position> = HashSet::new();
                let mut to_check: Vec<Position> = vec![];
                for y in 0..matrix_shared2.len() {
                    to_check.push((y, matrix_shared2[0].len() - 1));
                }
                for x in 0..matrix_shared2[0].len()-1 {
                    to_check.push((matrix_shared2.len() - 1, x));
                }

                while let Some(x) = to_check.pop() {
                    thread_set.insert(x);
                    Solution::check_matrix(
                        matrix_shared2.clone(),
                        &mut thread_set,
                        &x,
                        &mut to_check,
                    );
                }
                // println!("{:?}", (Ocean::Atlantic, &thread_set));
                (Ocean::Atlantic, thread_set)
            })
            .unwrap();
        });
        let value = rx.recv();
        // println!("{:?}", value);
        let value2 = rx.recv();
        // println!("{:?}", value2);
        (value.unwrap().1)
            .intersection(&(value2.unwrap().1))
            .cloned()
            .collect()
    }
}

fn main() {
    println!("{:?}", Solution::pacific_atlantic(vec![
                vec![1, 2, 2, 3, 5],
                vec![3, 2, 3, 4, 4],
                vec![2, 4, 5, 3, 1],
                vec![6, 7, 1, 4, 5],
                vec![5, 1, 1, 2, 4]
            ]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::pacific_atlantic_inner(vec![
                vec![1, 2, 2, 3, 5],
                vec![3, 2, 3, 4, 4],
                vec![2, 4, 5, 3, 1],
                vec![6, 7, 1, 4, 5],
                vec![5, 1, 1, 2, 4]
            ]),
            hashset! {(0, 4), (1, 3), (1, 4), (2, 2), (3, 0), (3, 1), (4, 0)}
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::pacific_atlantic_inner(vec![
                vec![3, 3, 3],
                vec![3, 1, 3],
                vec![0, 2, 4]
            ]),
            hashset! {(0,0),(0,1),(0,2),(1,0),(1,2),(2,0),(2,1),(2,2)}
        );
    }
}
