use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io;
use std::process::{Command, Stdio};
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

fn log_activity(project_name: &str, note: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("activity_log.log")
        .expect("Cannot open log file");

        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

        writeln!(file, "[{}] Started: {} | Note: {}", timestamp, project_name, note)
            .expect("Cannot write to log");
}


#[derive(Serialize, Deserialize, Debug)]
struct Project {
    name: String,
    path: String,
    launch_commands: String,
    last_notes: String,
}


impl Project {
    fn get_notes_list(&self) -> Vec<String> {
        self.last_notes
            .split(", ")
            .filter(|s: &&str| !s.is_empty()) // removes empty strings if they exist
            .map(|s: &str| s.to_string()) // converts &s to owned string
            .collect()
    }

    fn get_commands_list(&self) -> Vec<String> {
        self.launch_commands
            .split(", ")
            .filter(|s: &&str| !s.is_empty()) // removes empty strings if they exist
            .map(|s: &str| format!("({})", s.to_string()))
            //.map(|s: &str| f"({s.to_string()})")
            //.map(|s: &str| (s.to_string())) // converts &s to owned string
            .collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    projects: Vec<Project>,
}

fn wake_up_project(project: &Project, commands: &Vec<String>) {
    println!("🚀 Waking up: {}", project.name);
    println!("📝 Last Note: \"{}\"", project.last_notes);
    println!("---------------------------------------");

    let log_filename = format!("{}.log", project.name.to_lowercase().replace(" ", "_"));

    //let log_file = fs::File::create(&log_filename)
    //    .expect("Failed to create log file");

    let log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // Clears the log on every launch
        .open(&log_filename)
        .expect("Failed to open log.");
    
    let error_file = log_file.try_clone().expect("Failed to clone log file handle");

    println!("Streaming logs to {}", log_filename);

    let combined_command = format!("echo '--- Starting in: ' $(pwd) && {}", commands.join(" & "));

    log_activity(&project.name, &combined_command);

    // Standard macOS shell is zsh
    let spawn_outcome = Command::new("zsh")
        .arg("-c")
        .arg(&combined_command)
        .current_dir(Path::new(&project.path))
        .stdout(Stdio::from(log_file)) // Redirect standard output
        .stderr(Stdio::from(error_file)) // Redirect errors
        .spawn();
        //.expect("Failed to launch project");

    log_activity(&project.name, &project.last_notes);

    match spawn_outcome {
        Ok(child) => println!("✅ Process started with PID: {}", child.id()),
        Err(e) => eprintln!("❌ Failed to launch: {}", e),
    }
}



fn main() {
    // 1. Load the config (Simplified for now: assumes projects.json is in current folder)
    let data = fs::read_to_string("src/config/projects.json")
        .expect("Unable to read projects.json. Does it exist?");
    
    let config: Config = serde_json::from_str(&data)
        .expect("JSON was not well-formatted");

    for (i, project) in config.projects.iter().enumerate() {
        let notes = project.get_notes_list();
        println!("{}. {}\nnotes:\n", i+1, project.name);
        for (j, note) in notes.iter().enumerate() {
            println!("{}. {}\n", j+1, note);
        }
    }

    print!("\nSelect a project number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let choice: usize = input.trim().parse::<usize>()
        .map(|n| n.saturating_sub(1))
        .expect("Please enter a valid number");
    
        if let Some(selected) = config.projects.get(choice) {
            let commands = selected.get_commands_list();
            wake_up_project(selected, &commands);
        } else {
            println!("Invalid selection. Please restart.");
        }
}
