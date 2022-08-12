use crate::parse_args::parse_dimensions::{parse_dimensions_string, ParseDimensionsError::*};

#[test]
fn test_parse_dimensions() {
	assert_eq!(parse_dimensions_string("3x4x5"),       Ok((3.,4.,5.)));
	assert_eq!(parse_dimensions_string("3.2x4.5x2.5"), Ok((3.2,4.5,2.5)));
}

#[test]
fn test_0_dimensions() {
	assert_eq!(parse_dimensions_string(""), Err(ZeroDimensions))
}

#[test]
fn test_only_3_dimensions() {
	assert_eq!(parse_dimensions_string("3"),       Err(MustHave3Dimensions {provided: 1}));
	assert_eq!(parse_dimensions_string("3x2"),     Err(MustHave3Dimensions {provided: 2}));
	assert_eq!(parse_dimensions_string("3x3x3x3"), Err(MustHave3Dimensions {provided: 4}));
}

#[test]
fn test_invalid_float_dimension() {
	assert_eq!(parse_dimensions_string("INVALIDx3x4"), Err(InvalidFloat {dimension: 1}));
	assert_eq!(parse_dimensions_string("3x3xINVALID"), Err(InvalidFloat {dimension: 3}));
	assert_eq!(parse_dimensions_string("INVALIDx5"),   Err(InvalidFloat {dimension: 1}));
}

#[test]
fn test_negative_or_zero_dimensions() {
	assert_eq!(parse_dimensions_string("0x4x5"),  Err(NegativeOr0Dimension {dimension: 1}));
	assert_eq!(parse_dimensions_string("4x0x5"),  Err(NegativeOr0Dimension {dimension: 2}));
	assert_eq!(parse_dimensions_string("3x-1x2"), Err(NegativeOr0Dimension {dimension: 2}));
}