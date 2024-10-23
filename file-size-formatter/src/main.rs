use std::env;
use std::fmt;
use std::process;

enum Unit {
    Kilobyte(i64),
    Megabyte(i64),
    Gigabyte(i64),
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Unit::Kilobyte(v) => {
                let bytes = v as f64 * 1024.0;
                let kilobytes = v as f64;
                let megabytes = kilobytes / 1024.0;
                let gigabytes = megabytes / 1024.0;
                write!(f, "Sizes {{ bytes: \"{} bytes\", kilobytes: \"{} kilobytes\", megabytes: \"{} megabytes\", gigabytes: \"{} gigabytes\" }}",
                       bytes, kilobytes, megabytes, gigabytes)
            },
            Unit::Megabyte(v) => {
                let bytes = v as f64 * 1024.0 * 1024.0;
                let kilobytes = v as f64 * 1024.0;
                let megabytes = v as f64;
                let gigabytes = megabytes / 1024.0;
                write!(f, "Sizes {{ bytes: \"{} bytes\", kilobytes: \"{} kilobytes\", megabytes: \"{} megabytes\", gigabytes: \"{} gigabytes\" }}",
                       bytes, kilobytes, megabytes, gigabytes)
            },
            Unit::Gigabyte(v) => {
                let bytes = v as f64 * 1024.0 * 1024.0 * 1024.0;
                let kilobytes = v as f64 * 1024.0 * 1024.0;
                let megabytes = v as f64 * 1024.0;
                let gigabytes = v as f64;
                write!(f, "Sizes {{ bytes: \"{} bytes\", kilobytes: \"{} kilobytes\", megabytes: \"{} megabytes\", gigabytes: \"{} gigabytes\" }}",
                       bytes, kilobytes, megabytes, gigabytes)
            },
        }
    }
}

fn retrieve_unit(args: &Vec<String>) -> Option<Unit> {
    if args.len() != 3 {
        println!("3 Arguments expected, {} given.", args.len());
        return None;
    }

    let val_arg = &args[1];
    let value = match val_arg.parse::<i64>() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("First argument is not an integer.");
            return None;
        }
    };

    let unit_arg = &args[2];
    let unit_enum = match unit_arg.as_str() {
        "kb" => Unit::Kilobyte(value),
        "mb" => Unit::Megabyte(value),
        "gb" => Unit::Gigabyte(value),
        _ => {
            eprintln!("The second argument must be one of the following units: kb, mb, gb.");
            return None;
        }
    };

    Some(unit_enum)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match retrieve_unit(&args) {
        Some(unit) => println!("{}", unit),
        None => {
            println!("Invalid arguments provided.");
            process::exit(1);
        }
    }
}