use std::str::FromStr;

use crate::http;
use crate::util;

#[derive(Debug)]
pub struct Log {
    datetime: util::DateTime,
    ip: util::Ip,
    method: http::Method,
    path: String,
    status: http::StatusCode,
    duration_ms: i32
}

impl Log {
    pub fn new(log_str: &str) -> Self {
        let params: Vec<&str> = log_str.split(',').collect();

        let datetime = util::DateTime::new(params[0])
            .unwrap();
        let ip = util::Ip::new(params[1])
            .unwrap();
        let method = http::Method::from_str(params[2])
            .unwrap();
        let path = String::from(params[3]);
        let status = http::StatusCode::from_str(params[4])
            .unwrap();
        let duration_ms = params[5].parse::<i32>()
            .unwrap();

        Self { 
            datetime,
            ip,
            method,
            path,
            status, 
            duration_ms 
        }
    }
}