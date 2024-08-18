use std::{ffi::OsStr, path::Path};
mod content;
mod extensions;

pub fn detect_content_type(filename: String) -> String {
    let extension = Path::new(filename.as_str())
        .extension()
        .map(OsStr::to_str)
        .flatten();
    let detect_by_content = || -> Option<String> { content::detect_by_content(filename.clone()) };
    let candidate = extension
        .map(extensions::detect_extension)
        .flatten()
        .or_else(detect_by_content);
    if let Some(result) = candidate {
        result
    } else {
        "".to_string()
    }
}
