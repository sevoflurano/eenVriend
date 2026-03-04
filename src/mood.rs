use std::fs::{self, OpenOptions};
use std::io::Write;

const MOODS_FILE: &str = "moods.csv";

#[derive(Clone)]
struct Entry {
    date: String,
    mood: u8,
}

pub fn set_today() {
    let today = today_yyyy_mm_dd();

    println!("Mood for {today} (1..10):");
    crate::io::prompt("> ");
    let mood = crate::io::read_u8();

    if !(1..=10).contains(&mood) {
        println!("Invalid mood. Use 1..10.");
        return;
    }

    let mut entries = load_entries();

    if let Some(e) = entries.iter_mut().find(|e| e.date == today) {
        e.mood = mood;
        println!("Updated: {today} -> {mood}");
    } else {
        entries.push(Entry {
            date: today.clone(),
            mood,
        });
        println!("Saved: {today} -> {mood}");
    }

    entries.sort_by(|a, b| a.date.cmp(&b.date));

    if let Err(e) = save_entries(&entries) {
        eprintln!("Error saving moods.csv: {e}");
    }
}

pub fn show_history() {
    let entries = load_entries();
    if entries.is_empty() {
        println!("No mood history yet. Save today's mood first.");
        return;
    }

    println!("Mood history:");
    for e in entries {
        println!(" {}  {}", e.date, mood_bar(e.mood));
    }
}

pub fn show_average() {
    let entries = load_entries();
    if entries.is_empty() {
        println!("No data to calculate average.");
        return;
    }

    let sum: u32 = entries.iter().map(|e| e.mood as u32).sum();
    let avg = sum as f32 / entries.len() as f32;
    println!("Average ({} days): {:.2}", entries.len(), avg);
}

fn load_entries() -> Vec<Entry> {
    let content = match fs::read_to_string(MOODS_FILE) {
        Ok(s) => s,
        Err(_) => return vec![],
    };

    content.lines().filter_map(parse_line).collect()
}

fn save_entries(entries: &[Entry]) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(MOODS_FILE)?;

    for e in entries {
        writeln!(file, "{},{}", e.date, e.mood)?;
    }
    Ok(())
}

fn parse_line(line: &str) -> Option<Entry> {
    let mut parts = line.split(',');
    let date = parts.next()?.trim().to_string();
    let mood = parts.next()?.trim().parse::<u8>().ok()?;
    if !(1..=10).contains(&mood) {
        return None;
    }
    Some(Entry { date, mood })
}

fn mood_bar(mood: u8) -> String {
    let filled = "█".repeat(mood as usize);
    let empty = "░".repeat((10 - mood) as usize);
    format!("[{}{}] {}", filled, empty, mood)
}

// try to get the data from the system

fn today_yyyy_mm_dd() -> String {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let out = Command::new("powershell")
            .args(["-NoProfile", "-Command", "Get-Date -Format yyyy-MM-dd"])
            .output()
            .expect("Failed to get date (powershell)");
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    }

    #[cfg(not(target_os = "windows"))]
    {
        use std::process::Command;
        let out = Command::new("date")
            .arg("+%F")
            .output()
            .expect("Failed to get date (date)");
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    }
}
