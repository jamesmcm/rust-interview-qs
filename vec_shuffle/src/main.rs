/*
 *
 * Transpose array so rows become columns and vice versa
a = [ [1,2,3], [4,5,6], [7,8,9] ]

output = [ [1,4,7], [2,5,8], [3,6,9] ]
*/

struct Solution {}

impl Solution {
    fn transpose_clone<T: Clone>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
        (0..input[0].len())
            .map(|f| {
                input
                    .iter()
                    .cloned()
                    .flat_map(|x| {
                        x.iter()
                            .cloned()
                            .enumerate()
                            .filter(|y| y.0 == f)
                            .map(|z| z.1)
                            .collect::<Vec<T>>()
                    })
                    .collect::<Vec<T>>()
            })
            .collect()
    }

    fn transpose_noclone<T>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
        let l: usize = input[0].len();
        let mut output: Vec<Vec<T>> = Vec::new();
        for _i in 0..l {
            output.push(Vec::new());
        }

        input
            .into_iter()
            .for_each(|mut x| x.drain(..).enumerate().for_each(|y| output[y.0].push(y.1)));
        output
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        assert_eq!(
            Solution::transpose_clone(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        );
    }

    #[test]
    fn test_noclone() {
        assert_eq!(
            Solution::transpose_noclone(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        );
    }
}
