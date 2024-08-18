use std::fs::File;
use std::io::Read;

/**
 * Detects binary vs utf-8 file contents. On failure returns None, indicating empty/unavailable content.
 */
pub fn detect_by_content(filename: String) -> Option<String> {
    let file = File::open(filename).ok()?;
    let mut content = Vec::new();
    let size = file.take(2048).read_to_end(&mut content).ok()?;
    if size == 0 {
        return None;
    }
    let mut binary = loop {
        if let Some(tail) = content.pop() {
            if tail & 0b10000000u8 != 0 {
                continue;
            } else {
                content.push(tail);
                break false;
            }
        } else {
            break true;
        }
    };
    if !binary {
        binary = String::from_utf8(content).is_err();
    }
    if binary {
        Some("application/octet-stream".to_string())
    } else {
        Some("text/plain".to_string())
    }
}
