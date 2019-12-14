use crate::input::Input;
use std::cmp;
use std::collections::HashSet;
use std::error::Error;

/// Represents coordinates
type Coords = (usize, usize);
type RelCoords = (isize, isize);

/// Get the angle between the direction and straight up.
fn rel_angle(rel: RelCoords) -> f32 {
    (rel.0 as f32).atan2(rel.1 as f32)
}

fn coords_to_rel(c: Coords) -> RelCoords {
    (c.0 as isize, c.1 as isize)
}

fn add_rel_to_coords(c: Coords, rel: RelCoords) -> RelCoords {
    let cr = coords_to_rel(c);
    (cr.0 + rel.0, cr.1 + rel.1)
}

fn sub_coords(a: Coords, b: Coords) -> RelCoords {
    let ar = coords_to_rel(a);
    let br = coords_to_rel(b);
    (ar.0 - br.0, ar.1 - br.1)
}

/// Represents the asteroid map.
type AsteroidMap = HashSet<Coords>;

/// Calculate the greatest common divisor of two numbers.
fn calc_gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        return a;
    }

    calc_gcd(b, a % b)
}

/// Get the direction vector of relative coordinates.
fn get_direction(pos: RelCoords) -> RelCoords {
    let gcd = calc_gcd(pos.0, pos.1).abs();
    (pos.0 / gcd, pos.1 / gcd)
}

/// Generate direction vectors for a square's border with a given radius.
/// Starts at 12 o'clock and rotates clockwise.
fn iter_square_offsets(rad: isize) -> impl Iterator<Item = RelCoords> {
    let irad = rad - 1;

    let top_left = (-rad..0).map(move |n| (n, rad));
    let top_right = (0..rad).map(move |n| (n, rad));
    let right = (-irad..=rad).rev().map(move |n| (rad, n));
    let bot = (-irad..=rad).rev().map(move |n| (n, -rad));
    let left = (-rad..rad).map(move |n| (-rad, n));

    top_right
        .chain(right)
        .chain(bot)
        .chain(left)
        .chain(top_left)
}

/// Generate offsets for all possible positions in an area with the given radius.
/// The origin (0, 0) is excluded and the max radius is inclusive.
/// Offsets are returned in clockwise rotation starting from 12 o'clock layer by layer.
fn iter_area_offsets(max_radius: isize) -> impl Iterator<Item = RelCoords> {
    (1..=max_radius).flat_map(iter_square_offsets)
}

/// Get the biggest distance to a border for a position.
fn get_max_radius(size: Coords, pos: Coords) -> usize {
    let max_x = cmp::max(pos.0, size.0 - pos.0 - 1);
    let max_y = cmp::max(pos.1, size.1 - pos.1 - 1);
    cmp::max(max_x, max_y)
}

/// Generate all visible asteroids from a position.
fn ray_cast_from_pos<'a>(
    map: &'a AsteroidMap,
    size: Coords,
    start: Coords,
) -> impl Iterator<Item = Coords> + 'a {
    let max_scan_radius = get_max_radius(size, start) as isize;
    let mut invalids_dirs = HashSet::new();
    iter_area_offsets(max_scan_radius).filter_map(move |offset| {
        let (x, y) = add_rel_to_coords(start, offset);
        if x < 0 || y < 0 {
            return None;
        }

        let pos = (x as usize, y as usize);

        if !map.contains(&pos) {
            return None;
        }

        let dir = get_direction(offset);
        if !invalids_dirs.insert(dir) {
            return None;
        }

        Some(pos)
    })
}

fn parse_input(i: &Input) -> (AsteroidMap, Coords) {
    let map = i
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(x, c)| if c == '#' { Some((x, y)) } else { None })
        })
        .collect::<AsteroidMap>();

    let mut max_x = 0;
    let mut max_y = 0;
    for &(x, y) in map.iter() {
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    }

    (map, (max_x + 1, max_y + 1))
}

/// Find the coordinates of the asteroid with the most visible other asteroids.
/// Returns the amount of other asteroids and its position.
fn find_most_visible_asteroids(map: &AsteroidMap, size: Coords) -> (usize, Coords) {
    let mut max = 0;
    let mut max_pos = (0, 0);

    for &pos in map {
        let c = ray_cast_from_pos(map, size, pos).count();
        if c > max {
            max = c;
            max_pos = pos;
        }
    }

    (max, max_pos)
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let (map, size) = parse_input(i);
    let (count, _) = find_most_visible_asteroids(&map, size);

    Ok(count.to_string())
}

