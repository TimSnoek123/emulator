use crate::cpu::{CPU, MemoryBus};
use crate::registers::{Registers, FlagsRegister};

mod cpu;
mod registers;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env::split_paths;

fn main() {
    let cpu = &mut CPU {
        registers: registers::Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: false,
            },
            h: 0,
            l: 0,
        },
        pc: 0,
        bus: MemoryBus{
            memory: [0; 0xFFFF]
        }
        ,stop: false
    };

    let file = File::open("gbroot.gb");

     file.unwrap().read(&mut cpu.bus.memory);

    while !cpu.stop  {
        cpu.step();
//        println!("register a value: {}", cpu.registers.a)
//        println!("current length: {}", cpu.pc);

    }

    println!("Done");
    if cpu.bus.memory[0xff02] == 0x81 {
        let c = cpu.bus.memory[0xff01];
        println!("{}", c);
        cpu.bus.memory[0xff02] = 0x0;
    }
}
