use std::io;

fn main() {
    // Step 1: Define roles and job titles with their APS levels in a vector
    let roles = vec![
        ("Office Administrator", vec![
            ("Intern", "APS 1-2"),
            ("Administrator", "APS 3-5"),
            ("Senior Administrator", "APS 5-8"),
            ("Office Manager", "EL1 8-10"),
            ("Director", "EL2 10-13"),
            ("CEO", "SES"),
        ]),
        ("Academic", vec![
            ("Research Assistant", "APS 3-5"),
            ("PhD Candidate", "APS 5-8"),
            ("Post-Doc Researcher", "EL1 8-10"),
            ("Senior Lecturer", "EL2 10-13"),
            ("Dean", "SES"),
        ]),
        ("Lawyer", vec![
            ("Paralegal", "APS 1-2"),
            ("Junior Associate", "APS 3-5"),
            ("Associate", "APS 5-8"),
            ("Senior Associate 1-2", "EL1 8-10"),
            ("Senior Associate 3-4", "EL2 10-13"),
            ("Partner", "SES"),
        ]),
        ("Teacher", vec![
            ("Placement", "APS 1-2"),
            ("Classroom Teacher", "APS 3-5"),
            ("Snr Teacher", "APS 5-8"),
            ("Leading Teacher", "EL1 8-10"),
            ("Deputy Principal", "EL2 10-13"),
            ("Principal", "SES"),
        ]),
    ];

    // Step 2: Get inputs from the user
    let role = get_input("Enter your role (e.g., Lawyer, Teacher, Academic, Office Administrator): ");
    let job_title = get_input("Enter your job title: ");
    let years_of_experience: u32 = get_input("Enter your years of experience: ")
        .trim()
        .parse()
        .expect("Please enter a valid number for years of experience.");

    // Step 3: Find the base APS level for the role and job title
    let base_aps_level = get_base_aps_level(&roles, &role, &job_title);

    // Step 4: Adjust the APS level based on years of experience
    if let Some(level) = base_aps_level {
        let validated_level = adjust_level_based_on_experience(level, years_of_experience);
        println!(
            "Role: {}, Job Title: {}, Years of Experience: {}, APS Level: {}",
            role, job_title, years_of_experience, validated_level
        );
    } else {
        println!("Invalid role or job title: Role '{}', Job Title '{}'", role, job_title);
    }
}

// Function to get input from the user
fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

// Function to get the base APS level based on the role and job title
fn get_base_aps_level<'a>(
    roles: &'a Vec<(&str, Vec<(&str, &str)>)>,
    role: &str,
    job_title: &str,
) -> Option<&'a str> {
    for (r, titles) in roles {
        if r == &role {
            for (title, aps_level) in titles {
                if *title == job_title {
                    return Some(aps_level);
                }
            }
        }
    }
    None
}

// Function to adjust the APS level based on years of experience
fn adjust_level_based_on_experience(aps_level: &str, years_of_experience: u32) -> &str {
    match aps_level {
        "APS 1-2" if years_of_experience < 3 => "APS 1-2",
        "APS 3-5" if years_of_experience >= 3 && years_of_experience < 5 => "APS 3-5",
        "APS 5-8" if years_of_experience >= 5 && years_of_experience <= 8 => "APS 5-8",
        "EL1 8-10" if years_of_experience > 8 && years_of_experience <= 10 => "EL1 8-10",
        "EL2 10-13" if years_of_experience > 10 && years_of_experience <= 13 => "EL2 10-13",
        "SES" if years_of_experience > 13 => "SES",
        _ => aps_level, // Default to original APS level if experience doesn't match
    }
}
