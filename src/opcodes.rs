//
// 6502 Instruction Set:
//	https://www.masswerk.at/6502/6502_instruction_set.html
//	http://www.6502.org/tutorials/6502opcodes.html
//
// 6502 Programming Manual:
//	http://archive.6502.org/datasheets/synertek_programming_manual.pdf
//
// 6502 Guide:
//	https://www.nesdev.org/obelisk-6502-guide/index.html

use super::cpu::Cpu;

pub struct Opcodes {
	pub instruction: fn(&mut Cpu) -> (),
	pub address_mode: fn(&mut Cpu) -> u16,
}


//
// Opcodes

fn adc(cpu: &mut Cpu) {
	cpu.fetch();

	let tmp: u16 = cpu.ac as u16 + cpu.fetched as u16 + cpu.get_flag('C') as u16;
	cpu.set_flag('C', tmp > 255);
	cpu.set_flag('Z', (tmp & 0x00FF) == 0);
	cpu.set_flag('N', (tmp & 0x80) != 0);
	cpu.set_flag('V', ((tmp & cpu.ac as u16) & (tmp ^ cpu.fetched as u16) & 0x0080) != 0);

	cpu.ac = tmp as u8;
}
fn and(cpu: &mut Cpu) {
	cpu.fetch();

	cpu.ac &= cpu.fetched;
	cpu.set_flag('Z', cpu.ac == 0);
	cpu.set_flag('N', (cpu.ac & 0x80) != 0);
}
fn asl(cpu: &mut Cpu) {
	cpu.fetch();

	let tmp: u16 = (cpu.fetched as u16) << 1;
	cpu.set_flag('C', (cpu.fetched & 0x80) != 0);
	cpu.set_flag('Z', (tmp & 0x00FF) == 0);
	cpu.set_flag('N', (tmp & 0x80) != 0);

	if LOOK_UP[cpu.opcode as usize].address_mode == imp {
		cpu.ac = tmp as u8;
	} else {
		cpu.mem[cpu.addr_abs as usize] = tmp as u8;
	}
}
fn bcc(cpu: &mut Cpu) {
	if cpu.get_flag('C') == 0 {
		cpu.addr_abs = cpu.pc + cpu.addr_rel;
		cpu.pc = cpu.addr_abs;
	}
}
fn bcs(cpu: &mut Cpu) {
	if cpu.get_flag('C') == 1 {
		cpu.addr_abs = cpu.pc + cpu.addr_rel;
		cpu.pc = cpu.addr_abs;
	}
}
fn beq(cpu: &mut Cpu) {
	if cpu.get_flag('Z') == 1 {
		cpu.addr_abs = cpu.pc + cpu.addr_rel;
		cpu.pc = cpu.addr_abs;
	}
}
fn bit(cpu: &mut Cpu) {
	cpu.fetch();

	let tmp = cpu.ac & cpu.fetched;
	cpu.set_flag('N', (cpu.fetched & 0x80) != 0);
	cpu.set_flag('V', (cpu.fetched & 0x40) != 0);
	cpu.set_flag('Z', (tmp & 0x00FF) == 0);
}
fn bmi(cpu: &mut Cpu) {
	if cpu.get_flag('N') == 1 {
		cpu.addr_abs = cpu.pc + cpu.addr_rel;
		cpu.pc = cpu.addr_abs;
	}
}
fn bne(cpu: &mut Cpu) {
	if cpu.get_flag('Z') == 0 {
		cpu.addr_abs = cpu.pc + cpu.addr_rel;
		cpu.pc = cpu.addr_abs;
	}
}
fn bpl(cpu: &mut Cpu) {
	if cpu.get_flag('N') == 0 {
		cpu.addr_abs = cpu.pc + cpu.addr_rel;
		cpu.pc = cpu.addr_abs;
	}
}
fn brk(cpu: &mut Cpu) {
	cpu.pc += 1;
	
	cpu.mem[0x0100 + cpu.sp as usize] = ((cpu.pc & 0xFF00) >> 8) as u8;
	cpu.sp = cpu.sp.overflowing_sub(1).0;
	cpu.mem[0x0100 + cpu.sp as usize] = (cpu.pc & 0x00FF) as u8;
	cpu.sp = cpu.sp.overflowing_sub(1).0;

	cpu.set_flag('B', true);
	cpu.mem[0x0100 + cpu.sp as usize] = cpu.sr;
	cpu.sp = cpu.sp.overflowing_sub(1).0;
	cpu.set_flag('B', false);

	cpu.pc = cpu.mem[0xFFFE] as u16 | ((cpu.mem[0xFFFF] as u16) << 8);
}
fn bvc(cpu: &mut Cpu) {
	if cpu.get_flag('V') == 0 {
		cpu.addr_abs = cpu.pc + cpu.addr_rel;
		cpu.pc = cpu.addr_abs;
	}
}
fn bvs(cpu: &mut Cpu) {
	if cpu.get_flag('V') == 1 {
		cpu.addr_abs = cpu.pc + cpu.addr_rel;
		cpu.pc = cpu.addr_abs;
	}
}
fn clc(cpu: &mut Cpu) {
	cpu.set_flag('C', false);
}
fn cld(cpu: &mut Cpu) {
	cpu.set_flag('D', false);
}
fn cli(cpu: &mut Cpu) {
	cpu.set_flag('I', false);
}
fn clv(cpu: &mut Cpu) {
	cpu.set_flag('V', false);
}
fn cmp(cpu: &mut Cpu) {
	cpu.fetch();

	let tmp = cpu.ac.overflowing_sub(cpu.fetched).0;
	cpu.set_flag('C', cpu.ac >= cpu.fetched);
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);
}
fn cpx(cpu: &mut Cpu) {
	cpu.fetch();

	let tmp = cpu.x.overflowing_sub(cpu.fetched).0;
	cpu.set_flag('C', cpu.x >= cpu.fetched);
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);
}
fn cpy(cpu: &mut Cpu) {
	cpu.fetch();

	let tmp = cpu.y.overflowing_sub(cpu.fetched).0;
	cpu.set_flag('C', cpu.y >= cpu.fetched);
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);
}
fn dec(cpu: &mut Cpu) {
	cpu.fetch();

	let tmp = cpu.fetched.overflowing_sub(1).0;
	cpu.mem[cpu.addr_abs as usize] = tmp;
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);

}
fn dex(cpu: &mut Cpu) {
	let tmp = cpu.x.overflowing_sub(1).0;
	cpu.x = tmp;
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);
}
fn dey(cpu: &mut Cpu) {
	let tmp = cpu.y.overflowing_sub(1).0;
	cpu.y = tmp;
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);
}
fn eor(cpu: &mut Cpu) {
	cpu.fetch();

	let tmp = cpu.ac ^ cpu.fetched;
	cpu.ac = tmp;
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);
}
fn inc(cpu: &mut Cpu) {
	cpu.fetch();

	let tmp = cpu.fetched.overflowing_add(1).0;
	cpu.mem[cpu.addr_abs as usize] = tmp;
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);
}
fn inx(cpu: &mut Cpu) {
	let tmp = cpu.x.overflowing_add(1).0;
	cpu.x = tmp;
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);
}
fn iny(cpu: &mut Cpu) {
	let tmp = cpu.y.overflowing_add(1).0;
	cpu.y = tmp;
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);
}
fn jmp(cpu: &mut Cpu) {
	cpu.pc = cpu.addr_abs;
}
fn jsr(cpu: &mut Cpu) {
	cpu.pc -= 1;

	cpu.mem[0x0100 + cpu.sp as usize] = ((cpu.pc & 0xFF00) >> 8) as u8;
	cpu.sp = cpu.sp.overflowing_sub(1).0;
	cpu.mem[0x0100 + cpu.sp as usize] = (cpu.pc & 0x00FF) as u8;
	cpu.sp = cpu.sp.overflowing_sub(1).0;

	cpu.pc = cpu.addr_abs;
}
fn lda(cpu: &mut Cpu) {
	cpu.fetch();

	cpu.ac = cpu.fetched;
	cpu.set_flag('Z', cpu.ac == 0x00);
	cpu.set_flag('N', (cpu.ac & 0x80) != 0);
}
fn ldx(cpu: &mut Cpu) {
	cpu.fetch();

	cpu.x = cpu.fetched;
	cpu.set_flag('Z', cpu.x == 0x00);
	cpu.set_flag('N', (cpu.x & 0x80) != 0);
}
fn ldy(cpu: &mut Cpu) {
	cpu.fetch();

	cpu.y = cpu.fetched;
	cpu.set_flag('Z', cpu.y == 0x00);
	cpu.set_flag('N', (cpu.y & 0x80) != 0);
}
fn lsr(cpu: &mut Cpu) {
	cpu.fetch();

	let tmp = cpu.fetched >> 1;
	cpu.set_flag('C', (cpu.fetched & 0x1) != 0);
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);

	if LOOK_UP[cpu.opcode as usize].address_mode == imp {
		cpu.ac = tmp;
	} else {
		cpu.mem[cpu.addr_abs as usize] = tmp;
	}
}
fn nop(_: &mut Cpu) { }
fn ora(cpu: &mut Cpu) {
	cpu.fetch();

	let tmp = cpu.ac | cpu.fetched;
	cpu.ac = tmp;
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);
}
fn pha(cpu: &mut Cpu) {
	cpu.mem[0x0100 + cpu.sp as usize] = cpu.ac;
	cpu.sp = cpu.sp.overflowing_sub(1).0;
}
fn php(cpu: &mut Cpu) {
	cpu.mem[0x0100 + cpu.sp as usize] = cpu.sr;
	cpu.sp = cpu.sp.overflowing_sub(1).0;
}
fn pla(cpu: &mut Cpu) {
	cpu.sp = cpu.sp.overflowing_add(1).0;
	cpu.ac = cpu.mem[0x0100 + cpu.sp as usize];
	cpu.set_flag('Z', cpu.ac == 0x00);
	cpu.set_flag('N', (cpu.ac & 0x80) != 0);
}
fn plp(cpu: &mut Cpu) {
	cpu.sp = cpu.sp.overflowing_add(1).0;
	cpu.sr = cpu.mem[0x0100 + cpu.sp as usize];
}
fn rol(cpu: &mut Cpu) {
	cpu.fetch();

	let tmp: u16 = ((cpu.fetched as u16) << 1) + cpu.get_flag('C') as u16;
	cpu.set_flag('C', (cpu.fetched & 0x80) != 0);
	cpu.set_flag('Z', (tmp & 0x00FF) == 0);
	cpu.set_flag('N', (tmp & 0x80) != 0);

	if LOOK_UP[cpu.opcode as usize].address_mode == imp {
		cpu.ac = tmp as u8;
	} else {
		cpu.mem[cpu.addr_abs as usize] = tmp as u8;
	}
}
fn ror(cpu: &mut Cpu) {
	cpu.fetch();

	let tmp = (cpu.fetched >> 1) | (cpu.get_flag('C') * 0x80);
	cpu.set_flag('C', (cpu.fetched & 0x1) != 0);
	cpu.set_flag('Z', (tmp & 0x00FF) == 0);
	cpu.set_flag('N', (tmp & 0x80) != 0);

	if LOOK_UP[cpu.opcode as usize].address_mode == imp {
		cpu.ac = tmp as u8;
	} else {
		cpu.mem[cpu.addr_abs as usize] = tmp as u8;
	}
}
fn rti(cpu: &mut Cpu) {
	cpu.sp = cpu.sp.overflowing_add(1).0;
	cpu.sr = cpu.mem[0x0100 + cpu.sp as usize];

	cpu.sp = cpu.sp.overflowing_add(1).0;
	cpu.pc = cpu.mem[0x0100 + cpu.sp as usize] as u16;
	cpu.sp = cpu.sp.overflowing_add(1).0;
	cpu.pc |= (cpu.mem[0x0100 + cpu.sp as usize] as u16) << 8;

}
fn rts(cpu: &mut Cpu) {
	cpu.sp = cpu.sp.overflowing_add(1).0;
	cpu.pc = cpu.mem[0x0100 + cpu.sp as usize] as u16;
	cpu.sp = cpu.sp.overflowing_add(1).0;
	cpu.pc |= (cpu.mem[0x0100 + cpu.sp as usize] as u16) << 8;

	cpu.pc += 1;
}
fn sbc(cpu: &mut Cpu) {
	cpu.fetch();

	let val: u16 = cpu.fetched as u16 ^ 0x00FF;
	let tmp: u16 = cpu.ac as u16 + val + cpu.get_flag('C') as u16;
	cpu.set_flag('C', tmp > 255);
	cpu.set_flag('Z', (tmp & 0x00FF) == 0);
	cpu.set_flag('N', (tmp & 0x80) != 0);
	cpu.set_flag('V', ((tmp & cpu.ac as u16) & (tmp ^ val) & 0x0080) != 0);

	cpu.ac = tmp as u8;
}
fn sec(cpu: &mut Cpu) {
	cpu.set_flag('C', true);
}
fn sed(cpu: &mut Cpu) {
	cpu.set_flag('D', true);
}
fn sei(cpu: &mut Cpu) {
	cpu.set_flag('I', true);
}
fn sta(cpu: &mut Cpu) {
	cpu.mem[cpu.addr_abs as usize] = cpu.ac;
}
fn stx(cpu: &mut Cpu) {
	cpu.mem[cpu.addr_abs as usize] = cpu.x;
}
fn sty(cpu: &mut Cpu) {
	cpu.mem[cpu.addr_abs as usize] = cpu.y;
}
fn tax(cpu: &mut Cpu) {
	cpu.x = cpu.ac;
	cpu.set_flag('Z', cpu.x == 0);
	cpu.set_flag('N', (cpu.x & 0x80) != 0);
}
fn tay(cpu: &mut Cpu) {
	cpu.y = cpu.ac;
	cpu.set_flag('Z', cpu.y == 0);
	cpu.set_flag('N', (cpu.y & 0x80) != 0);
}
fn tsx(cpu: &mut Cpu) {
	cpu.x = cpu.sp;
	cpu.set_flag('Z', cpu.x == 0);
	cpu.set_flag('N', (cpu.x & 0x80) != 0);
}
fn txa(cpu: &mut Cpu) {
	cpu.ac = cpu.x;
	cpu.set_flag('Z', cpu.ac == 0);
	cpu.set_flag('N', (cpu.ac & 0x80) != 0);
}
fn txs(cpu: &mut Cpu) {
	cpu.sp = cpu.x;
}
fn tya(cpu: &mut Cpu) {
	cpu.ac = cpu.y;
	cpu.set_flag('Z', cpu.y == 0);
	cpu.set_flag('N', (cpu.y & 0x80) != 0);
}
fn xxx(_: &mut Cpu) { }



