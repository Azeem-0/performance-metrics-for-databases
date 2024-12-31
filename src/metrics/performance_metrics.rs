use std::time::Instant;

use crate::utils::write_metrics_into_file::write_metrics_into_file;

pub fn performance_metrics(start_time: Instant, end_time: Instant, message: &str) {
    let duration = end_time.duration_since(start_time);

    let seconds = duration.as_secs();
    let milliseconds = duration.subsec_millis();
    let minutes = seconds / 60;
    let seconds = seconds % 60;

    let metrics_message = format!("{} {}m {}s {}ms\n", message, minutes, seconds, milliseconds);

    println!("{}", metrics_message);

    write_metrics_into_file(metrics_message);
}
