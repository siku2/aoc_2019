use crate::input::Input;
use crate::lib::intcode::{Code, Machine};
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::iter;
use std::ops;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl<T> Vec2<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> ops::Add for Vec2<T>
where
    T: ops::Add,
{
    type Output = Vec2<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> ops::AddAssign for Vec2<T>
where
    T: ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> ops::Neg for Vec2<T>
where
    T: ops::Neg,
{
    type Output = Vec2<T::Output>;

    fn neg(self) -> Self::Output {
        Self::Output::new(-self.x, -self.y)
    }
}

impl<'a> Vec2<isize> {
    const UP: Vec2<isize> = Vec2 { x: 0, y: -1 };
    const DOWN: Vec2<isize> = Vec2 { x: 0, y: 1 };
    const LEFT: Vec2<isize> = Vec2 { x: -1, y: 0 };
    const RIGHT: Vec2<isize> = Vec2 { x: 1, y: 0 };

    const CARDINAL_DIRS: [Self; 4] = [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT];

    fn iter_cardinal_dirs() -> impl Iterator<Item = Self> + 'a {
        Self::CARDINAL_DIRS.iter().copied()
    }

    fn get_dir_num(dir: Vec2<isize>) -> Option<usize> {
        for (i, &d) in Self::CARDINAL_DIRS.iter().enumerate() {
            if d == dir {
                return Some(i);
            }
        }

        None
    }
}

impl Vec2<usize> {
    fn add_dir(&self, dir: Vec2<isize>) -> Option<Self> {
        let x = self.x as isize + dir.x;
        let y = self.y as isize + dir.y;
        if x < 0 || y < 0 {
            None
        } else {
            Some(Vec2::new(x as usize, y as usize))
        }
    }
}

const ROBOT_UP: char = '^';
const ROBOT_DOWN: char = 'v';
const ROBOT_LEFT: char = '<';
const ROBOT_RIGHT: char = '>';

struct Robot {
    pos: Vec2<usize>,
    facing: Vec2<isize>,
}

impl Robot {
    fn new(pos: Vec2<usize>, facing: Vec2<isize>) -> Self {
        Self { pos, facing }
    }
    fn get_direction(c: char) -> Option<Vec2<isize>> {
        let dir = match c {
            ROBOT_UP => Vec2::UP,
            ROBOT_DOWN => Vec2::DOWN,
            ROBOT_LEFT => Vec2::LEFT,
            ROBOT_RIGHT => Vec2::RIGHT,
            _ => return None,
        };

        Some(dir)
    }
    fn from_char(pos: Vec2<usize>, c: char) -> Option<Self> {
        let facing = Self::get_direction(c)?;
        Some(Self::new(pos, facing))
    }
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let chr = match self.facing {
            Vec2::UP => ROBOT_UP,
            Vec2::DOWN => ROBOT_DOWN,
            Vec2::LEFT => ROBOT_LEFT,
            Vec2::RIGHT => ROBOT_RIGHT,
            _ => '?',
        };

        write!(f, "{}", chr)
    }
}

const EOL: char = '\n';
const SPACE: char = '.';
const SCAFFOLD: char = '#';

struct Frame {
    map: HashMap<Vec2<usize>, char>,
    size: Vec2<usize>,
    robot: Robot,
}

impl Frame {
    fn from_output(output: &[Code]) -> Self {
        let mut map = HashMap::new();

        let mut x = 0;
        let mut y = 0;

        let mut max_x = 0;
        let mut robot = None;

        for code in output {
            let chr = *code as u8 as char;
            if chr == EOL {
                max_x = cmp::max(x, max_x);

                x = 0;
                y += 1;
                continue;
            }
            let pos = Vec2::new(x, y);
            if robot.is_none() {
                robot = Robot::from_char(pos, chr);
            }

            map.insert(pos, chr);
            x += 1;
        }

        max_x = cmp::max(x, max_x);

        let size = Vec2::new(max_x + 1, y + 1);

        Self {
            map,
            size,
            robot: robot.unwrap(),
        }
    }

    fn pos_is_type(&self, pos: &Vec2<usize>, chr: char) -> bool {
        self.map.get(pos).map(|c| c == &chr).unwrap_or(false)
    }

