use std::collections::HashMap;

use crate::{
    interrupts::InterruptHandler,
    mem::{Addressable, LinearMem},
    register::{Register, REGISTER_NO},
    Res,
};

const MEM: usize = 4096;

pub struct CVMachine {
    registers: [u32; 17],
    pub memory: Box<dyn Addressable>,
    pub halt: bool,
    interrupt_vector: HashMap<u8, Box<dyn InterruptHandler>>,
}

impl Default for CVMachine {
    fn default() -> Self {
        Self {
            registers: [0_u32; REGISTER_NO],
            memory: Box::new(LinearMem::new(MEM)),
            halt: false,
            interrupt_vector: HashMap::new(),
        }
    }
}
impl CVMachine {
    fn is_halted(&self) -> bool {
        self.halt
    }

    pub fn register<F>(&mut self, register: Register, map: F) -> u32
    where
        F: FnOnce(u32) -> u32,
    {
        let old = self.registers[register as usize];
        let new = map(old);
        if old != new {
            self.registers[register as usize] = new;
        }
        new
    }

    fn reset(&mut self) {
        self.registers = Default::default();
        self.memory = Box::new(LinearMem::new(MEM));
        self.halt = false;
    }

    pub fn state(&self) {
        let mut fmt = String::new();
        for r in 0..REGISTER_NO - 1 {
            let reg = Register::try_from(r as u8).expect("No error here");
            fmt.push_str(&format!("{:<5}: {:>4}", reg.to_string(), self.registers[r]));

            if (r + 1) % 4 == 0 {
                fmt.push_str(" \n");
            } else {
                fmt.push_str(" | ");
            }
        }
        let flags = self.registers[Register::CPSR as usize];
        println!("{fmt}{:<5}: {:032b}\t; 0x{:02x}\n", "cpsr", flags, flags);
    }

    fn push(&mut self, value: u32) -> Res<()> {
        let sp = self.register(Register::SP, |x| x + 4) - 4;
        self.memory.write_u32(sp, value)?;
        Ok(())
    }

    fn pop(&mut self) -> Res<u32> {
        let sp = self.register(Register::SP, |x| x - 4);
        let v = self.memory.read_u32(sp)?;
        Ok(v)
    }

    fn peek(&mut self) -> Res<u32> {
        let sp = self.register(Register::SP, |x| x) - 4;
        let v = self.memory.read_u32(sp)?;
        Ok(v)
    }

    fn define_interrupt(&mut self, idx: u8, handler: impl InterruptHandler + 'static) {
        self.interrupt_vector.insert(idx, Box::new(handler));
    }
}
