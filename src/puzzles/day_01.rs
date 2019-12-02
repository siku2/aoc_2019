use crate::input::Input;
use std::error::Error;

fn fuel_for_mass(mass: usize) -> usize {
    let third = mass / 3;
    if third < 2 {
        return 0;
    }

    third - 2
}

fn total_fuel_for_mass(mass: usize) -> usize {
    let fuel = fuel_for_mass(mass);
    if fuel == 0 {
        return 0;
    }

    fuel + total_fuel_for_mass(fuel)
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut masses = i.parse_lines().collect::<Result<Vec<_>, _>>()?;

    Ok(masses
        .drain(..)
        .map(fuel_for_mass)
        .sum::<usize>()
        .to_string())
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut masses = i.parse_lines().collect::<Result<Vec<_>, _>>()?;

    Ok(masses
        .drain(..)
        .map(total_fuel_for_mass)
        .sum::<usize>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        assert_eq!(first(&Input::new("12"))?, "2");
        assert_eq!(first(&Input::new("14"))?, "2");
        assert_eq!(first(&Input::new("1969"))?, "654");
        assert_eq!(first(&Input::new("100756"))?, "33583");

        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        assert_eq!(second(&Input::new("14"))?, "2");
        assert_eq!(second(&Input::new("1969"))?, "966");
        assert_eq!(second(&Input::new("100756"))?, "50346");

        Ok(())
    }
}
