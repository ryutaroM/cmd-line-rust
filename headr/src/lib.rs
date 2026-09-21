use clap::{App, Arg};
use std::error::Error;

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
            Arg::with_name("LINES")
                .short("n")
                .long("lines")
                .help("Print specified first lines")
                .default_value("10")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("BYTES")
                .short("c")
                .long("bytes")
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

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
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
