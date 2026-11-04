use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    QUERY,
    HEAD,
    OPTIONS,
    TRACE,
    CONNECT
}


impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match &s.to_uppercase() as &str {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PATCH" => Ok(Self::PATCH),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "QUERY" => Ok(Self::QUERY),
            "HEAD" => Ok(Self::HEAD),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "CONNECT" => Ok(Self::CONNECT),
            _ => Err(String::from("Invalid http method"))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum StatusCode {
    CONTINUE = 100,
    SWITCHING_PROTOCOLS = 101,
    PROCESSING = 102,
    EARLY_HINTS = 103,
    OK = 200,
    CREATED = 201,
    ACCEPTED = 202,
    NON_AUTHORITATIVE_INFORMATION = 203,
    NO_CONTENT = 204,
    RESET_CONTENT = 205,
    PARTIAL_CONTENT = 206,
    MULTI_STATUS = 207,
    ALREADY_REPORTED = 208,
    IM_USED = 226,
    MULTIPLE_CHOICES = 300,
    MOVED_PERMANENTLY = 301,
    FOUND = 302,
    SEE_OTHER = 303,
    NOT_MODIFIED = 304,
    USE_PROXY = 305,
    UNUSED = 306,
    TEMPORARY_REDIRECT = 307,
    PERMANENT_REDIRECT = 308,
    BAD_REQUEST = 400,
    UNAUTHORIZED = 401,
    PAYMENT_REQUIRED = 402,
    FORBIDDEN = 403,
    NOT_FOUND = 404,
    METHOD_NOT_ALLOWED = 405,
    NOT_ACCEPTABLE = 406,
    PROXY_AUTHNTICATION_REQUIRED = 407,
    REQUEST_TIMEOUT = 408,
    CONFLICT = 409,
    GONE = 410,
    LENGTH_REQUIRED = 411,
    PRECONDITION_FAILED = 412,
    CONTENT_TOO_LARGE = 413,
    URI_TOO_LONG = 414,
    UNSUPPORTED_MEDIA_TYPE = 415,
    RANGE_NOT_SATISFIABLE = 416,
    EXPECTATION_FAILED = 417,
    IM_A_TEAPOT = 418,
    MISDIRECTED_REQUEST = 421,
    UNPROCESSABLE_CONTENT = 422,
    LOCKED = 423,
    FAILED_DEPENDENCY = 424,
    TOO_EARLY = 425,
    UPGRADE_REQUIRED = 426,
    PRECONDITION_REQUIRED = 428,
    TOO_MANY_REQUESTS = 429,
    REQUEST_HEADER_FIELDS_TOO_LARGE = 431,
    UNAVAILABLE_FOR_LEGAL_REASONS = 451,
    INTERNAL_SERVER_ERROR = 500,
    NOT_IMPLEMENTED = 501,
    BAD_GATEWAY = 502,
    SERVICE_UNAVAILABLE = 503,
    GATEWAY_TIMEOUT = 504,
    HTTP_VERSION_NOT_SUPPORTED = 505,
    VARIANT_ALSO_NEGOTIATES = 506,
    INSUFFICIENT_STORAGE = 507,
    LOOP_DETECTED = 508,
    NOT_EXTENDED = 510,
    NETWORK_AUTHENTICATION_REQUIRED = 511
}

impl FromStr for StatusCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "100" => Ok(Self::CONTINUE),
            "101" => Ok(Self::SWITCHING_PROTOCOLS),
            "102" => Ok(Self::PROCESSING),
            "103" => Ok(Self::EARLY_HINTS),

            "200" => Ok(Self::OK),
            "201" => Ok(Self::CREATED),
            "202" => Ok(Self::ACCEPTED),
            "203" => Ok(Self::NON_AUTHORITATIVE_INFORMATION),
            "204" => Ok(Self::NO_CONTENT),
            "205" => Ok(Self::RESET_CONTENT),
            "206" => Ok(Self::PARTIAL_CONTENT),
            "207" => Ok(Self::MULTI_STATUS),
            "208" => Ok(Self::ALREADY_REPORTED),
            "226" => Ok(Self::IM_USED),

            "300" => Ok(Self::MULTIPLE_CHOICES),
            "301" => Ok(Self::MOVED_PERMANENTLY),
            "302" => Ok(Self::FOUND),
            "303" => Ok(Self::SEE_OTHER),
            "304" => Ok(Self::NOT_MODIFIED),
            "305" => Ok(Self::USE_PROXY),
            "306" => Ok(Self::UNUSED),
            "307" => Ok(Self::TEMPORARY_REDIRECT),
            "308" => Ok(Self::PERMANENT_REDIRECT),

            "400" => Ok(Self::BAD_REQUEST),
            "401" => Ok(Self::UNAUTHORIZED),
            "402" => Ok(Self::PAYMENT_REQUIRED),
            "403" => Ok(Self::FORBIDDEN),
            "404" => Ok(Self::NOT_FOUND),
            "405" => Ok(Self::METHOD_NOT_ALLOWED),
            "406" => Ok(Self::NOT_ACCEPTABLE),
            "407" => Ok(Self::PROXY_AUTHNTICATION_REQUIRED),
            "408" => Ok(Self::REQUEST_TIMEOUT),
            "409" => Ok(Self::CONFLICT),
            "410" => Ok(Self::GONE),
            "411" => Ok(Self::LENGTH_REQUIRED),
            "412" => Ok(Self::PRECONDITION_FAILED),
            "413" => Ok(Self::CONTENT_TOO_LARGE),
            "414" => Ok(Self::URI_TOO_LONG),
            "415" => Ok(Self::UNSUPPORTED_MEDIA_TYPE),
            "416" => Ok(Self::RANGE_NOT_SATISFIABLE),
            "417" => Ok(Self::EXPECTATION_FAILED),
            "418" => Ok(Self::IM_A_TEAPOT),
            "421" => Ok(Self::MISDIRECTED_REQUEST),
            "422" => Ok(Self::UNPROCESSABLE_CONTENT),
            "423" => Ok(Self::LOCKED),
            "424" => Ok(Self::FAILED_DEPENDENCY),
            "425" => Ok(Self::TOO_EARLY),
            "426" => Ok(Self::UPGRADE_REQUIRED),
            "428" => Ok(Self::PRECONDITION_REQUIRED),
            "429" => Ok(Self::TOO_MANY_REQUESTS),
            "431" => Ok(Self::REQUEST_HEADER_FIELDS_TOO_LARGE),
            "451" => Ok(Self::UNAVAILABLE_FOR_LEGAL_REASONS),

            "500" => Ok(Self::INTERNAL_SERVER_ERROR),
            "501" => Ok(Self::NOT_IMPLEMENTED),
            "502" => Ok(Self::BAD_GATEWAY),
            "503" => Ok(Self::SERVICE_UNAVAILABLE),
            "504" => Ok(Self::GATEWAY_TIMEOUT),
            "505" => Ok(Self::HTTP_VERSION_NOT_SUPPORTED),
            "506" => Ok(Self::VARIANT_ALSO_NEGOTIATES),
            "507" => Ok(Self::INSUFFICIENT_STORAGE),
            "508" => Ok(Self::LOOP_DETECTED),
            "510" => Ok(Self::NOT_EXTENDED),
            "511" => Ok(Self::NETWORK_AUTHENTICATION_REQUIRED),
            _ => Err(String::from("Invalid http status code"))
        }
    }
}