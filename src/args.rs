use std::env;
pub struct Args {
    pub help: bool,
    pub file: Option<String>,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            help: false,
            file: None,
        }
    }
}

fn is_flag(argument: &String) -> bool {
    argument.bytes().nth(0) == Some('-' as u8)
}

fn handle_flag(argument: String, parsed: &mut Args) {
    match argument.as_str() {
        "-h" | "--help" => parsed.help = true,
        _ => panic!("Unknown flag {}", argument),
    }
}

fn handle_positional(argument: String, parsed: &mut Args) {
    if parsed.file.is_none() {
        parsed.file = Some(argument)
    } else {
        panic!("Invalid argument {}", argument)
    }
}

pub fn handle_arguments() -> Args {
    let mut args = Default::default();
    for argument in env::args().skip(1) {
        if is_flag(&argument) {
            handle_flag(argument, &mut args);
        } else {
            handle_positional(argument, &mut args);
        }
    }
    args
}

pub fn print_usage() {
    println!(
        "{}",
        "Usage:\n\
        \tdata-url [file]\tConvert file to data-url\n\
        \tdata-url --help\tPrint this usage"
    )
}
