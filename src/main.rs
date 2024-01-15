use std::io::{self, BufRead};
use std::thread;
use std::process::Command;
use std::sync::{Arc, Mutex};

fn main() {
    let stdin = io::stdin();
    let cmd = std::env::args().nth(1).expect("no command given");

    // Create thread handle vector
    let mut handles = Vec::new();
    // Create command output vector
    let outputs = Arc::new(Mutex::new(Vec::new()));

    // Spawn each command in its own thread
    for (idx, line) in stdin.lock().lines().enumerate() {
        let outputs = outputs.clone();
        // Replace {} in each line with given command
        let line = line.unwrap();
        let cmd = cmd.replace("{}", &line);

        let handler = thread::spawn(move || {
            let output = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(["/C", &cmd])
                    .output()
                    .expect("failed to execute process")
            } else {
                Command::new("bash")
                    .arg("-c")
                    .arg(&cmd)
                    .output()
                    .expect("failed to execute process")
            };

            // Push command output to thread
            outputs.lock().unwrap().push((idx, output));
        });
        handles.push(handler);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Sort output using indices
    let mut outputs = outputs.lock().unwrap();
    outputs.sort_by(|a, b| a.0.cmp(&b.0));

    // Display output
    for output in outputs.iter() {
        let output = String::from_utf8_lossy(&output.1.stdout);
        print!("{}", output);
    }
}
