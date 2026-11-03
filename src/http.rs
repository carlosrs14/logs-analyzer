#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE
}

#[derive(Debug)]
pub enum StatusCode {
    OK = 200,
    CREATED = 201,
    NOCONTENT = 204,
    UNAUTHORIZED = 401,
    FORBIDDEN = 403
}