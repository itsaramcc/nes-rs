

mod cpu;
mod opcodes;


fn main() {
    let mut cpu = cpu::Cpu::init();
    cpu.mem[cpu.pc as usize] = 0x69;
    cpu.mem[cpu.pc as usize +1] = 0x07;
    cpu.cycle();
    println!("Hello, world!");
}
