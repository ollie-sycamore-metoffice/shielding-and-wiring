use std::env;
use parse_args::Subcommand;
use calculations::*;

mod parse_args;
mod min_macro;
mod calculations;


#[cfg(test)]
mod tests;

/// Prints the usage message, before exiting with status 1.
fn print_usage_and_exit() -> ! {
	println!("USAGE: shielding-and-wiring [shielding|wiring] (--file FILE) [width]x[height]x[depth]");
	std::process::exit(1);
}

fn main() {
	// parse inputted arguments
	let (subcommand, boxes) = match parse_args::parse_args(env::args()) {
		Ok(a) => a,
		Err(e) => {
			println!("Error: {e}");
			print_usage_and_exit();
		}
	};
	
	println!(
		"Calculating for {} box{}",
		boxes.len(),
		if boxes.len() == 1 {""} else {"es"}
	);
	
	// execute relevant subcommand
	match subcommand {
		Subcommand::Shielding => {
			println!("Needed shielding: {}mm^2", calculate_total_shielding(&boxes));
		}
		Subcommand::Wiring => {
			println!("Needed wiring: {}mm", calculate_total_wiring(&boxes));
		}
	}
}
