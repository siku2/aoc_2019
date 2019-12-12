use crate::input::Input;
use std::error::Error;
use std::fmt;
use std::ops;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Vec3 {
    const ZERO: Vec3 = Vec3::new(0, 0, 0);

    const fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn from_string(mut s: &str) -> Result<Self, Box<dyn Error>> {
        s = s.trim_start_matches('<').trim_end_matches('>');
        let parts = s
            .split(',')
            .map(|s| {
                let raw = s.trim().split('=').last().ok_or_else(|| "invalid format")?;
                raw.parse().map_err(|_| "parse error")
            })
            .take(3)
            .collect::<Result<Vec<isize>, _>>()?;

        if parts.len() != 3 {
            return Err("invalid format".into());
        }

        Ok(Self::new(parts[0], parts[1], parts[2]))
    }

    fn abs_x(&self) -> usize {
        self.x.abs() as usize
    }

    fn abs_y(&self) -> usize {
        self.y.abs() as usize
    }

    fn abs_z(&self) -> usize {
        self.z.abs() as usize
    }

    fn abs_component_sum(&self) -> usize {
        self.abs_x() + self.abs_y() + self.abs_z()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<x=")?;
        self.x.fmt(f)?;
        write!(f, ", y=")?;
        self.y.fmt(f)?;
        write!(f, ", z=")?;
        self.z.fmt(f)?;

        write!(f, ">")?;

        Ok(())
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Moon {
    position: Vec3,
    velocity: Vec3,
}

impl Moon {
    fn new(pos: Vec3) -> Self {
        Self {
            position: pos,
            velocity: Vec3::ZERO,
        }
    }

    fn potential_energy(&self) -> usize {
        self.position.abs_component_sum()
    }

    fn kinetic_energy(&self) -> usize {
        self.velocity.abs_component_sum()
    }

    fn total_energy(&self) -> usize {
        self.potential_energy() * self.kinetic_energy()
    }

    fn apply_gravity(ma: &mut Moon, mb: &mut Moon) {
        let (a_pos, b_pos) = (ma.position, mb.position);
        let (a_vel, b_vel) = (&mut ma.velocity, &mut mb.velocity);

        let x = (a_pos.x - b_pos.x).signum();
        a_vel.x -= x;
        b_vel.x += x;

        let y = (a_pos.y - b_pos.y).signum();
        a_vel.y -= y;
        b_vel.y += y;

        let z = (a_pos.z - b_pos.z).signum();
        a_vel.z -= z;
        b_vel.z += z;
    }

    fn apply_velocity(&mut self) {
        self.position += self.velocity;
    }
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(pos={:3}, vel={:3})", self.position, self.velocity)
    }
}

fn parse_input(i: &Input) -> Result<Vec<Moon>, Box<dyn Error>> {
    i.map_lines(|l| Vec3::from_string(l.trim()).map(Moon::new))
        .collect()
}

fn step(moons: &mut Vec<Moon>) {
    for i in 1..=moons.len() {
        let (left, right) = moons.split_at_mut(i);
        let mut moon_a = left.last_mut().unwrap();

        for mut moon_b in right.iter_mut() {
            Moon::apply_gravity(&mut moon_a, &mut moon_b);
        }

        moon_a.apply_velocity();
    }
}
fn calc_total_energy(moons: &[Moon]) -> usize {
    moons.iter().map(Moon::total_energy).sum()
}

fn steps(moons: &mut Vec<Moon>, n: usize) -> usize {
    for _ in 0..n {
        step(moons);
    }

    calc_total_energy(moons)
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut moons = parse_input(i)?;

    Ok(steps(&mut moons, 1000).to_string())
}

fn calc_lcm(a: isize, b: isize) -> isize {
    fn calc_gcd(a: isize, b: isize) -> isize {
        if b == 0 {
            return a;
        }
        calc_gcd(b, a % b)
    }
    (a * b).abs() / calc_gcd(a, b)
}

type AxisTuples = Vec<(isize, isize)>;

fn get_state_tuples(moons: &[Moon]) -> (AxisTuples, AxisTuples, AxisTuples) {
    let mut x = Vec::with_capacity(moons.len());
    let mut y = Vec::with_capacity(moons.len());
    let mut z = Vec::with_capacity(moons.len());

    for m in moons {
        x.push((m.position.x, m.velocity.x));
        y.push((m.position.y, m.velocity.y));
        z.push((m.position.z, m.velocity.z));
    }

    (x, y, z)
}

fn find_total_period(moons: &mut Vec<Moon>) -> usize {
    let (init_x, init_y, init_z) = get_state_tuples(&moons);

    let mut periods = Vec3::ZERO;

    let mut s = 0;
    while periods.x == 0 || periods.y == 0 || periods.z == 0 {
        step(moons);
        s += 1;

        let (x, y, z) = get_state_tuples(&moons);
        if periods.x == 0 && x == init_x {
            periods.x = s;
        }

        if periods.y == 0 && y == init_y {
            periods.y = s;
        }

        if periods.z == 0 && z == init_z {
            periods.z = s;
        }
    }

    calc_lcm(periods.x, calc_lcm(periods.y, periods.z)) as usize
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut moons = parse_input(i)?;
    Ok(find_total_period(&mut moons).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        let mut moons = parse_input(&Input::new(
            "
            <x=-1, y=0, z=2>
            <x=2, y=-10, z=-7>
            <x=4, y=-8, z=8>
            <x=3, y=5, z=-1>
            ",
        ))?;
        assert_eq!(steps(&mut moons, 10), 179);

        Ok(())
    }
}
