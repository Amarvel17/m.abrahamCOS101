fn main () {
	let p: f64 = 520_000_000.00;
	let r: f64 = 10.00;
	let n: f64 = 5.0;

	let a = p * (1.0 + (r/100.0)).powf(n) ; //Formula for Amount
	println!("Amount is {}", a);

	let i = a - p; // to get compound interest
	println!("The compound interest is {}", i);
}