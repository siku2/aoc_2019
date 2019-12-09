use crate::input::Input;
use std::collections::VecDeque;
use std::error::Error;

pub type Code = isize;

const OP_ADD: Code = 1;
const OP_MUL: Code = 2;
const OP_INP: Code = 3;
const OP_OUT: Code = 4;
const OP_JIT: Code = 5;
const OP_JIF: Code = 6;
const OP_LT: Code = 7;
const OP_EQ: Code = 8;
const OP_RBO: Code = 9;

const OP_HALT: Code = 99;

#[derive(Clone)]
pub struct Machine {
    code: Vec<Code>,
    instr_ptr: usize,
    relative_base: Code,
    input: VecDeque<Code>,
    output: Vec<Code>,
    pub debug: bool,
    halted: bool,
    pub wait_for_input: bool,
}

impl Machine {
    pub fn new(code: Vec<Code>) -> Machine {
        Machine {
            code,
            instr_ptr: 0,
            relative_base: 0,
            input: VecDeque::new(),
            output: Vec::new(),
            debug: false,
            halted: false,
            wait_for_input: false,
        }
    }

    pub fn from_input(i: &Input) -> Result<Machine, Box<dyn Error>> {
        Ok(Self::new(i.parse_csv().collect::<Result<_, _>>()?))
    }

    fn get_address(&self, addr: Code) -> Option<Code> {
        if addr >= self.code.len() as Code {
            return Some(0);
        }
        self.code.get(addr as usize).copied()
    }

    fn set_address(&mut self, addr: Code, val: Code) {
        if addr >= self.code.len() as Code {
            let new_len = (addr + 1) as usize;
            if self.debug {
                println!("EXPAND MEMORY {} -> {}", self.code.len(), new_len);
            }
            self.code.resize(new_len, 0);
        }

        self.code[addr as usize] = val;
    }

    fn write_to_param(
        &mut self,
        param_modes: Code,
        param: u32,
        val: Code,
    ) -> Result<(), Box<dyn Error>> {
        let mut addr = self.get_raw_param(param as usize)?;
        let mode = get_mode(param_modes, param);

        match mode {
            0 => (),
            2 => addr += self.relative_base,
            1 => return Err("cannot write to immediate mode param".into()),
            _ => return Err("invalid param mode".into()),
        }

        if self.debug {
            println!("WRITE {} -> [{}] (mode: {})", val, addr, mode);
        }

        self.set_address(addr, val);
        Ok(())
    }

    fn get_raw_param(&self, param: usize) -> Result<Code, Box<dyn Error>> {
        let addr = (self.instr_ptr + (param + 1)) as Code;
        let value = self.get_address(addr).ok_or_else(|| "invalid index")?;

        if self.debug {
            println!("READ {} at [{}] = {}", param, addr, value);
        }

        Ok(value)
    }

    fn get_param(&self, param_modes: Code, param: u32) -> Result<Code, Box<dyn Error>> {
        let value = self.get_raw_param(param as usize)?;

        match get_mode(param_modes, param) {
            0 => self
                .get_address(value)
                .ok_or_else(|| "invalid index value".into()),
            1 => Ok(value),
            2 => self
                .get_address(self.relative_base + value)
                .ok_or_else(|| "invalid index value".into()),
            _ => Err("invalid param mode".into()),
        }
    }

    pub fn run_once(&mut self) -> Result<bool, Box<dyn Error>> {
        let instruction = self
            .get_address(self.instr_ptr as Code)
            .ok_or("no opcode")?;
        let (opcode, param_modes) = (instruction % 100, instruction / 100);

        if self.debug {
            println!(
                "\nSTEP: instr={} op={}, modes={}",
                self.instr_ptr, opcode, param_modes
            );
        }

        match opcode {
            OP_ADD => {
                let (a, b) = (
                    self.get_param(param_modes, 0)?,
                    self.get_param(param_modes, 1)?,
                );
                let res = a + b;
                if self.debug {
                    println!("ADD {} + {} = {}", a, b, res);
                }
                self.write_to_param(param_modes, 2, res)?;
                self.instr_ptr += 4;
            }
            OP_MUL => {
                let (a, b) = (
                    self.get_param(param_modes, 0)?,
                    self.get_param(param_modes, 1)?,
                );
                let res = a * b;
                if self.debug {
                    println!("MUL {} * {} = {}", a, b, res);
                }
                self.write_to_param(param_modes, 2, res)?;
                self.instr_ptr += 4;
            }
            OP_INP => {
                if self.input.is_empty() {
                    if self.wait_for_input {
                        if self.debug {
                            println!("STOP (missing input)")
                        }

                        return Ok(false);
                    } else {
                        return Err("missing input".into());
                    }
                }

                let inp = self.input.pop_front().unwrap();
                if self.debug {
                    println!("INP {}", inp);
                }
                self.write_to_param(param_modes, 0, inp)?;
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
                let (a, b) = (
                    self.get_param(param_modes, 0)?,
                    self.get_param(param_modes, 1)?,
                );
                let lt = a < b;
                if self.debug {
                    println!("LT {} < {} = {}", a, b, lt);
                }
                self.write_to_param(param_modes, 2, lt as Code)?;
                self.instr_ptr += 4;
            }
            OP_EQ => {
                let (a, b) = (
                    self.get_param(param_modes, 0)?,
                    self.get_param(param_modes, 1)?,
                );
                let eq = a == b;
                if self.debug {
                    println!("EQ {} == {} = {}", a, b, eq);
                }
                self.write_to_param(param_modes, 2, eq as Code)?;
                self.instr_ptr += 4;
            }
            OP_RBO => {
                let rel = self.get_param(param_modes, 0)?;
                self.relative_base += rel;
                if self.debug {
                    println!("RBO {:+} = {}", rel, self.relative_base);
                }
                self.instr_ptr += 2;
            }
            OP_HALT => {
                if self.debug {
                    println!("HALT");
                }
                self.halted = true;
                return Ok(false);
            }
            _ => return Err(format!("unknown opcode: {}", opcode).into()),
        }

        Ok(true)
    }

    fn reset(&mut self) {
        self.instr_ptr = 0;
        self.relative_base = 0;
        self.halted = false;
        self.input.clear();
        self.output.clear();
    }

    pub fn run(&mut self, input: &[Code]) -> Result<Vec<Code>, Box<dyn Error>> {
        self.reset();
        input.iter().for_each(|&i| self.input.push_back(i));
        if self.wait_for_input {
            return Err("cannot wait for input when using run".into());
        }

        while self.run_once()? {}

        Ok(self.output.clone())
    }

    pub fn is_done(&self) -> bool {
        self.halted
    }

    pub fn last_output(&self) -> Option<&Code> {
        self.output.last()
    }

    pub fn start(&mut self, input: &[Code]) -> Result<(bool), Box<dyn Error>> {
        self.reset();
        self.wait_for_input = true;

        self.send(input)
    }

    pub fn send(&mut self, input: &[Code]) -> Result<(bool), Box<dyn Error>> {
        if self.halted {
            return Err("cannot send input on halted machine".into());
        }

        input.iter().for_each(|&i| self.input.push_back(i));

        while self.run_once()? {}

        Ok(self.halted)
    }
}

fn get_mode(param_modes: Code, param: u32) -> Code {
    (param_modes / 10isize.pow(param)) % 10
}
