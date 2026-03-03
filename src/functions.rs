use std::fs::OpenOptions;
use std::io::Write;

pub fn greet_user() {
    let mut name = String::new();
    println!("Insert your username:");
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim();
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
    println!("Say to me, how are you right now? 1 to 10!");
    let mut mood = String::new();
    std::io::stdin()
        .read_line(&mut mood)
        .expect("Failed to read line");
    let mood = mood
        .trim()
        .parse::<u8>()
        .expect("Something get wrong with mood. Try again.");
    if mood >= 1 && mood <= 10 {
        let filled = "-".repeat(mood as usize);
        let empty = "".repeat((10 - mood) as usize);
        println!("[{}{}] {}", filled, empty, mood);
    }
}

fn user_option() {
    let mut option = String::new();
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
