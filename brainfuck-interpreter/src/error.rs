use colored::*;
use std::{
    error::Error,
    fmt::{
        self,
        Display,
    },
    io,
};

#[derive(Debug)]
pub enum RuntimeError {
    Io(io::Error),
    CellUnderflow,
    CellOverflow,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RuntimeError::Io(ref error) => {
                write!(
                    f,
                    "{} unexpected io error occurred ({})",
                    "runtime error:".red().bold(),
                    error,
                )
            },
            RuntimeError::CellUnderflow => {
                write!(
                    f,
                    "{} attempted to access a negative cell",
                    "runtime error:".red().bold(),
                )
            },
            RuntimeError::CellOverflow => {
                write!(
                    f,
                    "{} attempted to access a cell, which is above the cell limit",
                    "runtime error:".red().bold(),
                )
            },
        }
    }
}

impl Error for RuntimeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RuntimeError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for RuntimeError {
    #[inline]
    fn from(err: io::Error) -> RuntimeError {
        RuntimeError::Io(err)
    }
}

#[cfg(test)]
mod runtime_error {
    mod traits {
        mod display {
            use crate::error::RuntimeError;
            use colored::*;
            use std::{
                error::Error,
                io,
            };

            #[test]
            fn io() {
                let error_with_not_found = RuntimeError::Io(io::ErrorKind::NotFound.into());
                assert_eq!(
                    format!("{}", error_with_not_found),
                    format!(
                        "{} unexpected io error occurred ({})",
                        "runtime error:".red().bold(),
                        error_with_not_found.source().unwrap(),
                    ),
                );

                let error_with_permission_denied =
                    RuntimeError::Io(io::ErrorKind::PermissionDenied.into());
                assert_eq!(
                    format!("{}", error_with_permission_denied),
                    format!(
                        "{} unexpected io error occurred ({})",
                        "runtime error:".red().bold(),
                        error_with_permission_denied.source().unwrap(),
                    ),
                );
            }

            #[test]
            fn cell_underflow() {
                let error = RuntimeError::CellUnderflow;
                assert_eq!(
                    format!("{}", error),
                    format!(
                        "{} attempted to access a negative cell",
                        "runtime error:".red().bold(),
                    ),
                );
            }

            #[test]
            fn cell_overflow() {
                let error = RuntimeError::CellOverflow;
                assert_eq!(
                    format!("{}", error),
                    format!(
                        "{} attempted to access a cell, which is above the cell limit",
                        "runtime error:".red().bold(),
                    ),
                );
            }
        }

        mod error {
            use crate::error::RuntimeError;
            use std::{
                error::Error,
                io,
            };

            #[test]
            fn source_on_io() {
                let error_with_not_found = RuntimeError::Io(io::ErrorKind::NotFound.into());
                assert!(error_with_not_found.source().is_some());
                assert_eq!(
                    format!("{}", error_with_not_found.source().unwrap()),
                    format!("{}", io::Error::from(io::ErrorKind::NotFound)),
                );

                let error_with_permission_denied =
                    RuntimeError::Io(io::ErrorKind::PermissionDenied.into());
                assert!(error_with_permission_denied.source().is_some());
                assert_eq!(
                    format!("{}", error_with_permission_denied.source().unwrap()),
                    format!("{}", io::Error::from(io::ErrorKind::PermissionDenied)),
                );
            }

            #[test]
            fn source_on_cell_underflow() {
                let error = RuntimeError::CellUnderflow;
                assert!(error.source().is_none());
            }

            #[test]
            fn source_on_cell_overflow() {
                let error = RuntimeError::CellOverflow;
                assert!(error.source().is_none());
            }
        }

        mod from {
            mod io_error {
                use crate::error::RuntimeError;
                use std::io;

                #[test]
                fn from() {
                    let error_from_not_found: RuntimeError =
                        io::Error::from(io::ErrorKind::NotFound).into();
                    match error_from_not_found {
                        RuntimeError::Io(ref error) => {
                            assert_eq!(error.kind(), io::ErrorKind::NotFound)
                        },
                        _ => unreachable!(),
                    }

                    let error_from_permission_denied: RuntimeError =
                        io::Error::from(io::ErrorKind::PermissionDenied).into();
                    match error_from_permission_denied {
                        RuntimeError::Io(ref error) => {
                            assert_eq!(error.kind(), io::ErrorKind::PermissionDenied)
                        },
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}


#[derive(Debug)]
pub enum SyntaxError {
    MissingOpeningBracket(usize, usize),
    MissingClosingBracket(usize, usize),
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SyntaxError::MissingOpeningBracket(line, column) => {
                write!(
                    f,
                    "{} unable to find the opening bracket of ']' at {}:{}",
                    "syntax error:".red().bold(),
                    line,
                    column,
                )
            },
            SyntaxError::MissingClosingBracket(line, column) => {
                write!(
                    f,
                    "{} unable to find the closing bracket of '[' at {}:{}",
                    "syntax error:".red().bold(),
                    line,
                    column,
                )
            },
        }
    }
}

impl Error for SyntaxError {}

#[cfg(test)]
mod syntax_error {
    mod traits {
        mod display {
            use crate::error::SyntaxError;
            use colored::*;

            #[test]
            fn missing_closing_bracket() {
                for line in 1..=10 {
                    for column in 1..=10 {
                        let error = SyntaxError::MissingClosingBracket(line, column);
                        assert_eq!(
                            format!("{}", error),
                            format!(
                                "{} unable to find the closing bracket of '[' at {}:{}",
                                "syntax error:".red().bold(),
                                line,
                                column,
                            ),
                        );
                    }
                }
            }

            #[test]
            fn missing_opening_bracket() {
                for line in 1..=10 {
                    for column in 1..=10 {
                        let error = SyntaxError::MissingOpeningBracket(line, column);
                        assert_eq!(
                            format!("{}", error),
                            format!(
                                "{} unable to find the opening bracket of ']' at {}:{}",
                                "syntax error:".red().bold(),
                                line,
                                column,
                            ),
                        );
                    }
                }
            }
        }
    }
}
