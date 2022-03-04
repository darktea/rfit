use std::fs::File;
use std::fs::OpenOptions;
use std::io::{prelude::*, BufReader, BufWriter};

use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed for io error: {}", source))]
    IoError { source: std::io::Error },
    #[snafu(display("failed. error is: {}", message))]
    ArgumentError { message: String },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Config<'a> {
    pub from_filename: &'a String,
    pub to_filename: &'a String,
}

/// format the file from Config.from_filename to config.to_filename
/// # Errors
/// if have a IO Error, return IoSnafu which keep the error context
pub fn run(config: &Config) -> Result<()> {
    let file = File::open(config.from_filename).context(IoSnafu)?;
    let reader = BufReader::new(file);

    // fail if file exists
    let to_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(config.to_filename)
        .context(IoSnafu)?;
    let mut writer = BufWriter::new(to_file);

    let eof = "\n\n";
    for line in reader.lines() {
        let string_line = line.context(IoSnafu)?;
        // skip specific UTF-8 char in the start of the line
        let skip_line = trim_start_str(string_line.as_str());
        // trim all whitespace in the line
        let bytes_line = trim_ascii_whitespace(skip_line.as_bytes());

        // ignore empty lines
        if bytes_line.is_empty() {
            continue;
        }

        // write line content into write buffer
        writer.write_all(bytes_line).context(IoSnafu)?;
        // write empty line into write buffer
        writer.write_all(eof.as_bytes()).context(IoSnafu)?;
    }
    // do flush at the last
    writer.flush().context(IoSnafu)?;

    Ok(())
}

fn trim_ascii_whitespace(x: &[u8]) -> &[u8] {
    let from = match x.iter().position(|x| !x.is_ascii_whitespace()) {
        Some(i) => i,
        None => return &x[0..0],
    };
    let to = x.iter().rposition(|x| !x.is_ascii_whitespace()).unwrap();
    &x[from..=to]
}

fn trim_start_str(x: &str) -> &str {
    let pc = '\u{3000}';
    let length = x.len();
    if length == 0 {
        return &x[0..0];
    }

    // take_while on the char：'\u{3000}'
    let i = x.chars().take_while(|c| *c == pc).count();

    // because the bytes length of the '\u{3000}' is 3, should use i * 3
    let cut_length = i * 3;

    // cut all '\u{3000}' in the begin
    &x[cut_length..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        assert_eq!("", trim_start_str("\u{3000}"));
    }

    #[test]
    fn two_result() {
        assert_eq!(
            "第六百一十八章 墨蛟与神宫",
            trim_start_str("第六百一十八章 墨蛟与神宫")
        );
    }

    #[test]
    fn three_result() {
        assert_eq!(
            "“别老一惊一乍的！又怎么了！？”",
            trim_start_str("\u{3000}“别老一惊一乍的！又怎么了！？”")
        );
    }

    #[test]
    fn four_result() {
        assert_eq!(
            "“别老一惊一乍的！又怎么了！？”",
            trim_start_str("\u{3000}\u{3000}\u{3000}\u{3000}“别老一惊一乍的！又怎么了！？”")
        );
    }

    #[test]
    fn five_result() {
        assert_eq!(
            "“别老一惊\u{3000}一乍的！又怎\u{3000}么了！？”",
            trim_start_str(
                "\u{3000}\u{3000}\u{3000}\u{3000}“别老一惊\u{3000}一乍的！又怎\u{3000}么了！？”"
            )
        );
    }
}
