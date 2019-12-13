use crate::input::Input;
use crate::lib::intcode::Machine;
use colored::Colorize;
use std::collections::HashMap;
use std::error::Error;
use std::io;

type Position = (isize, isize);
type TileID = u8;

const EMPTY: TileID = 0;
const WALL: TileID = 1;
const BLOCK: TileID = 2;
const HOR_PADDLE: TileID = 3;
const BALL: TileID = 4;

fn render_tile(w: &mut impl io::Write, tile: TileID) -> io::Result<()> {
    let s = match tile {
        EMPTY => " ".on_black(),
        WALL => "+".green(),
        BLOCK => "#".white().on_white(),
        HOR_PADDLE => "-".bold().red(),
        BALL => "o".blue(),
        _ => "?".red(),
    };

    write!(w, "{}", s)
}

type Map = HashMap<Position, TileID>;

fn render_map(w: &mut impl io::Write, map: &Map, size: Position) -> io::Result<()> {
    for y in 0..size.1 {
        for x in 0..size.0 {
            let tile = map.get(&(x, y)).copied().unwrap_or(EMPTY);
            render_tile(w, tile)?;
        }

        writeln!(w)?;
    }

    Ok(())
}

fn print_map(map: &Map, size: Position) {
    let stdout = io::stdout();
    let mut out_lock = stdout.lock();
    render_map(&mut out_lock, map, size).unwrap();
}

type GameState = (Map, Position, isize);

fn run_game(m: &mut Machine, render: bool) -> Result<GameState, Box<dyn Error>> {
    let mut map = Map::new();
    let mut max_pos: Position = (0, 0);
    let mut score = 0;

    let mut ball_x: isize = 0;
    let mut paddle_x: isize = 0;

    'outer: loop {
        while m.output.len() < 3 {
            if !m.run_once()? {
                if m.is_done() {
                    break 'outer;
                }
                let input = (ball_x - paddle_x).signum();
                m.input.push_back(input);

                if render {
                    print_map(&map, (max_pos.0 + 1, max_pos.1 + 1));
                    std::thread::sleep(std::time::Duration::from_millis(250));
                }
            }
        }

        let out = m.take_output();
        let (x, y) = (out[0], out[1]);
        if x == -1 {
            score = out[2];
            continue;
        }

        let tile = out[2] as TileID;
        match tile {
            BALL => ball_x = x,
            HOR_PADDLE => paddle_x = x,
            _ => (),
        }

        if x > max_pos.0 {
            max_pos.0 = x;
        }
        if y > max_pos.1 {
            max_pos.1 = y;
        }

        map.insert((x, y), tile);
    }

    Ok((map, (max_pos.0 + 1, max_pos.1 + 1), score))
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut m = Machine::from_input(i)?;
    let (map, _, _) = run_game(&mut m, false)?;

    Ok(map
        .values()
        .filter(|&&tile| tile == BLOCK)
        .count()
        .to_string())
}

fn play_game(m: &mut Machine, render: bool) -> Result<GameState, Box<dyn Error>> {
    m.start();
    m.write(0, 2);

    run_game(m, render)
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut m = Machine::from_input(i)?;
    let (_, _, score) = play_game(&mut m, false)?;

    Ok(score.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
