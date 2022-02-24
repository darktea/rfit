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
        let skip_line = trim_start_str(string_line.as_str());
        let bytes_line = trim_ascii_whitespace(skip_line.as_bytes());

        // number of current multi empty lines
        if bytes_line.is_empty() {
            continue;
        }

        writer.write_all(bytes_line).context(IoSnafu)?;
        writer.write_all(eof.as_bytes()).context(IoSnafu)?;
    }
    writer.flush().context(IoSnafu)?;

    Ok(())
}

pub fn trim_ascii_whitespace(x: &[u8]) -> &[u8] {
    let from = match x.iter().position(|x| !x.is_ascii_whitespace()) {
        Some(i) => i,
        None => return &x[0..0],
    };
    let to = x.iter().rposition(|x| !x.is_ascii_whitespace()).unwrap();
    &x[from..=to]
}

pub fn trim_start_str(x: &str) -> &str {
    let pc = '\u{3000}';
    let mut i = 0;

    let mut un_match = false;
    for c in x.chars() {
        if pc == c {
            i += 1;
        } else {
            un_match = true;
            break;
        }
    }

    if un_match {
        &x[i * 3..]
    } else {
        &x[0..0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let str_skip = String::from("\u{3000}");

        assert_eq!("", trim_start_str(str_skip.as_str()));
    }

    #[test]
    fn two_result() {
        let byte_same = "第六百一十八章 墨蛟与神宫";

        assert_eq!(byte_same, trim_start_str(byte_same));
    }

    #[test]
    fn three_result() {
        let str_full = String::from("\u{3000}“别老一惊一乍的！又怎么了！？”");
        let str_content = String::from("“别老一惊一乍的！又怎么了！？”");

        assert_eq!(str_content, trim_start_str(str_full.as_str()));
    }

    #[test]
    fn four_result() {
        let str_full =
            String::from("\u{3000}\u{3000}\u{3000}\u{3000}“别老一惊一乍的！又怎么了！？”");
        let str_content = String::from("“别老一惊一乍的！又怎么了！？”");

        assert_eq!(str_content, trim_start_str(str_full.as_str()));
    }
}
