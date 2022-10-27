fn main() {
	let p:f64 = 210000.0;
	let t:f64 = 10.0;
	let r:f64 = 3.0;

	// formula
	let a = p * ( 1 - (r / 100.0)).pow(t) ; 
	println!(" This is the Amount  {}", a);

}