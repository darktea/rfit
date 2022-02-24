use log::error;
use log::info;
use std::env;

use rfit::Config;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        error!("failed. bad arguments number");
        return;
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
