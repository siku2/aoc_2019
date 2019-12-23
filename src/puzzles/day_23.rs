use crate::input::Input;
use crate::lib::intcode;
use std::error;

type Error = Box<dyn error::Error>;

#[derive(Clone)]
struct Packet {
    destination: usize,
    x: isize,
    y: isize,
}

impl Packet {
    fn from_output(output: &[intcode::Code]) -> Option<Self> {
        if output.len() != 3 {
            return None;
        }

        Some(Self {
            destination: output[0] as usize,
            x: output[1],
            y: output[2],
        })
    }

    fn to_input(&self, input: &mut impl Extend<intcode::Code>) {
        input.extend([self.x, self.y].iter().copied());
    }
}

#[derive(Clone)]
struct Computer {
    machine: intcode::Machine,
    address: Option<usize>,
    idle: bool,
}

impl Computer {
    fn new(mut machine: intcode::Machine) -> Self {
        machine.start();
        Self {
            machine,
            address: None,
            idle: false,
        }
    }

    fn from_input(i: &Input) -> Result<Self, Error> {
        intcode::Machine::from_input(i).map(Self::new)
    }

    fn give_address(&mut self, address: usize) -> Result<(), Error> {
        if self.address.is_some() {
            return Ok(());
        }

        self.machine.send(address as intcode::Code)?;
        self.address = Some(address);
        Ok(())
    }
    fn receive_packet(&mut self, packet: Packet) {
        packet.to_input(&mut self.machine.input);
        self.idle = false;
    }

    fn step(&mut self) -> Result<Option<Packet>, Error> {
        let ran = self.machine.run_once()?;
        if !ran && !self.machine.is_done() {
            self.idle = true;
            self.machine.input.push_back(-1);
            self.machine.run_once()?;
        }

        let packet = Packet::from_output(&self.machine.output);
        if packet.is_some() {
            self.machine.output.clear();
        }

        Ok(packet)
    }
}

struct Network {
    computers: Vec<Computer>,
    computers_idle: bool,
    packets: Vec<Packet>,
    nat_packet: Option<Packet>,
}

impl Network {
    fn new(computers: Vec<Computer>) -> Self {
        Self {
            computers,
            computers_idle: false,
            packets: Vec::new(),
            nat_packet: None,
        }
    }

    fn from_input(i: &Input) -> Result<Self, Error> {
        let computers = vec![Computer::from_input(i)?; 50];
        Ok(Self::new(computers))
    }

    fn setup(&mut self) -> Result<(), Error> {
        for (i, c) in self.computers.iter_mut().enumerate() {
            c.give_address(i)?;
        }

        Ok(())
    }

    fn get_computer(&mut self, address: usize) -> Option<&mut Computer> {
        self.computers.get_mut(address)
    }

    fn send_packets(&mut self) -> Result<(), Error> {
        if self.packets.is_empty() {
            return Ok(());
        }

        let packets: Vec<_> = self.packets.drain(..).collect();
        for packet in packets {
            if let Some(comp) = self.get_computer(packet.destination) {
                comp.receive_packet(packet);
            } else if packet.destination == 255 {
                self.nat_packet = Some(packet);
            } else {
                return Err("packet without destination".into());
            }
        }

        Ok(())
    }

    fn step_computers(&mut self) -> Result<(), Error> {
        let mut all_idle = true;

        for c in self.computers.iter_mut() {
            if let Some(packet) = c.step()? {
                self.packets.push(packet);
            }
            if !c.idle {
                all_idle = false;
            }
        }

        self.computers_idle = all_idle && self.packets.is_empty();

        Ok(())
    }

    fn run_until_nat_packet(&mut self) -> Result<Packet, Error> {
        self.setup()?;

        loop {
            self.step_computers()?;
            self.send_packets()?;
            if let Some(packet) = &self.nat_packet {
                return Ok(packet.clone());
            }
        }
    }

    fn run_until_nat_twice(&mut self) -> Result<isize, Error> {
        self.setup()?;
        let mut prev_y: Option<isize> = None;
        loop {
            self.step_computers()?;
            self.send_packets()?;
            if self.computers_idle {
                if let Some(packet) = self.nat_packet.clone() {
                    let comp = self
                        .get_computer(0)
                        .ok_or_else(|| "expected computer with address 0")?;
                    if let Some(prev_y) = prev_y {
                        if packet.y == prev_y {
                            return Ok(prev_y);
                        }
                    }

                    prev_y = Some(packet.y);
                    comp.receive_packet(packet);
                }
            }
        }
    }
}

pub fn first(i: &Input) -> Result<String, Error> {
    let mut network = Network::from_input(i)?;
    let packet = network.run_until_nat_packet()?;

    Ok(packet.y.to_string())
}

pub fn second(i: &Input) -> Result<String, Error> {
    let mut network = Network::from_input(i)?;
    let y = network.run_until_nat_twice()?;
    Ok(y.to_string())
}
