use crate::input::Input;
use crate::lib::intcode::{Code, Machine};
use colored::{ColoredString, Colorize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;

type Vec2 = (isize, isize);
type Rect = (Vec2, Vec2);
type Color = u8;

const BLACK: Color = 0;
const WHITE: Color = 1;

fn vec2_add(a: Vec2, b: Vec2) -> Vec2 {
    (a.0 + b.0, a.1 + b.1)
}

fn vec2_rot(v: Vec2, rots: i8) -> Vec2 {
    const COS: [isize; 4] = [1, 0, -1, 0];

    let i = rots.rem_euclid(4) as usize;
    let (c, s) = (COS[i], COS[(i + 3) % 4]);

    (v.0 * c - v.1 * s, v.0 * s + v.1 * c)
}

struct PaintRobot {
    machine: Machine,
    direction: Vec2,
    position: Vec2,
    pub colors: HashMap<Vec2, Color>,
}

impl PaintRobot {
    fn new(machine: Machine, start_panel: Color) -> PaintRobot {
        let mut colors = HashMap::new();
        colors.insert((0, 0), start_panel);

        PaintRobot {
            machine,
            colors,
            direction: (0, 1),
            position: (0, 0),
        }
    }

    fn from_input(i: &Input, start_panel: Color) -> Result<PaintRobot, Box<dyn Error>> {
        Machine::from_input(i).map(|m| Self::new(m, start_panel))
    }

    /// Rotates the current direction rots times 90° anti-clockwise.
    fn rotate(&mut self, rots: i8) {
        self.direction = vec2_rot(self.direction, rots);
    }

    fn step(&mut self) {
        self.position = vec2_add(self.position, self.direction)
    }

    fn get_output(&mut self) -> Result<(Color, i8), Box<dyn Error>> {
        let mut out = self.machine.take_output();
        if out.len() != 2 {
            return Err("didn't receive two output values from machine".into());
        }

        let mut iter = out.drain(..);

        let color = iter.next().unwrap() as Color;
        let rots = 2 * (iter.next().unwrap() as i8) - 1;
        Ok((color, rots))
    }

    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.machine.start();

        while !self.machine.is_done() {
            let color = self.colors.get(&self.position).copied().unwrap_or(BLACK);
            self.machine.send(color as Code)?;

            let (new_color, rots) = self.get_output()?;
            self.colors.insert(self.position, new_color);
            self.rotate(-rots);
            self.step();
        }

        Ok(())
    }
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut robot = PaintRobot::from_input(i, BLACK)?;
    robot.run()?;

    Ok(robot.colors.len().to_string())
}

fn get_bounds<T>(map: &HashMap<Vec2, T>) -> Rect {
    let (mut tl_x, mut tl_y) = (std::isize::MAX, std::isize::MAX);
    let (mut br_x, mut br_y) = (std::isize::MIN, std::isize::MIN);

    for &(x, y) in map.keys() {
        if x < tl_x {
            tl_x = x;
        }
        if x > br_x {
            br_x = x;
        }

        if y < tl_y {
            tl_y = y;
        }
        if y > br_y {
            br_y = y;
        }
    }

    ((tl_x, tl_y), (br_x, br_y))
}

fn format_color_map(colors: &HashMap<Vec2, Color>, bounds: Rect) -> String {
    let (tl, br) = bounds;

    let mut s = String::new();

    for y in (tl.1..=br.1).rev() {
        for x in tl.0..=br.0 {
            let color = colors.get(&(x, y)).copied().unwrap_or(BLACK);

            let mut text: ColoredString = color.to_string().as_str().into();
            match color {
                BLACK => text = text.on_black().black(),
                WHITE => text = text.on_white().white(),
                _ => (),
            }
            write!(&mut s, "{}", text).unwrap();
        }

        s.write_char('\n').unwrap();
    }

    s
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut robot = PaintRobot::from_input(i, WHITE)?;
    robot.run()?;

    let colors = robot.colors;
    let bounds = get_bounds(&colors);

    Ok(format!("\n{}", format_color_map(&colors, bounds)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_rot() {
        // anti-clockwise
        assert_eq!(vec2_rot((-1, 0), 1), (0, -1));
        assert_eq!(vec2_rot((0, -1), 1), (1, 0));
        assert_eq!(vec2_rot((1, 0), 1), (0, 1));
        assert_eq!(vec2_rot((0, 1), 1), (-1, 0));

        // clockwise
        assert_eq!(vec2_rot((0, 1), -1), (1, 0));
        assert_eq!(vec2_rot((1, 0), -1), (0, -1));
        assert_eq!(vec2_rot((0, -1), -1), (-1, 0));
        assert_eq!(vec2_rot((-1, 0), -1), (0, 1));
    }
}
