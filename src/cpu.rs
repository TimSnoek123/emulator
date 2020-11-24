use crate::registers;
use crate::cpu::ArithmeticTarget::A;


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


    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn insert_into_position(&mut self, position: usize, value: u8) {
        self.memory[position] = value;
        println!("Current test: {}", value);
    }
}


impl CPU {
    pub fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);

//        println!("Current pc: {}", self.pc);
//        if instruction_byte > 0 {
//            println!("Current byte: {}", instruction_byte);

//        }
        self.execute(instruction_byte);
    }

    pub fn execute(&mut self, opcode: u8) {
        match opcode {
            0x00 => {
//               println!("found: {:x?}", opcode);
            },
            0x87 => {
                self.add(self.registers.a);
               println!("found: {:x?}", opcode);
            }
            0x80 => {
                self.add(self.registers.b);
               println!("found: {:x?}", opcode);
            }
            0x81 => {
                self.add(self.registers.c);
               println!("found: {:x?}", opcode);
            }
            0x82 => {
                self.add(self.registers.d);
                println!("found: {:x?}", opcode);
            }
            0x83 => {
                self.add(self.registers.e);
                println!("found: {:x?}", opcode);
            }
            0x84 => {
                self.add(self.registers.h);
                println!("found: {:x?}", opcode);
            }
            0x85 => {
                self.add(self.registers.l);
                println!("found: {:x?}", opcode);
            }
            0x86 => {
                self.add(self.bus.read_byte(self.registers.get_hl()));
                println!("found: {:x?}", opcode);
            }
            _ => {println!("didnt find: {}", opcode)}
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
        }
    }

    pub fn load(&mut self, target: ArithmeticTarget, target_value: ArithmeticTarget){
        let value = self.get_register_value_by_target(target_value);
        self.write_to_target(value, target);
    }

    fn load_into_address(&mut self, address: u8, target_value: ArithmeticTarget){
        let value = self.get_register_value_by_target(target_value);
        self.bus.memory[address] = value;
    }

    fn add(&mut self, value: u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);

        self.registers.set_zero(new_value == 0);
        self.registers.set_subtract(false);
        self.registers.set_carry(did_overflow);
        self.registers.set_half_carry((self.registers.a & 0xF) + (value & 0xF) > 0xF);

        println!("Old a val: {}, new a value {}", self.registers.a, new_value);

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
        let co = value & 0x80;
        let new_value = value.rotate_left(1);

        self.registers.set_zero(value == 0);
        self.registers.set_subtract(false);
        self.registers.set_half_carry(false);
        self.registers.set_carry(co != 0);

        self.write_to_target(new_value, target);
    }
}