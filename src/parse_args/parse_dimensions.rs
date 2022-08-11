use thiserror::Error;

/// Contains information on why a dimensions string could not be parsed
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseDimensionsError {
	#[error("Cannot have 0 dimensions!")]
	ZeroDimensions,
	#[error("Must have exactly 3 dimensions (not {{provided}})")]
	MustHave3Dimensions { provided: usize },
	#[error("Dimension {dimension} is not a valid number")]
	InvalidFloat { dimension: usize },
	#[error("Dimension {dimension} is less than or equal to 0")]
	NegativeOr0Dimension { dimension: usize }
}

/// Parses a string containg dimensions in the format `WxLxH` where `W`,`H` and `L` are floats
pub fn parse_dimensions_string(string: &str) -> Result<(f64,f64,f64), ParseDimensionsError> {
	use ParseDimensionsError::*;
	
	if string.is_empty() {
		return Err(ZeroDimensions);
	}
	
	let mut dimensions = Vec::new();
	for (i, segment) in string.split("x").into_iter().enumerate() {
		let value: f64 = segment.parse()
			.map_err(|_| InvalidFloat{dimension: i+1})?;
		if value <= 0. {
			return Err(NegativeOr0Dimension {dimension: i+1});
		}
		
		dimensions.push(value);
	}
	
	
	if dimensions.len() != 3 {
		return Err(MustHave3Dimensions {provided: dimensions.len()});
	}
	
	return Ok((dimensions[0], dimensions[1], dimensions[2]));
}