//
// Address Modes

fn abs(cpu: &mut Cpu) -> u16 {
	let lo = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;
	let hi = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;

	cpu.addr_abs = (hi << 8) | lo;

	cpu.addr_abs
}

fn abx(cpu: &mut Cpu) -> u16 {
	let lo = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;
	let hi = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;

	cpu.addr_abs = ((hi << 8) | lo) + cpu.x as u16;

	cpu.addr_abs
}	

fn aby(cpu: &mut Cpu) -> u16 {
	let lo = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;
	let hi = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;

	cpu.addr_abs = ((hi << 8) | lo) + cpu.y as u16;

	cpu.addr_abs
}


fn imm(cpu: &mut Cpu) -> u16 {
	cpu.addr_abs = cpu.pc;
	cpu.pc += 1;

	cpu.addr_abs
}

pub fn imp(cpu: &mut Cpu) -> u16 {
	cpu.fetched = cpu.ac;

	cpu.addr_abs
}

fn ind(cpu: &mut Cpu) -> u16 {
	let lo = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;
	let hi = cpu.mem[cpu.pc as usize +1] as u16;
	cpu.pc += 1;	

	let tmp = (hi << 8) | lo;

	cpu.addr_abs = ((cpu.mem[tmp as usize + 1] as u16) << 8) | cpu.mem[tmp as usize] as u16;

	cpu.addr_abs
}

