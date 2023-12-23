//
// 6502 Instruction Set:
//	https://www.masswerk.at/6502/6502_instruction_set.html
//	http://www.6502.org/tutorials/6502opcodes.html
//
// 6502 Programming Manual:
//	http://archive.6502.org/datasheets/synertek_programming_manual.pdf
//  https://www.nesdev.org/NESDoc.pdf
//
// 6502 Guide:
//	https://www.nesdev.org/obelisk-6502-guide/index.html


mod opcodes;
use opcodes::*;

// 6502 Processor
pub struct Cpu {
	pub opcode: u8,

	pub mem: [u8; 64*1024],
	pub pc: u16,
	pub sp: u8,				// Stack locatated between $0100 and $01FF
							// decrement on push, increment on pop
	pub ac: u8,
	pub x: u8,
	pub y: u8,
	pub sr: u8,				// Negative, Overflow, 1, Break, Decimal, Interrupt, Zero, Carry

	pub addr_abs: u16,
	pub addr_rel: u16,
	pub fetched: u8,

	pub cycles: u8,
	pub global_clock: u128,
}

impl Cpu {

	pub fn init() -> Self {
		Self {
			opcode: 0x00,

			mem: [0; 64*1024],
			pc: 0x0200,
			sp: 0xFF,
			ac: 0x00,
			x: 0x00,
			y: 0x00,
			sr: 0b00100000,

			addr_abs: 0,
			addr_rel: 0,
			fetched: 0,

			cycles: 0,
			global_clock: 0
		}
	}

	pub fn cycle(&mut self) {
		self.global_clock += 1;
		if self.cycles != 0 {
			self.cycles -= 1;
			return;
		}

		self.opcode = self.mem[self.pc as usize];
		self.pc += 1;

		self.cycles = LOOK_UP[self.opcode as usize].cycles;
		let extra_cycles1 = (LOOK_UP[self.opcode as usize].address_mode)(self);
		let extra_cycles2 = (LOOK_UP[self.opcode as usize].instruction)(self);

		self.cycles += extra_cycles1 & extra_cycles2;
	}

	pub fn fetch(&mut self) -> u8 {
		if LOOK_UP[self.opcode as usize].address_mode != imp {
			self.fetched = self.mem[self.addr_abs as usize];
		}

		self.fetched
	}


	//
	// Flags

	pub fn get_flag(&self, flag: char) -> u8 {
		match flag.to_ascii_uppercase() {
			'C' => (self.sr & 0b00000001) >> 0,
			'Z' => (self.sr & 0b00000010) >> 1,
			'I' => (self.sr & 0b00000100) >> 2,
			'D' => (self.sr & 0b00001000) >> 3,
			'B' => (self.sr & 0b00010000) >> 4,
			'V' => (self.sr & 0b01000000) >> 6,
			'N' => (self.sr & 0b10000000) >> 7,

			_ => 0
		}
	}

	pub fn set_flag(&mut self, flag: char, cond: bool) {
		if cond {
			self.sr |= match flag.to_ascii_uppercase() {
				'C' => 0b00000001,
				'Z' => 0b00000010,
				'I' => 0b00000100,
				'D' => 0b00001000,
				'B' => 0b00010000,
				'V' => 0b01000000,
				'N' => 0b10000000,

				_ => self.sr
			}
		} else {
			self.sr &= match flag.to_ascii_uppercase() {
				'C' => 0b11111110,
				'Z' => 0b11111101,
				'I' => 0b11111011,
				'D' => 0b11110111,
				'B' => 0b11101111,
				'V' => 0b10111111,
				'N' => 0b01111111,
	
				_ => self.sr
			}
		}
	}

}