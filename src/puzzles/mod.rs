use crate::input::Input;

use std::error::Error;

pub mod day_01;
pub mod day_02;
pub mod day_03;

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
        _ => None,
    }
}
