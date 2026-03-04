pub fn run() {
    greet_user();

    loop {
        print_menu();
        crate::io::prompt("> ");
        match crate::io::read_u8() {
            1 => {
                if let Err(e) = crate::venting::record() {
                    eprintln!("Failed to record feelings: {e}");
                }
            }
            2 => crate::mood::set_today(),
            3 => crate::mood::show_history(),
            4 => crate::mood::show_average(),
            5 => break,
            _ => println!("Invalid option"),
        }

        println!();
    }

    println!("Exiting...");
}

fn greet_user() {
    println!("Insert your username:");
    crate::io::prompt("> ");
    let name = crate::io::read_line();

    println!("Hello, {}!", if name.is_empty() { "friend" } else { &name });
}

fn print_menu() {
    println!("What do you want to do?");
    println!(" 1) Put your feelings into words (append to venting.txt)");
    println!(" 2) Save today's mood (1..10) into moods.csv");
    println!(" 3) Show mood history");
    println!(" 4) Show mood average");
    println!(" 5) Exit");
}
