use std::time::Instant;

pub fn performance_metrics(start_time: Instant, end_time: Instant, message: &str) {
    let duration = end_time.duration_since(start_time);

    let seconds = duration.as_secs();
    let milliseconds = duration.subsec_millis();
    let minutes = seconds / 60;
    let seconds = seconds % 60;

    println!("{} {}m {}s {}ms", message, minutes, seconds, milliseconds);
    println!();
}
