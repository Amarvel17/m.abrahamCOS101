fn main() {
	let t: f64 = 450000.00;
	let m: f64 = 1500000.00;
	let h: f64 = 750000.00;
	let d: f64 = 2850000.00;
	let a: f64 = 250000.00;

	let tq: f64 = 2.0;
	let mq: f64 = 1.0;
	let hq: f64 = 3.0;
	let dq: f64 = 3.0;
	let aq: f64 = 1.0;

	let avg = ((t * tq) + (m * mq) + (h * hq) + (d * dq) + (a * aq))/(tq + mq + hq + dq + aq); // average

	println!("The average of the total amount of sales is {}", avg);
}