use std::io;

fn main() 
{
   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();
   let mut input4 = String::new();
   
    println!("Enter discriminant");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let discriminant:f32 = input4.trim().parse().expect("Not a valid number");
    
    println!("Enter a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let _a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let _b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let _c:f32 = input3.trim().parse().expect("Not a valid number");

    if discriminant > 0.0 {
       let _roots_str = "-b +/- discriminant*0.5)/ 2a";
       println!("roots a,b,c of a quadratic equation: {}", discriminant);
    }

}
