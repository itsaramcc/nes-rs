use super::*;

pub struct Opcodes {
	pub instruction: fn(&mut Cpu) -> u8,
	pub address_mode: fn(&mut Cpu) -> u8,
	pub cycles: u8,
}


//
// Opcodes

fn adc(cpu: &mut Cpu) -> u8 {
	cpu.fetch();

	let tmp: u16 = cpu.ac as u16 + cpu.fetched as u16 + cpu.get_flag('C') as u16;
	cpu.set_flag('C', tmp > 255);
	cpu.set_flag('Z', (tmp & 0x00FF) == 0);
	cpu.set_flag('N', (tmp & 0x80) != 0);
	cpu.set_flag('V', ((tmp & cpu.ac as u16) & (tmp ^ cpu.fetched as u16) & 0x0080) != 0);

	cpu.ac = tmp as u8;

	1
}
fn and(cpu: &mut Cpu) -> u8 {
	cpu.fetch();

	cpu.ac &= cpu.fetched;
	cpu.set_flag('Z', cpu.ac == 0);
	cpu.set_flag('N', (cpu.ac & 0x80) != 0);

	1
}
fn asl(cpu: &mut Cpu) -> u8 {
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

	0
}
fn bcc(cpu: &mut Cpu) -> u8 {
	if cpu.get_flag('C') == 0 {
		cpu.addr_abs = cpu.pc + cpu.addr_rel;

		cpu.cycles += 1;
		if cpu.addr_abs & 0xFF00 != cpu.pc & 0xFF00 { cpu.cycles += 1; }

		cpu.pc = cpu.addr_abs;
	}

	0
}
fn bcs(cpu: &mut Cpu) -> u8 {
	if cpu.get_flag('C') == 1 {
		cpu.addr_abs = cpu.pc + cpu.addr_rel;

		cpu.cycles += 1;
		if cpu.addr_abs & 0xFF00 != cpu.pc & 0xFF00 { cpu.cycles += 1; }

		cpu.pc = cpu.addr_abs;
	}

	0
}
fn beq(cpu: &mut Cpu) -> u8 {
	if cpu.get_flag('Z') == 1 {
		cpu.addr_abs = cpu.pc + cpu.addr_rel;

		cpu.cycles += 1;
		if cpu.addr_abs & 0xFF00 != cpu.pc & 0xFF00 { cpu.cycles += 1; }

		cpu.pc = cpu.addr_abs;
	}

	0
}
fn bit(cpu: &mut Cpu) -> u8 {
	cpu.fetch();

	let tmp = cpu.ac & cpu.fetched;
	cpu.set_flag('N', (cpu.fetched & 0x80) != 0);
	cpu.set_flag('V', (cpu.fetched & 0x40) != 0);
	cpu.set_flag('Z', (tmp & 0x00FF) == 0);

	0
}
fn bmi(cpu: &mut Cpu) -> u8 {
	if cpu.get_flag('N') == 1 {
		cpu.addr_abs = cpu.pc + cpu.addr_rel;

		cpu.cycles += 1;
		if cpu.addr_abs & 0xFF00 != cpu.pc & 0xFF00 { cpu.cycles += 1; }

		cpu.pc = cpu.addr_abs;
	}

	0
}
fn bne(cpu: &mut Cpu) -> u8 {
	if cpu.get_flag('Z') == 0 {
		cpu.addr_abs = cpu.pc + cpu.addr_rel;

		cpu.cycles += 1;
		if cpu.addr_abs & 0xFF00 != cpu.pc & 0xFF00 { cpu.cycles += 1; }

		cpu.pc = cpu.addr_abs;
	}

	0
}
fn bpl(cpu: &mut Cpu) -> u8 {
	if cpu.get_flag('N') == 0 {
		cpu.addr_abs = cpu.pc + cpu.addr_rel;

		cpu.cycles += 1;
		if cpu.addr_abs & 0xFF00 != cpu.pc & 0xFF00 { cpu.cycles += 1; }

		cpu.pc = cpu.addr_abs;
	}

	0
}
fn brk(cpu: &mut Cpu) -> u8 {
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

	0
}
fn bvc(cpu: &mut Cpu) -> u8 {
	if cpu.get_flag('V') == 0 {
		cpu.addr_abs = cpu.pc + cpu.addr_rel;

		cpu.cycles += 1;
		if cpu.addr_abs & 0xFF00 != cpu.pc & 0xFF00 { cpu.cycles += 1; }

		cpu.pc = cpu.addr_abs;
	}

	0
}
fn bvs(cpu: &mut Cpu) -> u8 {
	if cpu.get_flag('V') == 1 {
		cpu.addr_abs = cpu.pc + cpu.addr_rel;

		cpu.cycles += 1;
		if cpu.addr_abs & 0xFF00 != cpu.pc & 0xFF00 { cpu.cycles += 1; }

		cpu.pc = cpu.addr_abs;
	}

	0
}
fn clc(cpu: &mut Cpu) -> u8 {
	cpu.set_flag('C', false);

	0
}
fn cld(cpu: &mut Cpu) -> u8 {
	cpu.set_flag('D', false);

	0
}
fn cli(cpu: &mut Cpu) -> u8 {
	cpu.set_flag('I', false);

	0
}
fn clv(cpu: &mut Cpu) -> u8 {
	cpu.set_flag('V', false);

	0
}
fn cmp(cpu: &mut Cpu) -> u8 {
	cpu.fetch();

	let tmp = cpu.ac.overflowing_sub(cpu.fetched).0;
	cpu.set_flag('C', cpu.ac >= cpu.fetched);
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);

	1
}
fn cpx(cpu: &mut Cpu) -> u8 {
	cpu.fetch();

	let tmp = cpu.x.overflowing_sub(cpu.fetched).0;
	cpu.set_flag('C', cpu.x >= cpu.fetched);
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);

	0
}
fn cpy(cpu: &mut Cpu) -> u8 {
	cpu.fetch();

	let tmp = cpu.y.overflowing_sub(cpu.fetched).0;
	cpu.set_flag('C', cpu.y >= cpu.fetched);
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);

	0
}
fn dec(cpu: &mut Cpu) -> u8 {
	cpu.fetch();

	let tmp = cpu.fetched.overflowing_sub(1).0;
	cpu.mem[cpu.addr_abs as usize] = tmp;
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);

	0
}
fn dex(cpu: &mut Cpu) -> u8 {
	let tmp = cpu.x.overflowing_sub(1).0;
	cpu.x = tmp;
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);

	0
}
fn dey(cpu: &mut Cpu) -> u8 {
	let tmp = cpu.y.overflowing_sub(1).0;
	cpu.y = tmp;
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);

	0
}
fn eor(cpu: &mut Cpu) -> u8 {
	cpu.fetch();

	let tmp = cpu.ac ^ cpu.fetched;
	cpu.ac = tmp;
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);

	1
}
fn inc(cpu: &mut Cpu) -> u8 {
	cpu.fetch();

	let tmp = cpu.fetched.overflowing_add(1).0;
	cpu.mem[cpu.addr_abs as usize] = tmp;
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);

	0
}
fn inx(cpu: &mut Cpu) -> u8 {
	let tmp = cpu.x.overflowing_add(1).0;
	cpu.x = tmp;
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);

	0
}
fn iny(cpu: &mut Cpu) -> u8 {
	let tmp = cpu.y.overflowing_add(1).0;
	cpu.y = tmp;
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);

	0
}
fn jmp(cpu: &mut Cpu) -> u8 {
	cpu.pc = cpu.addr_abs;

	0
}
fn jsr(cpu: &mut Cpu) -> u8 {
	cpu.pc -= 1;

	cpu.mem[0x0100 + cpu.sp as usize] = ((cpu.pc & 0xFF00) >> 8) as u8;
	cpu.sp = cpu.sp.overflowing_sub(1).0;
	cpu.mem[0x0100 + cpu.sp as usize] = (cpu.pc & 0x00FF) as u8;
	cpu.sp = cpu.sp.overflowing_sub(1).0;

	cpu.pc = cpu.addr_abs;

	0
}
fn lda(cpu: &mut Cpu) -> u8 {
	cpu.fetch();

	cpu.ac = cpu.fetched;
	cpu.set_flag('Z', cpu.ac == 0x00);
	cpu.set_flag('N', (cpu.ac & 0x80) != 0);

	1
}
fn ldx(cpu: &mut Cpu) -> u8 {
	cpu.fetch();

	cpu.x = cpu.fetched;
	cpu.set_flag('Z', cpu.x == 0x00);
	cpu.set_flag('N', (cpu.x & 0x80) != 0);

	1
}
fn ldy(cpu: &mut Cpu) -> u8 {
	cpu.fetch();

	cpu.y = cpu.fetched;
	cpu.set_flag('Z', cpu.y == 0x00);
	cpu.set_flag('N', (cpu.y & 0x80) != 0);

	1
}
fn lsr(cpu: &mut Cpu) -> u8 {
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

	0
}
fn nop(_: &mut Cpu) -> u8 { 0 }
fn ora(cpu: &mut Cpu) -> u8 {
	cpu.fetch();

	let tmp = cpu.ac | cpu.fetched;
	cpu.ac = tmp;
	cpu.set_flag('Z', tmp == 0x00);
	cpu.set_flag('N', (tmp & 0x80) != 0);

	1
}
fn pha(cpu: &mut Cpu) -> u8 {
	cpu.mem[0x0100 + cpu.sp as usize] = cpu.ac;
	cpu.sp = cpu.sp.overflowing_sub(1).0;

	0
}
fn php(cpu: &mut Cpu) -> u8 {
	cpu.mem[0x0100 + cpu.sp as usize] = cpu.sr;
	cpu.sp = cpu.sp.overflowing_sub(1).0;

	0
}
fn pla(cpu: &mut Cpu) -> u8 {
	cpu.sp = cpu.sp.overflowing_add(1).0;
	cpu.ac = cpu.mem[0x0100 + cpu.sp as usize];
	cpu.set_flag('Z', cpu.ac == 0x00);
	cpu.set_flag('N', (cpu.ac & 0x80) != 0);

	0
}
fn plp(cpu: &mut Cpu) -> u8 {
	cpu.sp = cpu.sp.overflowing_add(1).0;
	cpu.sr = cpu.mem[0x0100 + cpu.sp as usize];

	0
}
fn rol(cpu: &mut Cpu) -> u8 {
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

	0
}
fn ror(cpu: &mut Cpu) -> u8 {
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

	0
}
fn rti(cpu: &mut Cpu) -> u8 {
	cpu.sp = cpu.sp.overflowing_add(1).0;
	cpu.sr = cpu.mem[0x0100 + cpu.sp as usize];

	cpu.sp = cpu.sp.overflowing_add(1).0;
	cpu.pc = cpu.mem[0x0100 + cpu.sp as usize] as u16;
	cpu.sp = cpu.sp.overflowing_add(1).0;
	cpu.pc |= (cpu.mem[0x0100 + cpu.sp as usize] as u16) << 8;

	0
}
fn rts(cpu: &mut Cpu) -> u8 {
	cpu.sp = cpu.sp.overflowing_add(1).0;
	cpu.pc = cpu.mem[0x0100 + cpu.sp as usize] as u16;
	cpu.sp = cpu.sp.overflowing_add(1).0;
	cpu.pc |= (cpu.mem[0x0100 + cpu.sp as usize] as u16) << 8;

	cpu.pc += 1;

	0
}
fn sbc(cpu: &mut Cpu) -> u8 {
	cpu.fetch();

	let val: u16 = cpu.fetched as u16 ^ 0x00FF;
	let tmp: u16 = cpu.ac as u16 + val + cpu.get_flag('C') as u16;
	cpu.set_flag('C', tmp > 255);
	cpu.set_flag('Z', (tmp & 0x00FF) == 0);
	cpu.set_flag('N', (tmp & 0x80) != 0);
	cpu.set_flag('V', ((tmp & cpu.ac as u16) & (tmp ^ val) & 0x0080) != 0);

	cpu.ac = tmp as u8;

	1
}
fn sec(cpu: &mut Cpu) -> u8 {
	cpu.set_flag('C', true);

	0
}
fn sed(cpu: &mut Cpu) -> u8 {
	cpu.set_flag('D', true);

	0
}
fn sei(cpu: &mut Cpu) -> u8 {
	cpu.set_flag('I', true);

	0
}
fn sta(cpu: &mut Cpu) -> u8 {
	cpu.mem[cpu.addr_abs as usize] = cpu.ac;

	0
}
fn stx(cpu: &mut Cpu) -> u8 {
	cpu.mem[cpu.addr_abs as usize] = cpu.x;

	0
}
fn sty(cpu: &mut Cpu) -> u8 {
	cpu.mem[cpu.addr_abs as usize] = cpu.y;

	0
}
fn tax(cpu: &mut Cpu) -> u8 {
	cpu.x = cpu.ac;
	cpu.set_flag('Z', cpu.x == 0);
	cpu.set_flag('N', (cpu.x & 0x80) != 0);

	0
}
fn tay(cpu: &mut Cpu) -> u8 {
	cpu.y = cpu.ac;
	cpu.set_flag('Z', cpu.y == 0);
	cpu.set_flag('N', (cpu.y & 0x80) != 0);

	0
}
fn tsx(cpu: &mut Cpu) -> u8 {
	cpu.x = cpu.sp;
	cpu.set_flag('Z', cpu.x == 0);
	cpu.set_flag('N', (cpu.x & 0x80) != 0);

	0
}
fn txa(cpu: &mut Cpu) -> u8 {
	cpu.ac = cpu.x;
	cpu.set_flag('Z', cpu.ac == 0);
	cpu.set_flag('N', (cpu.ac & 0x80) != 0);

	0
}
fn txs(cpu: &mut Cpu) -> u8 {
	cpu.sp = cpu.x;

	0
}
fn tya(cpu: &mut Cpu) -> u8 {
	cpu.ac = cpu.y;
	cpu.set_flag('Z', cpu.y == 0);
	cpu.set_flag('N', (cpu.y & 0x80) != 0);

	0
}
fn xxx(_: &mut Cpu) -> u8 { 0 }



