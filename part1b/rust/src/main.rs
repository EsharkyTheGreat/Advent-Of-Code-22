fn main() {
    let contents = include_str!("../input.txt").split("\n\n");
    let mut max1 = 0;
    let mut max2 = 0;
    let mut max3 = 0;
    for elfbag in contents {
        let calories = elfbag
            .split('\n')
            .map(|x| x.parse::<usize>().unwrap())
            .sum::<usize>();
        if calories > max3 {
            if calories > max2 {
                if calories > max1 {
                    max3 = max2;
                    max2 = max1;
                    max1 = calories;
                } else {
                    max3 = max2;
                    max2 = calories;
                }
            } else {
                max3 = calories;
            }
        }
    }
    print!("{}", max1 + max2 + max3);
}
