use crate::input::Input;
use std::collections::VecDeque;
use std::error::Error;

type Code = isize;

const OP_ADD: Code = 1;
const OP_MUL: Code = 2;
const OP_INP: Code = 3;
const OP_OUT: Code = 4;
const OP_JIT: Code = 5;
const OP_JIF: Code = 6;
const OP_LT: Code = 7;
const OP_EQ: Code = 8;

const OP_HALT: Code = 99;

#[derive(Clone)]
struct IntMachine {
    code: Vec<Code>,
    instr_ptr: usize,
    input: VecDeque<Code>,
    output: Vec<Code>,
    debug: bool,
}

impl IntMachine {
    fn new(code: Vec<Code>) -> IntMachine {
        IntMachine {
            code,
            instr_ptr: 0,
            input: VecDeque::new(),
            output: Vec::new(),
            debug: false,
        }
    }

    fn from_input(i: &Input) -> Result<IntMachine, Box<dyn Error>> {
        Ok(Self::new(i.parse_csv().collect::<Result<_, _>>()?))
    }

    fn get_raw_param(&self, param: usize) -> Result<Code, Box<dyn Error>> {
        self.code
            .get(self.instr_ptr + (param + 1))
            .copied()
            .ok_or_else(|| "invalid index".into())
    }

    fn get_param(&self, param_modes: Code, param: usize) -> Result<Code, Box<dyn Error>> {
        let value = self.get_raw_param(param)?;
        let mode = (param_modes / 10isize.pow(param as u32)) % 10;

        match mode {
            0 => self
                .code
                .get(value as usize)
                .copied()
                .ok_or_else(|| "invalid index value".into()),
            1 => Ok(value),
            _ => Err("invalid param mode".into()),
        }
    }

    fn run_once(&mut self) -> Result<bool, Box<dyn Error>> {
        let instruction = *self.code.get(self.instr_ptr).ok_or("no opcode")?;
        let (opcode, param_modes) = (instruction % 100, instruction / 100);

        if self.debug {
            println!(
                "STEP: instr={} op={}, modes={}",
                self.instr_ptr, opcode, param_modes
            );
        }

        match opcode {
            OP_ADD => {
                let res = self.get_param(param_modes, 0)? + self.get_param(param_modes, 1)?;
                let store = self.get_raw_param(2)?;
                if self.debug {
                    println!("ADD {} -> [{}]", res, store);
                }
                self.code[store as usize] = res;
                self.instr_ptr += 4;
            }
            OP_MUL => {
                let res = self.get_param(param_modes, 0)? * self.get_param(param_modes, 1)?;
                let store = self.get_raw_param(2)?;
                if self.debug {
                    println!("MUL {} -> [{}]", res, store);
                }
                self.code[store as usize] = res;
                self.instr_ptr += 4;
            }
            OP_INP => {
                let store = self.get_raw_param(0)?;
                let inp = self.input.pop_front().ok_or("not enough inputs")?;
                if self.debug {
                    println!("INP {} -> [{}]", inp, store);
                }
                self.code[store as usize] = inp;
                self.instr_ptr += 2;
            }
            OP_OUT => {
                let out = self.get_param(param_modes, 0)?;
                if self.debug {
                    println!("OUT {}", out);
                }
                self.output.push(out);
                self.instr_ptr += 2;
            }
            OP_JIT => {
                let expr = self.get_param(param_modes, 0)?;
                if self.debug {
                    println!("JIT {}", expr);
                }
                if expr != 0 {
                    let index = self.get_param(param_modes, 1)? as usize;
                    if self.debug {
                        println!("JUMP TO {} (from {})", index, self.instr_ptr);
                    }
                    self.instr_ptr = index;
                } else {
                    self.instr_ptr += 3;
                }
            }
            OP_JIF => {
                let expr = self.get_param(param_modes, 0)?;
                if self.debug {
                    println!("JIF {}", expr);
                }
                if expr == 0 {
                    let index = self.get_param(param_modes, 1)? as usize;
                    if self.debug {
                        println!("JUMP TO {} (from {})", index, self.instr_ptr);
                    }
                    self.instr_ptr = index;
                } else {
                    self.instr_ptr += 3;
                }
            }
            OP_LT => {
                let lt = self.get_param(param_modes, 0)? < self.get_param(param_modes, 1)?;
                let store = self.get_raw_param(2)?;
                if self.debug {
                    println!("LT {} -> [{}]", lt, store);
                }
                self.code[store as usize] = lt as isize;
                self.instr_ptr += 4;
            }
            OP_EQ => {
                let eq = self.get_param(param_modes, 0)? == self.get_param(param_modes, 1)?;
                let store = self.get_raw_param(2)?;
                if self.debug {
                    println!("EQ {} -> [{}]", eq, store);
                }
                self.code[store as usize] = eq as isize;
                self.instr_ptr += 4;
            }
            OP_HALT => return Ok(false),
            _ => return Err(format!("unknown opcode: {}", opcode).into()),
        }

        Ok(true)
    }

    fn run(&mut self, input: &[Code]) -> Result<Vec<Code>, Box<dyn Error>> {
        self.input.clear();
        input.iter().for_each(|&i| self.input.push_back(i));
        self.output.clear();

        while self.run_once()? {}

        Ok(self.output.clone())
    }
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut m = IntMachine::from_input(i)?;
    let output = m.run(&[1])?;

    output
        .last()
        .map(|v| v.to_string())
        .ok_or_else(|| "failed to get diagnostics".into())
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut m = IntMachine::from_input(i)?;
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
        let m = IntMachine::from_input(&Input::new("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"))?;

        assert_eq!(m.clone().run(&[8])?, vec!(1000));
        assert_eq!(m.clone().run(&[7])?, vec!(999));
        assert_eq!(m.clone().run(&[9])?, vec!(1001));

        Ok(())
    }
}