//
// Address Modes

fn abs(cpu: &mut Cpu) -> u8 {
	let lo = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;
	let hi = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;

	cpu.addr_abs = (hi << 8) | lo;

	0
}

fn abx(cpu: &mut Cpu) -> u8 {
	let lo = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;
	let hi = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;

	cpu.addr_abs = ((hi << 8) | lo) + cpu.x as u16;

	if cpu.addr_abs & 0xFF00 != hi << 8 { return 1; }

	0
}	

fn aby(cpu: &mut Cpu) -> u8 {
	let lo = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;
	let hi = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;

	cpu.addr_abs = ((hi << 8) | lo) + cpu.y as u16;

	if cpu.addr_abs & 0xFF00 != hi << 8 { return 1; }

	0
}


fn imm(cpu: &mut Cpu) -> u8 {
	cpu.addr_abs = cpu.pc;
	cpu.pc += 1;

	0
}

pub fn imp(cpu: &mut Cpu) -> u8 {
	cpu.fetched = cpu.ac;

	0
}

fn ind(cpu: &mut Cpu) -> u8 {
	let lo = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;
	let hi = cpu.mem[cpu.pc as usize +1] as u16;
	cpu.pc += 1;	

	let tmp = (hi << 8) | lo;

	cpu.addr_abs = ((cpu.mem[tmp as usize + 1] as u16) << 8) | cpu.mem[tmp as usize] as u16;

	0
}

