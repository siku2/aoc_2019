use crate::input::Input;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::error::Error;
use std::ops;

#[derive(Clone, Copy, Debug)]
struct Direction {
    x: isize,
    y: isize,
}

impl Direction {
    const UP: Self = Self::new(0, -1);
    const DOWN: Self = Self::new(0, 1);
    const LEFT: Self = Self::new(-1, 0);
    const RIGHT: Self = Self::new(1, 0);

    const CARDINAL_DIRS: [Self; 4] = [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT];

    const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    fn eq_x(&self, other: &Direction) -> bool {
        if self.x == other.x {
            return true;
        }
        if other.x == 0 {
            return self.x == 0;
        }
        if self.x.signum() != other.x.signum() {
            return false;
        }

        self.x % other.x == 0
    }
    fn eq_y(&self, other: &Direction) -> bool {
        if self.y == other.y {
            return true;
        }
        if other.y == 0 {
            return self.y == 0;
        }
        if self.y.signum() != other.y.signum() {
            return false;
        }

        self.y % other.y == 0
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Direction) -> bool {
        self.eq_x(other) && self.eq_y(other)
    }
}

impl ops::Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl ops::Mul<isize> for Direction {
    type Output = Direction;

    fn mul(self, rhs: isize) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    const MAX: Self = Self::new(std::usize::MAX, std::usize::MAX);
    const MIN: Self = Self::new(std::usize::MIN, std::usize::MIN);

    const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn surrounding<'a>(&'a self) -> impl Iterator<Item = Position> + 'a {
        Direction::CARDINAL_DIRS
            .iter()
            .filter_map(move |dir| self.checked_add_dir(*dir))
    }

    fn checked_add_dir(&self, dir: Direction) -> Option<Self> {
        let x = usize::try_from(self.x as isize + dir.x).ok()?;
        let y = usize::try_from(self.y as isize + dir.y).ok()?;

        Some(Position::new(x, y))
    }

    fn grow_to(&mut self, pos: &Position) {
        if pos.x > self.x {
            self.x = pos.x;
        }
        if pos.y > self.y {
            self.y = pos.y;
        }
    }

    fn shrink_to(&mut self, pos: &Position) {
        if self.x > pos.x {
            self.x = pos.x;
        }
        if self.y > pos.y {
            self.y = pos.y;
        }
    }
}

impl ops::Sub for Position {
    type Output = Direction;

    fn sub(self, rhs: Self) -> Self::Output {
        let dx = self.x as isize - rhs.x as isize;
        let dy = self.y as isize - rhs.y as isize;
        Direction::new(dx, dy)
    }
}

fn get_label_direction(labels: &HashMap<Position, char>, pos: Position) -> Option<Direction> {
    let mut it = pos
        .surrounding()
        .filter(|pos| labels.contains_key(&pos))
        .map(|next_pos| next_pos - pos);
    let dir = it.next()?;

    if it.next().is_some() {
        return None;
    }

    if !(dir == Direction::RIGHT || dir == Direction::DOWN) {
        return None;
    }

    Some(dir)
}

fn get_portal_pos(
    start: Position,
    dir: Direction,
    label_len: usize,
    passages: &HashSet<Position>,
) -> Option<Position> {
    if let Some(prev_start) = start.checked_add_dir(-dir) {
        if passages.contains(&prev_start) {
            return Some(prev_start);
        }
    }

    if let Some(after_end) = start.checked_add_dir(dir * label_len as isize) {
        if passages.contains(&after_end) {
            return Some(after_end);
        }
    }

    None
}

