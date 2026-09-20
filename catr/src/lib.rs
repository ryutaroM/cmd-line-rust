use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(cfg: Config) -> MyResult<()> {
    for f in cfg.files {
        match open(&f) {
            Err(e) => eprintln!("Failed to open {}: {}", f, e),
            Ok(buffer) => {
                let mut idx = 0;
                for lines in buffer.lines() {
                    if let Ok(s) = lines {
                        if cfg.number_lines {
                            idx = idx + 1;
                            println!("{:>6}\t{}", idx, s)
                        } else if cfg.number_nonblank_lines {
                            if s != "" {
                                idx = idx + 1;
                                println!("{:>6}\t{}", idx, s)
                            } else {
                                println!("{}", s)
                            }
                        } else {
                            println!("{}", s)
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("ryutaroM")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input Files")
                .min_values(1)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("Print number of lines")
                .conflicts_with("number_nonblank_lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("Do not print number of blank lines")
                .takes_value(false),
        )
        .get_matches();

    let files = match matches.values_of_lossy("files") {
        Some(s) => s,
        None => vec![],
    };

    let number_lines = matches.is_present("number_lines");

    let number_nonblank_lines = matches.is_present("number_nonblank_lines");

    Ok(Config {
        files,
        number_lines,
        number_nonblank_lines,
    })
}

fn open(f: &str) -> MyResult<Box<dyn BufRead>> {
    match f {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(f)?))),
    }
}
