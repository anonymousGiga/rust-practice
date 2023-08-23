use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;

fn get_cpu_frequency() -> Result<f64, Box<dyn Error>> {
    let file = File::open("/proc/cpuinfo")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if let Some(freq_str) = line.strip_prefix("cpu MHz\t\t: ") {
            let frequency: f64 = freq_str.parse()?;
            return Ok(frequency * 1e6);
        }
    }

    Err("Failed to get CPU frequency from /proc/cpuinfo".into())
}

fn main() {
    match get_cpu_frequency() {
        Ok(frequency) => println!("CPU frequency: {} Hz", frequency),
        Err(err) => eprintln!("Error: {}", err),
    }
}