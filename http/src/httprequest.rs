use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/2.0" => Version::V2_0,
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

impl From<String> for HttpRequest {
    fn from(s: String) -> HttpRequest {
        let mut lines = s.lines();
        let mut store: HashMap<String, String> = HashMap::new();

        let request_line: Vec<&str> = lines.next().map(|s| s.split(" ").collect()).unwrap();

        let (method, path, version) = match request_line.as_slice() {
            [first, second, third] => (*first, *second, *third),
            _ => panic!("Vector does not contain exactly three elements"),
        };

        for line in lines.by_ref().take_while(|&line| !line.is_empty()) {
            let header = line.split(":").collect::<Vec<&str>>();

            if header.len() >= 2 {
                let key = header[0].trim().to_string();
                let value = header[1].trim().to_string();
                store.insert(key, value);
            }
        }

        let content = lines.next().unwrap_or_else(|| "");

        HttpRequest {
            method: Method::from(method),
            version: Version::from(version),
            resource: Resource::Path(path.to_string()),
            headers: store,
            msg_body: content.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let s: String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), "localhost".into());
        headers_expected.insert("Accept".into(), "*/*".into());
        headers_expected.insert("User-Agent".into(), "curl/7.64.1".into());
        let req: HttpRequest = s.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }
}
