use std::env;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

use crate::utils;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    PSH = 0,
    POP = 1,
    ADD = 2,
    SUB = 3,
    MUL = 4,
    DIV = 5,
    SET = 6,
    HLT = 7,
    UNK = 9,
}

const STACK_SIZE: usize = 256;
const NUM_OF_REGISTERS: usize = 8;
// main registers we can add more later;
const IP: usize = 6;
const SP: usize = 7;

pub struct VM {
    running: bool,
    stack: [i32; STACK_SIZE],
    registers: [i32; NUM_OF_REGISTERS],
    error: bool,
}

impl VM {
    pub fn new() -> Self {
        let mut vm = VM {
            running: false,
            stack: [0; STACK_SIZE],
            registers: [0; NUM_OF_REGISTERS],
            error: false,
        };
        vm.registers[SP] = -1;
        vm.registers[IP] = 0;
        vm
    }

    fn sp(&self) -> i32 {
        self.registers[SP]
    }

    fn sp_mut(&mut self) -> &mut i32 {
        &mut self.registers[SP]
    }

    fn ip(&self) -> i32 {
        self.registers[IP]
    }

    fn ip_mut(&mut self) -> &mut i32 {
        &mut self.registers[IP]
    }

    fn fetch(&self, program: &[i32]) -> i32 {
        program[self.ip() as usize]
    }

    fn push(&mut self, value: i32) -> bool {
        if self.sp() < STACK_SIZE as i32 - 1 {
            *self.sp_mut() += 1;
            self.stack[self.sp() as usize] = value;
            true
        } else {
            eprint!("STACK OVERFLOW");
            self.error = true;
            false
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if self.sp() > 0 {
            let value = self.stack[self.sp() as usize];
            *self.sp_mut() -= 1;
            Some(value)
        } else {
            eprintln!("STACK UNDERFLOW");
            self.error = true;
            None
        }
    }

    fn add(&mut self) -> bool {
        if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
            match b.checked_add(a) {
                Some(res) => {
                    self.push(res);
                    true
                }
                None => {
                    eprintln!("Error : Integer overflow in addition");
                    self.error = true;
                    false
                }
            }
        } else {
            self.error = true;
            false
        }
    }

    fn sub(&mut self) -> bool {
        if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
            match b.checked_sub(a) {
                Some(res) => {
                    self.push(res);
                    true
                }
                None => {
                    eprintln!("Error: Integer overflow in subtraction");
                    self.error = true;
                    false
                }
            }
        } else {
            self.error = true;
            false
        }
    }

    fn mult(&mut self) -> bool {
        if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
            match b.checked_mul(a) {
                Some(result) => {
                    self.push(result);
                    true
                }
                None => {
                    eprintln!("Error: Integer overflow in multiplication");
                    self.error = true;
                    false
                }
            }
        } else {
            self.error = true;
            false
        }
    }

    fn divide(&mut self) -> bool {
        if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
            if a == 0 {
                eprintln!("Error: Cannot Divide By Zero");
                self.push(b);
                self.push(a);
                self.error = true;
                return false;
            }
            match b.checked_div(a) {
                Some(result) => {
                    self.push(result);
                    true
                }
                None => {
                    eprintln!("Error: Integer overflow in Division");
                    self.error = true;
                    false
                }
            }
        } else {
            self.error = true;
            false
        }
    }

    fn eval(&mut self, instr: i32, program: &[i32]) {
        match instr {
            x if x == Instruction::HLT as i32 => {
                self.running = false;
            }
            x if x == Instruction::PSH as i32 => {
                *self.ip_mut() += 1;
                let value = program[self.ip() as usize];
                if !self.push(value) {
                    self.running = false;
                }
            }
            x if x == Instruction::POP as i32 => {
                if self.pop().is_none() {
                    self.running = false;
                }
            }
            x if x == Instruction::ADD as i32 => {
                if !self.add() {
                    self.running = false;
                }
            }
            x if x == Instruction::SUB as i32 => {
                if !self.sub() {
                    self.running = false;
                }
            }
            x if x == Instruction::MUL as i32 => {
                if !self.mult() {
                    self.running = false;
                }
            }
            x if x == Instruction::DIV as i32 => {
                if !self.divide() {
                    self.running = false;
                }
            }
            x if x == Instruction::SET as i32 => {
                println!("SET not implemented yet");
                self.error = true;
                self.running = false;
            }
            _ => {
                println!("Unknown instruction: {}", instr);
                self.error = true;
                self.running = false;
            }
        }
    }
}

pub fn run_program(program: Vec<i32>, log_file: &mut File) -> Result<Option<i32>, ()> {
    let mut vm = VM::new();
    vm.running = true;

    while vm.running {
        let ip = vm.ip();
        if ip < 0 || (ip as usize) >= program.len() {
            writeln!(
                log_file,
                "Error: Program terminated without HLT or invalid IP"
            )
            .ok();
            vm.error = true;
            break;
        }

        let instr = vm.fetch(&program);
        vm.eval(instr, &program);

        writeln!(
            log_file,
            "IP: {}, SP: {}, Instr: {}, Stack: {:?}",
            vm.ip(),
            vm.sp(),
            instr,
            &vm.stack[0..=vm.sp().max(0) as usize]
        )
        .ok();

        *vm.ip_mut() += 1;
    }

    if vm.error {
        Err(())
    } else if vm.sp() >= 0 {
        Ok(Some(vm.stack[vm.sp() as usize]))
    } else {
        Ok(None)
    }
}
