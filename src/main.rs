#[macro_use]
extern crate rocket;

use std::fs::OpenOptions;
use std::io::Write;

#[get("/")]
fn hello() -> &'static str {
    "Hello, World!"
}

#[get("/log")]
fn log_visit() -> &'static str {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs.txt")
        .unwrap();

    writeln!(file, "{} - Visitor", now).unwrap();
    "Log added successfully"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, log_visit])
}
