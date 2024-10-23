use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file_content(file_path: &str) {
    match File::open(file_path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            println!("File content:");

            for line in reader.lines() {
                match line {
                    Ok(content) => println!("{}", content),
                    Err(e) => eprintln!("Error reading line: {}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open file: '{}' - {}", file_path, e);
        }
    }
}

fn main() {
    match env::current_dir() {
        Ok(path) => println!("Current directory: {}. \
            Note that both absolute and relative paths are accepted for the file reader.", path.display()),
        Err(e) => eprintln!("Failed to get current directory: {}", e),
    }

    loop {
        println!("Please enter the file path:");

        let mut file_path = String::new();
        if let Err(e) = io::stdin().read_line(&mut file_path) {
            eprintln!("Something akward happened, could not read the line: {}", e);
        }        

        read_file_content(&file_path.trim());

        println!("Do you want to stop reading files? Type 'yes' to stop.");

        let mut response = String::new();
        if let Err(e) = io::stdin().read_line(&mut response) {
            eprintln!("Something akward happened, could not read the line: {}", e);
        }        

        if response.trim().eq_ignore_ascii_case("yes") {
            break;
        }
    }

    println!("Goodbye!");
}