extern crate chrono;
extern crate clap;

use std::{error, fs, io};

use chrono::{Datelike, TimeZone, Utc};
use clap::{App, Arg};

use input::Input;

mod puzzles;
mod input;

fn get_app<'a, 'b>() -> App<'a, 'b> {
    App::new("advent of code")
        .version("1.0")
        .author("Simon Berger")
        .arg(Arg::with_name("day")
            .short("d")
            .long("day")
            .value_name("PART")
            .help("set the puzzle day")
            .takes_value(true))
        .arg(Arg::with_name("part")
            .short("p")
            .long("part")
            .value_name("PART")
            .possible_values(&["first", "second", "both"])
            .default_value("both")
            .help("which part of the day to solve"))
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .index(1)
            .default_value("STDIN"))
}

fn get_day(value: Option<&str>) -> Result<u8, Box<dyn error::Error>> {
    if let Some(raw) = value {
        if let Ok(day) = raw.parse() {
            return Ok(day);
        }

        return Err("couldn't parse day value".into());
    }

    let now = Utc::now().date();
    if now >= Utc.ymd(2019, 12, 1) && now <= Utc.ymd(2019, 12, 25) {
        return Ok(now.day() as u8);
    }

    return Err("not in the required day range".into());
}

type Part = u8;

const FIRST_PART: Part = 2 << 0;
const SECOND_PART: Part = 2 << 1;

fn get_part(value: Option<&str>) -> Part {
    match value.unwrap_or("both") {
        "first" => FIRST_PART,
        "second" => SECOND_PART,
        "both" => FIRST_PART | SECOND_PART,
        _ => unreachable!(),
    }
}

fn get_input(value: Option<&str>) -> io::Result<Input> {
    let fp = value.unwrap_or("STDIN");
    if fp == "STDIN" {
        println!("Provide the puzzle input (Use two newlines to stop reading):");
        return read_until_two_newlines(io::stdin().lock())
            .and_then(|s| Ok(Input::new(s.as_str())));
    }

    return fs::File::open(fp
    ).and_then(|mut file| Input::from_reader(&mut file));
}

fn solve_puzzle(day: u8, part: Part, input: Input) {
    println!("Solving day {}", day);

    if part & FIRST_PART != 0 {
        if let Some(solution) = puzzles::run_puzzle(day, false, &input) {
            println!("First: {}", solution);
        } else {
            println!("no first part");
        }
    }

    if part & SECOND_PART != 0 {
        if let Some(solution) = puzzles::run_puzzle(day, true, &input) {
            println!("Second: {}", solution);
        } else {
            println!("no second part");
        }
    }
}

fn main() {
    let matches = get_app().get_matches();

    let day;
    match get_day(matches.value_of("day")) {
        Ok(d) => day = d,
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    if day <= 0 || day >= 25 {
        println!("day must be between 1 and 25 (both inclusive)");
        return;
    }

    let part = get_part(matches.value_of("part"));

    let input;
    match get_input(matches.value_of("INPUT")) {
        Ok(i) => input = i,
        Err(e) => {
            println!("couldn't read input file: {}", e);
            return;
        }
    }

    solve_puzzle(day, part, input);
}

fn read_until_two_newlines(reader: impl io::BufRead) -> io::Result<String> {
    let mut s = String::new();
    let mut exit_next = false;

    for line in reader.lines() {
        let line = line?;
        s.push_str(line.as_str());
        s.push('\n');

        if line.is_empty() {
            if exit_next {
                break;
            }

            exit_next = true;
        } else {
            exit_next = false;
        }
    }

    return Ok(s);
}
