extern crate base64_mime;
mod args;

use std::{fs::File, io::Write};

use base64_mime::Base64Writer;
fn main() {
    let args = args::handle_arguments();
    if args.help || args.file.is_none() {
        args::print_usage();
        return;
    }
    let mut file = File::open(args.file.unwrap()).unwrap();
    let mut stdout = std::io::stdout();
    stdout.write_all("data:;base64,".as_bytes()).unwrap();
    let mut writer = Base64Writer::new(stdout);
    std::io::copy(&mut file, &mut writer).unwrap();
    writer.flush().unwrap();
}
