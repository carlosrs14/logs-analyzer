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
        let secs = params[5][..params[5].len()-1].parse::<i16>().map_err(|e| e.to_string())?;

        return Ok(Self { year, month, day, hour, mins, secs });
    }

    fn validate_datetime(datetime_str: &str) -> bool {
        return  true;
    }
}

#[derive(Debug)]
pub struct Ip {
    value: String
}

impl Ip {
    pub fn new(ip_str: &str) -> Result<Self, String> {
        if !Self::validate_ip(&ip_str) {
            return Err(String::from("Invalid ip"));
        }
        return Ok(Self { value: String::from(ip_str) });
    }

    fn validate_ip(ip_str: &str) -> bool {
        return true;
    }
}