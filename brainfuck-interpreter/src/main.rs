use brainfuck_interpreter::Interpreter;
use clap::{
    App,
    Arg,
};
use colored::*;
use memmap::Mmap;
use std::{
    convert::TryFrom,
    fs::OpenOptions,
    process,
};

fn main() {
    let args = App::new("brainfuck-interpreter")
        .version("1.0.0")
        .about("A simple Brainfuck interpreter written in Rust")
        .author("Umut S. <umutsahin@protonmail.com>")
        .arg(
            Arg::with_name("SCRIPT")
                .help("Sets the script to execute")
                .index(1)
                .required(true),
        )
        .get_matches();

    let script_file = OpenOptions::new()
        .read(true)
        .write(false)
        .open(args.value_of("SCRIPT").unwrap())
        .unwrap_or_else(|error| {
            eprintln!(
                "{} unable to open the {} file ({})",
                "io error:".red().bold(),
                "SCRIPT".red().bold(),
                error,
            );
            process::exit(1);
        });

    let script_file_in_memory = unsafe {
        match Mmap::map(&script_file) {
            Err(error) => {
                eprintln!(
                    "{} unable to map the provided {} file into memory ({})",
                    "memory mapping error:".red().bold(),
                    "SCRIPT".red().bold(),
                    error,
                );
                drop(script_file);
                process::exit(1);
            },
            Ok(script_file_in_memory) => script_file_in_memory,
        }
    };

    let interpreter = match Interpreter::try_from(&script_file_in_memory[..]) {
        Err(error) => {
            eprintln!("{}", error);
            drop(script_file_in_memory);
            drop(script_file);
            process::exit(1);
        },
        Ok(interpreter) => interpreter,
    };

    if let Err(error) = interpreter.execute() {
        eprintln!("{}", error);
        drop(interpreter);
        drop(script_file_in_memory);
        drop(script_file);
        process::exit(1);
    }
}
