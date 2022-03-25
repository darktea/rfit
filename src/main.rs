use clap::{Arg, Command};
use log::error;
use log::info;

use rfit::Config;

fn main() {
    env_logger::init();

    let matches = Command::new("rfit")
        .arg(
            Arg::new("infile")
                .short('i')
                .help("read from an input txt file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("outfile")
                .short('o')
                .help("write the result to an output txt file")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let from_filename = matches.value_of("infile").unwrap().to_string();
    let to_filename = matches.value_of("outfile").unwrap().to_string();

    let config = Config {
        from_filename: &from_filename,
        to_filename: &to_filename,
    };

    info!(
        "try to format for the file: {}, and the formatted file is: {}",
        config.from_filename, config.to_filename
    );

    match rfit::run(&config) {
        Ok(()) => {
            // set the log level: export RUST_LOG=info
            info!(
                "Success!. Format for the file: {}, and the formatted file is: {}",
                config.from_filename, config.to_filename
            );
        }
        Err(e) => {
            error!("failed. the error is: {}", e);
        }
    };
}
