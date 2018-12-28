use clap::{App, Arg};
use std::error::Error;
use std::io;
use std::io::{BufRead, BufReader, Write};

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
        .arg(Arg::with_name("Test output").short("t").long("test_output"))
        .get_matches();
    let debug = matches.is_present("Enable debug output");
    let disable_gpio = matches.is_present("Disable GPIO");
    let test_output = matches.is_present("Test output");
    let mut stdin = BufReader::new(io::stdin());
    let mut buf = Vec::with_capacity(64);
    let mut gpio = gpio::init_gpio(disable_gpio).expect("Failed to initialize GPIO");

    // 1mhz clock with 50% duty cycle for 6581
    /*
    let pwm =
        pwm::Pwm::with_frequency(pwm::Channel::Pwm0, 1000000.0, 0.5, pwm::Polarity::Normal, true)
            .unwrap();
    */

    /*
    if pwm.is_err() && !disable_gpio {
        // Error out if gpio is not disabled and PWM setup failed
        let _ = pwm.expect("Failed to set PWM on pin 18");
    }

    let pwm = pwm.unwrap();
    */

    const REPORT_FRAME_INTERVAL: usize = 10;
    let mut frame_count = 0;

    if test_output {
        println!("Test output");
        gpio::output_to_gpio(&mut *gpio, 0x1, 0x10);
        gpio::output_to_gpio(&mut *gpio, 0x5, 0x9);
        gpio::output_to_gpio(&mut *gpio, 0x6, 0x84);
        gpio::output_to_gpio(&mut *gpio, 0x18, 0xF);
        gpio::output_to_gpio(&mut *gpio, 0x4, 0x11);
    } else {
        loop {
            if let Err(e) = stdin.read_until(FRAME_ENDING_BYTE, &mut buf) {
                eprintln!("{}", e.description());
                std::process::exit(1);
            }

            if buf.len() % 2 == 1 {
                // Pop the frame ending byte if the length of
                // the buffer is odd. If it is even, the 0xFF
                // byte does not end the frame, as it is a value
                // for a 6581 register.
                buf.pop();

                frame_count += 1;
                if frame_count % REPORT_FRAME_INTERVAL == 0 {
                    print!("Processed {} frames\r", frame_count);
                    io::stdout().flush().ok().expect("Could not flush stdout");
                }
            }

            if debug {
                println!("Frame: {:?}", buf);
            }

            for chunk in buf.chunks(2) {
                if chunk.len() < 2 {
                    // This should not happen
                    println!("NO DATA FOR ADDR 0x{:2X}", chunk[0]);
                } else {
                    let addr = chunk[0];
                    let data = chunk[1];
                    assert!(addr < 0x20); // Address is only 5 bits

                    gpio::output_to_gpio(&mut *gpio, addr, data);
                }
            }

            buf.clear();
        }
    }
}
