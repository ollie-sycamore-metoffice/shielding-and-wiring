use std::str::FromStr;

/// Subcommand: either `shielding` or `wiring`
#[derive(PartialEq, Eq, Debug)]
pub enum Subcommand {
	Shielding,
	Wiring
}

impl FromStr for Subcommand {
	type Err = ();

	fn from_str(string: &str) -> Result<Self, Self::Err> {
		match string {
			"shielding" => Ok(Self::Shielding),
			"wiring"    => Ok(Self::Wiring),
			_           => Err(())
		}
	}
}