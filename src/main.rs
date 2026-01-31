use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let title = get_title()?;
    println!("{title}");

    let os = get_os()?;
    println!("OS: {os}");

    let kernel = get_kernel()?;
    println!("Kernel: {kernel}");

    let uptime = get_uptime()?;
    println!("Uptime: {uptime}");

    Ok(())
}

fn get_os() -> Result<String, io::Error> {
    let contents = fs::read_to_string("/etc/os-release")?;

    for line in contents.lines() {
        if let Some(val) = line.strip_prefix("PRETTY_NAME=") {
            return Ok(val.trim_matches('"').to_string());
        }
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "PRETTY NAME NOT FOUND",))
}

fn get_title() -> Result<String, io::Error> {
    let mut title = String::from("");

    let username = whoami::username()?;
    let hostname = fs::read_to_string("/etc/hostname")?;

    title.push_str(&username);
    title += "@";
    title.push_str(&hostname);
    Ok(title)
}

fn get_kernel() -> Result<String, io::Error> {
    let ostype = fs::read_to_string("/proc/sys/kernel/ostype")?;
    let osrelease = fs::read_to_string("/proc/sys/kernel/osrelease")?;

    Ok(format!("{} {}",
        ostype.trim(),
        osrelease.trim()
    ))
}

fn get_uptime() -> Result<String, io::Error> {
    let uptime_seconds = fs::read_to_string("/proc/uptime")?;
    let uptime_formatted = hrtime::from_sec(
        uptime_seconds
            .chars()
            .take_while(|&ch| ch != '.')
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    );

    Ok(uptime_formatted)
}
