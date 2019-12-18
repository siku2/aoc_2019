use crate::input::Input;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::error::Error;

type Position = (usize, usize);
fn surrounding_positions((x, y): Position) -> Vec<Position> {
    let mut surroundings = vec![(x + 1, y), (x, y + 1)];

    if x > 0 {
        surroundings.push((x - 1, y));
    }
    if y > 0 {
        surroundings.push((x, y - 1));
    }

    surroundings
}
type DoorID = char;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct SharedState {
    keys: BTreeSet<DoorID>,
    positions: Vec<Position>,
}

impl SharedState {
    fn new(keys: BTreeSet<DoorID>, positions: Vec<Position>) -> Self {
        Self { keys, positions }
    }
}

#[derive(Debug)]
struct Vault {
    walls: HashSet<Position>,
    doors: HashMap<Position, DoorID>,
    keys: HashMap<Position, DoorID>,
    starts: Vec<Position>,
}

impl Vault {
    fn new(
        walls: HashSet<Position>,
        doors: HashMap<Position, DoorID>,
        keys: HashMap<Position, DoorID>,
        starts: Vec<Position>,
    ) -> Self {
        Self {
            walls,
            doors,
            keys,
            starts,
        }
    }

    fn from_input(i: &Input) -> Result<Vault, Box<dyn Error>> {
        let mut walls = HashSet::new();
        let mut doors = HashMap::new();
        let mut keys = HashMap::new();

        let mut starts = Vec::new();

        for (y, line) in i.lines().enumerate() {
            for (x, tile) in line.chars().enumerate() {
                match tile {
                    '#' => {
                        walls.insert((x, y));
                    }
                    '.' => (),
                    '@' => starts.push((x, y)),
                    _ => {
                        if tile.is_ascii_uppercase() {
                            doors.insert((x, y), tile);
                        } else {
                            keys.insert((x, y), tile.to_ascii_uppercase());
                        }
                    }
                }
            }
        }
        Ok(Vault::new(walls, doors, keys, starts))
    }

    fn split(&mut self) -> bool {
        if self.starts.len() != 1 {
            return false;
        }

        let (start_x, start_y) = self.starts[0];

        for x in start_x - 1..=start_x + 1 {
            self.walls.insert((x, start_y));
        }

        for y in start_y - 1..=start_y + 1 {
            self.walls.insert((start_x, y));
        }
        self.starts = vec![
            (start_x - 1, start_y - 1),
            (start_x - 1, start_y + 1),
            (start_x + 1, start_y + 1),
            (start_x + 1, start_y - 1),
        ];

        true
    }

    fn valid_position(&self, keys: &BTreeSet<DoorID>, pos: &Position) -> bool {
        if self.walls.contains(pos) {
            return false;
        }

        if let Some(door_id) = self.doors.get(pos) {
            return keys.contains(door_id);
        }

        return true;
    }

    fn bfs_keys(
        &self,
        keys: &BTreeSet<DoorID>,
        start_pos: Position,
    ) -> HashMap<DoorID, (Position, usize)> {
        let mut key_distances = HashMap::new();
        let mut seen = HashSet::new();
        seen.insert(start_pos);

        let mut queue = VecDeque::new();
        queue.push_back((start_pos, 0));

        while let Some((pos, prev_dist)) = queue.pop_front() {
            for pos in surrounding_positions(pos) {
                if !self.valid_position(keys, &pos) || seen.contains(&pos) {
                    continue;
                }
                seen.insert(pos);

                let dist = prev_dist + 1;
                if let Some(door_id) = self.keys.get(&pos) {
                    if !keys.contains(door_id) {
                        key_distances.insert(*door_id, (pos, dist));
                        continue;
                    }
                }

                queue.push_back((pos, dist));
            }
        }

        key_distances
    }

    fn steps_from_shared_state(
        &self,
        cache: &mut HashMap<SharedState, usize>,
        state: SharedState,
    ) -> usize {
        if let Some(dist) = cache.get(&state) {
            return *dist;
        }

        if state.keys.len() == self.keys.len() {
            cache.insert(state, 0);
            return 0;
        }

        let mut min_dist = std::usize::MAX;
        for (state_idx, pos) in state.positions.iter().enumerate() {
            let key_distances = self.bfs_keys(&state.keys, *pos);
            for (door_id, (key_pos, mut dist)) in key_distances {
                let mut next_state = state.clone();
                next_state.positions[state_idx] = key_pos;
                next_state.keys.insert(door_id);
                dist += self.steps_from_shared_state(cache, next_state);
                if dist < min_dist {
                    min_dist = dist;
                }
            }
        }

        cache.insert(state, min_dist);
        min_dist
    }

    fn steps_from_start(&self) -> usize {
        let mut cache = HashMap::new();
        let state = SharedState::new(BTreeSet::new(), self.starts.clone());
        self.steps_from_shared_state(&mut cache, state)
    }
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let vault = Vault::from_input(i)?;
    Ok(vault.steps_from_start().to_string())
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut vault = Vault::from_input(i)?;
    if !vault.split() {
        return Err("couldn't split vault".into());
    }
    Ok(vault.steps_from_start().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            first(&Input::new(
                "
                #########
                #b.A.@.a#
                #########
                "
            ))?,
            "8"
        );
        assert_eq!(
            first(&Input::new(
                "
                ########################
                #@..............ac.GI.b#
                ###d#e#f################
                ###A#B#C################
                ###g#h#i################
                ########################
                "
            ))?,
            "81"
        );

        Ok(())
    }
}
