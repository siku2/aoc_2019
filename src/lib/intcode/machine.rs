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

const MOD_POSITION: Code = 0;
const MOD_IMMEDIATE: Code = 1;
const MOD_RELATIVE: Code = 2;

#[derive(Clone)]
pub struct Machine {
    code: Vec<Code>,
    instr_ptr: usize,
    relative_base: Code,
    pub input: VecDeque<Code>,
    pub output: Vec<Code>,
    pub debug: bool,
    halted: bool,
    wait_for_input: bool,
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

    fn read(&self, addr: Code) -> Code {
        self.code.get(addr as usize).copied().unwrap_or_default()
    }

    pub fn write(&mut self, addr: Code, val: Code) {
        if addr >= self.code.len() as Code {
            let new_len = (addr + 1) as usize;
            if self.debug {
                println!("EXPAND MEMORY {} -> {}", self.code.len(), new_len);
            }
            self.code.resize(new_len, 0);
        }

        self.code[addr as usize] = val;
    }

    fn get_param(&self, param_modes: Code, param: u32) -> (Code, Code) {
        (
            (param_modes / 10isize.pow(param)) % 10,
            (self.instr_ptr + 1 + param as usize) as Code,
        )
    }

    fn resolve_address(&self, mode: Code, addr: Code) -> Result<Code, Box<dyn Error>> {
        match mode {
            MOD_POSITION => Ok(self.read(addr)),
            MOD_IMMEDIATE => Ok(addr),
            MOD_RELATIVE => Ok(self.read(addr) + self.relative_base),
            _ => Err("invalid param mode".into()),
        }
    }

    fn write_to_param(
        &mut self,
        param_modes: Code,
        param: u32,
        val: Code,
    ) -> Result<(), Box<dyn Error>> {
        let (mode, param_addr) = self.get_param(param_modes, param);
        if mode == MOD_IMMEDIATE {
            return Err("immediate param mode disallowed by spec".into());
        }
        let addr = self.resolve_address(mode, param_addr)?;
        if self.debug {
            println!(
                "WRITE to param {} at [{}]={}: {} -> [{}]",
                param,
                param_addr,
                self.read(param_addr),
                val,
                addr
            );
        }

        self.write(addr, val);
        Ok(())
    }

    fn read_from_param(&self, param_modes: Code, param: u32) -> Result<Code, Box<dyn Error>> {
        let (mode, param_addr) = self.get_param(param_modes, param);
        let addr = self.resolve_address(mode, param_addr)?;
        let value = self.read(addr);
        if self.debug {
            println!(
                "READ from param {} at [{}]={}: [{}] = {}",
                param,
                param_addr,
                self.read(param_addr),
                addr,
                value
            );
        }
        Ok(value)
    }

    pub fn run_once(&mut self) -> Result<bool, Box<dyn Error>> {
        let instruction = self.read(self.instr_ptr as Code);
        let (opcode, param_modes) = (instruction % 100, instruction / 100);

        if self.debug {
            println!(
                "\nSTEP: pos={} op={}, modes={}",
                self.instr_ptr, opcode, param_modes
            );
        }

        match opcode {
            OP_ADD => {
                let (a, b) = (
                    self.read_from_param(param_modes, 0)?,
                    self.read_from_param(param_modes, 1)?,
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
                    self.read_from_param(param_modes, 0)?,
                    self.read_from_param(param_modes, 1)?,
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
                let out = self.read_from_param(param_modes, 0)?;
                if self.debug {
                    println!("OUT {}", out);
                }
                self.output.push(out);
                self.instr_ptr += 2;
            }
            OP_JIT => {
                let expr = self.read_from_param(param_modes, 0)?;
                if self.debug {
                    println!("JIT {}", expr);
                }
                if expr != 0 {
                    let index = self.read_from_param(param_modes, 1)? as usize;
                    if self.debug {
                        println!("JUMP TO {} (from {})", index, self.instr_ptr);
                    }
                    self.instr_ptr = index;
                } else {
                    self.instr_ptr += 3;
                }
            }
            OP_JIF => {
                let expr = self.read_from_param(param_modes, 0)?;
                if self.debug {
                    println!("JIF {}", expr);
                }
                if expr == 0 {
                    let index = self.read_from_param(param_modes, 1)? as usize;
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
                    self.read_from_param(param_modes, 0)?,
                    self.read_from_param(param_modes, 1)?,
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
                    self.read_from_param(param_modes, 0)?,
                    self.read_from_param(param_modes, 1)?,
                );
                let eq = a == b;
                if self.debug {
                    println!("EQ {} == {} = {}", a, b, eq);
                }
                self.write_to_param(param_modes, 2, eq as Code)?;
                self.instr_ptr += 4;
            }
            OP_RBO => {
                let rel = self.read_from_param(param_modes, 0)?;
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

    pub fn run_until_stop(&mut self) -> Result<bool, Box<dyn Error>> {
        while self.run_once()? {}

        Ok(self.halted)
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
        self.run_until_stop()?;

        Ok(self.take_output())
    }

    pub fn is_done(&self) -> bool {
        self.halted
    }

    pub fn take_output(&mut self) -> Vec<Code> {
        let out = self.output.clone();
        self.output.clear();

        out
    }

    pub fn start(&mut self) {
        self.reset();
        self.wait_for_input = true;
    }

    pub fn send(&mut self, input: Code) -> Result<bool, Box<dyn Error>> {
        if !self.wait_for_input {
            return Err("start wasn't called".into());
        } else if self.halted {
            return Err("cannot send input on halted machine".into());
        }

        self.input.push_back(input);

        self.run_until_stop()
    }

    pub fn send_ascii(&mut self, input: &str) -> Result<bool, Box<dyn Error>> {
        for inp in input.chars().map(|c| c as Code) {
            if self.send(inp)? {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
