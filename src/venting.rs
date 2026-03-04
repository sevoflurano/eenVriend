use std::fs::OpenOptions;
use std::io::Write;

pub fn record() -> Result<(), Box<dyn std::error::Error>> {
    println!("Insert your feelings:");
    crate::io::prompt("> ");
    let entrada = crate::io::read_line();

    if entrada.is_empty() {
        println!("(empty) Nothing saved.");
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("venting.txt")?;

    writeln!(file, "{entrada}")?;
    println!("Your feelings have been recorded.");
    Ok(())
}
