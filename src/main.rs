use std::process::Command;
use std::env;

fn main() {
    // Get the current user's username
    let username_output = Command::new("whoami")
        .output()
        .expect("Failed to execute command");

    let username = String::from_utf8_lossy(&username_output.stdout).trim().to_string();

    // Get the system hostname
    let hostname_output = Command::new("hostname")
        .output()
        .expect("Failed to execute command");

    let hostname = String::from_utf8_lossy(&hostname_output.stdout).trim().to_string();
    // Get the system uptime
    let uptime_output = Command::new("cat")
        .arg("/proc/uptime")
        .output()
        .expect("Failed to execute command");

    let uptime_str = String::from_utf8_lossy(&uptime_output.stdout).trim().to_string();
    let uptime_secs = uptime_str.split('.').next().unwrap().parse::<u64>().unwrap();

    let uptime_minutes = (uptime_secs / 60) % 60;
    let uptime_hours = uptime_secs / 3600;

    let uptime = format!("{:02}:{:02}", uptime_hours, uptime_minutes);

    // Get the available RAM
    let meminfo_output = Command::new("cat")
        .arg("/proc/meminfo")
        .output()
        .expect("Failed to execute command");

    let meminfo_str = String::from_utf8_lossy(&meminfo_output.stdout);

    let total_mem_mb = meminfo_str
        .lines()
        .find(|line| line.starts_with("MemTotal:"))
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap() / 1024;

    let free_mem_mb = meminfo_str
        .lines()
        .find(|line| line.starts_with("MemAvailable:"))
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap() / 1024;

    let used_mem_mb = total_mem_mb - free_mem_mb;

    /* penguin ascii
 __  
( 0`-
/  \ 
|__| 
^  ^ 
    */

    println!("\x1b[91m  __    \x1b[0mUser: \x1b[93m{}\x1b[0m", username);
    println!("\x1b[91m ( 0`-  \x1b[0mHostname: \x1b[92m{}\x1b[0m", hostname);
    println!("\x1b[91m /  \\   \x1b[0mUptime: \x1b[96m{}\x1b[0m", uptime);
    println!("\x1b[91m |__|   \x1b[0mRAM: \x1b[94m{} MB | {} MB\x1b[0m", used_mem_mb, total_mem_mb);
    match env::var("SHELL") {
        Ok(val) => {
            let shell: String = val;
            println!("\x1b[91m ^  ^   \x1b[0mShell: \x1b[95m{}\x1b[0m", shell);
        }
        Err(_) => {
            println!("no");
        }
    }
    //println!("Shell: \x1b[95m{}\x1b[0m", shell);
    println!();
}

