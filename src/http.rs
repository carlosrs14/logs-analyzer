use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE
}


impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PATCH" => Ok(Self::PATCH),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            _ => Err(String::from("Invalid http method"))
        }
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum StatusCode {
    OK = 200,
    CREATED = 201,
    NO_CONTENT = 204,
    UNAUTHORIZED = 401,
    FORBIDDEN = 403,
    INTERNAL_SERVER_ERROR = 500
}

impl FromStr for StatusCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "200" => Ok(Self::OK),
            "201" => Ok(Self::CREATED),
            "204" => Ok(Self::NO_CONTENT),
            "401" => Ok(Self::UNAUTHORIZED),
            "403" => Ok(Self::FORBIDDEN),
            "500" => Ok(Self::INTERNAL_SERVER_ERROR),
            _ => Err(String::from("Invalid http status code"))
        }
    }
}