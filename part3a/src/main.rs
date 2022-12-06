use std::collections::HashSet;
fn main() {
    let res: usize = include_str!("../input.txt").lines().map(|x| {
        let (part1,part2) = x.split_at(x.len() / 2);
        let set: HashSet<char > = HashSet::from_iter(part1.chars());
        let filtered = part2.chars().filter(|x| set.contains(&x)).collect::<Vec<char>>()[0]; 
        if filtered.is_lowercase() {
            (filtered as u8 - 96) as usize
        }else{
            (filtered as u8 - 65 + 27) as usize
        }
    }).sum();
    print!("{}",res);
}