fn get_portal_labels(
    labels: &HashMap<Position, char>,
    passages: &HashSet<Position>,
) -> HashMap<Position, String> {
    let mut finished_labels = HashMap::new();

    for (&start_pos, c) in labels.iter() {
        let dir_opt = get_label_direction(labels, start_pos);
        if dir_opt.is_none() {
            continue;
        }
        let dir = dir_opt.unwrap();

        let mut label = c.to_string();
        let mut i = 1isize;
        loop {
            if let Some(pos) = start_pos.checked_add_dir(dir * i) {
                if let Some(c) = labels.get(&pos) {
                    label.push(*c);
                    i += 1;
                    continue;
                }
            }

            break;
        }

        if let Some(portal_pos) = get_portal_pos(start_pos, dir, label.len(), passages) {
            finished_labels.insert(portal_pos, label);
        }
    }

    finished_labels
}

fn portals_from_portal_labels(labels: &HashMap<Position, String>) -> HashMap<Position, Position> {
    let mut portals: HashMap<Position, Position> = HashMap::new();
    let mut label_pos = HashMap::new();

    for (pos, label) in labels.iter() {
        let pos = *pos;
        if let Some(&other_pos) = label_pos.get(label) {
            portals.insert(pos, other_pos);
            portals.insert(other_pos, pos);
        } else {
            label_pos.insert(label, pos);
        }
    }

    portals
}

#[derive(Clone, Debug)]
struct Rect {
    tl: Position,
    br: Position,
}

impl Rect {
    const MIN: Self = Self::new(Position::MAX, Position::MIN);

    const fn new(tl: Position, br: Position) -> Self {
        Self { tl, br }
    }

    fn resize_to(&mut self, pos: &Position) {
        self.tl.shrink_to(pos);
        self.br.grow_to(pos);
    }

    fn x_range(&self) -> ops::RangeInclusive<usize> {
        (self.tl.x..=self.br.x)
    }

    fn y_range(&self) -> ops::RangeInclusive<usize> {
        (self.tl.y..=self.br.y)
    }

    fn on_border(&self, pos: &Position) -> bool {
        if pos.x == self.tl.x || pos.x == self.br.x {
            return self.y_range().contains(&pos.y);
        }
        if pos.y == self.tl.y || pos.y == self.br.y {
            return self.x_range().contains(&pos.x);
        }
        false
    }
}

type RecursivePosition = (Position, usize);

struct Maze {
    passages: HashSet<Position>,
    inner_portals: HashMap<Position, Position>,
    outer_portals: HashMap<Position, Position>,
    portal_labels: HashMap<Position, String>,
}

impl Maze {
    fn from_input(i: &Input) -> Self {
        let mut passages = HashSet::new();
        let mut labels = HashMap::new();

        let mut outer_rect = Rect::MIN;

        for (abs_y, line) in i.raw.lines().enumerate() {
            for (abs_x, c) in line.chars().enumerate() {
                let pos = Position::new(abs_x, abs_y);

                match c {
                    '#' => {
                        outer_rect.resize_to(&pos);
                    }
                    ' ' => (),
                    '.' => {
                        passages.insert(pos);
                    }
                    _ => {
                        labels.insert(pos, c);
                    }
                }
            }
        }

        let portal_labels = get_portal_labels(&labels, &passages);
        let mut inner_portals = HashMap::new();
        let mut outer_portals = HashMap::new();

        for (from, to) in portals_from_portal_labels(&portal_labels) {
            if outer_rect.on_border(&from) {
                outer_portals.insert(from, to);
            } else {
                inner_portals.insert(from, to);
            }
        }

        Maze {
            passages,
            portal_labels,
            inner_portals,
            outer_portals,
        }
    }

    fn portal_position(&self, portal: &str) -> Option<Position> {
        self.portal_labels
            .iter()
            .find(|(_, label)| label == &portal)
            .map(|(pos, _)| *pos)
    }

