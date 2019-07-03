#![allow(non_snake_case)]

pub mod lc3;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use lc3::*;

fn main() {
    // This program will:
    // 1. Take a filename as input and attempt to open it, if it fails, retry
    // 2. read the file treating every two bytes as a u16 into a buffer
    // 3. initialise the state machine
    // print!("Enter a filename: ");
    // let _ = stdout().flush().unwrap();
    // let mut filename = String::new();
    // match stdin().read_line(&mut filename) {
    //     Ok(_) => (),
    //     Err(msg) => println!("Read failed: {}", msg),
    // }

    // Alternate - arguments

    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let filename = &args[1];

    let mut f = match File::open(filename) {
        Ok(f) => f,
        Err(msg) => panic!("Error: {}", msg),
    };
    let mut machine = lc3::LC3StateMachine::new();
    let mut addr = statemachine_init(&mut machine, &mut f);
    let mut instruction;
    loop {
        instruction = machine.addr_read(addr);
        if addr > 0x3fff {
            panic!("Out of bounds!");
        }
        let opcode = instruction >> 12;
        match opcode {
            13 => {
                panic!("Invalid opcode.")
            },
            _ => {
                println!("opcode is {}", opcode);
            }
        }
        addr+=1

    }

}

pub fn statemachine_init(machine: &mut LC3StateMachine, f: &mut File) -> u16 {
    // Process:
    // Read 2 bytes from file
    // Parse bytes as u16
    // Store u16 in temp[]
    // temp[0] = start_addr
    // mem[start_addr] = temp[1]
    // mem[start_addr+1] = temp[2]
    // ...
    // meme[start_addr+N-1] = temp[N]
    let mut temp = [0u16; 4096];
    let mut buffer = [0u8; 2];
    let mut bytes = f.metadata().unwrap().len();
    let mut i = 1;
    while bytes > 0 {
        match f.read(&mut buffer) {
            Ok(count) => {
                bytes -= count as u64;
                temp[i] = merge_two_bytes(buffer[0] as u16, buffer[1] as u16);
                i = i + 1;
            },
            Err(msg) => println!("Error: {:?}", msg),
        }
    }
    let start_addr = temp[0];
    for i in 1..=4095 {
        machine.addr_write(start_addr+(i-1), temp[i as usize]);
    }
    start_addr
}
