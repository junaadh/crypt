use std::collections::HashMap;

use crate::{
    memory::{Addressable, LineMem},
    processor::{CPSRflags, Condition, Instruction, Register, DPI, LSI, SCI},
    types::Operand,
    Res,
};

use super::{InterruptHandler, InterruptVector};

pub struct CpuCore {
    registers: [u32; 16],
    flags: CPSRflags,
    memory: Box<dyn Addressable>,
    pub state: bool,
}

pub struct Cpu {
    core: CpuCore,
    interrupt_table: HashMap<u8, InterruptVector>,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            core: CpuCore {
                registers: [0u32; 16],
                flags: CPSRflags::default(),
                memory: Box::new(LineMem::new(0x1000)),
                state: false,
            },
            interrupt_table: HashMap::new(),
        }
    }
}

impl Cpu {
    pub(crate) fn is_halted(&self) -> bool {
        self.core.state
    }

    pub(crate) fn register<F>(&mut self, register: Register, map: F) -> u32
    where
        F: FnOnce(u32) -> u32,
    {
        let old = self.core.registers[register as usize];
        let new = map(old);
        if old != new {
            self.core.registers[register as usize] = new;
        }
        new
    }

    pub(crate) fn reset(&mut self) {
        self.core.registers = Default::default();
        self.core.memory = Box::new(LineMem::new(0x1000));
        self.core.state = false;
    }

    pub(crate) fn dump(&self) {
        let mut fmt = String::new();
        for r in 0..16 {
            let reg = Register::try_from(r as u8).expect("No error here");
            fmt.push_str(&format!(
                "{:<5}: {:>4}",
                reg.to_string(),
                self.core.registers[r]
            ));

            if (r + 1) % 4 == 0 {
                fmt.push_str(" \n");
            } else {
                fmt.push_str(" | ");
            }
        }
        println!("{fmt}{s}", s = self.core.flags);
    }

    // TODO:?? todo!(stack)

    pub fn define_interrupt(&mut self, idx: u8, handler: InterruptVector) {
        self.interrupt_table.insert(idx, handler);
    }

    pub fn execute(&mut self) -> Res<()> {
        self.core.state = true;
        while self.core.state {
            self.step()?;
            self.dump();
        }
        Ok(())
    }

    pub fn load_program(&mut self, program: &[u8], addr: u32) -> Res<()> {
        for (idx, &byte) in program.iter().enumerate() {
            self.core.memory.write_u8(addr + idx as u32, byte)?;
        }
        Ok(())
    }

    pub fn step(&mut self) -> Res<()> {
        let pc = self.register(Register::PC, |x| x + 4) - 4;
        let byte_code = self.core.memory.read_u32(pc)?;

        let instruction = Instruction::try_from(byte_code)?;
        match instruction {
            Instruction::Add(DPI {
                cond,
                rn,
                rd,
                operand,
                ..
            }) => {
                if !self.core.flags.validate(cond) {
                    return Ok(());
                }

                let value = self.register(rn, |x| x);
                let rm = match operand {
                    Operand::Reg(r) => self.register(r, |x| x),
                    Operand::Imm(imm) => imm.as_signed() as u32,
                };
                let res = value.wrapping_add(rm);
                self.register(rd, |_| res);

                Ok(())
            }
            Instruction::Mov(DPI {
                cond, rd, operand, ..
            }) => {
                if !self.core.flags.validate(cond) {
                    return Ok(());
                }

                let imm = match operand {
                    Operand::Imm(imm) => imm.as_signed() as u32,
                    _ => unreachable!("shudnt be here coz mov doesnt accept register as rm"),
                };

                self.register(rd, |_| imm);

                Ok(())
            }
            Instruction::Svc(SCI {
                cond,
                interrupt_key,
                ..
            }) => {
                if !self.core.flags.validate(cond) {
                    return Ok(());
                }

                let int = self.interrupt_table.get(&interrupt_key).unwrap();
                int.handle(&mut self.core, 0)?;

                Ok(())
            }
            _ => todo!(),
        }
    }
}
