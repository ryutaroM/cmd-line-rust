use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, stdout, BufRead, BufReader, Read, Write},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("ryutaroM")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input FILES")
                .min_values(1)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Print specified first lines")
                .default_value("10")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .help("Print specified first bytes")
                .conflicts_with("lines")
                .takes_value(true),
        )
        .get_matches();

    let files = match matches.values_of_lossy("files") {
        Some(s) => s,
        None => vec![],
    };

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|s| format!("illegal line count -- {}", s))?
        .unwrap();

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|s| format!("illegal byte count -- {}", s))?;

    Ok(Config {
        files,
        lines,
        bytes,
    })
}

fn open(f: &str) -> MyResult<Box<dyn BufRead>> {
    match f {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(f)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for (i, f) in config.files.iter().enumerate() {
        match open(&f) {
            Err(err) => eprintln!("{}: {}", f, err),
            Ok(mut handle) => {
                if config.bytes.is_some() {
                    let mut inc = 1;
                    let mut collect_bytes: Vec<u8> = vec![];
                    if i > 0 {
                        println!();
                    }
                    if config.files.len() > 1 {
                        println!("==> {} <==", &f);
                    }

                    for b in handle.bytes() {
                        inc += 1;
                        collect_bytes.push(b?);
                        if inc > config.bytes.unwrap() {
                            break;
                        }
                    }
                    write!(
                        stdout(),
                        "{}",
                        String::from_utf8_lossy(collect_bytes.as_slice()).to_string()
                    )?;
                } else if config.lines > 0 {
                    if i > 0 {
                        println!();
                    }
                    if config.files.len() > 1 {
                        println!("==> {} <==", &f);
                    }

                    let mut loops = 0;
                    loop {
                        loops += 1;
                        let mut buf = String::new();
                        let result = handle.read_line(&mut buf);
                        if result? == 0 || loops > config.lines {
                            break;
                        }
                        print!("{}", buf);
                    }
                }
            }
        }
    }
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(val.into()),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());
}
