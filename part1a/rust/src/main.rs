fn main() {
    let contents = include_str!("../input.txt")
        .split("\n\n")
        .collect::<Vec<_>>();
    let mut result = 0;
    for elf_bag in contents {
        let elf_sum = elf_bag
            .split('\n')
            .map(|x| x.parse::<usize>().unwrap())
            .sum::<usize>();
        result = std::cmp::max(elf_sum, result)
    }
    print!("{}", result);
}
