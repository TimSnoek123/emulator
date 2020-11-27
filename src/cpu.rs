use crate::registers;
use crate::cpu::ArithmeticTarget::{A, B, C, D, E, H, L, IMM8};
use std::borrow::Borrow;


enum Instruction {
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    CP(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    INC(ArithmeticTarget),
    DEC(ArithmeticTarget),
    RLC(ArithmeticTarget),
}

#[derive(Copy, Clone)]
enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    IMM8
}
enum ArithmeticTarget16 {
    AF,
    BC,
    DE,
    HL
}

pub struct CPU {
    pub registers: registers::Registers,
    pub pc: u16,
    pub bus: MemoryBus,
    pub stop: bool,
}

#[derive(Copy, Clone)]
pub struct MemoryBus {
    pub memory: [u8; 0xFFFF]
}

impl MemoryBus {
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn insert_into_position(&mut self, position: usize, value: u8) {
        self.memory[position] = value;
    }
}


impl CPU {
    pub fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);

        self.execute(instruction_byte);
    }

    pub fn execute(&mut self, opcode: u8) {
        match opcode {
            0x00 => {},
            0x06 => self.load(B, IMM8),
            0x0E => self.load(C, IMM8),
            0x16 => self.load(D, IMM8),
            0x1E => self.load(E, IMM8),
            0x26 => self.load(H, IMM8),
            0x2E => self.load(L, IMM8),
            0x7F => self.load(A, A),
            0x78 => self.load(A, B),
            0x79 => self.load(A, C),
            0x7A => self.load(A, D),
            0x7B => self.load(A, E),
            0x7C => self.load(A, H),
            0x7D => self.load(A, L),
            0x7E => self.load_address_into(A, self.registers.get_hl()),
            0x40 => self.load(B, B),
            0x41 => self.load(B, C),
            0x42 => self.load(B, D),
            0x43 => self.load(B, E),
            0x44 => self.load(B, H),
            0x45 => self.load(B, L),
            0x46 => self.load_address_into(B, self.registers.get_hl()),
            0x47 => self.load(B, A),
            0x48 => self.load(C, B),
            0x49 => self.load(C, C),
            0x4A => self.load(C, D),
            0x4B => self.load(C, E),
            0x4C => self.load(C, H),
            0x4D => self.load(C, L),
            0x4E => self.load_address_into(C, self.registers.get_hl()),
            0x4F => self.load(C, A),
                0x87 => {
                self.add(self.registers.a);
            }
            0x80 => {
                self.add(self.registers.b);
            }
            0x81 => {
                self.add(self.registers.c);
            }
            0x82 => {
                self.add(self.registers.d);
            }
            0x83 => {
                self.add(self.registers.e);
            }
            0x84 => {
                self.add(self.registers.h);
            }
            0x85 => {
                self.add(self.registers.l);
            }
            0x86 => {
                self.add(self.bus.read_byte(self.registers.get_hl()));
            }
            _ => {}
        }

        let (value, overflow) = self.pc.overflowing_add(1);
        self.stop = value == 0xFFFF;

        if !self.stop {
            self.pc = value;
        }

    }

    fn get_register_value_by_target(&mut self, target: ArithmeticTarget) -> u8 {
        match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::IMM8 => self.get_immediate8()
        }
    }

    fn write_to_target(&mut self, value: u8, target: ArithmeticTarget) {
        match target {
            ArithmeticTarget::A => self.registers.a = value,
            ArithmeticTarget::B => self.registers.b = value,
            ArithmeticTarget::C => self.registers.c = value,
            ArithmeticTarget::D => self.registers.d = value,
            ArithmeticTarget::E => self.registers.e = value,
            ArithmeticTarget::H => self.registers.h = value,
            ArithmeticTarget::L => self.registers.l = value,
            ArithmeticTarget::IMM8 => panic!("Can't write to immediate 8 !")
            //TODO: Set imm8 in other enum
        }
    }

    fn get_immediate8(&mut self) -> u8 {
        self.bus.read_byte(self.pc.wrapping_add(1))
    }

    pub fn load(&mut self, target: ArithmeticTarget, val: ArithmeticTarget){
        let value = self.get_register_value_by_target(val);
        self.write_to_target(value, target);
    }

    pub fn load_address_into(&mut self, target: ArithmeticTarget, address: u16){
        let value = self.bus.read_byte(address);
        self.write_to_target(value, target);
    }

    fn load_into_address(&mut self, address: u8, target_value: ArithmeticTarget){
        let value = self.get_register_value_by_target(target_value);
        self.bus.memory[address as usize] = value;
    }

    pub fn load_16(&mut self){

    }

    fn add(&mut self, value: u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);

        self.registers.set_zero(new_value == 0);
        self.registers.set_subtract(false);
        self.registers.set_carry(did_overflow);
        self.registers.set_half_carry((self.registers.a & 0xF) + (value & 0xF) > 0xF);

        self.registers.a = new_value
    }

    fn adc(&mut self, value: u8) {
        let carry = self.registers.f.carry as u8;

        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        let (new_value_carry, did_overflow_carry) = new_value.overflowing_add(carry);

        self.registers.set_zero(new_value_carry == 0);
        self.registers.set_subtract(false);
        self.registers.set_half_carry((self.registers.a & 0xF) + (value & 0xF) + carry > 0xF);
        self.registers.set_carry(self.registers.a as u16 + value as u16 + carry as u16 > 0xff);

        self.registers.a = new_value_carry
    }

    fn sub(&mut self, value: u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);

        self.registers.set_zero(new_value == 0);
        self.registers.set_subtract(true);
        self.registers.set_carry(did_overflow);
        self.registers.set_half_carry((self.registers.a & 0xF) - (value & 0xF) < 0xF);

        self.registers.a = new_value;
    }

    fn sbc(&mut self, value: u8) {
        let carry = self.registers.f.carry as u8;

        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        let (new_value_carry, did_overflow_carry) = new_value.overflowing_sub(carry);

        self.registers.set_zero(new_value_carry == 0);
        self.registers.set_subtract(true);
        self.registers.set_carry(did_overflow || did_overflow_carry);
        self.registers.set_half_carry((self.registers.a & 0xF) - (value & 0xF) - (carry & 0xF) < 0xF);

        self.registers.a = new_value_carry;
    }

    fn cp(&mut self, value: u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);

        self.registers.set_zero(new_value == 0);
        self.registers.set_subtract(true);
        self.registers.set_carry(did_overflow);
        self.registers.set_half_carry((self.registers.a & 0xF) - (value & 0xF) < 0xF);
    }

    fn and(&mut self, value: u8) {
        self.registers.a &= value;

        self.registers.set_zero(self.registers.a == 0);
        self.registers.set_subtract(false);
        self.registers.set_half_carry(true);
        self.registers.set_carry(false);
    }

    fn or(&mut self, value: u8) {
        self.registers.a |= value;

        self.registers.set_zero(self.registers.a == 0);
        self.registers.set_subtract(false);
        self.registers.set_half_carry(false);
        self.registers.set_carry(false);
    }

    fn xor(&mut self, value: u8) {
        self.registers.a ^= value;

        self.registers.set_zero(self.registers.a == 0);
        self.registers.set_subtract(false);
        self.registers.set_half_carry(false);
        self.registers.set_carry(false);
    }

    fn inc(&mut self, value: u8, target: ArithmeticTarget) {
        let (new_value, did_overflow) = value.overflowing_add(1);

        self.registers.set_zero(value == 0);
        self.registers.set_subtract(false);
        self.registers.set_half_carry(value & 0xf == 0xf);

        self.write_to_target(new_value, target);
    }

    fn dec(&mut self, value: u8, target: ArithmeticTarget) {
        let (new_value, did_overflow) = value.overflowing_sub(1);

        self.registers.set_zero(value == 0);
        self.registers.set_subtract(true);
        self.registers.set_half_carry(value & 0xf == 0);

        self.write_to_target(new_value, target);
    }

    fn rlc(&mut self, value: u8, target: ArithmeticTarget) {
        let c = (value & 0x80) >> 7 == 0x01;
        let new_value = value.rotate_left(1);

        self.registers.set_zero(value == 0x00);
        self.registers.set_subtract(false);
        self.registers.set_half_carry(false);
        self.registers.set_carry(c);

        self.write_to_target(new_value, target);
    }
}