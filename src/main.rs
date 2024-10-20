use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!("{}", time);
}
