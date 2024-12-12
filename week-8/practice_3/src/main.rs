use std::fs;

fn main() {
    fs::remove_file("welcome_message.txt").expect("could not remove file");
    println!("file is removed");
}
