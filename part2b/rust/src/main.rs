fn main() {
    print!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|x| match x {
                "A X" => 1 + 0,
                "B X" => 2 + 0,
                "C X" => 3 + 0,
                "A Y" => 1 + 3,
                "B Y" => 2 + 3,
                "C Y" => 3 + 3,
                "A Z" => 1 + 6,
                "B Z" => 2 + 6,
                "C Z" => 3 + 6,
                _ => 0,
            })
            .sum::<usize>(),
    );
}
