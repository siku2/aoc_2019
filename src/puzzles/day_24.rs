use crate::input::Input;
use std::collections::{BTreeMap, HashSet};
use std::error;
use std::fmt;

type Error = Box<dyn error::Error>;

fn should_toggle(is_bug: bool, neighbors: usize) -> bool {
    if is_bug {
        neighbors != 1
    } else {
        neighbors == 1 || neighbors == 2
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Layout {
    cells: Vec<bool>,
    width: usize,
    height: usize,
}

impl Layout {
    fn empty(width: usize, height: usize) -> Self {
        let cells = vec![false; width * height];
        Self {
            cells,
            width,
            height,
        }
    }

    fn from_input(i: &Input) -> Self {
        let mut width = 0;
        let mut cells = Vec::with_capacity(i.raw.len());

        let mut height = 0;
        for (y, line) in i.lines().enumerate() {
            height = y + 1;
            for (x, c) in line.chars().enumerate() {
                if x >= width {
                    width = x + 1;
                }
                cells.push(c == '#');
            }
        }

        Self {
            cells,
            width,
            height,
        }
    }

    fn is_bug(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.cells
            .get(y * self.width + x)
            .copied()
            .unwrap_or_default()
    }

    fn count_surrounding_bugs(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        if x >= 1 && self.is_bug(x - 1, y) {
            count += 1;
        }
        if y >= 1 && self.is_bug(x, y - 1) {
            count += 1;
        }

        if self.is_bug(x + 1, y) {
            count += 1;
        }
        if self.is_bug(x, y + 1) {
            count += 1;
        }

        count
    }

    fn step(&mut self) {
        let mut toggles = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if should_toggle(self.is_bug(x, y), self.count_surrounding_bugs(x, y)) {
                    toggles.push((x, y));
                }
            }
        }

        toggles.iter().for_each(|&(x, y)| self.do_toggle(x, y));
    }

    fn do_toggle(&mut self, x: usize, y: usize) {
        let index = y * self.width + x;
        self.cells[index] = !self.cells[index];
    }

    fn bug_count(&self) -> usize {
        self.cells.iter().filter(|alive| **alive).count()
    }

    fn biodiversity(&self) -> usize {
        let mut total = 0;
        for (i, alive) in self.cells.iter().enumerate() {
            if *alive {
                total += 1 << i;
            }
        }

        total
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = if self.is_bug(x, y) { '#' } else { '.' };
                write!(f, "{}", c)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn first(i: &Input) -> Result<String, Error> {
    let mut layout = Layout::from_input(i);
    let mut seen = HashSet::new();

    while !seen.contains(&layout) {
        seen.insert(layout.clone());
        layout.step();
    }
    Ok(layout.biodiversity().to_string())
}

struct RecursiveLayout {
    levels: BTreeMap<isize, Layout>,
    width: usize,
    height: usize,
    recursion_pos: (usize, usize),
}

impl RecursiveLayout {
    fn from_input(i: &Input) -> Self {
        let mut levels = BTreeMap::new();
        let layout = Layout::from_input(i);
        let (width, height) = (layout.width, layout.height);
        levels.insert(0, layout);
        Self {
            levels,
            width,
            height,
            recursion_pos: (width / 2, height / 2),
        }
    }

    fn count_border(&self, layout: &Layout, border: u8) -> usize {
        match border {
            0 => (0..self.width).filter(|&x| layout.is_bug(x, 0)).count(),
            1 => (0..self.height)
                .filter(|&y| layout.is_bug(self.width - 1, y))
                .count(),
            2 => (0..self.width)
                .filter(|&x| layout.is_bug(x, self.height - 1))
                .count(),
            3 => (0..self.height).filter(|&y| layout.is_bug(0, y)).count(),
            _ => panic!("invalid border"),
        }
    }

    fn is_bug(&self, level: isize, x: usize, y: usize) -> bool {
        self.levels
            .get(&level)
            .map(|layout| layout.is_bug(x, y))
            .unwrap_or_default()
    }

    fn count_outer_neighbors(&self, layout: &Layout, x: usize, y: usize) -> usize {
        let mut count = 0;
        let (rx, ry) = self.recursion_pos;

        if x == 0 && layout.is_bug(rx - 1, ry) {
            count += 1;
        }
        if x == self.width - 1 && layout.is_bug(rx + 1, ry) {
            count += 1;
        }

        if y == 0 && layout.is_bug(rx, ry - 1) {
            count += 1;
        }
        if y == self.height - 1 && layout.is_bug(rx, ry + 1) {
            count += 1;
        }

        count
    }

    fn count_inner_neighbors(&self, layout: &Layout, x: usize, y: usize) -> usize {
        let mut count = 0;
        let (rx, ry) = self.recursion_pos;

        if x == rx {
            if y == ry - 1 {
                count += self.count_border(layout, 0);
            }
            if y == ry + 1 {
                count += self.count_border(layout, 2);
            }
        }

        if y == ry {
            if x == rx - 1 {
                count += self.count_border(layout, 3);
            }
            if x == rx + 1 {
                count += self.count_border(layout, 1);
            }
        }

        count
    }

    fn count_neighbors(&self, level: isize, x: usize, y: usize) -> usize {
        let mut count = 0;
        if let Some(layout) = self.levels.get(&level) {
            count += layout.count_surrounding_bugs(x, y);
        }
        if let Some(layout) = self.levels.get(&(level - 1)) {
            count += self.count_outer_neighbors(layout, x, y);
        }
        if let Some(layout) = self.levels.get(&(level + 1)) {
            count += self.count_inner_neighbors(layout, x, y);
        }

        count
    }

    fn get_toggles<'a>(
        &'a self,
        level: isize,
    ) -> impl Iterator<Item = (isize, (usize, usize))> + 'a {
        (0..self.height)
            .flat_map(move |y| (0..self.width).map(move |x| (x, y)))
            .filter(move |&pos| pos != self.recursion_pos)
            .filter(move |&(x, y)| {
                should_toggle(self.is_bug(level, x, y), self.count_neighbors(level, x, y))
            })
            .map(move |pos| (level, pos))
    }

    fn do_toggle(&mut self, level: isize, x: usize, y: usize) {
        if let Some(layout) = self.levels.get_mut(&level) {
            layout.do_toggle(x, y);
        } else {
            let mut layout = Layout::empty(self.width, self.height);
            layout.do_toggle(x, y);
            self.levels.insert(level, layout);
        }
    }

    fn get_min_max_level(&self) -> (isize, isize) {
        let mut keys = self.levels.keys();
        (
            keys.next().copied().unwrap_or_default(),
            keys.next_back().copied().unwrap_or_default(),
        )
    }

    fn step(&mut self) {
        let mut toggles: Vec<_> = self
            .levels
            .keys()
            .flat_map(|&level| self.get_toggles(level))
            .collect();

        let (min_level, max_level) = self.get_min_max_level();
        toggles.extend(self.get_toggles(min_level - 1));
        toggles.extend(self.get_toggles(max_level + 1));

        toggles
            .iter()
            .for_each(|&(l, (x, y))| self.do_toggle(l, x, y));
    }

    fn bug_count(&self) -> usize {
        self.levels.values().map(|l| l.bug_count()).sum()
    }

    fn run_minutes(&mut self, minutes: usize) {
        for _ in 0..minutes {
            self.step();
        }
    }
}

impl fmt::Display for RecursiveLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (level, layout) in self.levels.iter() {
            writeln!(f, "Level {}\n{}", level, layout)?;
        }

        Ok(())
    }
}

pub fn second(i: &Input) -> Result<String, Error> {
    let mut layout = RecursiveLayout::from_input(i);
    layout.run_minutes(200);
    Ok(layout.bug_count().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Error> {
        assert_eq!(
            first(&Input::new(
                "
                ....#
                #..#.
                #..##
                ..#..
                #....
                "
            ))?,
            "2129920"
        );

        Ok(())
    }
    #[test]
    fn test_second() {
        let mut layout = RecursiveLayout::from_input(&Input::new(
            "
            ....#
            #..#.
            #.?##
            ..#..
            #....
            ",
        ));
        layout.run_minutes(10);
        assert_eq!(layout.bug_count(), 99);
    }
}
