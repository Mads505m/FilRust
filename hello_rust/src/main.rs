use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};

const FILE_NAME: &str = "data.txt";

fn main() {
    loop {
        println!("Select an option:");
        println!("1. Create file and add text");
        println!("2. Read file content");
        println!("3. Add text to file");
        println!("4. Clear file content");
        println!("5. Delete file");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim().parse::<u32>() {
            Ok(1) => create_file(),
            Ok(2) => read_file(),
            Ok(3) => add_to_file(),
            Ok(4) => clear_file(),
            Ok(5) => remove_file(),
            Ok(6) => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}

fn create_file() {
    let mut file = File::create(FILE_NAME).expect("Couldn't create file");
    file.write_all(b"You just added text to the file! Well done ;)\n")
        .expect("Could not add text to the file");
    println!("The file '{}' has been created and the text has been added.", FILE_NAME);
}

fn read_file() {
    let message: String = fs::read_to_string(FILE_NAME).expect("Unable to read file");
    println!("The following text in {} is:\n {}",FILE_NAME, message);
}

fn add_to_file() {
    let mut file = OpenOptions::new().append(true).open(FILE_NAME)
        .expect("Could not open file");
    file.write_all(b"This is extra text!\n").expect("Could not add text to the file");
    println!("Text has been added to '{}'.", FILE_NAME);
}

fn clear_file() {
    File::create(FILE_NAME).expect("Could not clear file content");
    println!("The content of '{}' has been cleared.", FILE_NAME);
}

fn remove_file() {
    if fs::remove_file(FILE_NAME).is_ok() {
        println!("The file '{}' has been deleted.", FILE_NAME);
    } else {
        println!("The file '{}' does not exist or has already been deleted.", FILE_NAME);
    }
}