    fn find_path(
        &self,
        start: RecursivePosition,
        end: RecursivePosition,
        ignore_level: bool,
    ) -> Option<usize> {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));

        while let Some((prev_rpos, prev_dist)) = queue.pop_front() {
            if !seen.insert(prev_rpos) {
                continue;
            }
            let (prev_pos, prev_level) = prev_rpos;
            if prev_rpos == end || (ignore_level && prev_pos == end.0) {
                return Some(prev_dist);
            }

            let next_dist = prev_dist + 1;

            for pos in prev_pos.surrounding() {
                let rpos = (pos, prev_level);
                if !self.passages.contains(&pos) {
                    continue;
                }

                queue.push_back((rpos, next_dist));
            }

            if prev_level > 0 {
                if let Some(pos) = self.outer_portals.get(&prev_pos) {
                    queue.push_back(((*pos, prev_level - 1), next_dist));
                }
            }

            if let Some(pos) = self.inner_portals.get(&prev_pos) {
                queue.push_back(((*pos, prev_level + 1), next_dist));
            }
        }

        None
    }

    fn distance_between(&self, start: &str, end: &str) -> Option<usize> {
        let start_pos = self.portal_position(start)?;
        let end_pos = self.portal_position(end)?;

        self.find_path((start_pos, 0), (end_pos, 0), true)
    }

    fn recursive_distance_between(&self, start: &str, end: &str) -> Option<usize> {
        let start_pos = self.portal_position(start)?;
        let end_pos = self.portal_position(end)?;

        self.find_path((start_pos, 0), (end_pos, 0), false)
    }
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let m = Maze::from_input(i);

    m.distance_between("AA", "ZZ")
        .ok_or_else(|| "couldn't find path".into())
        .map(|v| v.to_string())
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let m = Maze::from_input(i);

    m.recursive_distance_between("AA", "ZZ")
        .ok_or_else(|| "couldn't find path".into())
        .map(|v| v.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            &first(&Input::new(
                "
                A           
                A           
         #######.#########  
         #######.........#  
         #######.#######.#  
         #######.#######.#  
         #######.#######.#  
         #####  B    ###.#  
       BC...##  C    ###.#  
         ##.##       ###.#  
         ##...DE  F  ###.#  
         #####    G  ###.#  
         #########.#####.#  
       DE..#######...###.#  
         #.#########.###.#  
       FG..#########.....#  
         ###########.#####  
                    Z       
                    Z             
                "
            ))?,
            "23"
        );

        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            &second(&Input::new(
                "
                Z L X W       C                 
                Z P Q B       K                 
     ###########.#.#.#.#######.###############  
     #...#.......#.#.......#.#.......#.#.#...#  
     ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
     #.#...#.#.#...#.#.#...#...#...#.#.......#  
     #.###.#######.###.###.#.###.###.#.#######  
     #...#.......#.#...#...#.............#...#  
     #.#########.#######.#.#######.#######.###  
     #...#.#    F       R I       Z    #.#.#.#  
     #.###.#    D       E C       H    #.#.#.#  
     #.#...#                           #...#.#  
     #.###.#                           #.###.#  
     #.#....OA                       WB..#.#..ZH
     #.###.#                           #.#.#.#  
   CJ......#                           #.....#  
     #######                           #######  
     #.#....CK                         #......IC
     #.###.#                           #.###.#  
     #.....#                           #...#.#  
     ###.###                           #.#.#.#  
   XF....#.#                         RF..#.#.#  
     #####.#                           #######  
     #......CJ                       NM..#...#  
     ###.#.#                           #.###.#  
   RE....#.#                           #......RF
     ###.###        X   X       L      #.#.#.#  
     #.....#        F   Q       P      #.#.#.#  
     ###.###########.###.#######.#########.###  
     #.....#...#.....#.......#...#.....#.#...#  
     #####.#.###.#######.#######.###.###.#.#.#  
     #.......#.......#.#.#.#.#...#...#...#.#.#  
     #####.###.#####.#.#.#.#.###.###.#.###.###  
     #.......#.....#.#...#...............#...#  
     #############.#.#.###.###################  
                  A O F   N                     
                  A A D   M                        
                "
            ))?,
            "396"
        );

        Ok(())
    }
}
