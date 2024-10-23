fn main() {
	let p: f64 = 210000.00;
	let r: f64 = 5.0;
	let t: f64 = 3.0;

	let a: f64 = p * (1.0 - (r/100.0)).powf(t);



	println!("The value of the television after {} years is {}",t,a);
}