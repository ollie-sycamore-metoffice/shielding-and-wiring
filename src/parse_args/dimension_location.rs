use std::fmt::{Display, Formatter, self};

/// Stores where a dimension is located (i.e. it's position in file or cmd line arguments)
#[derive(Debug)]
pub enum DimensionsLocation {
	File {line: usize},
	Arg {pos: usize}
}

impl Display for DimensionsLocation {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::File{ line } => write!(f, "line {}", line),
			Self::Arg{ pos }   => write!(f, "arg pos {}", pos)
		}
	}
}