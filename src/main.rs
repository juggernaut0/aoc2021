use clap::{App, Arg};
use log::Level;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
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
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod util;

const DAYS: [&dyn Solution; 25] = [
    &day1::Solution,
    &day2::Solution,
    &day3::Solution,
    &day4::Solution,
    &day5::Solution,
    &day6::Solution,
    &day7::Solution,
    &day8::Solution,
    &day9::Solution,
    &day10::Solution,
    &day11::Solution,
    &day12::Solution,
    &day13::Solution,
    &day14::Solution,
    &day15::Solution,
    &day16::Solution,
    &day17::Solution,
    &day18::Solution,
    &day19::Solution,
    &day20::Solution,
    &day21::Solution,
    &day22::Solution,
    &day23::Solution,
    &day24::Solution,
    &day25::Solution,
];

fn main() {
    let matches = App::new("aoc2021")
        .about("Advent of Code 2021")
        .arg(Arg::with_name("day").required(true))
        .arg(Arg::with_name("puzzle").required(true))
        .arg(Arg::with_name("log_level")
            .long("level")
            .help("Logging level")
            .takes_value(true)
            .default_value("info"))
        .get_matches();

    let level_match: &str = &matches.value_of("log_level").map(|it| it.to_lowercase()).unwrap();
    let log_level = match level_match {
        "trace" => Level::Trace,
        "debug" => Level::Debug,
        "info" => Level::Info,
        "warn" => Level::Warn,
        "error" => Level::Error,
        _ => Level::Info,
    };

    simple_logger::init_with_level(log_level).unwrap();



    let day: usize = matches.value_of("day")
        .and_then(|it| it.parse().ok())
        .filter(|it| *it > 0 && *it <= 25)
        .expect("Expected a day as a number 1-25");
    let puzzle: u32 = matches.value_of("puzzle")
        .and_then(|it| it.parse().ok())
        .filter(|it| *it == 1 || *it == 2)
        .expect("Expected a puzzle as 1 or 2");

    let input = {
        let dss = format!("input/{}-{}.txt", day, puzzle);
        let ds = std::path::Path::new(&dss);
        if ds.exists() {
            std::fs::read_to_string(ds).unwrap()
        } else {
            std::fs::read_to_string(format!("input/{}.txt", day)).unwrap()
        }
    };

    let solution = DAYS[day-1];
    let answer = match puzzle {
        1 => solution.solve_1(input),
        2 => solution.solve_2(input),
        _ => unreachable!(),
    };
    println!("{}", answer);
}

trait Solution {
    fn solve_1(&self, input: String) -> String;
    fn solve_2(&self, input: String) -> String;
}