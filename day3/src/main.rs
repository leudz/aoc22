const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

fn main() {}

fn shared_items(input: &str) -> impl Iterator<Item = char> + '_ {
    input.lines().map(|rucksack| {
        let (first_half, second_half) = rucksack.split_at(rucksack.len() / 2);

        first_half
            .chars()
            .find(|&item| second_half.contains(item))
            .unwrap()
    })
}

fn priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'a' as u32 + 1,
        'A'..='Z' => item as u32 - 'A' as u32 + 27,
        _ => unreachable!(),
    }
}

fn total_priority(input: &str) -> u32 {
    shared_items(input).map(priority).sum()
}

fn group_shared_items(mut input: &str) -> impl Iterator<Item = char> + '_ {
    std::iter::repeat(()).map_while(move |_| {
        (!input.is_empty()).then(|| {
            let mut rucksacks = input.splitn(4, '\n');

            let rucksack1 = rucksacks.next().unwrap();
            let rucksack2 = rucksacks.next().unwrap();
            let rucksack3 = rucksacks.next().unwrap();
            input = rucksacks.next().unwrap_or("");

            rucksack1
                .chars()
                .find(|&item| rucksack2.contains(item) && rucksack3.contains(item))
                .unwrap()
        })
    })
}

fn group_total_priority(input: &str) -> u32 {
    group_shared_items(input).map(priority).sum()

    // Alternative impl

    // let mut total_priority = 0;

    // while !input.is_empty() {
    //     let mut rucksacks = input.splitn(4, '\n');

    //     let rucksack1 = rucksacks.next().unwrap();
    //     let rucksack2 = rucksacks.next().unwrap();
    //     let rucksack3 = rucksacks.next().unwrap();
    //     input = rucksacks.next().unwrap_or("");

    //     total_priority += rucksack1
    //         .chars()
    //         .find(|&item| rucksack2.contains(item) && rucksack3.contains(item))
    //         .map(priority)
    //         .unwrap();
    // }

    // total_priority
}
