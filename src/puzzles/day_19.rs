use crate::input::Input;
use crate::lib::intcode::{Code, Machine};
use std::cmp;
use std::error::Error;

fn scan_once(mut m: Machine, x: Code, y: Code) -> Result<bool, Box<dyn Error>> {
    m.run(&[x, y])?
        .first()
        .ok_or_else(|| "no output from drone".into())
        .map(|out| *out == 1)
}

fn count_beam_area(m: &Machine, area: usize) -> Result<usize, Box<dyn Error>> {
    let mut sum = 0;
    for x in 0..area {
        for y in 0..area {
            if scan_once(m.clone(), x as Code, y as Code)? {
                sum += 1;
            }
        }
    }

    Ok(sum)
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let m = Machine::from_input(i)?;
    count_beam_area(&m, 50).map(|c| c.to_string())
}

fn find_beam_start(
    m: &Machine,
    x: Code,
    start: usize,
    beam: bool,
) -> Result<usize, Box<dyn Error>> {
    let mut y = start;
    loop {
        if scan_once(m.clone(), x, y as Code)? == beam {
            break;
        }
        y += 1;
    }

    Ok(y)
}

fn get_beam_bounds(
    m: &Machine,
    x: Code,
    prev_bounds: (usize, usize),
) -> Result<(usize, usize), Box<dyn Error>> {
    let start_y = find_beam_start(m, x, prev_bounds.0, true)?;
    let end_y = find_beam_start(m, x, cmp::max(prev_bounds.1, start_y), false)?;
    Ok((start_y, end_y))
}

fn find_area(m: &Machine, width: usize) -> Result<(usize, usize), Box<dyn Error>> {
    if width == 0 {
        return Ok((0, 0));
    }
    let width_i = width - 1;
    let mut left_bounds = (0, 0);
    let mut right_bounds = (0, 0);
    let bounds_at = |x, bounds: &mut (usize, usize)| -> Result<(usize, usize), Box<dyn Error>> {
        *bounds = get_beam_bounds(m, x as Code, *bounds)?;
        Ok(*bounds)
    };

    let mut start_x: usize = 0;
    loop {
        let (end_y_start, end_y_end) = bounds_at(start_x + width_i, &mut right_bounds)?;
        if end_y_end - end_y_start < width {
            start_x += 1;
            continue;
        }
        let (_, start_y_end) = bounds_at(start_x, &mut left_bounds)?;

        let height = start_y_end.checked_sub(end_y_start).unwrap_or_default();
        if height >= width {
            return Ok((start_x, end_y_start));
        }

        start_x += 1;
    }
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let m = Machine::from_input(i)?;
    find_area(&m, 100).map(|(x, y)| (10_000 * x + y).to_string())
}
