use crate::input::Input;

use std::error::Error;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;

pub fn run_puzzle(day: u8, second: bool, input: &Input) -> Option<Result<String, Box<dyn Error>>> {
    match day {
        1 => Some(if second {
            day_01::second(input)
        } else {
            day_01::first(input)
        }),
        2 => Some(if second {
            day_02::second(input)
        } else {
            day_02::first(input)
        }),
        3 => Some(if second {
            day_03::second(input)
        } else {
            day_03::first(input)
        }),
        4 => Some(if second {
            day_04::second(input)
        } else {
            day_04::first(input)
        }),
        5 => Some(if second {
            day_05::second(input)
        } else {
            day_05::first(input)
        }),
        6 => Some(if second {
            day_06::second(input)
        } else {
            day_06::first(input)
        }),
        7 => Some(if second {
            day_07::second(input)
        } else {
            day_07::first(input)
        }),
        8 => Some(if second {
            day_08::second(input)
        } else {
            day_08::first(input)
        }),
        9 => Some(if second {
            day_09::second(input)
        } else {
            day_09::first(input)
        }),
        10 => Some(if second {
            day_10::second(input)
        } else {
            day_10::first(input)
        }),
        _ => None,
    }
}
