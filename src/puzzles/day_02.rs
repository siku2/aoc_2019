use crate::input::Input;
use std::error::Error;

type OpCode = usize;
type Program = Vec<OpCode>;

const OP_ADD: OpCode = 1;
const OP_MUL: OpCode = 2;
const OP_HALT: OpCode = 99;

fn add(p: &mut Program, a: OpCode, b: OpCode, s: OpCode) {
    let index = p[s];
    p[index] = p[p[a]] + p[p[b]];
}
fn mul(p: &mut Program, a: OpCode, b: OpCode, s: OpCode) {
    let index = p[s];
    p[index] = p[p[a]] * p[p[b]];
}

fn run_program(mut program: Program) -> Result<OpCode, Box<dyn Error>> {
    let mut index = 0;
    loop {
        let op = program[index];
        match op {
            OP_ADD => add(&mut program, index + 1, index + 2, index + 3),
            OP_MUL => mul(&mut program, index + 1, index + 2, index + 3),
            OP_HALT => break,
            _ => return Err("unknown opcode".into()),
        }

        index += 4;
    }

    Ok(program[0])
}

fn run_program_with_input(
    mut program: Program,
    noun: OpCode,
    verb: OpCode,
) -> Result<OpCode, Box<dyn Error>> {
    program[1] = noun;
    program[2] = verb;

    run_program(program)
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let program: Program = i.parse_csv().collect::<Result<Vec<_>, _>>()?;
    run_program_with_input(program, 12, 1).and_then(|i| Ok(i.to_string()))
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let program: Program = i.parse_csv().collect::<Result<Vec<_>, _>>()?;

    let mut solution: Option<_> = None;

    for noun in 0..100 {
        for verb in 0..100 {
            let res = run_program_with_input(program.clone(), noun, verb)?;
            if res == 19_690_720 {
                solution = Some((noun, verb));
                break;
            }
        }
    }

    solution
        .map(|r| (100 * r.0 + r.1).to_string())
        .ok_or_else(|| "no possibility found".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            run_program([1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50].to_vec())?,
            3500
        );
        assert_eq!(run_program([1, 0, 0, 0, 99].to_vec())?, 2);
        assert_eq!(run_program([2, 3, 0, 3, 99].to_vec())?, 2);
        assert_eq!(run_program([2, 4, 4, 5, 99, 0].to_vec())?, 2);
        assert_eq!(run_program([1, 1, 1, 4, 99, 5, 6, 0, 99].to_vec())?, 30);

        Ok(())
    }
}
