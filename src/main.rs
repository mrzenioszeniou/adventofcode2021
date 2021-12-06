mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod util;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: adventofcode2021 DAY");
        std::process::exit(1)
    }

    let day: usize = if let Ok(day) = args[1].parse() {
        day
    } else {
        eprintln!("Couldn't parse day");
        std::process::exit(1);
    };

    match day {
        1 => print_solution(day1::solve()),
        2 => print_solution(day2::solve()),
        3 => print_solution(day3::solve()),
        4 => print_solution(day4::solve()),
        5 => print_solution(day5::solve()),
        6 => print_solution(day6::solve()),
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
