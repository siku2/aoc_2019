use crate::input::Input;
use crate::lib::intcode::{Code, Machine};
use std::error::Error;

fn get_output(out: &[Code]) -> Result<String, Box<dyn Error>> {
    if out.len() != 1 {
        return Err(format!("something went wrong: {:?}", out).into());
    }
    Ok(out.last().unwrap().to_string())
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut m = Machine::from_input(i)?;
    let out = m.run(&[1])?;
    get_output(&out)
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut m = Machine::from_input(i)?;
    let out = m.run(&[2])?;
    get_output(&out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine() -> Result<(), Box<dyn Error>> {
        let mut m = Machine::from_input(&Input::new(
            "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
        ))?;
        assert_eq!(
            m.run(&[])?,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99],
        );

        let mut m = Machine::from_input(&Input::new("1102,34915192,34915192,7,4,7,99,0"))?;
        assert_eq!(m.run(&[])?, vec![1219070632396864]);

        let mut m = Machine::from_input(&Input::new("104,1125899906842624,99"))?;
        assert_eq!(m.run(&[])?, vec![1125899906842624]);

        Ok(())
    }
}
