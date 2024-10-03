use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    version: Version,
    pub resource: Resource,
    headers: HashMap<String, String>,
    msg_body: String,
}

impl HttpRequest {
    fn default() -> Self {
        HttpRequest {
            method: Method::Uninitialized,
            version: Version::V1_1,
            resource: Resource::Path("".to_string()),
            headers: HashMap::new(),
            msg_body: "".to_string(),
        }
    }
}

impl From<String> for HttpRequest {
    fn from(value: String) -> Self {
        let mut http_request = HttpRequest::default();

        for line in value.lines() {
            if line.contains("HTTP") {
                let mut words = line.split_whitespace();
                http_request.method = words.next().unwrap().into();
                http_request.resource = Resource::Path(words.next().unwrap().to_string());
                http_request.version = words.next().unwrap().into();
            } else if line.contains(":") {
                let mut header_items = line.split(":");
                let key = header_items.next().unwrap().to_string();
                let value = header_items.next().unwrap().to_string();
                http_request.headers.insert(key, value);
            } else if line.len() == 0 {} else {
                http_request.msg_body = line.to_string();
            }
        }

        http_request
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let s: String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\n\
            User-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("Accept".into(), " */*".into());
        headers_expected.insert("User-Agent".into(), " curl/7.64.1".into());
        let req: HttpRequest = s.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}
