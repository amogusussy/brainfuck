use std::io::{self, Read};

use crate::lexer::{self, Instructions};

const U8_MAX: u8 = 0xff;

#[derive(Debug)]
pub struct Program {
    instructions: Vec<lexer::Instructions>,
    data_tape: Vec<u8>,
    data_pointer: usize,
}

impl Program {
    pub fn new(instructions: &str) -> Program{
        Program {
            instructions: lexer::lexer(&instructions),
            data_tape: Vec::new(),
            data_pointer: 0,
        }
    }

    fn is_end(&self) -> bool {
        self.data_tape.len() >= self.data_pointer
    }

    fn inc_data_pointer(&mut self) {
        if self.is_end() {
            self.data_tape.push(0);
        }
        self.data_pointer += 1;
    }

    fn dec_data_pointer(&mut self) {
        if self.data_pointer == 0 {
            println!("FATAL ERROR!! POINTER VALUE CANNOT BE <0");
            panic!();
        }
        self.data_pointer -= 1;
    }

    fn fill_for_ip(&mut self, ip: usize) {
        while self.data_tape.len() <= ip {
            self.data_tape.push(0);
        }
    }

    fn inc_data_value(&mut self) {
        if self.data_tape[self.data_pointer] == U8_MAX {
            self.data_tape[self.data_pointer] = 0;
        } else {
            self.data_tape[self.data_pointer] += 1;
        }
    }
    
    fn dec_data_value(&mut self) {
        if self.data_tape[self.data_pointer] == 0 {
            self.data_tape[self.data_pointer] = U8_MAX;
        } else {
            self.data_tape[self.data_pointer] -= 1;
        }
    }

    pub fn run(&mut self) {
        let mut ip: usize = 0;
        while ip < self.instructions.len() {
            self.fill_for_ip(ip);
            match &self.instructions[ip] {
                Instructions::RightPointer => self.inc_data_pointer(),
                Instructions::LeftPointer => self.dec_data_pointer(),
                Instructions::Incremement => self.inc_data_value(),
                Instructions::Decrement => self.dec_data_value(),
                Instructions::Output => {
                    print!("{}", self.data_tape[self.data_pointer] as char);
                },
                Instructions::Input => {
                    let mut buf = [0u8, 1];
                    match io::stdin().read(&mut buf) {
                        Ok(0) => self.data_tape[self.data_pointer] = 0,
                        Ok(_) => self.data_tape[self.data_pointer] = buf[0],
                        Err(_) => panic!("smthing went oopsies :(")
                    };
                },
                Instructions::StartWhile => {
                    if self.data_tape[self.data_pointer] == 0 {
                        // jump to next ]
                        let mut seen_open: u64 = 0;
                        'searching_loop: loop {
                            ip += 1;
                            match self.instructions[ip] {
                                Instructions::StartWhile => seen_open += 1,
                                Instructions::EndWhile => {
                                    seen_open -= 1;
                                    if seen_open == 0 {
                                        break 'searching_loop
                                    }
                                },
                                _ => continue
                            }
                        }
                    }
                },
                Instructions::EndWhile => {
                    if self.data_tape[self.data_pointer] != 0 {
                        // jump to previous [
                        let mut seen_close: u64 = 0;
                        'searching_loop: loop {
                            ip -= 1;
                            match self.instructions[ip] {
                                Instructions::StartWhile => {
                                    if seen_close == 0 {
                                        break 'searching_loop
                                    }
                                    seen_close -= 1;
                                },
                                Instructions::EndWhile => seen_close += 1,
                                _ => continue
                            }
                        }
                    }
                },
            }
            ip += 1;
        }
    }
}
