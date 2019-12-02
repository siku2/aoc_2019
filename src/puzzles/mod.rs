use crate::input::Input;

pub mod day_01;

pub fn run_puzzle(day: u8, second: bool, input: &Input) -> Option<String> {
    match day {
        1 => Some(if second { day_01::second(input) } else { day_01::first(input) }),
        _ => None,
    }
}