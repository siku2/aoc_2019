use crate::input::Input;

fn fuel_for_mass(mass: usize) -> usize {
    let third = mass / 3;
    if third < 2 {
        return 0;
    }

    return third - 2;
}

fn total_fuel_for_mass(mass: usize) -> usize {
    let fuel = fuel_for_mass(mass);
    if fuel <= 0 {
        return 0;
    }

    return fuel + total_fuel_for_mass(fuel);
}

pub fn first(i: &Input) -> String {
    i.usize_lines().map(fuel_for_mass).sum::<usize>().to_string()
}

pub fn second(i: &Input) -> String {
    i.usize_lines().map(total_fuel_for_mass).sum::<usize>().to_string()
}