/// Get the coordinates of the nth asteroid destroyed by a laser rotation clockwise.
fn get_nth_lasered_asteroid(
    map: &mut AsteroidMap,
    size: Coords,
    start: Coords,
    n: usize,
) -> Coords {
    let mut hits: Vec<_>;
    let mut total = 0;

    // loop through as many rotations of the laser as required to reach a total of n hits
    loop {
        hits = ray_cast_from_pos(map, size, start).collect();
        total += hits.len();
        if total >= n {
            break;
        }

        for pos in hits {
            map.remove(&pos);
        }
    }

    // sort all hits by their angle from 12 o'clock
    hits.sort_by(|&p1, &p2| {
        let a1 = rel_angle(sub_coords(p1, start));
        let a2 = rel_angle(sub_coords(p2, start));

        a1.partial_cmp(&a2).unwrap()
    });

    hits[total - n]
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let (mut map, size) = parse_input(i);
    let (_, pos) = find_most_visible_asteroids(&map, size);

    let last_asteroid = get_nth_lasered_asteroid(&mut map, size, pos, 200);

    Ok((100 * last_asteroid.0 + last_asteroid.1).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_spiral_offset() {
        assert_eq!(
            iter_square_offsets(1).collect::<Vec<_>>(),
            vec![
                (0, 1),
                (1, 1),
                (1, 0),
                (1, -1),
                (0, -1),
                (-1, -1),
                (-1, 0),
                (-1, 1),
            ]
        );
        assert_eq!(
            iter_square_offsets(2).collect::<Vec<_>>(),
            vec![
                (0, 2),
                (1, 2),
                (2, 2),
                (2, 1),
                (2, 0),
                (2, -1),
                (2, -2),
                (1, -2),
                (0, -2),
                (-1, -2),
                (-2, -2),
                (-2, -1),
                (-2, 0),
                (-2, 1),
                (-2, 2),
                (-1, 2),
            ]
        );

        for i in 1..10 {
            assert_eq!(
                iter_area_offsets(i).count() as isize,
                (2 * i + 1).pow(2) - 1,
            );
        }
    }

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            first(&Input::new(
                "
                .#..#
                .....
                #####
                ....#
                ...##
                "
            ))?,
            "8"
        );
        assert_eq!(
            first(&Input::new(
                "
                ......#.#.
                #..#.#....
                ..#######.
                .#.#.###..
                .#..#.....
                ..#....#.#
                #..#....#.
                .##.#..###
                ##...#..#.
                .#....####
                "
            ))?,
            "33"
        );
        assert_eq!(
            first(&Input::new(
                "
                .#..##.###...#######
                ##.############..##.
                .#.######.########.#
                .###.#######.####.#.
                #####.##.#.##.###.##
                ..#####..#.#########
                ####################
                #.####....###.#.#.##
                ##.#################
                #####.##.###..####..
                ..######..##.#######
                ####.##.####...##..#
                .#####..#.######.###
                ##...#.##########...
                #.##########.#######
                .####.#.###.###.#.##
                ....##.##.###..#####
                .#.#.###########.###
                #.#.#.#####.####.###
                ###.##.####.##.#..##
                "
            ))?,
            "210"
        );

        Ok(())
    }

    #[test]
    fn test_laser() -> Result<(), Box<dyn Error>> {
        let (mut map, size) = parse_input(&Input::new(
            "
            .#..##.###...#######
            ##.############..##.
            .#.######.########.#
            .###.#######.####.#.
            #####.##.#.##.###.##
            ..#####..#.#########
            ####################
            #.####....###.#.#.##
            ##.#################
            #####.##.###..####..
            ..######..##.#######
            ####.##.####...##..#
            .#####..#.######.###
            ##...#.##########...
            #.##########.#######
            .####.#.###.###.#.##
            ....##.##.###..#####
            .#.#.###########.###
            #.#.#.#####.####.###
            ###.##.####.##.#..##
            ",
        ));
        assert_eq!(
            get_nth_lasered_asteroid(&mut map, size, (11, 13), 200),
            (8, 2)
        );

        Ok(())
    }
}
