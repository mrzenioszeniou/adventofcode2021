mod day1;
mod day2;
mod day3;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: adventofcode2019 DAY");
        std::process::exit(1)
    }

    let day: usize = args[1].parse().expect("Couldn't parse day");

    match day {
        1 => print_solution(day1::solve()),
        2 => print_solution(day2::solve()),
        3 => print_solution(day3::solve()),
        _ => {
            eprintln!("No implementation available for day {}", day);
            std::process::exit(1);
        }
    }
}

fn print_solution<A, B>(solution: (A, B))
where
    A: std::fmt::Debug + Sized,
    B: std::fmt::Debug + Sized,
{
    println!("PART 1:{:?}\nPART 2:{:?}", solution.0, solution.1);
}
