use mos6502::cpu;
use mos6502::instruction::{DecodedInstr, Nmos6502, OpInput};
use mos6502::memory::Bus;
use mos6502::memory::Memory;
use std::fs::read;

const MAX_INSTR: u32 = 100000;

fn main() {
    let zero_page_data = [13];

    // Load the binary file from disk
    let program = match read("examples/asm/fib.bin") {
        Ok(data) => data,
        Err(err) => {
            println!("Error reading euclid.bin: {}", err);
            return;
        }
    };

    let mut cpu = cpu::CPU::new(Memory::new(), Nmos6502);

    cpu.memory.set_bytes(0x00, &zero_page_data);
    cpu.memory.set_bytes(0x1000, &program);
    cpu.registers.program_counter = 0x1000;

    let mut i = 0;
    loop {
        println!(
            "A: {}, X: {}, Y: {}, SP: {:x}, PC: {:x}, status: {:?}",
            cpu.registers.accumulator,
            cpu.registers.index_x,
            cpu.registers.index_y,
            cpu.registers.stack_pointer.0,
            cpu.registers.program_counter,
            cpu.registers.status
        );

        let r = cpu.registers.program_counter;
        if let Some(next_instr) = cpu.fetch_next_and_decode() {
            print!("[{:x}] {:?} ", r, next_instr.0);
            match next_instr.1 {
                OpInput::UseImplied => println!("[implied]"),
                OpInput::UseImmediate(val) => {
                    println!("#${val}");
                }
                OpInput::UseRelative(addr) => {
                    let byte = cpu
                        .memory
                        .get_byte(cpu.registers.program_counter.wrapping_add(addr));
                    println!("r{addr} = {byte}");
                }
                OpInput::UseAddress(addr) => {
                    let byte = cpu.memory.get_byte(addr);
                    println!("a{addr:x} = {byte:x}");
                }
            }
        }
        cpu.registers.program_counter = r;

        if !cpu.single_step() || cpu.registers.program_counter == 0 || i >= MAX_INSTR {
            println!("Ran {i} instructions");
            break;
        }
        i += 1;
    }

    println!("Fib(n) is {}", cpu.memory.get_byte(0));
}
