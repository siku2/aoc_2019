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
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;

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
        11 => Some(if second {
            day_11::second(input)
        } else {
            day_11::first(input)
        }),
        12 => Some(if second {
            day_12::second(input)
        } else {
            day_12::first(input)
        }),
        13 => Some(if second {
            day_13::second(input)
        } else {
            day_13::first(input)
        }),
        14 => Some(if second {
            day_14::second(input)
        } else {
            day_14::first(input)
        }),
        15 => Some(if second {
            day_15::second(input)
        } else {
            day_15::first(input)
        }),
        _ => None,
    }
}
