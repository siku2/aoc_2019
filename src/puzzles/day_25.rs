use crate::input::Input;
use crate::lib::intcode;
use std::collections::HashSet;
use std::error;

type Error = Box<dyn error::Error>;

const PRESSURE_SENSITIVE_FLOOR: &str = "== Pressure-Sensitive Floor ==";
const TOO_HEAVY: &str = "Alert! Droids on this ship are heavier than the detected value!";
const TOO_LIGHT: &str = "Alert! Droids on this ship are lighter than the detected value!";

const CANT_GO_THAT_WAY: &str = "You can't go that way.";

struct DummyRand {
    state: usize,
}

impl DummyRand {
    const MAX: usize = std::usize::MAX;
    const MAGIC: usize = 0x2545_F491_4F6C_DD1D;

    fn new(seed: usize) -> Self {
        Self { state: seed }
    }

    fn rand(&mut self) -> usize {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(Self::MAGIC)
    }

    fn max(&mut self, n: usize) -> usize {
        let g = Self::MAX - (Self::MAX % (n + 1));
        let mut r;
        loop {
            r = self.rand();
            if r < g {
                break;
            }
        }

        r % (n + 1)
    }

    fn range(&mut self, min: usize, max: usize) -> usize {
        debug_assert!(min <= max);
        let diff = max - min;
        self.max(diff) + min
    }

    fn bool(&mut self) -> bool {
        self.rand() % 2 == 0
    }

    fn shuffle<T>(&mut self, elements: &mut [T]) {
        for i in 0..elements.len() - 2 {
            let j = self.range(i, elements.len() - 1);
            elements.swap(i, j);
        }
    }

    fn binary_sample<'a, T>(
        &'a mut self,
        it: impl Iterator<Item = T> + 'a,
    ) -> impl Iterator<Item = T> + 'a {
        it.filter(move |_| self.bool())
    }
}

struct Droid {
    original_machine: intcode::Machine,
    rng: DummyRand,
    forbidden: HashSet<String>,
    all_items: HashSet<String>,
    machine: intcode::Machine,
    items: HashSet<String>,
}

impl<'a> Droid {
    const DIRECTIONS: [&'a str; 4] = ["east", "west", "north", "south"];

    fn new(mut m: intcode::Machine) -> Self {
        m.start();
        let mut forbidden = HashSet::new();
        forbidden.insert(String::from("infinite loop"));
        forbidden.insert(String::from("giant electromagnet"));
        Self {
            machine: m.clone(),
            original_machine: m,
            rng: DummyRand::new(1),
            forbidden,
            all_items: HashSet::new(),
            items: HashSet::new(),
        }
    }

    fn from_input(i: &Input) -> Result<Self, Error> {
        intcode::Machine::from_input(i).map(Self::new)
    }

    fn reset_machine(&mut self) {
        self.machine = self.original_machine.clone();
    }

    fn reset(&mut self) {
        self.reset_machine();

        self.items = self
            .rng
            .binary_sample(self.all_items.iter())
            .cloned()
            .collect();
    }

    fn handle_pressure_sensitive_floor(&mut self, output: &str) -> Option<usize> {
        if output.contains(TOO_HEAVY) || output.contains(TOO_LIGHT) {
            self.reset();
            return None;
        }

        output
            .split_ascii_whitespace()
            .filter_map(|w| w.parse::<usize>().ok())
            .next()
    }

    fn handle_items(&mut self, output: &str) -> Result<bool, Error> {
        for line in output.lines() {
            if !line.starts_with("- ") {
                continue;
            }

            let item = line[2..].to_string();
            if Self::DIRECTIONS.contains(&item.as_str()) || self.forbidden.contains(&item) {
                continue;
            }

            if !self.all_items.contains(&item) {
                self.all_items.insert(item.clone());
            } else if !self.items.contains(&item) {
                continue;
            }

            self.machine.send_ascii(&format!("take {}\n", item))?;

            if self.machine.is_done() {
                self.items.remove(&item);
                self.forbidden.insert(item.clone());
                self.reset_machine();
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn choose_next_dir(&mut self, output: &str) -> Result<(), Error> {
        let mut directions: Vec<&str> = Self::DIRECTIONS.to_vec();
        self.rng.shuffle(&mut directions);

        let dir = directions
            .drain(..)
            .find(|dir| output.contains(dir))
            .ok_or_else(|| "no possible direction")?;

        self.machine.send_ascii(dir)?;
        self.machine.send_ascii("\n")?;

        Ok(())
    }

    fn run_until_next_command(&mut self) -> Result<String, Error> {
        self.machine.run_until_stop()?;
        let output = self
            .machine
            .take_ascii_output()
            .ok_or_else(|| "non-ascii output")?;

        if output.contains(CANT_GO_THAT_WAY) {
            return Err("reached invalid position".into());
        }

        Ok(output)
    }

    fn run(&mut self) -> Result<usize, Error> {
        loop {
            let output = self.run_until_next_command()?;
            if output.contains(PRESSURE_SENSITIVE_FLOOR) {
                if let Some(result) = self.handle_pressure_sensitive_floor(&output) {
                    return Ok(result);
                }
                continue;
            }

            if !self.handle_items(&output)? {
                continue;
            }

            self.choose_next_dir(&output)?;
        }
    }
}

pub fn first(i: &Input) -> Result<String, Error> {
    let mut droid = Droid::from_input(i)?;
    droid.run().map(|v| v.to_string())
}

pub fn second(i: &Input) -> Result<String, Error> {
    unimplemented!()
}
