use std::env;
use std::fs::create_dir_all;
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{fs::File, io::Write};

use chrono::prelude::*;
use dotenv::dotenv;
use shellexpand::tilde;

fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    let start = Instant::now();
    let file_path_raw = format!(
        "{}/{}.txt",
        tilde(&env::var("LOGIN_TRACKER_PATH").expect("LOGIN_TRACKER_PATH")),
        get_time_and_date(false),
    )
    .to_lowercase();

    let file_path = Path::new(&file_path_raw);
    if let Some(parent) = file_path.parent() {
        create_dir_all(parent)?;
    }

    let to_sleep = Duration::from_secs(60 * 5);

    loop {
        let mut file = File::create(&file_path)?;
        let content_to_write = format!(
            "{}\n\n{}",
            get_time_and_date(true),
            format_duration(start.elapsed())
        );
        file.write_all(content_to_write.as_bytes())?;
        sleep(to_sleep)
    }
}

fn get_time_and_date(for_file_content: bool) -> String {
    let local_time = Local::now();
    let formatted_time = local_time
        .format(if !for_file_content {
            "%A-%Y_%H-%M"
        } else {
            "Time: %H:%M\nDay: %A\nYear: %Y"
        })
        .to_string();
    formatted_time
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let days = total_seconds / 86400; // 86400 seconds in a day
    let hours = (total_seconds % 86400) / 3600; // 3600 seconds in an hour
    let minutes = (total_seconds % 3600) / 60; // 60 seconds in a minute

    format!("Total elapsed time: {}d, {}h, {}m", days, hours, minutes)
}
