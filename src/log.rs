use std::str::FromStr;

use crate::http;
use crate::util;

#[derive(Debug)]
pub struct Log {
    pub datetime: util::DateTime,
    pub ip: util::Ip,
    pub method: http::Method,
    pub path: String,
    pub status: http::StatusCode,
    pub duration_ms: i32
}

impl Log {
    pub fn new(log_str: &str) -> Result<Self, String> {
        let trimmed = log_str.trim();
        if trimmed.is_empty() {
            return Err(String::from("Empty log line"));
        }

        let params: Vec<&str> = trimmed.split(',').collect();

        if params.len() != 6 {
            return Err(String::from("Invalid log line format: expected 6 comma-separated fields"));
        }

        let datetime = util::DateTime::new(params[0].trim())?;
        let ip = util::Ip::new(params[1].trim())?;
        let method = http::Method::from_str(params[2].trim())?;
        let path = String::from(params[3].trim());
        let status = http::StatusCode::from_str(params[4].trim())?;
        let duration_ms = params[5].trim().parse::<i32>().map_err(|e| e.to_string())?;

        Ok(Self { 
            datetime,
            ip,
            method,
            path,
            status, 
            duration_ms 
        })
    }
}