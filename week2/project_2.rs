fn main() {
	let _toshiba:f64 = 2.0 * 450000.00;
	let _mac:f64 = 1.0 * 1500000.00;
	let _hp:f64 = 3.0 * 750000.00;
	let _dell:f64 = 3.0 * 2850000.00;
	let _acer:f64 = 1.0 * 250000.00;
	let _num = 2;

	let _power = _num ^ 2;
	println!("the power off all the products {} ",_power );

	let _sum = _toshiba + _mac + _hp + _dell + _acer;
	println!("the sum off all the products {} ",_sum );

	let _average = _sum / 5.0 ;
	println!("the AVERAGE off all the products {} ",_average );
}