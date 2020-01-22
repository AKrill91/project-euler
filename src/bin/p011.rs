extern crate common;
extern crate env_logger;
#[macro_use]
extern crate log;

use std::time::Instant;

#[derive(Copy, Clone)]
enum Direction {
    Horizontal,
    Vertical,
    DiagonalRight,
    DiagonalLeft
}

fn main() {
    env_logger::init();

    let start = Instant::now();

    let grid = vec!(
        vec!(08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08),
        vec!(49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00),
        vec!(81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65),
        vec!(52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91),
        vec!(22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80),
        vec!(24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50),
        vec!(32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70),
        vec!(67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21),
        vec!(24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72),
        vec!(21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95),
        vec!(78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92),
        vec!(16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57),
        vec!(86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58),
        vec!(19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40),
        vec!(04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66),
        vec!(88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69),
        vec!(04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36),
        vec!(20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16),
        vec!(20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54),
        vec!(01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48),
    );

    let result = compute(&grid, 4);

    info!("Result is {}", result);

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}

fn compute(grid: &Vec<Vec<u64>>, count: usize) -> u64 {
    let height = grid.len();
    let width = grid.first().unwrap().len();

    (0..=height - 1)
        .flat_map(
            move |y| {
                (0..=width - 1)
                    .flat_map(
                        move |x| [Direction::Horizontal, Direction::Vertical, Direction::DiagonalRight, Direction::DiagonalLeft]
                            .iter()
                            .map(move |d| get_points(x, y, count, *d))
                    )
            }
        )
        .map(
            |points| points.iter()
                .map(|&point| get_at_position(grid, point))
                .filter(|n| n.is_some())
                .map(|n| n.unwrap())
                .product()
        )
        .max()
        .unwrap()
}

fn get_points(x: usize, y: usize, count: usize, direction: Direction) -> Vec<(usize, usize)> {
    let offset: (i32, i32) = match direction {
        Direction::Horizontal => (1, 0),
        Direction::Vertical => (0, 1),
        Direction::DiagonalRight => (1, 1),
        Direction::DiagonalLeft => (-1, 1)
    };

    (0..count as i32).map(|n| (x as i32 + offset.0 * n, y as i32 + offset.1 * n))
        .filter(|point| point.0 >= 0 && point.1 >= 0)
        .map(|point| (point.0 as usize, point.1 as usize))
        .collect()
}

fn get_at_position(grid: &Vec<Vec<u64>>, position: (usize, usize)) -> Option<u64> {
    grid.get(position.0)
        .unwrap_or(&vec!())
        .get(position.1)
        .copied()
}

#[cfg(test)]
mod tests {
    #[test]
    fn small_grid_horizontal() {
        let grid = vec!(
            vec!(1, 2, 3),
            vec!(4, 5, 6),
            vec!(7, 8, 9)
        );

        assert_eq!(504, super::compute(&grid, 3));
    }

    #[test]
    fn small_grid_vertical() {
        let grid = vec!(
            vec!(8, 2, 3),
            vec!(9, 5, 6),
            vec!(7, 1, 4)
        );

        assert_eq!(504, super::compute(&grid, 3));
    }

    #[test]
    fn small_grid_diagonal() {
        let grid = vec!(
            vec!(7, 2, 3),
            vec!(4, 8, 6),
            vec!(1, 5, 9)
        );

        assert_eq!(504, super::compute(&grid, 3));
    }

    #[test]
    fn medium_grid_diagonal() {
        let grid = vec!(
            vec!(21, 2, 3, 4, 5),
            vec!(6, 22, 8, 9, 10),
            vec!(11, 12, 23, 14, 15),
            vec!(16, 17, 18, 24, 20),
            vec!(1, 7, 13, 19, 25)
        );

        assert_eq!(303_600, super::compute(&grid, 4));
    }

    #[test]
    fn medium_grid_diagonal_left() {
        let grid = vec!(
            vec!(1, 2, 3, 4, 25),
            vec!(6, 7, 8, 24, 10),
            vec!(11, 12, 23, 14, 15),
            vec!(16, 22, 18, 19, 20),
            vec!(21, 17, 13, 9, 5)
        );

        assert_eq!(303_600, super::compute(&grid, 4));
    }
}