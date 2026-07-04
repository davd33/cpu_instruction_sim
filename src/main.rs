mod cpu_sim;
mod decode;
mod cycles_stats;

use cpu_sim::{CPU8086};
use cycles_stats::CyclesStat;
use std::process::exit;
use std::{env, fs};
fn print_usage(program: &str) {
    println!("Usage: {} binary_path [--sim | --dis] [--help]", program);
}

#[derive(PartialEq)]
enum AppMode {
    Simulation,
    Disassembly,
}

struct ProgramArgs {
    command: AppMode,
    file_path: String,
    debug: bool,
}

fn parse_args() -> ProgramArgs {
    let args: Vec<String> = env::args().collect();

    let program_name = args[0].clone();
    let file_path = args[1].clone();
    let mut app_command: Option<AppMode> = Some(AppMode::Disassembly);
    let mut debugging = false;
    for i in 2..args.len() {
        match args[i].as_ref() {
            "--sim" => {
                app_command = Some(AppMode::Simulation);
            }
            "--dis" => {
                app_command = Some(AppMode::Disassembly);
            }
            "--help" => {
                print_usage(&program_name);
                exit(0);
            }
            "--debug" => {
                debugging = true;
            }
            _ => {
                println!("Unknown command: {}", args[i]);
                print_usage(&program_name);
                exit(1);
            }
        }
    }

    if let Some(command) = app_command {
        ProgramArgs { command, file_path, debug: debugging }
    } else {
        println!("Could not find app command: {}", program_name);
        print_usage(&program_name);
        exit(1);
    }
}

const DIRECT_ADDRESS_MOD: u8 = 0x00;
const D8_MOD: u8 = 0x40;
const D16_MOD: u8 = 0x80;
const MOD11: u8 = 0xC0;

fn main() {
    let mut cycles_stats = vec![];
    cycles_stats.push(CyclesStat::new("program start", cycles_stats::rdtsc()));

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file name for an ASM to be decoded.");
        exit(1);
    }

    let program_args: &ProgramArgs = &parse_args();

    cycles_stats.push(CyclesStat::new("args read", cycles_stats::rdtsc()));

    if let Ok(asm_bytes) = fs::read(program_args.file_path.clone()) {

        let mut cpu: Option<CPU8086> = None;
        if program_args.command == AppMode::Simulation {
            cpu = Some(CPU8086::new());
        }

        let mut current = 0;
        while current < asm_bytes.len() {
            current = decode::process_instruction(
                &program_args.command,
                program_args.debug,
                &asm_bytes,
                current,
                &mut cpu,
                &decode::mod11_registers_table(),
                &decode::reg_mem_registers_table(),
                &decode::get_commands_map());
        }

        if let Some(cpu) = cpu {
            println!("\nFinal registers:{}", cpu);
        }
    } else {
        println!("File not found");
    }

    cycles_stats.push(CyclesStat::new("inst. stream printed", cycles_stats::rdtsc()));
    let mut current_cycles = cycles_stats[0].cycles;
    for c in &cycles_stats {
        eprintln!("{} {}", c.label, c.cycles - current_cycles);
        current_cycles = c.cycles;
    }
}
