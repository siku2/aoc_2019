use crate::input::Input;

use std::error::Error;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;

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
        _ => None,
    }
}
