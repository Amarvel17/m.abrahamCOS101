use std::io;
fn main() {
    println!("Welcome to the requirement program for Ernst and Young Global Limited");

    let mut input =String::new();
    println!("How many candidates do you want to enter?:");
    io::stdin().read_line(&mut input).expect("failed to read input");
    let n: u32 = input.trim().parse().expect("Invalid input");

    let mut staff_vector: Vec<(String, f32)> = Vec::new();

    for x in 0..n {
        let mut input1 =String::new();
        let mut input2 =String::new();

         println!("Enter name:");
         io::stdin().read_line(&mut input1).expect("failed to read input");
         let name: String = input1.trim().parse().expect("Invalid input");

          println!("Enter years of experience:");
          io::stdin().read_line(&mut input2).expect("failed to read input");
          let years_of_experience: f32 = input2.trim().parse().expect("Invalid input");

          let staff: (String, f32) = (name, years_of_experience);
          staff_vector.push(staff);

          println!("Added staff {} successfully", x);
    }
    staff_vector.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let exp_staff = &staff_vector[0];

    println!("The most experienced staff is {} with {} years of experience", exp_staff.0, exp_staff.1 );
}
