use crate::parse_args::{parse_args, ParseArgsError::*, parse_file_contents, dimension_location::DimensionsLocation};

fn s(s: &'static [&str]) -> impl Iterator<Item = String> + 'static {
	s.into_iter().map(|x| String::from(*x)).into_iter()
}

#[test]
fn test_parse_args() {
	assert!(parse_args(s(&["", "shielding", "1x1x1"])).is_ok());
	assert!(parse_args(s(&["", "ShIeLdInG", "1X1x1"])).is_ok());
	assert!(parse_args(s(&["", "wiring", "1x1X1"])).is_ok());
	assert!(parse_args(s(&["", "WiRiNg", "1X1x1"])).is_ok());
}


#[test]
fn no_subcommand() {
	
	assert_eq!(parse_args(s(&[])),   Err(NoSubcommand));
	assert_eq!(parse_args(s(&[""])), Err(NoSubcommand));
}

#[test]
fn no_dimensions() {
	assert_eq!(parse_args(s(&["", "shielding"])),   Err(NoDimensions));
}

#[test]
fn file_string() {
	let mut contents = Vec::new();
	assert!(parse_file_contents(&mut contents,
		"10x2x3\n5x4x3\n2x3x4\n\t\n"
	).is_ok());
	
	assert_eq!(contents,
		[
			(10.,2.,3.),
			(5.,4.,3.),
			(2.,3.,4.)
		]
	);
}

#[test]
fn file_string_error() {
	let mut contents = Vec::new();
	let result = parse_file_contents(&mut contents,
		"10x2x3\n5x4x3\n2x3x4a\n\t\n"
	);
	assert!(result.is_err());
	match result.err().unwrap() {
		InvalidDimensions(_, location) => match location {
			DimensionsLocation::File{line} => assert_eq!(line, 3),
			_ => unreachable!()
		}
		_ => unreachable!()
	}
}