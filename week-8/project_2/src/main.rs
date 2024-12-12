use std::fs::File;
use std::io::Write;

fn main() {
    // Define a struct to hold student details
    struct Student {
        name: String,
        matric_number: String,
        department: String,
        level: u32,
    
    }

    // Create a list of students
    let students = vec![
        Student {
            name: String::from("Oluchi Mordi"),
            matric_number: String::from("ACC10211111"),
            department: String::from("Accounting"),
            level: 300,
            
        },
        Student {
            name: String::from("Adams Aliyu"),
            matric_number: String::from("ECO10110101"),
            department: String::from("Economics"),
            level: 100,
        },
        Student {
            name: String::from("Shania Bolade"),
            matric_number: String::from("CSC10328828"),
            department: String::from("Computer"),
            level: 200,
        },
        Student {
            name: String::from("Adekunle Gold"),
            matric_number: String::from("EEE11020202"),
            department: String::from("Electrical"),
            level: 200,
        },
        Student {
            name: String::from("Blanca Edemoh"),
            matric_number: String::from("MEE10202001"),
            department: String::from("Mechanical"),
            level: 100,
        },
        
    ];

    // Display and save the details
    let mut file = File::create("student_details.txt").expect("Unable to create file");

    let header = "============================\n  PAU STUDENT INFORMATION   \n============================\n";
    println!("{}", header);
    file.write_all(header.as_bytes()).expect("Unable to write to file");

    for (index, student) in students.iter().enumerate() {
        let student_info = format!(
            "Student {}:\nName: {}\nMatric Number: {}\nDepartment: {}\nLevel:  {:.2}\n----------------------------\n",
            index + 1,
            student.name,
            student.matric_number,
            student.department,
            student.level,
        );

        println!("{}", student_info);
        file.write_all(student_info.as_bytes()).expect("Unable to write to file");
    }

    let footer = "End of student information.\n";
    println!("{}", footer);
    file.write_all(footer.as_bytes()).expect("Unable to write to file");
}
