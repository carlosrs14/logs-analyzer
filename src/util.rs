#[derive(Debug)]
pub struct DateTime {
    year: i16,
    month: i16,
    day: i16,
    hour: i16,
    mins: i16,
    secs: i16
}

impl DateTime {
    pub fn new(datetime_str: &str) -> Result<Self, String> {
        if !Self::validate_datetime(datetime_str) {
            return Err(String::from("Invalid datetime"));
        }
        
        let params: Vec<&str> = datetime_str
            .split(['-', ':', 'T', 'Z'])
            .collect();

        if params.len() < 6 {
            return Err(String::from("Invalid datetime format"));
        }

        let year = params[0].parse::<i16>().map_err(|e| e.to_string())?;
        let month = params[1].parse::<i16>().map_err(|e| e.to_string())?;
        let day = params[2].parse::<i16>().map_err(|e| e.to_string())?;
        let hour = params[3].parse::<i16>().map_err(|e| e.to_string())?;
        let mins = params[4].parse::<i16>().map_err(|e| e.to_string())?;
        let secs = params[5].parse::<i16>().map_err(|e| e.to_string())?;

        return Ok(Self { year, month, day, hour, mins, secs });
    }

    fn validate_datetime(datetime_str: &str) -> bool {
        if !datetime_str.contains('T') {
            return false;
        }

        let parts: Vec<&str> = datetime_str.split(['-', ':', 'T', 'Z']).collect();
        if parts.len() < 6 {
            return false;
        }

        let year = match parts[0].parse::<i16>() {
            Ok(y) if y > 0 => y,
            _ => return false,
        };
        let month = match parts[1].parse::<i16>() {
            Ok(m) if (1..=12).contains(&m) => m,
            _ => return false,
        };
        let day = match parts[2].parse::<i16>() {
            Ok(d) if (1..=31).contains(&d) => d,
            _ => return false,
        };
        match parts[3].parse::<i16>() {
            Ok(h) if (0..=23).contains(&h) => (),
            _ => return false,
        };
        match parts[4].parse::<i16>() {
            Ok(m) if (0..=59).contains(&m) => (),
            _ => return false,
        };
        match parts[5].parse::<i16>() {
            Ok(s) if (0..=59).contains(&s) => (),
            _ => return false,
        };

        let days_in_month = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
                if is_leap { 29 } else { 28 }
            }
            _ => return false,
        };

        day <= days_in_month
    }
}

#[derive(Debug)]
pub struct Ip {
    value: String
}

impl Ip {
    pub fn new(ip_str: &str) -> Result<Self, String> {
        if !Self::validate_ip(ip_str) {
            return Err(String::from("Invalid ip"));
        }
        return Ok(Self { value: String::from(ip_str) });
    }

    fn validate_ip(ip_str: &str) -> bool {
        ip_str.parse::<std::net::IpAddr>().is_ok()
    }
}