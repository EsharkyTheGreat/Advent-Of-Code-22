fn main() {
    let _contents = include_str!("../input.txt")
        .lines().filter(|x| {
            let (first_range,second_range) = x.split_once(',').unwrap();
            let x = first_range.split_once('-').unwrap();
            let y = second_range.split_once('-').unwrap();
            let lfr :usize = x.0.parse().unwrap();
            let hfr :usize = x.1.parse().unwrap();
            let lsr :usize = y.0.parse().unwrap();
            let hsr :usize = y.1.parse().unwrap();
            println!("{}",first_range);
            println!("{}",second_range);
            println!("{}",lfr);
            println!("{}",hfr);
            println!("{}",lsr);
            println!("{}",hsr);
//            let frlen = hfr - lfr + 1;
//            let srlen = hsr - lsr + 1;
            if (lfr >= lsr && hfr <= hsr) || (lsr >= lfr && hsr <= hfr) {
                true
            }else{
                false
            } 
        }).count();
    print!("{}",_contents);
}
