use std::fs::OpenOptions;
use std::io::{self, Write};

pub fn greet_user(mut name: String) {
    println!("Insert your username:");
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!(
        "Hello, {}! How can I help you today?\n 1) Put your feelings into words.\n 2) Change the daily moodbar. 3) Exit.",
        name.trim()
    );
}

fn exit_app() {
    println!("Exiting...");
    std::process::exit(0);
}
fn get_user_venting() -> Result<(), Box<dyn std::error::Error>> {
    let mut entrada = String::new();
    println!("Insert your feelings:");
    std::io::stdin().read_line(&mut entrada)?;
    let entrada = entrada.trim();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("venting.txt")?;
    writeln!(file, "{entrada}")?;
    println!("Your feelings have been recorded.");
    Ok(())
}

fn change_moodbar() {
    println!("Changing the daily moodbar...");
}

pub fn user_option(mut option: String) {
    std::io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");
    match option.trim().parse::<u8>() {
        Ok(1) => {
            if let Err(e) = get_user_venting() {
                eprintln!("Failed to record feelings: {e}");
            }
        }
        Ok(2) => change_moodbar(),
        Ok(3) => exit_app(),
        _ => println!("Invalid option"),
    }
}
