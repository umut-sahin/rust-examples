use std::{
    io::{
        self,
        Write,
    },
    process,
};

fn main() {
    println!("- Could you enter an integer, please?");
    print!("+ ");

    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let entered_integer = input.trim().parse::<i32>().unwrap_or_else(|_| {
        eprintln!("- You did not enter an integer. That was not nice.");
        drop(input);
        process::exit(1);
    });

    println!(
        "- You entered {}. Thank you for your cooperation.",
        entered_integer,
    );
}
