use clap::{App, Arg, SubCommand};
use rppal::pwm;
use std::error::Error;
use std::io;
use std::io::{BufRead, BufReader};

mod gpio;

const FRAME_ENDING_BYTE: u8 = 0xFF;

fn main() {
    let matches = App::new("Raspberry Pi SID shield")
        .arg(
            Arg::with_name("Enable debug output")
                .short("d")
                .long("debug"),
        )
        .arg(
            Arg::with_name("Disable GPIO")
                .short("g")
                .long("disable_gpio"),
        )
        .get_matches();
    let debug = matches.is_present("Enable debug output");
    let disable_gpio = matches.is_present("Disable GPIO");
    let mut stdin = BufReader::new(io::stdin());
    let mut buf = Vec::with_capacity(64);
    let mut gpio = gpio::init_gpio(disable_gpio).expect("Failed to initialize GPIO");

    // 1mhz clock with 50% duty cycle for 6581
    let pwm =
        pwm::Pwm::with_frequency(pwm::Channel::Pwm0, 1000.0, 0.5, pwm::Polarity::Normal, true);

    if pwm.is_err() && !disable_gpio {
        // Error out if gpio is not disabled and PWM setup failed
        let _ = pwm.expect("Failed to set PWM on pin 18");
    }

    loop {
        if let Err(e) = stdin.read_until(FRAME_ENDING_BYTE, &mut buf) {
            eprintln!("{}", e.description());
            std::process::exit(1);
        }
        // Pop the frame ending byte
        buf.pop();

        for chunk in buf.chunks(2) {
            if chunk.len() < 2 {
                println!("NO DATA FOR ADDR {:02X}", chunk[0]);
            } else {
                let addr = chunk[0];
                let data = chunk[1];
                assert!(addr < 0x20); // Address is only 5 bits

                gpio::output_to_gpio(&mut *gpio, addr, data);
            }
        }

        if debug {
            println!("Frame: {:?}", buf);
        }

        buf.clear();
    }
}
