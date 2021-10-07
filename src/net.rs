use std::net::TcpStream;
use std::io::{Write, Read};

pub enum Scheme {
    HTTP,
    HTTPS,
}

pub fn resolve_scheme(uri: &str) -> Result<Scheme, String> {
    if uri.starts_with("https://") {
        return Ok(Scheme::HTTPS)
    }
    if uri.starts_with("http://") {
        return Ok(Scheme::HTTP)
    }

    Err(format!("Unknown scheme in uri: {}", uri))
}

pub fn send_http_request(addr: &str) -> Result<String, std::io::Error> {
    let mut content = String::default();
    let mut stream = TcpStream::connect(format!("{}:80", addr))?;
    stream.write(b"GET /index.html HTTP/1.0\r\nHost: example.org\r\n\r\n")?;
    stream.read_to_string(&mut content)?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_scheme() {
        assert!(matches!(resolve_scheme("http://www.google.com"), Ok(Scheme::HTTP)));
        assert!(matches!(resolve_scheme("https://www.google.com"), Ok(Scheme::HTTPS)));
        assert!(resolve_scheme("ftp://www.google.com").is_err());
    }
}