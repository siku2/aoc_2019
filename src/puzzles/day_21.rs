use crate::input::Input;
use crate::lib::intcode;
use std::error::Error;

fn render_output(out: &[intcode::Code]) -> String {
    let mut s = String::with_capacity(out.len());
    s.push_str("Failed:\n");
    s.extend(out.iter().map(|c| *c as u8 as char));
    s
}

fn get_output(m: &mut intcode::Machine) -> Result<String, Box<dyn Error>> {
    let out = m.take_output();
    let last = *out.last().ok_or_else(|| "no output")?;
    if last > std::u8::MAX as isize {
        Ok(last.to_string())
    } else {
        Ok(render_output(&out))
    }
}

fn send_code(m: &mut intcode::Machine, code: &str) -> Result<(), Box<dyn Error>> {
    for instr in code.lines() {
        let instr = instr.trim();
        if instr.is_empty() {
            continue;
        }

        m.send_ascii(instr)?;
        m.send(10)?;
    }

    Ok(())
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut m = intcode::Machine::from_input(i)?;
    m.start();

    // (!A or !B or !C) and D
    send_code(
        &mut m,
        "
        OR A T
        AND B T
        AND C T
        NOT T J
        AND D J        
        ",
    )?;
    m.output.clear();
    m.send_ascii("WALK\n")?;

    get_output(&mut m)
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut m = intcode::Machine::from_input(i)?;
    m.start();

    // (!A or !B or !C) and (E or H) and D
    send_code(
        &mut m,
        "
        OR A T
        AND B T
        AND C T
        NOT T T
        OR E J
        OR H J
        AND T J
        AND D J        
        ",
    )?;
    m.output.clear();
    m.send_ascii("RUN\n")?;

    get_output(&mut m)
}
