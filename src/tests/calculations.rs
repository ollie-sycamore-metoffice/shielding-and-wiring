use crate::calculations::*;

#[test]
fn shielding_calc() {
	assert_eq!(calculate_shielding(&(2.,3.,4.)), 58.);
	assert_eq!(calculate_shielding(&(1.,10.,1.)), 43.);
	assert_eq!(calculate_shielding(&(5.,7.,4.)), 186.);
}

#[test]
fn wiring_calc() {
	assert_eq!(calculate_wiring(&(2.,3.,4.)),    34.);
	assert_eq!(calculate_wiring(&(1.,10.,1.)),   14.);
	assert_eq!(calculate_wiring(&(5.,7.,4.)), 158.);
}

#[test]
fn total_shielding_calc() {
	assert_eq!(calculate_total_shielding(&[(2.,3.,4.), (1.,10.,1.)]), 101.);
	assert_eq!(calculate_total_shielding(&[(5.,7.,4.), (1.,10.,1.)]), 229.);
}

#[test]
fn total_wiring_calc() {
	assert_eq!(calculate_total_wiring(&[(2.,3.,4.), (1.,10.,1.)]), 48.);
	assert_eq!(calculate_total_wiring(&[(5.,7.,4.), (1.,10.,1.)]), 172.);
}