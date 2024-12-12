use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // Define the categories and drinks as a nested structure
    let drinks = vec![
        ("Lager", vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"]),
        ("Stout", vec!["Legend", "Turbo King", "Williams"]),
        (
            "Non-Alcoholic",
            vec!["Malta", "Amstel Malta", "Malta Gold", "Fayrouz"],
        ),
    ];

    // Create and open a file to write the output
    let mut file = File::create("nigerian_breweries_drinks.txt")?;

    // Write a header to the file
    file.write_all(b"Nigerian Breweries Drinks Categories\n")?;
    file.write_all(b"==================================\n")?;

    // Write the drinks data into the file
    for (category, items) in drinks {
        file.write_all(format!("\n{}:\n", category).as_bytes())?; // Category header
        for drink in items {
            file.write_all(format!("  - {}\n", drink).as_bytes())?; // List each drink
        }
    }

    println!("File 'nigerian_breweries_drinks.txt' created successfully.");
    Ok(())
}
