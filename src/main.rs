use std::{
    fs::File,
    io::{BufRead, BufReader},
    process,
};
use logs_analyzer::{http, log, util::{CliArgs, print_usage}};

fn main() {
    let cli_args = match CliArgs::parse() {
        Ok(Some(args)) => args,
        Ok(None) => return,
        Err(err) => {
            eprintln!("Error: {}", err);
            print_usage();
            process::exit(1);
        }
    };

    let file = match File::open(&cli_args.file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: could not open '{}': {}", cli_args.file_path, e);
            process::exit(1);
        }
    };

    let mut reader = BufReader::new(file);
    let mut line_number: u32 = 0;
    let mut line = String::new();

    loop {
        line.clear();
        let read_lines = reader.read_line(&mut line);
        line_number += 1;

        match read_lines {
            Ok(0) => break,
            Ok(_) => {
                match log::Log::new(&line) {
                    Ok(log) => {
                        if let Some(m) = cli_args.method {
                            if log.method != m {
                                continue;
                            }
                        }
                        if let Some(s) = cli_args.status {
                            if log.status != s {
                                continue;
                            }
                        }
                        println!("{:?}", log);
                    }
                    Err(err) => eprintln!("Error: reading line number {}: {}", line_number, err),
                }
            }
            Err(err) => {
                eprintln!("Error: reading line number {}: {}", line_number, err);
                break;
            }
        }
    }
}
