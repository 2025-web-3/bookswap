use {
    crate::routes::{HttpError, Result as HttpResult},
    actix_web::{http::header::HeaderName, HttpRequest},
    nanoid::nanoid,
    regex::Regex,
};

/// Returns IPv4 address from given request. Allows X-Forwarded-For for debugging
pub fn debug_ip(request: &HttpRequest) -> HttpResult<String> {
    let socket = request
        .connection_info()
        .realip_remote_addr()
        .ok_or_else(|| HttpError::InvalidCredentials("IP address is not valid".to_string()))?
        .to_string();
    Ok(socket)
}

/// Returns IPv4 address from given request
pub fn extract_ip_from_request(request: &HttpRequest) -> HttpResult<String> {
    let socket = request
        .peer_addr()
        .ok_or_else(|| HttpError::InvalidCredentials("IP address is not valid".to_string()))?
        .ip()
        .to_canonical()
        .to_string();
    Ok(socket)
}

/// Returns data from specified header from given request
pub fn extract_header(request: &HttpRequest, header: HeaderName) -> HttpResult<&str> {
    let headers = request.headers();
    let header_value = headers.get(header.clone());
    header_value
        .ok_or_else(|| {
            HttpError::Header(format!("Header {} was not found", header.to_string().to_uppercase()))
        })?
        .to_str()
        .map_err(|_| HttpError::Header("".to_string()))
}

const _TOKEN_CHARS: [char; 33] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'A', 'B', 'C',
    'D', 'E', 'F', 'k', 'n', 'm', 'K', 'N', 'M', 'h', 'H', 'l', 'L', '.',
];

pub fn new_token(length: usize) -> String {
    nanoid!(length, &_TOKEN_CHARS)
}

pub fn is_valid_password(password: &str) -> bool {
    let length_regex = Regex::new(r".{8,}").unwrap();
    let upper_regex = Regex::new(r"[A-Z]").unwrap();
    let lower_regex = Regex::new(r"[a-z]").unwrap();
    let digit_regex = Regex::new(r"[0-9]").unwrap();
    let special_regex = Regex::new(r"[#?!@$%^&*-]").unwrap();

    length_regex.is_match(password)
        && upper_regex.is_match(password)
        && lower_regex.is_match(password)
        && digit_regex.is_match(password)
        && special_regex.is_match(password)
}
