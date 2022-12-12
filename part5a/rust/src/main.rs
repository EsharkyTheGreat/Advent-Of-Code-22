extern crate itertools;
use itertools::Itertools;

fn main() {
    let contents = include_str!("../input.txt");
    let (stack,moves) = contents.split_once("\n\n").unwrap(); 
    // println!("{}",stack);
    // println!("{}",moves);
    let (stack_str, platforms) = stack.rsplit_once('\n').unwrap(); 
    let platform_num = platforms.split_whitespace().last().unwrap().parse::<usize>().unwrap();
    // println!("{}",platform_num);
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); platform_num];
    stack_str.lines().rev().for_each(|y| {
        for (idx, mut chunk) in y.chars().chunks(4).into_iter().enumerate() {
            let crate_mid = chunk.nth(1).unwrap();
            if crate_mid.is_alphabetic() {
                stacks[idx].push(crate_mid);
            }
        }
    });
    // println!("{:?}",stacks);
    moves.lines().for_each(|x| {
        let mut word_iter = x.split_whitespace();
        let count = word_iter.nth(1).unwrap().parse::<usize>().unwrap();
        let from = word_iter.nth(1).unwrap().parse::<usize>().unwrap(); 
        let to = word_iter.nth(1).unwrap().parse::<usize>().unwrap(); 
        (0..count).for_each(|_c| {
            let last = stacks[from-1].pop().unwrap();
            stacks[to-1].push(last);
        })  
    });
    stacks.iter_mut().for_each(|x| {
        print!("{}",x.pop().unwrap());
    })
}
