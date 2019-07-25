use crate::{
    error::{
        RuntimeError,
        SyntaxError,
    },
    instruction::Instruction,
};
use std::{
    convert::TryFrom,
    fmt::{
        self,
        Display,
    },
    io::{
        Read,
        Write,
    },
};

pub const MEMORY_SIZE: usize = 30_000;

#[cfg(test)]
mod constants {
    use crate::interpreter::MEMORY_SIZE;

    #[test]
    fn memory_size() {
        assert_eq!(MEMORY_SIZE, 30_000);
    }
}


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Interpreter {
    program: Vec<Instruction>,
}

impl Interpreter {
    #[inline]
    pub fn new() -> Interpreter {
        Interpreter::default()
    }
}

impl Interpreter {
    pub fn execute(&self) -> Result<Vec<u8>, RuntimeError> {
        let stdin = std::io::stdin();
        let mut stdin = stdin.lock().bytes();

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        let mut tape = vec![0u8; MEMORY_SIZE];
        let mut reading_head_location = 0usize;

        let mut current_instruction_index = 0usize;
        while current_instruction_index != self.program.len() {
            match &self.program[current_instruction_index] {
                Instruction::MoveLeft(amount) => {
                    if *amount > reading_head_location {
                        return Err(RuntimeError::CellUnderflow);
                    }
                    reading_head_location -= *amount;
                },
                Instruction::MoveRight(amount) => {
                    if *amount > MEMORY_SIZE - (reading_head_location + 1) {
                        return Err(RuntimeError::CellOverflow);
                    }
                    reading_head_location += *amount;
                },
                Instruction::Increment(amount) => {
                    tape[reading_head_location] = tape[reading_head_location].wrapping_add(*amount);
                },
                Instruction::Decrement(amount) => {
                    tape[reading_head_location] = tape[reading_head_location].wrapping_sub(*amount);
                },
                Instruction::Read => {
                    match stdin.next() {
                        None => tape[reading_head_location] = 0,
                        Some(maybe_read) => tape[reading_head_location] = maybe_read?,
                    }
                },
                Instruction::Write => {
                    stdout.write_all(&tape[reading_head_location..=reading_head_location])?;
                },
                Instruction::StartLoop(end_of_loop) => {
                    if tape[reading_head_location] == 0 {
                        current_instruction_index = *end_of_loop;
                    }
                },
                Instruction::EndLoop(start_of_loop) => {
                    if tape[reading_head_location] != 0 {
                        current_instruction_index = *start_of_loop;
                    }
                },
            }
            current_instruction_index += 1;
        }

        Ok(tape)
    }

    pub fn load(&mut self, script: &[u8]) -> Result<(), SyntaxError> {
        let mut current_line = 1;
        let mut current_column = 1;

        let mut loop_balancer = Vec::new();

        let mut new_program = Vec::with_capacity(script.len());
        for token in script {
            match token {
                b'<' => {
                    match new_program.last_mut() {
                        Some(Instruction::MoveLeft(amount)) => {
                            *amount += 1;
                        },
                        Some(Instruction::MoveRight(amount)) => {
                            *amount -= 1;
                            if *amount == 0 {
                                new_program.pop();
                            }
                        },
                        _ => {
                            new_program.push(Instruction::MoveLeft(1));
                        },
                    };
                },
                b'>' => {
                    match new_program.last_mut() {
                        Some(Instruction::MoveRight(amount)) => {
                            *amount += 1;
                        },
                        Some(Instruction::MoveLeft(amount)) => {
                            *amount -= 1;
                            if *amount == 0 {
                                new_program.pop();
                            }
                        },
                        _ => {
                            new_program.push(Instruction::MoveRight(1));
                        },
                    };
                },
                b'+' => {
                    match new_program.last_mut() {
                        Some(Instruction::Increment(amount)) => {
                            *amount = amount.wrapping_add(1);
                            if *amount == 0 {
                                new_program.pop();
                            }
                        },
                        Some(Instruction::Decrement(amount)) => {
                            *amount -= 1;
                            if *amount == 0 {
                                new_program.pop();
                            }
                        },
                        _ => {
                            new_program.push(Instruction::Increment(1));
                        },
                    };
                },
                b'-' => {
                    match new_program.last_mut() {
                        Some(Instruction::Decrement(amount)) => {
                            *amount = amount.wrapping_add(1);
                            if *amount == 0 {
                                new_program.pop();
                            }
                        },
                        Some(Instruction::Increment(amount)) => {
                            *amount -= 1;
                            if *amount == 0 {
                                new_program.pop();
                            }
                        },
                        _ => {
                            new_program.push(Instruction::Decrement(1));
                        },
                    };
                },
                b',' => {
                    new_program.push(Instruction::Read);
                },
                b'.' => {
                    new_program.push(Instruction::Write);
                },
                b'[' => {
                    loop_balancer.push((new_program.len(), current_line, current_column));
                    new_program.push(Instruction::StartLoop(0));
                },
                b']' => {
                    let matching_start_loop_instruction_index =
                        if let Some((index, _, _)) = loop_balancer.pop() {
                            index
                        } else {
                            return Err(SyntaxError::MissingOpeningBracket(
                                current_line,
                                current_column,
                            ));
                        };

                    let current_instruction_index = new_program.len();
                    match &mut new_program[matching_start_loop_instruction_index] {
                        Instruction::StartLoop(matching_end_loop_instruction_index) => {
                            *matching_end_loop_instruction_index = current_instruction_index;
                        },
                        _ => unreachable!(),
                    }

                    new_program.push(Instruction::EndLoop(matching_start_loop_instruction_index));
                },
                b'\n' => {
                    current_line += 1;
                    current_column = 0;
                },
                _ => {},
            }
            current_column += 1;
        }

        if !loop_balancer.is_empty() {
            let (_, line, column) = loop_balancer.pop().unwrap();
            return Err(SyntaxError::MissingClosingBracket(line, column));
        }

        new_program.shrink_to_fit();
        self.program = new_program;

        Ok(())
    }
}

impl Default for Interpreter {
    #[inline]
    fn default() -> Interpreter {
        Interpreter {
            program: Vec::new(),
        }
    }
}

impl Display for Interpreter {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for instruction in &self.program {
            write!(f, "{}", instruction)?;
        }
        Ok(())
    }
}

impl TryFrom<&[u8]> for Interpreter {
    type Error = SyntaxError;

    #[inline]
    fn try_from(script: &[u8]) -> Result<Interpreter, SyntaxError> {
        let mut interpreter = Interpreter::new();
        interpreter.load(script)?;
        Ok(interpreter)
    }
}

#[cfg(test)]
mod interpreter {
    mod associated_functions {
        mod new {
            use crate::interpreter::Interpreter;

            #[test]
            fn new() {
                let interpreter = Interpreter::new();
                assert_eq!(interpreter, Interpreter::default());
            }
        }
    }

    mod traits {
        mod default {
            use crate::interpreter::Interpreter;

            #[test]
            fn default() {
                let interpreter = Interpreter::default();
                assert_eq!(
                    interpreter,
                    Interpreter {
                        program: Vec::new()
                    },
                );
            }
        }
    }
}
