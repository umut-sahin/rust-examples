use memmap::MmapMut;
use std::{
    fs::OpenOptions,
    process,
};

fn main() {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("memory-mapped-io/assets/hello-world.txt")
        .unwrap_or_else(|error| {
            eprintln!("io error: unable to open the input file ({})", error);
            process::exit(1);
        });

    let mut file_in_memory = unsafe {
        match MmapMut::map_mut(&file) {
            Err(error) => {
                eprintln!(
                    "memory mapping error: unable to map the input file into memory ({})",
                    error,
                );
                drop(file);
                process::exit(1);
            },
            Ok(file_in_memory) => file_in_memory,
        }
    };

    print!("Before: ");

    for byte in &mut file_in_memory[..] {
        let current_character = *byte as char;
        print!("{}", current_character);

        if current_character.is_ascii_uppercase() {
            *byte = current_character.to_ascii_lowercase() as u8;
        } else if current_character.is_ascii_lowercase() {
            *byte = current_character.to_ascii_uppercase() as u8;
        }
    }


    if let Err(error) = file_in_memory.flush() {
        eprintln!(
            "memory mapping error: unable to flush the mapped memory back into the input file ({})",
            error,
        );
        drop(file_in_memory);
        drop(file);
        process::exit(1);
    }

    print!("After: ");

    for byte in &file_in_memory[..] {
        print!("{}", *byte as char);
    }
}