    fn follow_line<'a>(
        &'a self,
        start: Vec2<usize>,
        dir: Vec2<isize>,
    ) -> impl Iterator<Item = Vec2<usize>> + 'a {
        let mut pos = start;

        iter::from_fn(move || {
            pos = pos.add_dir(dir)?;

            if self.pos_is_type(&pos, SCAFFOLD) {
                Some(pos)
            } else {
                None
            }
        })
    }

    fn iter_directions<'a>(&'a self, pos: Vec2<usize>) -> impl Iterator<Item = Vec2<isize>> + 'a {
        Vec2::iter_cardinal_dirs().filter_map(move |dir| {
            let p = pos.add_dir(dir)?;
            if self.pos_is_type(&p, SCAFFOLD) {
                Some(dir)
            } else {
                None
            }
        })
    }

    fn follow_path<'a>(
        &'a self,
        start: Vec2<usize>,
    ) -> Option<impl Iterator<Item = Vec2<usize>> + 'a> {
        let mut current_dir;
        if let Some(dir) = self.iter_directions(start).next() {
            current_dir = dir;
        } else {
            return None;
        }

        let mut current_line = self.follow_line(start, current_dir);
        let mut current_pos = start;

        Some(iter::from_fn(move || loop {
            match current_line.next() {
                Some(pos) => {
                    current_pos = pos;
                    return Some(pos);
                }
                None => {
                    current_dir = self
                        .iter_directions(current_pos)
                        .find(|&dir| dir != -current_dir)?;
                    current_line = self.follow_line(current_pos, current_dir);
                }
            }
        }))
    }

    fn find_intersections<'a>(&'a self) -> Option<impl Iterator<Item = Vec2<usize>> + 'a> {
        let dir = self.iter_directions(self.robot.pos).next()?;
        let start_pos = self.robot.pos.add_dir(dir)?;
        let mut path_iter = self.follow_path(start_pos)?;

        let mut seen = HashSet::new();

        Some(iter::from_fn(move || loop {
            let pos = path_iter.next()?;
            if !seen.insert(pos) {
                return Some(pos);
            }
        }))
    }

    fn iter_routine<'a>(&'a self) -> impl Iterator<Item = Instruction> + 'a {
        let mut current_pos = self.robot.pos;
        let mut current_dir = self.robot.facing;
        // this is just to avoid having to specify the type
        let mut current_line = self.follow_line(current_pos, current_dir);
        let mut has_current_line = false;

        iter::from_fn(move || loop {
            if has_current_line {
                let mut count = 0;
                for pos in &mut current_line {
                    count += 1;
                    current_pos = pos;
                }

                has_current_line = false;
                return Some(Instruction::Forward(count));
            }

            let prev_dir = current_dir;
            current_dir = self
                .iter_directions(current_pos)
                .find(|&dir| dir != -current_dir)?;
            current_line = self.follow_line(current_pos, current_dir);
            has_current_line = true;
            // if we need to turn, send the instruction
            if let Some(instr) = Instruction::from_direction(prev_dir, current_dir) {
                return Some(instr);
            }
        })
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let pos = Vec2::new(x, y);
                if pos == self.robot.pos {
                    self.robot.fmt(f)?;
                    continue;
                }

                let c = self.map.get(&pos).unwrap_or(&SPACE);
                c.fmt(f)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn get_frame(m: &mut Machine) -> Result<Frame, Box<dyn Error>> {
    m.run(&[]).map(|out| Frame::from_output(&out))
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut m = Machine::from_input(i)?;
    let frame = get_frame(&mut m)?;

    let intersections = frame
        .find_intersections()
        .ok_or_else(|| "couldn't find intersections")?;

    let checksum: usize = intersections.map(|pos| pos.x * pos.y).sum();
    Ok(checksum.to_string())
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Forward(usize),
    Turn(char),
}

const LEFT: Instruction = Instruction::Turn('L');
const RIGHT: Instruction = Instruction::Turn('R');

impl Instruction {
    fn from_direction(dir1: Vec2<isize>, dir2: Vec2<isize>) -> Option<Self> {
        let a1 = Vec2::get_dir_num(dir1)? as isize;
        let a2 = Vec2::get_dir_num(dir2)? as isize;
        match a2 - a1 {
            0 | 2 | -2 => None,
            1 | -3 => Some(RIGHT),
            3 | -1 => Some(LEFT),
            _ => unreachable!(),
        }
    }

    fn chars(&self) -> usize {
        match self {
            Self::Turn(_) => 1,
            Self::Forward(amount) => {
                if *amount < 10 {
                    1
                } else if *amount < 100 {
                    2
                } else {
                    amount.to_string().len()
                }
            }
        }
    }

    fn push_code(&self, code: &mut Vec<Code>) {
        match self {
            Self::Turn(c) => code.push(*c as Code),
            Self::Forward(amount) => {
                let s = amount.to_string();
                s.chars().for_each(|c| code.push(c as Code));
            }
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Turn(c) => write!(f, "{}", c),
            Self::Forward(amount) => write!(f, "{}", amount),
        }
    }
}

