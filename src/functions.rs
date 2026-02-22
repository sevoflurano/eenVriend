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

fn exit() {
    println!("Exiting...");
    std::process::exit(0);
}
fn get_user_venting(mut venting: String) {
    println!("Insert your feelings:");
    std::io::stdin()
        .read_line(&mut venting)
        .expect("Failed to read line");
    println!("Thank you for sharing your feelings!");
}

fn change_moodbar(mood: String, bar: String) {
    println!("Changing the daily moodbar...");
}

pub fn user_option(mut option: String) {
    std::io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");
    match option.trim().parse::<u8>() {
        Ok(1) => get_user_venting(String::new()),
        Ok(2) => change_moodbar(String::new(), String::new()),
        Ok(3) => exit(),
        _ => println!("Invalid option"),
    }
}
