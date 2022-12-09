extern crate itertools;
use itertools::Itertools;
fn main() {
    let res:usize= include_str!("../input.txt")
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|x| {
            for c in x[0].chars() {
                if x[1].contains(c) && x[2].contains(c) {
                    if c.is_lowercase() {
                        return (c as u8 - 96) as usize
                    }else{
                        return (c as u8 - 65 + 27) as usize
                    }
                }
            }
            1 as usize
        }).sum();
    print!("{}",res);
}
