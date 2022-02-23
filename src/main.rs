use log::error;
use log::info;
use std::env;

use rfit::{ArgumentError, Config, Error};

fn main() -> Result<(), Error> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return ArgumentError {
            message: String::from("not enough arguments"),
        }
        .fail();
    }

    let from_filename = args[1].clone();
    let to_filename = args[2].clone();

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
            info!(
                "Success!. Format for the file: {}, and the formatted file is: {}",
                config.from_filename, config.to_filename
            );
        }
        Err(e) => {
            error!("failed. the error is: {}", e);
        }
    };

    Ok(())
}
