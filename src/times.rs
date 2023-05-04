use chrono::Local;

pub fn get_now_unix() -> i64 {
    let now: chrono::DateTime<Local> = Local::now();
    return now.timestamp_millis();
}

pub fn get_formed_date() -> String {
    let now: chrono::DateTime<Local> = Local::now();
    return String::from(now.format("%m-%d-%Y").to_string());
}