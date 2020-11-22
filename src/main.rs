use crate::cpu::{CPU, MemoryBus};
use crate::registers::{Registers, FlagsRegister};

mod cpu;
mod registers;

use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut cpu = CPU {
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
        pc: 0x100,
        bus: MemoryBus{
            memory: [0; 0xFFFF]
        }
        ,stop: false
    };

    let file = File::open("test2.gb");
    let bus  = &mut cpu.bus.memory;

    let reader = file.unwrap().read(bus);



    while !cpu.stop {
        cpu.step();

//        if cpu.bus.memory[0xff02] == 0x81 {
//            let c = bus.memory[0xff01];
//            println!("{}", c);
//            bus.memory[0xff02] = 0x0;
//        }
    }
}
