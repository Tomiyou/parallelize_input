use std::io::{self, BufRead};
use std::process::{Command, Output, ExitStatus};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

enum ThreadEvent {
    Success(Output),
    Fail(Output),
    Finished,
}

fn execute_command(cmd: String, tx: Sender<ThreadEvent>) {
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

    if ExitStatus::success(&output.status) {
        tx.send(ThreadEvent::Success(output)).unwrap();
    } else {
        tx.send(ThreadEvent::Fail(output)).unwrap();
    }
}

fn print_outputs(rx: Receiver<ThreadEvent>) {
    loop {
        let event = rx.recv().unwrap();
        match event {
            ThreadEvent::Success(output) | ThreadEvent::Fail(output) => {
                let output = String::from_utf8_lossy(&output.stdout);
                print!("{}", output);
            },
            ThreadEvent::Finished => {
                return;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let cmd = std::env::args().nth(1).expect("no command given");

    // Create thread handle vector
    let mut handles = Vec::new();

    // Channel used to exchange parallel outputs
    let (tx, rx) = channel();

    // Spawn each command in its own thread
    for line in stdin.lock().lines() {
        // Replace {} in each line with given command
        let line = line.unwrap();
        let cmd = cmd.replace("{}", &line);
        let tx = tx.clone();

        // Launch each command in its own thread
        let handler = thread::spawn(move || execute_command(cmd, tx));
        handles.push(handler);
    }

    // Display output
    let display_thread = thread::spawn(move || print_outputs(rx));

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Wait for display thread
    tx.send(ThreadEvent::Finished).unwrap();
    display_thread.join().unwrap();
}
