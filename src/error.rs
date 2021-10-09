use std::convert::{From, AsRef};
use std::error::Error;
use std::fmt;

use self::MazeErrorKind::*;

pub type MazeResult<T> = Result<T, MazeError>;

#[derive(Debug, Eq, PartialEq)]
pub struct MazeError {
    pub kind: MazeErrorKind,
    pub details: String,
}

impl MazeError {
    pub fn new<T: AsRef<str>>(kind: MazeErrorKind, details: T) -> MazeError {
        MazeError {
            kind: kind,
            details: String::from(details.as_ref()),
        }
    }
}

impl fmt::Display for MazeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		let kind_desc: &str = self.kind.as_ref();
		write!(fmt, "{}: {}", kind_desc, self.to_string())
    }
}

/*
impl From<io::Error> for MazeError {
    fn from(err: io::Error) -> MazeError {
        MazeError::new(MazeIoError(err), "Io Error")
    }
}*/

impl Error for MazeError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug, Eq, PartialEq)] // #[derive(Clone, Debug, PartialEq, Eq)]
pub enum MazeErrorKind {
    CoordOutOfRange,
	NoStart,
	NoEnd,
	Other
}

impl AsRef<str> for MazeErrorKind {
    fn as_ref(&self) -> &str {
        match self {
            CoordOutOfRange => "Coords out of range",
			NoStart => "No start key",
            _ => "Maze Error",
        }
    }
}