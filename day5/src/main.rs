use std::str::FromStr;

const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

fn main() {}

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_ascii_whitespace();
        iter.next().unwrap();
        let count = iter.next().unwrap().parse().unwrap();
        iter.next().unwrap();
        let from = iter.next().unwrap().parse().unwrap();
        iter.next().unwrap();
        let to = iter.next().unwrap().parse().unwrap();

        Ok(Instruction { count, from, to })
    }
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let (stacks_input, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks = Vec::new();

    for height in stacks_input.lines() {
        for ((start, _), (end, _)) in height.match_indices('[').zip(height.match_indices(']')) {
            let stack = start / 4;
            let item = height[start + 1..end].parse::<char>().unwrap();

            if stacks.len() <= stack {
                stacks.extend((stacks.len()..stack + 1).map(|_| Vec::new()));
            }
            stacks[stack].push(item);
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    (
        stacks,
        instructions
            .lines()
            .map(|instruction| instruction.parse().unwrap())
            .collect(),
    )
}

fn rearrange_all_one_by_one(stacks: &mut [Vec<char>], instructions: &[Instruction]) {
    for instruction in instructions {
        for _ in 0..instruction.count {
            let item = stacks[instruction.from - 1].pop().unwrap();
            stacks[instruction.to - 1].push(item);
        }
    }
}

fn top_items_one_by_one(input: &str) -> String {
    let (mut stacks, instructions) = parse(input);
    rearrange_all_one_by_one(&mut stacks, &instructions);

    stacks.iter().map(|stack| *stack.last().unwrap()).collect()
}

fn rearrange_all(stacks: &mut [Vec<char>], instructions: &[Instruction]) {
    for instruction in instructions {
        let len = stacks[instruction.from - 1].len();
        let items = stacks[instruction.from - 1]
            .drain((len - instruction.count)..)
            .collect::<Vec<_>>();
        stacks[instruction.to - 1].extend(items);
    }
}

fn top_items(input: &str) -> String {
    let (mut stacks, instructions) = parse(input);
    rearrange_all(&mut stacks, &instructions);

    stacks.iter().map(|stack| *stack.last().unwrap()).collect()
}
