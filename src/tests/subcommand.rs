use crate::parse_args::subcommand::Subcommand;
use std::str::FromStr;

#[test]
fn test_subcommand_parse() {
	assert_eq!(Subcommand::from_str("shielding"), Ok(Subcommand::Shielding));
	assert_eq!(Subcommand::from_str("wiring"), Ok(Subcommand::Wiring));
}

#[test]
fn test_invalid_subcommand() {
	assert!(Subcommand::from_str("").is_err());
	assert!(Subcommand::from_str("aaa").is_err());
}