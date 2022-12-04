use std::str::FromStr;

const INPUT: &str = "A Y
B X
C Z";

fn main() {}

fn calculate_score(input: &[[Shape; 2]]) -> u32 {
    input
        .iter()
        .map(|&[shape1, shape2]| shape1.vs(shape2) + shape2 as u32)
        .sum()
}

fn parse(input: &str) -> Vec<[Shape; 2]> {
    input
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<Shape>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

fn calculate_shape_and_score(input: &[(Shape, Output)]) -> u32 {
    input
        .iter()
        .map(|&(shape, output)| output as u32 + output.shape(shape) as u32)
        .sum()
}

fn parse_output(input: &str) -> Vec<(Shape, Output)> {
    input
        .lines()
        .map(|s| {
            let mut iter = s.split_whitespace();

            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Output {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl Output {
    fn shape(self, shape: Shape) -> Shape {
        match (shape, self) {
            (Shape::Rock, Output::Lose) | (Shape::Paper, Output::Win) => Shape::Scissors,
            (Shape::Rock, Output::Win) | (Shape::Scissors, Output::Lose) => Shape::Paper,
            (Shape::Paper, Output::Lose) | (Shape::Scissors, Output::Win) => Shape::Rock,
            (_, Output::Draw) => shape,
        }
    }
}

impl FromStr for Output {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Output::Lose),
            "Y" => Ok(Output::Draw),
            "Z" => Ok(Output::Win),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn vs(self, other: Shape) -> u32 {
        match (self, other) {
            (Shape::Rock, Shape::Paper)
            | (Shape::Paper, Shape::Scissors)
            | (Shape::Scissors, Shape::Rock) => 6,
            (Shape::Rock, Shape::Rock)
            | (Shape::Paper, Shape::Paper)
            | (Shape::Scissors, Shape::Scissors) => 3,
            _ => 0,
        }
    }
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(()),
        }
    }
}
