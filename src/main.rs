use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::env;
use text_colorizer::*;


#[derive(Debug)]
struct Arguments {
    path:     String
}

fn print_usage() {
    eprintln!("{} - Hashes and displays the hashed contents of a specific file", "HASHING".green());
    eprintln!("Usage: hashing <path>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        print_usage();
        eprintln!("{} wrong number of arguments: expected 1 got, {}", "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        path:        args[0].clone()
    }
}

fn sha256_digest<R: Read>(mut reader: R) -> Digest {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = match reader.read(&mut buffer) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{} failed to read from buffer: {:?}", "Error:".red().bold(), e);
                std::process::exit(1);
            }
        };
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    context.finish()
}


fn main() {
    let args = parse_args();

    let input = match File::open(&args.path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to open file '{}': {:?}", "Error:".red().bold(), &args.path, e);
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(input);
    let digest = sha256_digest(reader);

    println!("SHA-256 digest of the content in {}: {}", &args.path, HEXUPPER.encode(digest.as_ref()));
}
