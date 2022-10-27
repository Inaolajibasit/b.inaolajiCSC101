fn main() {
	let l:f64 = 520000000.0;
	let t:f64 = 5.0;
	let r:f64 = 10.0;

	// formula
	let a = l * ( 1.0 + (r / 100.0)) * t; 
	println!(" This is the Amount  {}", a);

	let ci = l + a;
	println!(" This is the Compound Intrest  {}", ci);
}