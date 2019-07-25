use std::fmt::{
    self,
    Display,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Instruction {
    MoveLeft(usize),
    MoveRight(usize),
    Increment(u8),
    Decrement(u8),
    Read,
    Write,
    StartLoop(usize),
    EndLoop(usize),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::MoveLeft(amount) => {
                for _ in 0..amount {
                    write!(f, "<")?;
                }
                Ok(())
            },
            Instruction::MoveRight(amount) => {
                for _ in 0..amount {
                    write!(f, ">")?;
                }
                Ok(())
            },
            Instruction::Increment(amount) => {
                for _ in 0..amount {
                    write!(f, "+")?;
                }
                Ok(())
            },
            Instruction::Decrement(amount) => {
                for _ in 0..amount {
                    write!(f, "-")?;
                }
                Ok(())
            },
            Instruction::Read => write!(f, ","),
            Instruction::Write => write!(f, "."),
            Instruction::StartLoop(_) => write!(f, "["),
            Instruction::EndLoop(_) => write!(f, "]"),
        }
    }
}

#[cfg(test)]
mod instruction {
    mod traits {
        mod display {
            use crate::instruction::Instruction;

            #[test]
            fn move_left_with_zero() {
                let instruction = Instruction::MoveLeft(0);
                assert_eq!(format!("{}", instruction), "");
            }

            #[test]
            fn move_left_with_one() {
                let instruction = Instruction::MoveLeft(1);
                assert_eq!(format!("{}", instruction), "<");
            }

            #[test]
            fn move_left_with_more_than_one() {
                let instruction = Instruction::MoveLeft(3);
                assert_eq!(format!("{}", instruction), "<<<");
            }

            #[test]
            fn move_right_with_zero() {
                let instruction = Instruction::MoveRight(0);
                assert_eq!(format!("{}", instruction), "");
            }

            #[test]
            fn move_right_with_one() {
                let instruction = Instruction::MoveRight(1);
                assert_eq!(format!("{}", instruction), ">");
            }

            #[test]
            fn move_right_with_more_than_one() {
                let instruction = Instruction::MoveRight(3);
                assert_eq!(format!("{}", instruction), ">>>");
            }

            #[test]
            fn increment_with_zero() {
                let instruction = Instruction::Increment(0);
                assert_eq!(format!("{}", instruction), "");
            }

            #[test]
            fn increment_with_one() {
                let instruction = Instruction::Increment(1);
                assert_eq!(format!("{}", instruction), "+");
            }

            #[test]
            fn increment_with_more_than_one() {
                let instruction = Instruction::Increment(3);
                assert_eq!(format!("{}", instruction), "+++");
            }

            #[test]
            fn decrement_with_zero() {
                let instruction = Instruction::Decrement(0);
                assert_eq!(format!("{}", instruction), "");
            }

            #[test]
            fn decrement_with_one() {
                let instruction = Instruction::Decrement(1);
                assert_eq!(format!("{}", instruction), "-");
            }

            #[test]
            fn decrement_with_more_than_one() {
                let instruction = Instruction::Decrement(3);
                assert_eq!(format!("{}", instruction), "---");
            }

            #[test]
            fn read() {
                let instruction = Instruction::Read;
                assert_eq!(format!("{}", instruction), ",");
            }

            #[test]
            fn write() {
                let instruction = Instruction::Write;
                assert_eq!(format!("{}", instruction), ".");
            }

            #[test]
            fn start_loop_with_zero() {
                let instruction = Instruction::StartLoop(0);
                assert_eq!(format!("{}", instruction), "[");
            }

            #[test]
            fn start_loop_with_one() {
                let instruction = Instruction::StartLoop(1);
                assert_eq!(format!("{}", instruction), "[");
            }

            #[test]
            fn start_loop_with_more_than_one() {
                let instruction = Instruction::StartLoop(3);
                assert_eq!(format!("{}", instruction), "[");
            }

            #[test]
            fn end_loop_with_zero() {
                let instruction = Instruction::EndLoop(0);
                assert_eq!(format!("{}", instruction), "]");
            }

            #[test]
            fn end_loop_with_one() {
                let instruction = Instruction::EndLoop(1);
                assert_eq!(format!("{}", instruction), "]");
            }

            #[test]
            fn end_loop_with_more_than_one() {
                let instruction = Instruction::EndLoop(3);
                assert_eq!(format!("{}", instruction), "]");
            }
        }
    }
}