fn xid(cpu: &mut Cpu) -> u16 {
	let tmp = cpu.x.wrapping_add(cpu.mem[cpu.pc as usize]) as u16;
	cpu.pc += 1;

	let lo = cpu.mem[tmp as usize] as u16;
	let hi = cpu.mem[tmp as usize +1] as u16;

	cpu.addr_abs = (hi << 8) | lo;

	cpu.addr_abs
}

fn idy(cpu: &mut Cpu) -> u16 {
	let tmp = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;

	let lo = cpu.mem[tmp as usize] as u16;
	let hi = cpu.mem[tmp as usize +1] as u16;

	cpu.addr_abs = ((hi << 8) | lo) + cpu.y as u16;

	cpu.addr_abs
}

fn rel(cpu: &mut Cpu) -> u16 {
	cpu.addr_rel = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;

	if cpu.addr_rel & 0x80 != 0 {
		cpu.addr_rel |= 0xFF00;
	}

	cpu.addr_abs
}

fn zpg(cpu: &mut Cpu) -> u16 {
	cpu.addr_abs = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;
	
	cpu.addr_abs
}

fn zpx(cpu: &mut Cpu) -> u16 {
	cpu.addr_abs = cpu.x.wrapping_add(cpu.mem[cpu.pc as usize]) as u16;
	cpu.pc += 1;

	cpu.addr_abs
}

