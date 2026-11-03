use logs_analyzer::{log};


fn main() {
    let log =    log::Log::new("2026-11-02T10:00:00Z,192.168.1.10,GET,/users,200,45");

    println!("{:?}", log);
}
