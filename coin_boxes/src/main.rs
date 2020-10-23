// https://www.reddit.com/r/rust/comments/jeu7xj/faang_interview_question_in_rust/

// Given an MxN array of “boxes”, where each box contains some number of coins C[i][j],
// you want to maximize the number of coins you can take. You take coins by traversing
// row by row, taking all of the coins from ONE box in each row. However, any time you
// change the index of the box you take coins from, you must pay a “change fee” equal
// to ABS(x - y) where x and y are the previous and new row indices. Write a function
// that can determine the optimal set of boxes to take coins from in order to maximize
// your profit after change fees

// O(N*M*M) for N rows, M columns

pub fn coin_problem(mut input: Vec<Vec<i32>>) -> (i32, Vec<usize>) {
    let rows = input.len();
    let cols = input[0].len();
    let mut paths: Vec<Vec<usize>> = (0..cols).map(|x| vec![x]).collect();
    let mut best_elem: usize = 10000;

    for i in 1..rows {
        let mut new_paths: Vec<Vec<usize>> = Vec::with_capacity(cols);
        for j in 0..cols {
            let mut best_score = -100_000;
            for k in 0..cols {
                let cur_score = input[i][j] + input[i - 1][k] - (k as i32 - j as i32).abs();
                if cur_score > best_score {
                    best_score = cur_score;
                    best_elem = k;
                }
            }
            input[i][j] = best_score;
            // TODO: This allocation is bad, could instead use MxN grid of best path taken
            // And then reconstruct at the end
            let mut new_path = paths[best_elem].clone();
            new_path.push(j);
            new_paths.push(new_path);
        }
        paths = new_paths;
    }

    let (best_elem, score) = input
        .swap_remove(rows - 1)
        .into_iter()
        .enumerate()
        .max_by(|&(_ind1, val1), &(_ind2, val2)| val1.cmp(&val2))
        .unwrap();
    (score, paths.swap_remove(best_elem))
}

fn main() {
    println!(
        "{:?}",
        coin_problem(vec![vec![0, 0, 10, 0], vec![0, 2, 0, 3], vec![5, 0, 0, 0]])
    );

    println!(
        "{:?}",
        coin_problem(vec![
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 2],
            vec![1, 1, 1, 1, 1, 1, 1000],
        ])
    );
}
