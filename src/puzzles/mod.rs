use crate::input::Input;

use std::error::Error;

type PuzzleFn = dyn Fn(&Input) -> Result<String, Box<dyn Error>>;
type PuzzleMap = HashMap<(usize, usize), &'static PuzzleFn>;

macro_rules! day_modules {
    ( $( $x:ident ),* ) => {
        $(
            mod $x;
        )*

        use std::collections::HashMap;

        fn build_puzzle_map() -> PuzzleMap {
            let mut temp_map: PuzzleMap = HashMap::new();
            $(
                let mod_name = stringify!($x);
                let day = mod_name.rsplit('_').take(1).next().unwrap().parse().unwrap();

                temp_map.insert((day, 0), &$x::first);
                temp_map.insert((day, 1), &$x::second);
            )*
            temp_map
        }
    };
}

day_modules![
    day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, day_10, day_11, day_12,
    day_13, day_14, day_15, day_16, day_17, day_18, day_19, day_20, day_21, day_22, day_23, day_24,
    day_25
];

pub fn run_puzzle(day: u8, second: bool, input: &Input) -> Option<Result<String, Box<dyn Error>>> {
    let puzzles = build_puzzle_map();
    let puzzle = puzzles.get(&(day as usize, second as usize))?;

    Some(puzzle(input))
}
