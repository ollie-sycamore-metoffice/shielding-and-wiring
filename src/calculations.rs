use crate::min;

/// Calculates the total amount of shielding required for all provided boxes
pub fn calculate_total_shielding(boxes: &[(f64,f64,f64)]) -> f64 {
	let mut total = 0.;
	for dimensions in boxes {
		total += calculate_shielding(dimensions);
	}
	return total;
}

/// Calculates the total amount of wiring required for all provided boxes
pub fn calculate_total_wiring(boxes: &[(f64,f64,f64)]) -> f64 {
	let mut total = 0.;
	for dimensions in boxes {
		total += calculate_wiring(dimensions);
	}
	return total;
}

/// Calculates the amount of shielding required for a WxHxD size box
pub fn calculate_shielding((w,h,d): &(f64, f64, f64)) -> f64 {
	let a1 = w*h;
	let a2 = h*d;
	let a3 = w*d;
	
	let surface_area = (a1+a2+a3) * 2.;
	let extra = min!(a1,a2,a3);
	
	return extra + surface_area;
}

/// Calculates the amount of wiring required for a WxHxD size box
pub fn calculate_wiring((w,h,d): &(f64, f64, f64)) -> f64 {
	let volume = w*h*d;
	
	let mut sides = [w,h,d];
	sides.sort_by(|a,b| a.partial_cmp(b).unwrap());
	
	let shortest_circumference = (sides[0] + sides[1]) * 2.;
	
	return volume + shortest_circumference;
}
