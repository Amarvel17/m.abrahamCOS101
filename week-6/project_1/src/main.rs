use std::io;

fn equation(a:f32, b:f32, c:f32){
    let d = a/3.0 * (b + c);
    let e = 5.0 * b * a * c.powf(1.0);
    let f: f32 = a * b * c.powf(1.0);
    let g = 8.0 * b.powf(3.0) * a.powf(4.0) * c.powf(6.0);
    let h = 22.0/7.0 * b.powf(4.0) * a * c.powf(2.0);

    let formula = [("T", d), ("R",e), ("P", f), ("C", g), ("V", h)];


    println!("\nSelect an equation: [Choose either T, R, P, C or V]");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Inbalid input");
    let express = input1.trim().to_uppercase();

    let value = formula.iter().find(|&&(code, _)| code== express);
    if let Some(&(_,math)) = value{
        println!("Your answer is : {}", math);
        return;
    }
}



fn main() {
    println!("Hi!! this program is to solve for:
         T = Area of trapezium
         R = Area of rhombus
         P = Area of parallelogram
         C = Area of cube
         V = Volume of a cylinder");
    println!("\nYou are allowed to input only 3 variables : a,b,c");


    println!("\nEnter your dimension for a: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Invalid input");
    let a:f32 = input2.trim().parse().expect("Invalid input");

    println!("\nEnter your dimension for b: ");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Invalid input");
    let b:f32 = input3.trim().parse().expect("Invalid input");

    println!("\nEnter your dimension for c: ");
    let mut input4 = String::new();
    io::stdin().read_line(&mut input4).expect("Invalid input");
    let c:f32 = input4.trim().parse().expect("Invalid input");

    equation(a, b, c);
}
