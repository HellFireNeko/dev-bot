pub fn consume_until_crlf(iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<String, String> {
    let mut value = String::new();
    while let Some(c) = iter.next() {
        if c == '\r' { // Char is CR
            if let Some('\n') = iter.next() { // Char is LF
                return Ok(value);
            }
        }
        value.push(c);
    }
    Err("Expected CRLF".to_string()) // Uh oh, the provided iterator had no CRLF
}

pub fn consume_crlf(iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<(), String> {
    if let Some('\r') = iter.next() {
        if let Some('\n') = iter.next() {
            return Ok(());
        }
    }
    Err("Expected CRLF".to_string()) // Uh oh, the provided iterator had no CRLF
}

pub fn consume_n_chars(iter: &mut std::iter::Peekable<std::str::Chars>, n: usize) -> Result<String, String> {
    let mut value = String::new();
    for _ in 0..n {
        if let Some(c) = iter.next() { // We got a char, lets push it to value
            value.push(c);
        } else {
            return Err("Unexpected end of input".to_string()); // Uh oh, the provided iterator ended here! But we still wanted to read more
        }
    }
    Ok(value)
}