use crate::input::Input;
use crate::lib::intcode::{Code, Machine};
use std::collections::{HashMap, VecDeque};
use std::error::Error;

type Position = (isize, isize);
const CARDINALS: [Position; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn pos_add(p1: Position, p2: Position) -> Position {
    (p1.0 + p2.0, p1.1 + p2.1)
}

fn surrounding_positions(pos: Position) -> impl Iterator<Item = Position> {
    CARDINALS.iter().map(move |&dir| pos_add(pos, dir))
}

type Direction = u8;

fn iter_dirs() -> impl Iterator<Item = Direction> {
    (1..=4)
}

fn dir_as_pos(dir: Direction) -> Position {
    let i = if dir == 0 {
        CARDINALS.len()
    } else {
        ((dir - 1) % 4) as usize
    };

    CARDINALS[i]
}

type PosType = u8;
const WALL: PosType = 0;
const OXYGEN: PosType = 2;

fn try_move(m: &mut Machine, dir: Direction) -> Result<PosType, Box<dyn Error>> {
    m.send(dir as Code)?;
    m.take_output()
        .last()
        .ok_or_else(|| "got no response".into())
        .map(|&o| o as PosType)
}

type Map = HashMap<Position, PosType>;

fn try_dir(
    m: &mut Machine,
    map: &mut Map,
    pos: Position,
    dir: Direction,
) -> Result<(bool, Position), Box<dyn Error>> {
    let next_pos = pos_add(pos, dir_as_pos(dir));
    // we already know what's at the next position
    if map.contains_key(&next_pos) {
        return Ok((false, next_pos));
    }

    let pos_type = try_move(m, dir)?;
    map.insert(next_pos, pos_type);

    Ok((pos_type != WALL, next_pos))
}

fn flood_fill(mut m: Machine, map: &mut Map, pos: Position) -> Result<(), Box<dyn Error>> {
    m.start();

    let mut queue = VecDeque::new();
    queue.push_back((m, pos));

    while let Some((m, pos)) = queue.pop_front() {
        let mut mc = m.clone();

        for dir in iter_dirs() {
            let (ok, next_pos) = try_dir(&mut mc, map, pos, dir)?;
            if ok {
                queue.push_back((mc, next_pos));
                // we only need to replace mc if we actually moved.
                mc = m.clone();
            }
        }
    }

    Ok(())
}

fn build_map(m: Machine) -> Result<Map, Box<dyn Error>> {
    let mut map = Map::new();
    flood_fill(m, &mut map, (0, 0))?;
    Ok(map)
}

fn find_oxygen(map: &Map) -> Option<Position> {
    map.iter().find(|(_, &v)| v == OXYGEN).map(|(&k, _)| k)
}

fn all_distances(map: &Map, start: Position) -> HashMap<Position, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((pos, dist)) = queue.pop_front() {
        distances.insert(pos, dist);
        surrounding_positions(pos)
            .filter(|p| map.get(p).map(|&v| v != WALL).unwrap_or(false))
            .filter(|p| !distances.contains_key(p))
            .for_each(|p| queue.push_back((p, dist + 1)));
    }

    distances
}

fn build_distances(m: Machine) -> Result<HashMap<Position, usize>, Box<dyn Error>> {
    let map = build_map(m)?;
    find_oxygen(&map)
        .ok_or_else(|| "location not found".into())
        .map(|end_pos| all_distances(&map, end_pos))
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let m = Machine::from_input(i)?;
    let distances = build_distances(m)?;
    distances
        .get(&(0, 0))
        .ok_or_else(|| "no path found".into())
        .map(|v| v.to_string())
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let m = Machine::from_input(i)?;
    let distances = build_distances(m)?;
    Ok(distances.values().max().unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_as_pos() {
        assert_eq!(dir_as_pos(1), (0, 1));
        assert_eq!(dir_as_pos(2), (0, -1));
        assert_eq!(dir_as_pos(3), (1, 0));
        assert_eq!(dir_as_pos(4), (-1, 0));
    }
}
