extern crate base64_mime;
mod args;
mod content_detect;

use std::{fs::File, io::Write};

use base64_mime::Base64Writer;
use content_detect::detect_content_type;
fn main() {
    let args = args::handle_arguments();
    if args.help || args.file.is_none() {
        args::print_usage();
        return;
    }
    let filename = args.file.unwrap();
    let mut file = File::open(filename.clone()).unwrap();
    let mut stdout = std::io::stdout();
    let header = format!("data:{};base64,", detect_content_type(filename.clone()));
    stdout.write_all(header.as_bytes()).unwrap();
    let mut writer = Base64Writer::new(stdout);
    std::io::copy(&mut file, &mut writer).unwrap();
    writer.flush().unwrap();
}
