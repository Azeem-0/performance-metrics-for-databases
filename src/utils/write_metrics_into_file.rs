use std::{fs::OpenOptions, io::Write};

pub fn write_metrics_into_file(metrics_message: String) {
    let file_path = "performance-metrics.txt";

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Failed to open the file for writing");

    if let Err(e) = file.write_all(metrics_message.as_bytes()) {
        eprintln!("Error writing to file: {}", e);
    }
}
