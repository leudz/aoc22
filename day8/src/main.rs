use std::{cmp::Ordering, ops::ControlFlow};

const INPUT: &str = "30373
25512
65332
33549
35390";

fn main() {}

struct Grid(Vec<Vec<u32>>);

impl Grid {
    fn visible_count(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter().enumerate().filter(move |&(x, height)| {
                    x == 0
                        || x == self.max_x()
                        || y == 0
                        || y == self.max_y()
                        || (0..x).all(|x2| self.get(x2, y) < *height)
                        || (x + 1..self.max_x() + 1).all(|x2| self.get(x2, y) < *height)
                        || (0..y).all(|y2| self.get(x, y2) < *height)
                        || (y + 1..self.max_y() + 1).all(|y2| self.get(x, y2) < *height)
                })
            })
            .count()
    }

    fn highest_scenic_score(&self) -> u32 {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter().enumerate().map(move |(x, &height)| {
                    let view_distance_x_left = match (0..x).try_rfold(0, |view_distance, x2| {
                        match self.get(x2, y).cmp(&height) {
                            Ordering::Less => ControlFlow::Continue(view_distance + 1),
                            Ordering::Equal => ControlFlow::Break(view_distance + 1),
                            Ordering::Greater => ControlFlow::Break(view_distance + 1),
                        }
                    }) {
                        ControlFlow::Continue(view_distance)
                        | ControlFlow::Break(view_distance) => view_distance,
                    };
                    let view_distance_x_right =
                        match (x..self.max_x() + 1)
                            .skip(1)
                            .try_fold(0, |view_distance, x2| match self.get(x2, y).cmp(&height) {
                                Ordering::Less => ControlFlow::Continue(view_distance + 1),
                                Ordering::Equal => ControlFlow::Break(view_distance + 1),
                                Ordering::Greater => ControlFlow::Break(view_distance + 1),
                            }) {
                            ControlFlow::Continue(view_distance)
                            | ControlFlow::Break(view_distance) => view_distance,
                        };

                    let view_distance_y_left = match (0..y).try_rfold(0, |view_distance, y2| {
                        match self.get(x, y2).cmp(&height) {
                            Ordering::Less => ControlFlow::Continue(view_distance + 1),
                            Ordering::Equal => ControlFlow::Break(view_distance + 1),
                            Ordering::Greater => ControlFlow::Break(view_distance + 1),
                        }
                    }) {
                        ControlFlow::Continue(view_distance)
                        | ControlFlow::Break(view_distance) => view_distance,
                    };
                    let view_distance_y_right =
                        match (y..self.max_y() + 1)
                            .skip(1)
                            .try_fold(0, |view_distance, y2| match self.get(x, y2).cmp(&height) {
                                Ordering::Less => ControlFlow::Continue(view_distance + 1),
                                Ordering::Equal => ControlFlow::Break(view_distance + 1),
                                Ordering::Greater => ControlFlow::Break(view_distance + 1),
                            }) {
                            ControlFlow::Continue(view_distance)
                            | ControlFlow::Break(view_distance) => view_distance,
                        };

                    view_distance_x_left
                        * view_distance_x_right
                        * view_distance_y_left
                        * view_distance_y_right
                })
            })
            .max()
            .unwrap()
    }

    fn get(&self, x: usize, y: usize) -> u32 {
        self.0[y][x]
    }

    fn max_x(&self) -> usize {
        self.0[0].len() - 1
    }

    fn max_y(&self) -> usize {
        self.0.len() - 1
    }
}

fn parse(input: &str) -> Grid {
    Grid(
        input
            .lines()
            .map(|line| line.chars().map(|c| c as u32).collect::<Vec<_>>())
            .collect(),
    )
}
