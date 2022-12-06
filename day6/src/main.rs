const INPUT: &str = "nppdvjthqldpwncqszvftbrmjlhg";

fn main() {}

fn first_start_of_packet(input: &str) -> usize {
    input
        .as_bytes()
        .windows(4)
        .position(|signal| {
            !signal
                .iter()
                .enumerate()
                .any(|(i, c)| signal[i + 1..].contains(c))
        })
        .unwrap()
        + 4
}

fn first_start_of_message(input: &str) -> usize {
    input
        .as_bytes()
        .windows(14)
        .position(|signal| {
            !signal
                .iter()
                .enumerate()
                .any(|(i, c)| signal[i + 1..].contains(c))
        })
        .unwrap()
        + 14
}
