pub fn parse_header(line: &str) -> (&str, &str) {
    let mut line = line.split(":");
    let key = match line.next() {
        Some(key) => key,
        None => return ("", ""),
    };

    let value = match line.next() {
        Some(value) => value,
        None => "",
    };

    (key, value)
}

pub fn get_header(headers: &[u8], name: &str) -> Option<String> {
    let headers = String::from_utf8_lossy(headers).to_string();
    let mut headers = headers.lines();

    while let Some(header) = headers.next() {
        let (key, value) = parse_header(header);
        if key == name {
            return Some(value.trim().to_string());
        }
    }

    None
}