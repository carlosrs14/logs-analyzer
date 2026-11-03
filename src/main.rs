use std::{env, fs::File, io::{BufRead, BufReader}};

use logs_analyzer::{log};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Error: Invalid usage");
        print_usage();
        return;

    }

    let file = File::open(&args[1]);
    
    if file.is_err() {
        println!("Error: could not open that file");
        return;
    }

    let file =  file.unwrap();
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
                    Ok(log) => println!("{:?}", log),
                    Err(err) => println!("Error: reading line number {}: {}", line_number, err),
                }
            }
            Err(_) => {
                println!("Error: reading line number {}", line_number);
                break;
            }
        }
    }
}

fn print_usage() {
    println!("logs_analyzer <filename>");
}