use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    let my_pid = process::id();

    println!("System node initialized...");
    println!("Digital signature enabled(PID): {}", my_pid);
    println!("system status: monitoring for 10 seconds...");

    thread::sleep(Duration::from_secs(10));

    println!("System node {} completed monitoring. Shutting down...", my_pid);
}