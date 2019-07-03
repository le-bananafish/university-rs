#![allow(non_snake_case)]
#![allow(dead_code)]

const ADDR_SIZE: usize = 65536;
pub const USER_SIZE: usize = 0x1000; // 4096

pub fn merge_two_bytes(b0: u16, b1: u16) -> u16 {
    ((b0 << 8) ^ b1) as u16
}

#[derive(Copy, Clone)]
pub enum Condition {
    NEGATIVE,
    ZERO,
    POSITIVE,
}

impl From<Condition> for char {
    fn from(cond: Condition) -> char {
        match cond {
            Condition::NEGATIVE => 'N',
            Condition::ZERO => 'Z',
            Condition::POSITIVE => 'P',
        }
    }
}

impl From<u16> for Condition {
    fn from(value: u16) -> Condition {
        let value = value as i16;
        if value == 0 {
            Condition::ZERO
        } else if value > 0 {
            Condition::POSITIVE
        } else {
            Condition::NEGATIVE
        }
    }
}

#[derive(Copy, Clone)]
pub struct LC3StateMachine {
    r0: u16,
    r1: u16,
    r2: u16,
    r3: u16,
    r4: u16,
    r5: u16,
    r6: u16,
    r7: u16,
    pc: u16,
    ir: u16,
    cc: Condition,
    mem: [u16; ADDR_SIZE],
}

impl LC3StateMachine {
    pub fn new() -> Self {
        Self {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            pc: 0,
            ir: 0,
            cc: Condition::ZERO,
            mem: [0; ADDR_SIZE],
        }
    }

    pub fn print_state(&self) {
        println!("R0\t{:#X}", self.r0);
        println!("R1\t{:#X}", self.r1);
        println!("R2\t{:#X}", self.r2);
        println!("R3\t{:#X}", self.r3);
        println!("R4\t{:#X}", self.r4);
        println!("R5\t{:#X}", self.r5);
        println!("R6\t{:#X}", self.r6);
        println!("R7\t{:#X}", self.r7);
        println!("PC\t{:#X}", self.pc);
        println!("IR\t{:#X}", self.ir);
        println!("CC\t{}", char::from(self.cc));
        println!("==================");
    }

    pub fn print_mem(&self) {
        for i in 0..=ADDR_SIZE {
            println!("{}: {}", i, self.mem[i]);
        }
    }

    pub fn register_read(&self, src: u8) -> u16 {
        match src {
            0 => self.r0,
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            4 => self.r4,
            5 => self.r5,
            6 => self.r6,
            7 => self.r7,
            _ => 0,
        }
    }

    pub fn register_write(&mut self, dest: u8, value: u16) {
        match dest {
            0 => { self.r0 = value },
            1 => { self.r1 = value },
            2 => { self.r2 = value },
            3 => { self.r3 = value },
            4 => { self.r4 = value },
            5 => { self.r5 = value },
            6 => { self.r6 = value },
            7 => { self.r7 = value },
            _ => (),
        }
    }

    pub fn condition_write(&mut self, value: u16) {
        self.cc = Condition::from(value);
    }

    pub fn condition_read(&self) -> char {
        char::from(self.cc)
    }

    pub fn addr_write(&mut self, dest: u16, value: u16) {
        if 0x3000 <= dest && dest <= 0x3fff {
            self.mem[dest as usize] = value;
        } else {
            // panic!("MEM_WRITE_OUT_OF_BOUNDS")
            println!("Attempted write at {:#X}", dest);
        }
    }

    pub fn addr_read(&self, src: u16) -> u16 {
        if 0x3000 <= src && src <= 0x3fff {
            self.mem[src as usize]
        } else {
            // panic!("MEM_READ_OUT_OF_BOUNDS")
            println!("Attempted read at {:#X}", src);
            0
        }
    }
}

pub enum OPCODE {
    ADD = 1,
    AND = 5,
    BR = 0,
    JMP = 12, // RET = JMP w/ R7
    JSR = 4, // JSRR = JSR w/
    LD = 2,
    LDI = 10,
    LDR = 6,
    LEA = 14,
    NOT = 9,
    RTI = 8,
    ST = 3,
    STI = 11,
    STR = 7,
    TRAP = 15,
    RESV = 13,
}