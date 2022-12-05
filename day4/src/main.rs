use std::ops::RangeInclusive;

const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

fn main() {}

fn parse(input: &str) -> impl Iterator<Item = (RangeInclusive<u32>, RangeInclusive<u32>)> + '_ {
    input
        .lines()
        .map(|pair| pair.split_once(',').unwrap())
        .map(|(first, second)| {
            let (f_start, f_end) = first.split_once('-').unwrap();
            let (s_start, s_end) = second.split_once('-').unwrap();

            (
                f_start.parse().unwrap()..=f_end.parse().unwrap(),
                s_start.parse().unwrap()..=s_end.parse().unwrap(),
            )
        })
}

fn pair_fully_contained_count(input: &str) -> usize {
    parse(input)
        .filter(|(first, second)| {
            (first.contains(&second.start()) && first.contains(&second.end()))
                || (second.contains(&first.start()) && second.contains(&first.end()))
        })
        .count()
}

fn pair_partially_contained_count(input: &str) -> usize {
    parse(input)
        .filter(|(first, second)| {
            first.contains(&second.start())
                || first.contains(&second.end())
                || second.contains(&first.start())
                || second.contains(&first.end())
        })
        .count()
}
