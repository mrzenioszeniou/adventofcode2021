mod day1;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: adventofcode2019 DAY");
        std::process::exit(1)
    }

    let day: usize = args[1].parse().expect("Couldn't parse day");

    match day {
        1 => {
            let ans = day1::solve();
            println!("PART 1:{}\nPART 2:{}", ans.0, ans.1);
        }
        _ => {
            println!("No implementation available for day {}", day);
            std::process::exit(1);
        }
    }
}
