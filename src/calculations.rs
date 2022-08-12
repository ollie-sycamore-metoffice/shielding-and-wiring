use crate::object::Object;

/// Calculates the total amount of shielding required for all provided boxes
pub fn calculate_total_shielding(objects: &[Object]) -> f64 {
	let mut total = 0.;
	for object in objects {
		total += calculate_shielding(object);
	}
	return total;
}

/// Calculates the total amount of wiring required for all provided boxes
pub fn calculate_total_wiring(objects: &[Object]) -> f64 {
	let mut total = 0.;
	for object in objects {
		total += calculate_wiring(object);
	}
	return total;
}

/// Calculates the amount of shielding required for an object
pub fn calculate_shielding(object: &Object) -> f64 {
	let surface_areas = object.surface_areas();
	
	let surface_area: f64 = surface_areas.iter().sum();
	let extra = surface_areas.iter().min_by(|a,b| a.partial_cmp(b).unwrap()).unwrap();
	
	return extra + surface_area;
}

/// Calculates the amount of wiring required for an object
pub fn calculate_wiring(object: &Object) -> f64 {
	let volume = object.volume();	
	let shortest_circumference = object.shortest_circumference();
	
	return volume + shortest_circumference;
}
