use std::io::{self, Write};

pub fn prompt(msg: &str) {
    print!("{msg}");
    let _ = io::stdout().flush();
}

pub fn read_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s.trim().to_string()
}

pub fn read_u8() -> u8 {
    read_line().parse::<u8>().unwrap_or(0)
}