fn xid(cpu: &mut Cpu) -> u8 {
	let tmp = cpu.x.wrapping_add(cpu.mem[cpu.pc as usize]) as u16;
	cpu.pc += 1;

	let lo = cpu.mem[tmp as usize] as u16;
	let hi = cpu.mem[tmp as usize +1] as u16;

	cpu.addr_abs = (hi << 8) | lo;

	0
}

fn idy(cpu: &mut Cpu) -> u8 {
	let tmp = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;

	let lo = cpu.mem[tmp as usize] as u16;
	let hi = cpu.mem[tmp as usize +1] as u16;

	cpu.addr_abs = ((hi << 8) | lo) + cpu.y as u16;

	if cpu.addr_abs & 0xFF00 != hi << 8 { return 1; }

	0
}

fn rel(cpu: &mut Cpu) -> u8 {
	cpu.addr_rel = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;

	if cpu.addr_rel & 0x80 != 0 {
		cpu.addr_rel |= 0xFF00;
	}

	0
}

fn zpg(cpu: &mut Cpu) -> u8 {
	cpu.addr_abs = cpu.mem[cpu.pc as usize] as u16;
	cpu.pc += 1;
	
	0
}

fn zpx(cpu: &mut Cpu) -> u8 {
	cpu.addr_abs = cpu.x.wrapping_add(cpu.mem[cpu.pc as usize]) as u16;
	cpu.pc += 1;

	0
}

