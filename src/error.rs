use std::fmt;

/// Represents Errors that can occur during parsing.
///
/// The name of each corresponds to the type of error.
/// This enum implements the display trait, thus there is
/// nice outputs when printing:
///
/// ```rust
/// extern crate qasm;
///
/// println!("Got an error: {}", qasm::Error::UnsupportedVersion);
/// // "Got an error: Unsupported Version. Please Use OpenQASM Version 2.0"
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Error {
    MissingSemicolon,
    UnsupportedVersion,
    SourceError,
    MissingReal,
    MissingInt,
    MissingIdentifier,
    MissingVersion,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::MissingSemicolon => write!(f, "Missing Semicolon"),
            Error::UnsupportedVersion => {
                write!(f, "Unsupported Version. Please Use OpenQASM Version 2.0")
            }
            Error::SourceError => write!(f, "There Was An Error In Your Source Code"),
            Error::MissingReal => write!(f, "Missing A Real Number"),
            Error::MissingInt => write!(f, "Missing An Integer"),
            Error::MissingIdentifier => write!(f, "Missing An Identifier"),
            Error::MissingVersion => {
                write!(f, "Missing A Version Statement At The Start Of The File")
            }
        }
    }
}
