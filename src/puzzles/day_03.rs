use crate::input::Input;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::ops;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position(isize, isize);

impl Position {
    const ZERO: Position = Position(0, 0);

    fn from_dir(d: char) -> Position {
        match d {
            'R' => Position(1, 0),
            'D' => Position(0, -1),
            'L' => Position(-1, 0),
            'U' => Position(0, 1),
            _ => unreachable!(),
        }
    }

    fn from_seg(seg: &str) -> Position {
        let pos = Position::from_dir(seg.chars().nth(0).unwrap());
        let mag = seg[1..].parse().unwrap();
        pos * mag
    }

    fn as_direction(self) -> Position {
        self / self.magnitude()
    }

    fn magnitude(&self) -> isize {
        self.0.abs() + self.1.abs()
    }
}

impl ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Mul<isize> for Position {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl ops::Div<isize> for Position {
    type Output = Self;

    fn div(self, rhs: isize) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

fn iter_steps_in_dir(pos: Position, dir: Position) -> impl Iterator<Item = Position> {
    let mag = dir.magnitude();
    let dir = dir.as_direction();

    (1..=mag).map(move |i| pos + dir * i)
}

fn iter_wire_positions<'a>(wire: &'a [&str]) -> impl Iterator<Item = Position> + 'a {
    let mut current = Position::ZERO;

    wire.iter()
        .map(|seg| Position::from_seg(seg))
        .flat_map(move |seg| {
            let it = iter_steps_in_dir(current, seg);
            current = current + seg;
            it
        })
}

fn get_positions(wire: &[&str]) -> HashSet<Position> {
    iter_wire_positions(wire).collect::<HashSet<_>>()
}

fn get_wires(i: &Input) -> Vec<Vec<&str>> {
    i.map_lines(|l| l.split(',').collect()).collect()
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let wires = get_wires(i);

    let mut seen = HashSet::new();
    let mut distance = std::isize::MAX;

    for wire in wires {
        for pos in get_positions(&wire) {
            let pos_dist = pos.magnitude();

            if !seen.insert(pos) && pos_dist < distance {
                distance = pos_dist;
            }
        }
    }

    Ok(distance.to_string())
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let wires = get_wires(i);

    let mut positions = HashMap::new();
    let mut distance = std::usize::MAX;

    for wire in wires {
        let mut seen = HashSet::new();

        for (index, pos) in iter_wire_positions(&wire).enumerate() {
            if !seen.insert(pos) {
                continue;
            }

            let steps = index + 1;

            if let Some(first_steps) = positions.get(&pos) {
                let total_steps = first_steps + steps;
                if total_steps < distance {
                    distance = total_steps;
                }

                if steps < *first_steps {
                    positions.insert(pos, steps);
                }
            } else {
                positions.insert(pos, steps);
            }
        }
    }

    Ok(distance.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        assert_eq!(first(&Input::new("R8,U5,L5,D3\nU7,R6,D4,L4"))?, "6");

        assert_eq!(
            first(&Input::new(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            ))?,
            "159"
        );
        assert_eq!(
            first(&Input::new(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ))?,
            "135"
        );

        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        assert_eq!(second(&Input::new("R8,U5,L5,D3\nU7,R6,D4,L4"))?, "30");

        assert_eq!(
            second(&Input::new(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            ))?,
            "610"
        );
        assert_eq!(
            second(&Input::new(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ))?,
            "410"
        );

        Ok(())
    }
}
