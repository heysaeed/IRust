use crate::irust::options::Options;

use std::env;

const VERSION: &str = "1.1.2";

pub fn handle_args(options: &mut Options) -> bool {
    let args: Vec<String> = env::args().skip(1).collect();

    if !args.is_empty() {
        match args[0].as_str() {
            "-h" | "--help" => {
                print!(
                    "IRust: Cross Platform Rust REPL
        version: {}\n
        config file is in {}\n
        --help => shows this message
        --reset-config => reset IRust configuration to default",
                    VERSION,
                    Options::config_path()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|| "??".into())
                );
                return true;
            }

            "-v" | "--version" => {
                print!("{}", VERSION);
                return true;
            }

            "--reset-config" => {
                options.reset();
            }

            x => {
                eprintln!("Unknown argument: {}", x);
            }
        }
    }

    false
}
