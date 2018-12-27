use rppal::gpio::{Gpio, Level, Mode};
use std::thread;
use std::time::Duration;

const ADDR_DATA: u8 = 10;
const DATA: u8 = 24;
const SHIFT_CLOCK: u8 = 16;
const SHIFT_LATCH: u8 = 12;
const SHIFT_MR: u8 = 9;
const SID_CLOCK: u8 = 17; // TODO: Must be changed to 18 as PWM only supports GPIO pins 18 and 19!
const SID_CS: u8 = 4;
const SID_RESET: u8 = 23;

#[inline(always)]
fn get_level_for_bit(value: u8, bit: u8) -> Level {
    if value & (1 << bit) == 0 {
        Level::Low
    } else {
        Level::High
    }
}

pub fn output_to_gpio(gpio: &mut Gpio, addr: u8, data: u8) {
    for i in (0..8).rev() {
        let addr_level = get_level_for_bit(addr, i);
        let data_level = get_level_for_bit(data, i);
        gpio.write(ADDR_DATA, addr_level);
        gpio.write(DATA, data_level);

        // Pulse clock to write a bit into the shift register
        gpio.write(SHIFT_CLOCK, Level::High);
        gpio.write(SHIFT_CLOCK, Level::Low);
    }

    // Pulse latch to set the shift register output
    gpio.write(SHIFT_LATCH, Level::High);
    gpio.write(SHIFT_LATCH, Level::Low);

    output_to_6581(gpio);
}

fn output_to_6581(gpio: &mut Gpio) {
    gpio.write(SID_CS, Level::Low);
    thread::sleep(Duration::from_micros(2));
    gpio.write(SID_CS, Level::High);
}

fn reset_6581(gpio: &mut Gpio) {
    gpio.write(SID_RESET, Level::Low);
    thread::sleep(Duration::from_millis(1));
    gpio.write(SID_RESET, Level::High);

    // TODO: Check 6581 pin 7!
    gpio.write(SID_CS, Level::High);
}

pub fn init_gpio() -> rppal::gpio::Result<Gpio> {
    let mut gpio = Gpio::new()?;

    // Set up output pins
    gpio.set_mode(ADDR_DATA, Mode::Output);
    gpio.set_mode(DATA, Mode::Output);
    gpio.set_mode(SHIFT_CLOCK, Mode::Output);
    gpio.set_mode(SHIFT_LATCH, Mode::Output);
    gpio.set_mode(SHIFT_MR, Mode::Output);
    gpio.set_mode(SID_CLOCK, Mode::Output);
    gpio.set_mode(SID_CS, Mode::Output);
    gpio.set_mode(SID_RESET, Mode::Output);

    reset_6581(&mut gpio);

    gpio.write(SHIFT_MR, Level::Low);
    gpio.write(SHIFT_CLOCK, Level::Low);
    gpio.write(SHIFT_LATCH, Level::Low);

    gpio.write(SHIFT_MR, Level::High);

    Ok(gpio)
}
