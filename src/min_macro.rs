
/// Returns the minimum of all args provided
#[macro_export]
macro_rules! min {
	($x:expr) => {$x};
	($x:expr, $($xs: expr),+ $(,)?) => {{
		let a = $x;
		let b = min!( $($xs),+ );
		if a < b {a} else {b}
	}}
}