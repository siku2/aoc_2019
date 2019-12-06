use crate::input::Input;
use std::cmp;
use std::collections::HashMap;
use std::error::Error;

type Object<'a> = &'a str;
type Orbit<'a> = (Object<'a>, Object<'a>);
type OrbitList<'a> = Vec<Orbit<'a>>;
type OrbitMap<'a> = HashMap<Object<'a>, Object<'a>>;

fn orbits_from_input(i: &Input) -> Result<OrbitList, Box<dyn Error>> {
    let orbits: Option<Vec<Orbit>> = i
        .map_lines(|l| {
            let mut it = l.split(')');

            let o1 = it.next()?;
            let o2 = it.next()?;

            Some((o1, o2))
        })
        .collect();

    orbits.ok_or_else(|| "invalid orbits".into())
}

fn build_orbit_map(orbits: OrbitList) -> OrbitMap {
    let mut orbit_map: OrbitMap = HashMap::new();
    for (o1, o2) in orbits {
        orbit_map.insert(o2, o1);
    }

    orbit_map
}

fn get_orbit_depth(orbits: &OrbitMap, value: Object) -> usize {
    if let Some(v) = orbits.get(value) {
        get_orbit_depth(orbits, v) + 1
    } else {
        1
    }
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let orbits = build_orbit_map(orbits_from_input(i)?);
    Ok(orbits
        .keys()
        .map(|k| get_orbit_depth(&orbits, k) - 1)
        .sum::<usize>()
        .to_string())
}

fn get_path<'a>(orbits: &OrbitMap<'a>, start: Object<'a>) -> Vec<Object<'a>> {
    let mut path = vec![start];

    let mut current = start;
    while let Some(next) = orbits.get(current) {
        path.push(next);
        current = next;
    }

    path.reverse();

    path
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let orbits = build_orbit_map(orbits_from_input(i)?);

    let path_to_you = get_path(&orbits, "YOU");
    let path_to_santa = get_path(&orbits, "SAN");

    let min_len = cmp::min(path_to_you.len(), path_to_santa.len());
    let common = (0..min_len)
        .take_while(|&i| path_to_you[i] == path_to_santa[i])
        .last()
        .ok_or_else(|| "transfer impossible")?;
    let you_to_common = path_to_you.len() - common - 2;
    let santa_to_common = path_to_santa.len() - common - 2;

    Ok((you_to_common + santa_to_common).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            first(&Input::new(
                "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"
            ))?,
            "42"
        );

        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            second(&Input::new(
                "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"
            ))?,
            "4"
        );

        Ok(())
    }
}
