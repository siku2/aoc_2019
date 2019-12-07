use crate::input::Input;
use crate::lib::intcode::Machine;
use std::error::Error;

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut m = Machine::from_input(i)?;
    let output = m.run(&[1])?;

    output
        .last()
        .map(|v| v.to_string())
        .ok_or_else(|| "failed to get diagnostics".into())
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut m = Machine::from_input(i)?;
    let output = m.run(&[5])?;

    if output.len() != 1 {
        return Err("mismatched length".into());
    }

    Ok(output.first().unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        let m = Machine::from_input(&Input::new("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"))?;

        assert_eq!(m.clone().run(&[8])?, vec!(1000));
        assert_eq!(m.clone().run(&[7])?, vec!(999));
        assert_eq!(m.clone().run(&[9])?, vec!(1001));

        Ok(())
    }
}
