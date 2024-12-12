use std::fs::File;
use std::io::Write;

fn main() {
    // Arrays representing the datasets
    let commissioners = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiyeye",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let geopolitical_zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Append datasets into one output
    let mut merged_output = String::new();

    for i in 0..commissioners.len() {
        let entry = format!(
            "{} | {} | {} | {}\n",
            i + 1,
            commissioners[i],
            ministries[i],
            geopolitical_zones[i]
        );
        merged_output.push_str(&entry);
    }

    // Write the appended output to a file
    let file_name = "commissioner_ministries_geozones.txt";
    let mut file = File::create(file_name).expect("Could not create file");
    file.write_all(merged_output.as_bytes()).expect("Could not write to file");

    println!("Data has been written to {}", file_name);
}

