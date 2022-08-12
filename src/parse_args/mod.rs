use thiserror::Error;
use parse_dimensions::{parse_dimensions_string, ParseDimensionsError};
use std::str::FromStr;
use std::fs::File;
use std::io::Read;
use dimension_location::DimensionsLocation;
use crate::object::Object;

pub use subcommand::Subcommand;

pub mod parse_dimensions;
pub mod subcommand;
pub mod dimension_location;

/// Details why arguments could not be parsed
#[derive(Error, Debug)]
pub enum ParseArgsError {
	#[error("Please specify a subcommand")]
	NoSubcommand,
	#[error("Please specify a valid subcommand (either `shielding` or `wiring`)")]
	InvalidSubcommand,
	#[error("Zero objects have been specified")]
	NoObjects,
	#[error("--file expects file")]
	NoFileArg,
	#[error("Failed to read from file ({0})")]
	FileReadError(#[from] std::io::Error),
	#[error("Error parsing dimensions at {1}: {0}")]
	InvalidDimensions(ParseDimensionsError, DimensionsLocation)
}


/// Parses dimensions from file. Each dimension should be separated by a new line
pub fn parse_file_contents(objects_list: &mut Vec<Object>, file_string: &str) -> Result<(), ParseArgsError> {
	use ParseArgsError::*;
	for (lineno, line) in file_string.split("\n").enumerate() {
		let line = line.trim(); // remove whitespace
		if line == "" {
			continue;
		}
		
		let object = parse_dimensions_string(line)
			.map_err(|e| {
				InvalidDimensions(e, DimensionsLocation::File{line: lineno+1}) // record lineno when error occurs
			})?;
		objects_list.push(object);
	}
	Ok(())
} 

/// Parses arguments from the command line. Expects the format `[PROGRAMNAME, subcommand, (file), DIMENSIONS]`
pub fn parse_args(args: impl Iterator<Item=String>) -> Result<(Subcommand, Vec<Object>), ParseArgsError> {
	use ParseArgsError::*;
	
	let args: Vec<_> = args
		.skip(1) // skip first arg (will be executable file name)
		.map(|x| x.to_lowercase())
		.collect();
	
	if args.is_empty() {
		return Err(NoSubcommand);
	}
	
	let subcommand = match Subcommand::from_str(&args[0]) {
		Ok(s) => s,
		Err(_) => return Err(InvalidSubcommand)
	};
	
	if args.get(1).is_none() {
		return Err(NoObjects);
	}
	
	let mut objects_list = Vec::new();
	let mut current_cmd_arg; // for after the file loop, need to look at remaining args
	
	if args[1] == "--file" {
		if args.get(2).is_none () {
			return Err(NoFileArg);
		}
		
		// read from file
		let mut file =  File::open(&args[2])?;
		let mut contents = String::new();
		file.read_to_string(&mut contents)?;
		
		
		parse_file_contents(&mut objects_list, &contents)?;
		current_cmd_arg = 3; // 4th arg will be the next one
	}
	else {
		current_cmd_arg = 1; // 2nd arg will be the next one
	}
	

	// iterate until out of remaining args
	while let Some(dimensions_string) = args.get(current_cmd_arg) {
		let dimensions = parse_dimensions_string(dimensions_string)
			.map_err(|e| {
				InvalidDimensions(e, DimensionsLocation::Arg{pos: current_cmd_arg + 1}) // log arg position where error occured
			})?;
		
		// add each arg to dimensions list
		objects_list.push(dimensions);
		current_cmd_arg += 1;
	}
	
	// refuse to process no objects
	if objects_list.is_empty() {
		return Err(NoObjects);
	}
	
	
	
	Ok((subcommand, objects_list))
}

// ParseArgsError is compared only by discriminants (only used in testing)
impl PartialEq for ParseArgsError {
	fn eq(&self, other: &Self) -> bool {
		std::mem::discriminant(self) == std::mem::discriminant(other)
	}
}
impl Eq for ParseArgsError {}