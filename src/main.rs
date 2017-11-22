use std::error::Error;
use std::io::{BufRead, BufReader};
use std::io;

fn main() {
    let mut stdin = BufReader::new(io::stdin());
    let mut buf = Vec::new();

    loop {
        if let Err(e) = stdin.read_until(0xFF, &mut buf) {
            eprintln!("{}", e.description());
            std::process::exit(1);
        }
        // Pop the frame ending byte (0xFF)
        buf.pop();

        println!("Frame: {:?}", buf);
        buf.clear();
    }
}
