fn main() {
    let contents = include_str!("../input.txt").lines();
    let mut score = 0;
    for m in contents {
        let splitted = m.split(' ').collect::<Vec<_>>();
        let opp_mov = splitted[0];
        let my_mov = splitted[1];
        let my_val: usize = match my_mov {
            "Y" => 2,
            "X" => 1,
            "Z" => 3,
            _ => 0,
        };
        let results = match opp_mov {
            "A" => match my_mov {
                "X" => 3,
                "Y" => 6,
                "Z" => 0,
                _ => 0,
            },
            "B" => match my_mov {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => 0,
            },
            "C" => match my_mov {
                "X" => 6,
                "Y" => 0,
                "Z" => 3,
                _ => 0,
            },
            _ => 0,
        };
        score += my_val + results;
    }
    print!("{}", score);
}
