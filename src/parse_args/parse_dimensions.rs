use thiserror::Error;
use crate::object::Object;

/// Contains information on why a dimensions string could not be parsed
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseDimensionsError {
	#[error("Cannot have 0 dimensions!")]
	ZeroDimensions,
	#[error("Too many/few dimensions - must be either a cylinder or box")]
	InvalidObject,
	#[error("Dimension {dimension} is not a valid number")]
	InvalidFloat { dimension: usize },
	#[error("Dimension {dimension} is less than or equal to 0")]
	NegativeOr0Dimension { dimension: usize }
}

/// Parses a string containg dimensions in the format `WxLxH` where `W`,`H` and `L` are floats
pub fn parse_dimensions_string(string: &str) -> Result<Object, ParseDimensionsError> {
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
	
	let object = Object::from_values(&dimensions)
		.map_err(|_| InvalidObject)?;
	
	
	return Ok(object);
}