use crate::cpu::{CPU, MemoryBus};
use crate::registers::{Registers, FlagsRegister};

mod cpu;
mod registers;

use std::io::prelude::*;
use std::fs::File;
use imgui::sys::ImGuiIO_AddInputCharactersUTF8;
use imgui::{Window, ImStr, Condition};

extern crate azul;

use azul::prelude;

struct AzulDataModel { }

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
        pc: 0x100,
        bus: MemoryBus{
            memory: [0; 0xFFFF]
        }
        ,stop: false
    };

    let file = File::open("test1.gb");

     file.unwrap().read(&mut cpu.bus.memory);

    while !cpu.stop  {
        cpu.step();
//        println!("register a value: {}", cpu.registers.a)
//        println!("current length: {}", cpu.pc);

    }

    let mut app = App::new(AzulDataModel{}, AppConfig::default()).unwrap();

    println!("Done");
    if cpu.bus.memory[0xff02] == 0x81 {
        let c = cpu.bus.memory[0xff01];
        println!("{}", c);
        cpu.bus.memory[0xff02] = 0x0;
    }
}


