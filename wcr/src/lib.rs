use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("ryutaroM")
        .about("Rust wc")
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("Show line count")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("Show word count")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("Show byte count")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .help("Show character count")
                .takes_value(false)
                .conflicts_with("bytes"),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input file(s)")
                .default_value("-")
                .multiple(true),
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|f| !f) {
        lines = true;
        words = true;
        bytes = true;
    }

    let files = matches.values_of_lossy("files").unwrap();

    Ok(Config {
        files,
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(cfg: Config) -> MyResult<()> {
    let mut total = FileInfo {
        num_lines: 0,
        num_words: 0,
        num_bytes: 0,
        num_chars: 0,
    };
    for (i, f) in cfg.files.iter().enumerate() {
        match open(f) {
            Ok(handle) => {
                let cnt = count(handle)?;
                let mut output = String::new();
                if cfg.lines {
                    output.push_str(&format!("{:>8}", cnt.num_lines));
                    total.num_lines += cnt.num_lines
                }
                if cfg.words {
                    output.push_str(&format!("{:>8}", cnt.num_words));
                    total.num_words += cnt.num_words
                }
                if cfg.bytes {
                    output.push_str(&format!("{:>8}", cnt.num_bytes));
                    total.num_bytes += cnt.num_bytes
                }
                if cfg.chars {
                    output.push_str(&format!("{:>8}", cnt.num_chars));
                    total.num_chars += cnt.num_chars
                }
                if f != "-" {
                    output.push_str(&format!(" {}", f));
                }
                println!("{}", output);

                if cfg.files.len() > 1 && (cfg.files.len() - 1) == i {
                    let mut total_output = String::new();
                    if cfg.lines {
                        total_output.push_str(&format!("{:>8}", total.num_lines));
                    }
                    if cfg.words {
                        total_output.push_str(&format!("{:>8}", total.num_words));
                    }
                    if cfg.bytes {
                        total_output.push_str(&format!("{:>8}", total.num_bytes));
                    }
                    if cfg.chars {
                        total_output.push_str(&format!("{:>8}", total.num_chars));
                    }
                    println!("{} total", total_output);
                }
            }
            Err(e) => eprintln!("{}: {}", f, e),
        }
    }

    Ok(())
}

fn open(f: &str) -> MyResult<Box<dyn BufRead>> {
    match f {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(f)?))),
    }
}

pub fn count(mut f: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut buf = String::new();
    loop {
        let result = f.read_line(&mut buf)?;
        if result == 0 {
            break;
        }
        //lines
        num_lines += 1;
        //bytes
        num_bytes += result;
        //words
        num_words += buf.split_whitespace().count();
        //chars
        num_chars += buf.chars().count();
        buf.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
