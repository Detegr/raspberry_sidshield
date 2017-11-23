use std::error::Error;
use std::io::{BufRead, BufReader};
use std::io;
use std::thread;
use std::time::{Duration, SystemTime};

const FRAME_ENDING_BYTE: u8 = 0xFF;
const PLAY_LOOP_HZ: u64 = 50;
const PLAY_LOOP_MS: u64 = ((1.0 / PLAY_LOOP_HZ as f64) * 1000.0) as u64;

fn main() {
    let play_loop_duration = Duration::from_millis(PLAY_LOOP_MS);
    let mut stdin = BufReader::new(io::stdin());
    let mut buf = Vec::with_capacity(64);
    loop {
        let now = SystemTime::now();

        if let Err(e) = stdin.read_until(FRAME_ENDING_BYTE, &mut buf) {
            eprintln!("{}", e.description());
            std::process::exit(1);
        }
        // Pop the frame ending byte
        buf.pop();

        println!("Frame: {:?}", buf);
        buf.clear();

        let frame_duration = now.elapsed().unwrap();
        thread::sleep(play_loop_duration - frame_duration);
    }
}