fn zpy(cpu: &mut Cpu) -> u16 {
	cpu.addr_abs = cpu.y.wrapping_add(cpu.mem[cpu.pc as usize]) as u16;
	cpu.pc += 1;

	cpu.addr_abs
}

pub const LOOK_UP: [Opcodes; 256] = [
	Opcodes { instruction: brk, address_mode: imp }, Opcodes { instruction: ora, address_mode: xid }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: ora, address_mode: zpg }, Opcodes { instruction: asl, address_mode: zpg }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: php, address_mode: imp }, Opcodes { instruction: ora, address_mode: imm }, Opcodes { instruction: asl, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: ora, address_mode: abs }, Opcodes { instruction: asl, address_mode: abs }, Opcodes { instruction: xxx, address_mode: imp },
	Opcodes { instruction: bpl, address_mode: rel }, Opcodes { instruction: ora, address_mode: idy }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: ora, address_mode: zpx }, Opcodes { instruction: asl, address_mode: zpx }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: clc, address_mode: imp }, Opcodes { instruction: ora, address_mode: aby }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: ora, address_mode: abx }, Opcodes { instruction: asl, address_mode: abx }, Opcodes { instruction: xxx, address_mode: imp },
	Opcodes { instruction: jsr, address_mode: abs }, Opcodes { instruction: and, address_mode: xid }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: bit, address_mode: zpg }, Opcodes { instruction: and, address_mode: zpg }, Opcodes { instruction: rol, address_mode: zpg }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: plp, address_mode: imp }, Opcodes { instruction: and, address_mode: imm }, Opcodes { instruction: rol, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: bit, address_mode: abs }, Opcodes { instruction: and, address_mode: abs }, Opcodes { instruction: rol, address_mode: abs }, Opcodes { instruction: xxx, address_mode: imp },
	Opcodes { instruction: bmi, address_mode: rel }, Opcodes { instruction: and, address_mode: idy }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: and, address_mode: zpx }, Opcodes { instruction: rol, address_mode: zpx }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: sec, address_mode: imp }, Opcodes { instruction: and, address_mode: aby }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: and, address_mode: abx }, Opcodes { instruction: rol, address_mode: abx }, Opcodes { instruction: xxx, address_mode: imp },
	Opcodes { instruction: rti, address_mode: imp }, Opcodes { instruction: eor, address_mode: xid }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: eor, address_mode: zpg }, Opcodes { instruction: lsr, address_mode: zpg }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: pha, address_mode: imp }, Opcodes { instruction: eor, address_mode: imm }, Opcodes { instruction: lsr, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: jmp, address_mode: abs }, Opcodes { instruction: eor, address_mode: abs }, Opcodes { instruction: lsr, address_mode: abs }, Opcodes { instruction: xxx, address_mode: imp },
	Opcodes { instruction: bvc, address_mode: rel }, Opcodes { instruction: eor, address_mode: idy }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: eor, address_mode: zpx }, Opcodes { instruction: lsr, address_mode: zpx }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: cli, address_mode: imp }, Opcodes { instruction: eor, address_mode: aby }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: eor, address_mode: abx }, Opcodes { instruction: lsr, address_mode: abx }, Opcodes { instruction: xxx, address_mode: imp },
	Opcodes { instruction: rts, address_mode: imp }, Opcodes { instruction: adc, address_mode: xid }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: adc, address_mode: zpg }, Opcodes { instruction: ror, address_mode: zpg }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: pla, address_mode: imp }, Opcodes { instruction: adc, address_mode: imm }, Opcodes { instruction: ror, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: jmp, address_mode: ind }, Opcodes { instruction: adc, address_mode: abs }, Opcodes { instruction: ror, address_mode: abs }, Opcodes { instruction: xxx, address_mode: imp },
	Opcodes { instruction: bvs, address_mode: rel }, Opcodes { instruction: adc, address_mode: idy }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: adc, address_mode: zpx }, Opcodes { instruction: ror, address_mode: zpx }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: sei, address_mode: imp }, Opcodes { instruction: adc, address_mode: aby }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: adc, address_mode: abx }, Opcodes { instruction: ror, address_mode: abx }, Opcodes { instruction: xxx, address_mode: imp },
	Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: sta, address_mode: xid }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: sty, address_mode: zpg }, Opcodes { instruction: sta, address_mode: zpg }, Opcodes { instruction: stx, address_mode: zpg }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: dey, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: txa, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: sty, address_mode: abs }, Opcodes { instruction: sta, address_mode: abs }, Opcodes { instruction: stx, address_mode: abs }, Opcodes { instruction: xxx, address_mode: imp },
	Opcodes { instruction: bcc, address_mode: rel }, Opcodes { instruction: sta, address_mode: idy }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: sty, address_mode: zpx }, Opcodes { instruction: sta, address_mode: zpx }, Opcodes { instruction: stx, address_mode: zpy }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: tya, address_mode: imp }, Opcodes { instruction: sta, address_mode: aby }, Opcodes { instruction: txs, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: sta, address_mode: abx }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp },
	Opcodes { instruction: ldy, address_mode: imm }, Opcodes { instruction: lda, address_mode: xid }, Opcodes { instruction: ldx, address_mode: imm }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: ldy, address_mode: zpg }, Opcodes { instruction: lda, address_mode: zpg }, Opcodes { instruction: ldx, address_mode: zpg }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: tay, address_mode: imp }, Opcodes { instruction: lda, address_mode: imm }, Opcodes { instruction: tax, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: ldy, address_mode: abs }, Opcodes { instruction: lda, address_mode: abs }, Opcodes { instruction: ldx, address_mode: abs }, Opcodes { instruction: xxx, address_mode: imp },
	Opcodes { instruction: bcs, address_mode: rel }, Opcodes { instruction: lda, address_mode: idy }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: ldy, address_mode: zpx }, Opcodes { instruction: lda, address_mode: zpx }, Opcodes { instruction: ldx, address_mode: zpy }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: clv, address_mode: imp }, Opcodes { instruction: lda, address_mode: aby }, Opcodes { instruction: tsx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: ldy, address_mode: abx }, Opcodes { instruction: lda, address_mode: abx }, Opcodes { instruction: ldx, address_mode: aby }, Opcodes { instruction: xxx, address_mode: imp },
	Opcodes { instruction: cpy, address_mode: imm }, Opcodes { instruction: cmp, address_mode: xid }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: cpy, address_mode: zpg }, Opcodes { instruction: cmp, address_mode: zpg }, Opcodes { instruction: dec, address_mode: zpg }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: iny, address_mode: imp }, Opcodes { instruction: cmp, address_mode: imm }, Opcodes { instruction: dex, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: cpy, address_mode: abs }, Opcodes { instruction: cmp, address_mode: abs }, Opcodes { instruction: dec, address_mode: abs }, Opcodes { instruction: xxx, address_mode: imp },
	Opcodes { instruction: bne, address_mode: rel }, Opcodes { instruction: cmp, address_mode: idy }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: cmp, address_mode: zpx }, Opcodes { instruction: dec, address_mode: zpx }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: cld, address_mode: imp }, Opcodes { instruction: cmp, address_mode: aby }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: cmp, address_mode: abx }, Opcodes { instruction: dec, address_mode: abx }, Opcodes { instruction: xxx, address_mode: imp },
	Opcodes { instruction: cpx, address_mode: imm }, Opcodes { instruction: sbc, address_mode: xid }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: cpx, address_mode: zpg }, Opcodes { instruction: sbc, address_mode: zpg }, Opcodes { instruction: inc, address_mode: zpg }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: inx, address_mode: imp }, Opcodes { instruction: sbc, address_mode: imm }, Opcodes { instruction: nop, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: cpx, address_mode: abs }, Opcodes { instruction: sbc, address_mode: abs }, Opcodes { instruction: inc, address_mode: abs }, Opcodes { instruction: xxx, address_mode: imp },
	Opcodes { instruction: beq, address_mode: rel }, Opcodes { instruction: sbc, address_mode: idy }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: sbc, address_mode: zpx }, Opcodes { instruction: inc, address_mode: zpx }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: sed, address_mode: imp }, Opcodes { instruction: sbc, address_mode: aby }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: xxx, address_mode: imp }, Opcodes { instruction: sbc, address_mode: abx }, Opcodes { instruction: inc, address_mode: abx }, Opcodes { instruction: xxx, address_mode: imp },
];
