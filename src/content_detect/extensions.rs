pub fn detect_extension(extension: &str) -> Option<String> {
    //AI produced list of file extensions, with some additions
    let candidate = match extension {
        "csv" => "text/csv",
        "html" => "text/html",
        "htm" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "tif" => "image/tiff",
        "tiff" => "image/tiff",
        "jxl" => "image/jxl",
        "bmp" => "image/bmp",
        "svg" => "image/svg+xml",
        "pdf" => "application/pdf",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "ppt" => "application/vnd.ms-powerpoint",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        "zip" => "application/zip",
        "rar" => "application/x-rar-compressed",
        "7z" => "application/x-7z-compressed",
        "tar" => "application/x-tar",
        "gz" => "application/gzip",
        "bz2" => "application/x-bzip2",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" => "application/ogg",
        "m4a" => "audio/mp4",
        "mp4" => "video/mp4",
        "mpeg" => "video/mpeg",
        "avi" => "video/x-msvideo",
        "mov" => "video/quicktime",
        "flv" => "video/x-flv",
        "webm" => "video/webm",
        "exe" => "application/vnd.microsoft.portable-executable",
        "bat" => "application/x-msdos-program",
        "sh" => "application/x-sh",
        "txt" => "text/plain",
        _ => "",
    };
    return if candidate != "" {
        Some(candidate.to_string())
    } else {
        None
    };
}