fn zpy(cpu: &mut Cpu) -> u8 {
	cpu.addr_abs = cpu.y.wrapping_add(cpu.mem[cpu.pc as usize]) as u16;
	cpu.pc += 1;

	0
}

pub const LOOK_UP: [Opcodes; 256] = [
	Opcodes { instruction: brk, address_mode: imp, cycles: 7 }, Opcodes { instruction: ora, address_mode: xid, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: ora, address_mode: zpg, cycles: 3 }, Opcodes { instruction: asl, address_mode: zpg, cycles: 5 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: php, address_mode: imp, cycles: 3 }, Opcodes { instruction: ora, address_mode: imm, cycles: 2 }, Opcodes { instruction: asl, address_mode: imp, cycles: 2 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: ora, address_mode: abs, cycles: 4 }, Opcodes { instruction: asl, address_mode: abs, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 },
	Opcodes { instruction: bpl, address_mode: rel, cycles: 2 }, Opcodes { instruction: ora, address_mode: idy, cycles: 5 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: ora, address_mode: zpx, cycles: 4 }, Opcodes { instruction: asl, address_mode: zpx, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: clc, address_mode: imp, cycles: 2 }, Opcodes { instruction: ora, address_mode: aby, cycles: 4 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: ora, address_mode: abx, cycles: 4 }, Opcodes { instruction: asl, address_mode: abx, cycles: 7 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 },
	Opcodes { instruction: jsr, address_mode: abs, cycles: 6 }, Opcodes { instruction: and, address_mode: xid, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: bit, address_mode: zpg, cycles: 3 }, Opcodes { instruction: and, address_mode: zpg, cycles: 3 }, Opcodes { instruction: rol, address_mode: zpg, cycles: 5 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: plp, address_mode: imp, cycles: 4 }, Opcodes { instruction: and, address_mode: imm, cycles: 2 }, Opcodes { instruction: rol, address_mode: imp, cycles: 2 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: bit, address_mode: abs, cycles: 4 }, Opcodes { instruction: and, address_mode: abs, cycles: 4 }, Opcodes { instruction: rol, address_mode: abs, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 },
	Opcodes { instruction: bmi, address_mode: rel, cycles: 2 }, Opcodes { instruction: and, address_mode: idy, cycles: 5 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: and, address_mode: zpx, cycles: 4 }, Opcodes { instruction: rol, address_mode: zpx, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: sec, address_mode: imp, cycles: 2 }, Opcodes { instruction: and, address_mode: aby, cycles: 4 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: and, address_mode: abx, cycles: 4 }, Opcodes { instruction: rol, address_mode: abx, cycles: 7 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 },
	Opcodes { instruction: rti, address_mode: imp, cycles: 6 }, Opcodes { instruction: eor, address_mode: xid, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: eor, address_mode: zpg, cycles: 3 }, Opcodes { instruction: lsr, address_mode: zpg, cycles: 5 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: pha, address_mode: imp, cycles: 3 }, Opcodes { instruction: eor, address_mode: imm, cycles: 2 }, Opcodes { instruction: lsr, address_mode: imp, cycles: 2 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: jmp, address_mode: abs, cycles: 3 }, Opcodes { instruction: eor, address_mode: abs, cycles: 4 }, Opcodes { instruction: lsr, address_mode: abs, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 },
	Opcodes { instruction: bvc, address_mode: rel, cycles: 2 }, Opcodes { instruction: eor, address_mode: idy, cycles: 5 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: eor, address_mode: zpx, cycles: 4 }, Opcodes { instruction: lsr, address_mode: zpx, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: cli, address_mode: imp, cycles: 2 }, Opcodes { instruction: eor, address_mode: aby, cycles: 4 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: eor, address_mode: abx, cycles: 4 }, Opcodes { instruction: lsr, address_mode: abx, cycles: 7 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 },
	Opcodes { instruction: rts, address_mode: imp, cycles: 6 }, Opcodes { instruction: adc, address_mode: xid, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: adc, address_mode: zpg, cycles: 3 }, Opcodes { instruction: ror, address_mode: zpg, cycles: 5 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: pla, address_mode: imp, cycles: 4 }, Opcodes { instruction: adc, address_mode: imm, cycles: 2 }, Opcodes { instruction: ror, address_mode: imp, cycles: 2 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: jmp, address_mode: ind, cycles: 5 }, Opcodes { instruction: adc, address_mode: abs, cycles: 4 }, Opcodes { instruction: ror, address_mode: abs, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 },
	Opcodes { instruction: bvs, address_mode: rel, cycles: 2 }, Opcodes { instruction: adc, address_mode: idy, cycles: 5 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: adc, address_mode: zpx, cycles: 4 }, Opcodes { instruction: ror, address_mode: zpx, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: sei, address_mode: imp, cycles: 2 }, Opcodes { instruction: adc, address_mode: aby, cycles: 4 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: adc, address_mode: abx, cycles: 4 }, Opcodes { instruction: ror, address_mode: abx, cycles: 7 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 },
	Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: sta, address_mode: xid, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: sty, address_mode: zpg, cycles: 3 }, Opcodes { instruction: sta, address_mode: zpg, cycles: 3 }, Opcodes { instruction: stx, address_mode: zpg, cycles: 3 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: dey, address_mode: imp, cycles: 2 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: txa, address_mode: imp, cycles: 2 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: sty, address_mode: abs, cycles: 4 }, Opcodes { instruction: sta, address_mode: abs, cycles: 4 }, Opcodes { instruction: stx, address_mode: abs, cycles: 4 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 },
	Opcodes { instruction: bcc, address_mode: rel, cycles: 2 }, Opcodes { instruction: sta, address_mode: idy, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: sty, address_mode: zpx, cycles: 4 }, Opcodes { instruction: sta, address_mode: zpx, cycles: 4 }, Opcodes { instruction: stx, address_mode: zpy, cycles: 4 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: tya, address_mode: imp, cycles: 2 }, Opcodes { instruction: sta, address_mode: aby, cycles: 5 }, Opcodes { instruction: txs, address_mode: imp, cycles: 2 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: sta, address_mode: abx, cycles: 5 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 },
	Opcodes { instruction: ldy, address_mode: imm, cycles: 2 }, Opcodes { instruction: lda, address_mode: xid, cycles: 6 }, Opcodes { instruction: ldx, address_mode: imm, cycles: 2 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: ldy, address_mode: zpg, cycles: 3 }, Opcodes { instruction: lda, address_mode: zpg, cycles: 3 }, Opcodes { instruction: ldx, address_mode: zpg, cycles: 3 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: tay, address_mode: imp, cycles: 2 }, Opcodes { instruction: lda, address_mode: imm, cycles: 2 }, Opcodes { instruction: tax, address_mode: imp, cycles: 2 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: ldy, address_mode: abs, cycles: 4 }, Opcodes { instruction: lda, address_mode: abs, cycles: 4 }, Opcodes { instruction: ldx, address_mode: abs, cycles: 4 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 },
	Opcodes { instruction: bcs, address_mode: rel, cycles: 2 }, Opcodes { instruction: lda, address_mode: idy, cycles: 5 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: ldy, address_mode: zpx, cycles: 4 }, Opcodes { instruction: lda, address_mode: zpx, cycles: 4 }, Opcodes { instruction: ldx, address_mode: zpy, cycles: 4 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: clv, address_mode: imp, cycles: 2 }, Opcodes { instruction: lda, address_mode: aby, cycles: 4 }, Opcodes { instruction: tsx, address_mode: imp, cycles: 2 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: ldy, address_mode: abx, cycles: 4 }, Opcodes { instruction: lda, address_mode: abx, cycles: 4 }, Opcodes { instruction: ldx, address_mode: aby, cycles: 4 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 },
	Opcodes { instruction: cpy, address_mode: imm, cycles: 2 }, Opcodes { instruction: cmp, address_mode: xid, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: cpy, address_mode: zpg, cycles: 3 }, Opcodes { instruction: cmp, address_mode: zpg, cycles: 3 }, Opcodes { instruction: dec, address_mode: zpg, cycles: 5 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: iny, address_mode: imp, cycles: 2 }, Opcodes { instruction: cmp, address_mode: imm, cycles: 2 }, Opcodes { instruction: dex, address_mode: imp, cycles: 2 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: cpy, address_mode: abs, cycles: 4 }, Opcodes { instruction: cmp, address_mode: abs, cycles: 4 }, Opcodes { instruction: dec, address_mode: abs, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 },
	Opcodes { instruction: bne, address_mode: rel, cycles: 2 }, Opcodes { instruction: cmp, address_mode: idy, cycles: 5 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: cmp, address_mode: zpx, cycles: 4 }, Opcodes { instruction: dec, address_mode: zpx, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: cld, address_mode: imp, cycles: 2 }, Opcodes { instruction: cmp, address_mode: aby, cycles: 4 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: cmp, address_mode: abx, cycles: 4 }, Opcodes { instruction: dec, address_mode: abx, cycles: 7 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 },
	Opcodes { instruction: cpx, address_mode: imm, cycles: 2 }, Opcodes { instruction: sbc, address_mode: xid, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: cpx, address_mode: zpg, cycles: 3 }, Opcodes { instruction: sbc, address_mode: zpg, cycles: 3 }, Opcodes { instruction: inc, address_mode: zpg, cycles: 5 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: inx, address_mode: imp, cycles: 2 }, Opcodes { instruction: sbc, address_mode: imm, cycles: 2 }, Opcodes { instruction: nop, address_mode: imp, cycles: 2 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: cpx, address_mode: abs, cycles: 4 }, Opcodes { instruction: sbc, address_mode: abs, cycles: 4 }, Opcodes { instruction: inc, address_mode: abs, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 },
	Opcodes { instruction: beq, address_mode: rel, cycles: 2 }, Opcodes { instruction: sbc, address_mode: idy, cycles: 5 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: sbc, address_mode: zpx, cycles: 4 }, Opcodes { instruction: inc, address_mode: zpx, cycles: 6 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: sed, address_mode: imp, cycles: 2 }, Opcodes { instruction: sbc, address_mode: aby, cycles: 4 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 }, Opcodes { instruction: sbc, address_mode: abx, cycles: 4 }, Opcodes { instruction: inc, address_mode: abx, cycles: 7 }, Opcodes { instruction: xxx, address_mode: imp, cycles: 0 },
];
