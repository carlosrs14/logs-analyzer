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
    pub fn new(log_str: &str) -> Result<Self, String> {
        let params: Vec<&str> = log_str.split([',', '\n']).collect();

        if params.len() < 6 {
            return Err(String::from("Invalid log line format"));
        }

        let datetime = util::DateTime::new(params[0])?;
        let ip = util::Ip::new(params[1])?;
        let method = http::Method::from_str(params[2])?;
        let path = String::from(params[3]);
        let status = http::StatusCode::from_str(params[4])?;
        let duration_ms = params[5].parse::<i32>().map_err(|e| e.to_string())?;

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