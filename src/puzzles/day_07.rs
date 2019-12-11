use crate::input::Input;
use crate::lib::intcode::{Code, Machine};
use std::error::Error;

type AmplifierMachines = [Machine; 5];

fn make_machines(m: Machine) -> AmplifierMachines {
    [m.clone(), m.clone(), m.clone(), m.clone(), m.clone()]
}

type PhaseSettings = [Code; 5];

fn run_machines(
    machines: &mut AmplifierMachines,
    phases: &PhaseSettings,
) -> Result<Code, Box<dyn Error>> {
    let mut input: Code = 0;
    for (m, &phase) in machines.iter_mut().zip(phases.iter()) {
        let out = m.run(&[phase, input])?;
        input = out.get(0).copied().ok_or_else(|| "missing output")?;
    }

    Ok(input)
}

/// Generate all permutations of the given phase settings.
/// This is an implementation of [Heap's Algorithm](https://en.wikipedia.org/wiki/Heap%27s_algorithm)
/// but limited to 5 elements.
fn generate_permutations(items: &mut PhaseSettings) -> Vec<PhaseSettings> {
    let mut perms = Vec::new();

    fn permutations(k: usize, items: &mut PhaseSettings, perms: &mut Vec<PhaseSettings>) {
        if k == 1 {
            perms.push(items.clone());
            return;
        }

        permutations(k - 1, items, perms);

        for i in 0..k - 1 {
            match k % 2 {
                0 => items.swap(i, k - 1),
                1 => items.swap(0, k - 1),
                _ => unreachable!(),
            }

            permutations(k - 1, items, perms);
        }
    }

    permutations(items.len(), items, &mut perms);
    perms
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let machines = make_machines(Machine::from_input(i)?);
    let perms = generate_permutations(&mut [0, 1, 2, 3, 4]);

    let mut max_score = 0;
    for perm in perms {
        let score = run_machines(&mut machines.clone(), &perm)?;
        if score > max_score {
            max_score = score;
        }
    }

    Ok(max_score.to_string())
}

fn run_machines_loop(
    machines: &mut AmplifierMachines,
    phases: &PhaseSettings,
) -> Result<Code, Box<dyn Error>> {
    let mut input: Code = 0;
    for (m, &phase) in machines.iter_mut().zip(phases.iter()) {
        m.start();
        m.send(phase)?;
        m.send(input)?;
        input = *m.take_output().last().unwrap();
    }

    loop {
        for m in machines.iter_mut() {
            m.send(input)?;
            input = *m.take_output().last().unwrap();
        }

        if machines.last().unwrap().is_done() {
            break;
        }
    }

    Ok(input)
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let machines = make_machines(Machine::from_input(i)?);
    let perms = generate_permutations(&mut [5, 6, 7, 8, 9]);

    let mut max_score = 0;
    for perm in perms {
        let score = run_machines_loop(&mut machines.clone(), &perm)?;
        if score > max_score {
            max_score = score;
        }
    }

    Ok(max_score.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_machines() -> Result<(), Box<dyn Error>> {
        let mut machines = make_machines(Machine::from_input(&Input::new(
            "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0",
        ))?);
        assert_eq!(run_machines(&mut machines, &[4, 3, 2, 1, 0])?, 43210);

        let mut machines = make_machines(Machine::from_input(&Input::new(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        ))?);
        assert_eq!(run_machines(&mut machines, &[0, 1, 2, 3, 4])?, 54321);

        let mut machines = make_machines(Machine::from_input(&Input::new(
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
        ))?);
        assert_eq!(run_machines(&mut machines, &[1, 0, 4, 3, 2])?, 65210);

        Ok(())
    }

    #[test]
    fn test_run_machines_loop() -> Result<(), Box<dyn Error>> {
        let mut machines = make_machines(Machine::from_input(&Input::new(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        ))?);
        assert_eq!(
            run_machines_loop(&mut machines, &[9, 8, 7, 6, 5])?,
            139629729
        );

        let mut machines = make_machines(Machine::from_input(&Input::new(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
        ))?);
        assert_eq!(run_machines_loop(&mut machines, &[9, 7, 8, 5, 6])?, 18216);

        Ok(())
    }
}
