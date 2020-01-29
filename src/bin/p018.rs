extern crate common;
extern crate env_logger;
#[macro_use]
extern crate log;

use std::time::Instant;
use std::collections::HashMap;
use std::cmp::max;

type Tree = HashMap<(u32, u32), u32>;

trait SummableTree {
    fn max(&self) -> u32;
    fn max_from_position(&self, position: (u32, u32)) -> u32;
}

impl SummableTree for Tree {
    fn max(&self) -> u32 {
        self.max_from_position((0,0))
    }

    fn max_from_position(&self, position: (u32, u32)) -> u32 {
        let mut sum = *self.get(&position).unwrap();

        let left_position = (position.0, position.1 + 1);
        let right_position = (position.0 + 1, position.1 + 1);

        let left = self.get(&left_position);
        let right = self.get(&right_position);

        let left_max = if left.is_some() {
            self.max_from_position(left_position)
        } else {
            0
        };

        let right_max = if right.is_some() {
            self.max_from_position(right_position)
        } else {
            0
        };

        sum += max(left_max, right_max);

        sum
    }
}

fn main() {
    env_logger::init();

    let start = Instant::now();

    let input = "75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

    let result = compute(input);

    info!("Result is {}", result);

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}

fn parse_tree(input: &str) -> Tree {
    let mut out = HashMap::new();
    let mut row = 0;

    for line in input.lines() {
        let mut column = 0;

        for val in line.split_whitespace() {
            let position = (column, row);
            out.insert(position, val.parse::<u32>().unwrap());
            column += 1;
        }
        row += 1;
    }

    out
}

fn compute(input: &str) -> u32 {
    let tree = parse_tree(input);
    tree.max()
}

#[cfg(test)]
mod tests {
    use crate::parse_tree;

    #[test]
    fn parse_tree_small() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = "3
7 4
2 4 6
8 5 9 3";

        let tree = parse_tree(input);

        info!("{:?}", tree);

        assert_eq!(3, *tree.get(&(3,3)).unwrap());
    }

    #[test]
    fn sum_tree_small() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = "3
7 4
2 4 6
8 5 9 3";

        assert_eq!(23, super::compute(input));
    }

    #[test]
    fn sum_tree_small_flipped() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = "3
4 7
6 4 2
3 9 5 8";

        assert_eq!(23, super::compute(input));
    }
}