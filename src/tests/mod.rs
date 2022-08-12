mod parse_dimensions;
mod subcommand;
mod parse_args;
mod calculations;

#[test]
fn test_min_macro() {
	use crate::min;
	
	assert_eq!(min!(2,1,5,6,7), 1);
}



