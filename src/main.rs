use std::process::Command;

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

    println!("Username: {}", username);
    println!("Hostname: {}", hostname);
    println!("Uptime: {}", uptime);
    println!("RAM: {} MB | {} MB", used_mem_mb, total_mem_mb);
}

