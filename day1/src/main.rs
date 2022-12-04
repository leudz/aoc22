const INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

fn main() {}

fn find_elf_with_most_calories(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|n| n.parse::<u32>().unwrap()).sum())
        .max()
        .expect("no elf")
}

fn find_three_elves_with_most_calories(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
        .fold([0; 3], |mut max, calories| {
            max.iter_mut().min().map(|max| *max = calories.max(*max));
            max
        })
        .into_iter()
        .sum()
}