fn get_instruction_slice(instructions: &[Instruction], len: usize) -> &[Instruction] {
    let mut routine_chars = 0;
    let mut end = 0;

    for instr in instructions {
        let chars = instr.chars();
        if routine_chars + chars > len {
            break;
        }
        routine_chars += chars;
        end += 1;
    }

    &instructions[..=end]
}

fn find_instruction_pattern<'a>(
    instructions: &'a [Instruction],
    pattern: &'a [Instruction],
) -> impl Iterator<Item = (usize, usize)> + 'a {
    let mut start = 0;
    let end = instructions.len() - pattern.len();

    iter::from_fn(move || {
        while start <= end {
            let bounds = (start, start + pattern.len());
            let slice = &instructions[bounds.0..bounds.1];
            if slice == pattern {
                start = bounds.1;
                return Some(bounds);
            }

            start += 1;
        }

        None
    })
}

fn remove_instruction_pattern(
    instructions: &[Instruction],
    pattern: &[Instruction],
) -> Vec<Instruction> {
    if pattern.len() > instructions.len() {
        return instructions.to_vec();
    }

    let mut remaining = Vec::with_capacity(instructions.len());

    let mut next_start = 0;
    for (start, end) in find_instruction_pattern(instructions, pattern) {
        remaining.extend(instructions[next_start..start].iter().cloned());
        next_start = end;
    }

    remaining.extend(instructions[next_start..].iter().cloned());
    remaining
}

fn split_instructions(
    instructions: &[Instruction],
    num: usize,
    routines: &mut Vec<Vec<Instruction>>,
) -> bool {
    if num == 0 {
        return instructions.is_empty();
    }

    let max_len = cmp::min(instructions.len(), 10);

    for len in 2..=max_len {
        let routine = get_instruction_slice(instructions, len);
        let leftover = remove_instruction_pattern(instructions, routine);
        if split_instructions(&leftover, num - 1, routines) {
            routines.push(routine.to_vec());
            return true;
        }
    }

    false
}

fn instructions_to_code(instructions: &[Instruction], code: &mut Vec<Code>) {
    let instr_len = instructions.len();
    for (i, instr) in instructions.iter().enumerate() {
        instr.push_code(code);
        if i != instr_len - 1 {
            code.push(',' as Code);
        }
    }
}

struct MainRoutine {
    main: Vec<char>,
    a: Vec<Instruction>,
    b: Vec<Instruction>,
    c: Vec<Instruction>,
}

impl MainRoutine {
    fn from_instructions(instructions: &[Instruction]) -> Result<Self, Box<dyn Error>> {
        let mut routines = Vec::new();
        if !split_instructions(instructions, 3, &mut routines) {
            return Err("couldn't split instructions into 3 routines".into());
        }

        let mut drain = routines.drain(..);
        let a = drain.next().unwrap();
        let b = drain.next().unwrap();
        let c = drain.next().unwrap();

        let mut indices: Vec<_> = find_instruction_pattern(instructions, &a)
            .map(|(i, _)| (i, 'A'))
            .collect();
        indices.extend(find_instruction_pattern(instructions, &b).map(|(i, _)| (i, 'B')));
        indices.extend(find_instruction_pattern(instructions, &c).map(|(i, _)| (i, 'C')));

        indices.sort_by_key(|(i, _)| *i);

        Ok(Self {
            main: indices.iter().map(|(_, c)| *c).collect(),
            a,
            b,
            c,
        })
    }

    fn push_code(&self, code: &mut Vec<Code>) {
        for (i, &c) in self.main.iter().enumerate() {
            code.push(c as Code);
            if i != self.main.len() - 1 {
                code.push(',' as Code);
            }
        }
        code.push(EOL as Code);

        instructions_to_code(&self.a, code);
        code.push(EOL as Code);
        instructions_to_code(&self.b, code);
        code.push(EOL as Code);
        instructions_to_code(&self.c, code);
        code.push(EOL as Code);
    }
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut m = Machine::from_input(i)?;
    let frame = get_frame(&mut m.clone())?;

    m.write(0, 2);
    m.start();

    let instructions: Vec<_> = frame.iter_routine().collect();
    let routine = MainRoutine::from_instructions(&instructions)?;

    let mut inp = Vec::new();
    routine.push_code(&mut inp);

    m.input.extend(&inp);

    m.send('n' as Code)?;
    m.send(EOL as Code)?;

    if !m.is_done() {
        return Err("robot didn't finish".into());
    }

    m.take_output()
        .last()
        .ok_or_else(|| "received no output".into())
        .map(|v| v.to_string())
}
