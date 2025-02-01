use std::io;

fn main() {
    println!("Enter your role (administrator, project_manager, employee, customer, vendor):");

    let mut role = String::new();
    io::stdin()
        .read_line(&mut role)
        .expect("Failed to read input");

    let role = role.trim(); 

    match role {
        "administrator" => show_database_structure(),
        "project_manager" => show_project_table_structure(),
        "employee" => show_staff_table_structure(),
        "customer" => call_customer_table(),
        "vendor" => return_data_plan_table(),
        _ => println!("Invalid role. Please enter a valid role."),
    }
}

fn show_database_structure() {
    println!("Database Structure:");
    println!("- Projects Table\n- Staff Table\n- Customer Table\n- Dataplan Table");
}

fn show_project_table_structure() {
    println!("Project Table Structure:");
    println!("- PNo\n- Pname\n- Pduration\n- Project_Managerid");
}

fn show_staff_table_structure() {
    println!("Staff Table Structure:");
    println!("- Eid\n- Ename\n- DNo\n- Esal\n- Age\n- Phone");
}

fn call_customer_table() {
    println!("Customer Table");
    println!("");
}

fn return_data_plan() {
    println!("Returning the Dataplan Table...");
    println!("Displaying the details of the Dataplan Table...");
}

