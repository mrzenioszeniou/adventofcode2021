#![feature(map_first_last, int_abs_diff, int_roundings, destructuring_assignment)]
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod dir;
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
        7 => print_solution(day7::solve()),
        8 => print_solution(day8::solve()),
        9 => print_solution(day9::solve()),
        10 => print_solution(day10::solve()),
        11 => print_solution(day11::solve()),
        12 => print_solution(day12::solve()),
        13 => print_solution(day13::solve()),
        14 => print_solution(day14::solve()),
        15 => print_solution(day15::solve()),
        16 => print_solution(day16::solve()),
        17 => print_solution(day17::solve()),
        18 => print_solution(day18::solve()),
        19 => print_solution(day19::solve()),
        20 => print_solution(day20::solve()),
        21 => print_solution(day21::solve()),
        _ => {
            eprintln!("No implementation available for day {}", day);
            std::process::exit(1);
        }
    }
}

fn print_solution<A, B>(solution: (A, B))
where
    A: std::fmt::Display + Sized,
    B: std::fmt::Display + Sized,
{
    println!("PART 1:{}\nPART 2:{}", solution.0, solution.1);
}
