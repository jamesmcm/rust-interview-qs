// https://leetcode.com/problems/is-graph-bipartite/

use std::collections::HashSet;

struct Solution {}

#[derive(PartialEq)]
enum Colour {
    Red,
    Blue,
}

impl Colour {
    fn opposite(&self) -> Self {
        match self {
            Colour::Red => Colour::Blue,
            Colour::Blue => Colour::Red,
        }
    }
}

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut red: HashSet<i32> = HashSet::new();
        let mut blue: HashSet<i32> = HashSet::new();

        // Iterate through nodes
        // Set children to opposite colour - BFS or DFS
        // If this is impossible (colour already set to other) then not bipartite
        // Otherwise is bipartite
        //
        // For disconnected graph should be sufficient to show that all components are bipartite?
        let mut out = true;
        for (node, _edges) in graph.iter().enumerate() {
            if red.contains(&(node as i32)) || blue.contains(&(node as i32)) {
                continue;
            }

            let inout = Solution::recurse_colour(node as i32, &graph, red, blue, Colour::Red);
            out = out && inout.0;
            red = inout.1;
            blue = inout.2;
        }
        out
    }

    fn recurse_colour(
        node: i32,
        graph: &Vec<Vec<i32>>,
        mut red: HashSet<i32>,
        mut blue: HashSet<i32>,
        colour: Colour,
    ) -> (bool, HashSet<i32>, HashSet<i32>) {
        let (target_set, opposite_set) = match colour {
            Colour::Red => (&mut red, &mut blue),
            Colour::Blue => (&mut blue, &mut red),
        };
        if target_set.contains(&node) {
            return (true, red, blue);
        }
        if opposite_set.contains(&node) {
            return (false, red, blue);
        }
        target_set.insert(node);

        let mut out = true;
        for child in graph[node as usize].iter() {
            let inout = Solution::recurse_colour(*child, graph, red, blue, colour.opposite());
            out = out && inout.0;
            red = inout.1;
            blue = inout.2;
        }
        (out, red, blue)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true() {
        assert_eq!(
            Solution::is_bipartite(vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]]),
            true
        );
    }

    #[test]
    fn test_false() {
        assert_eq!(
            Solution::is_bipartite(vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]]),
            false
        );
    }
}
