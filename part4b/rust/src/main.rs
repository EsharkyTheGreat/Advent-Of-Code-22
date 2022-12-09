fn main() {
    let _contents: _ = include_str!("../input.txt")
        .lines().filter(|x| {
            let (first_range,second_range) = x.split_once(',').unwrap();
            let x = first_range.split_once('-').unwrap();
            let y = second_range.split_once('-').unwrap();
            let lfr :usize = x.0.parse().unwrap();
            let hfr :usize = x.1.parse().unwrap();
            let lsr :usize = y.0.parse().unwrap();
            let hsr :usize = y.1.parse().unwrap();
            if hfr < lsr || hsr < lfr{
                false
            }else{
                true
            } 
        }).count();
    print!("{}",_contents);
}
