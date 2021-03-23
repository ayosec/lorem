use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;

const HELP: &str = include_str!("HELP.txt");

#[derive(Debug)]
pub struct Options {
    pub source: Source,

    pub count_type: CountType,

    pub count: usize,

    pub wrap: Option<usize>,
}

#[derive(Debug)]
pub enum Source {
    Lipsum,

    Random,

    File(PathBuf),
}

#[derive(Debug)]
pub enum CountType {
    Chars,

    Words,
}

/// Parse arguments from command line.
pub fn get() -> Result<Options, String> {
    let mut options = Options {
        source: Source::Lipsum,
        count_type: CountType::Words,
        count: 100,
        wrap: None,
    };

    let mut args = env::args_os().skip(1);
    while let Some(arg) = args.next() {
        match arg.to_str().ok_or_else(|| invalid_arg(&arg))? {
            "-h" | "--help" => return Err(HELP.into()),

            "-c" => options.count_type = CountType::Chars,

            "-r" => options.source = Source::Random,

            "-f" => {
                let file = match args.next() {
                    Some(arg) => PathBuf::from(arg),
                    None => return Err(String::from("Missing value for -f")),
                };

                options.source = Source::File(file);
            }

            "-w" => {
                let width = match args.next() {
                    Some(arg) => match arg.to_str().and_then(|a| a.parse().ok()) {
                        Some(a) => a,
                        None => return Err(invalid_arg(&arg)),
                    },

                    None => return Err(String::from("Missing value for -w")),
                };

                options.wrap = Some(width);
            }

            free => match free.parse() {
                Ok(count) => options.count = count,
                Err(_) => return Err(invalid_arg(&arg)),
            },
        }
    }

    Ok(options)
}

/// Format a message for invalid arguments.
fn invalid_arg(arg: &OsStr) -> String {
    format!("Invalid argument: {}", arg.to_string_lossy())
}
