// #![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use hal::{
    gpio::*,
    prelude::*,
    stm32,
    i2c::*,
    delay::Delay,
    timer::Timer,
};
use nb;
use sgp30::*;
use cortex_m_semihosting::{hprintln, hprint};

#[entry]
fn main() -> ! {
    // Peripherals can only be taken once, so there is no sense in checking them.
    let board_peripherals = stm32::Peripherals::take().unwrap();
    let processor_peripherals = cortex_m::peripheral::Peripherals::take().unwrap();
        
    let reset_and_clock_control = board_peripherals.RCC;
    // Setup the system and peripheral clock speeds.
    let clocks = reset_and_clock_control.constrain().
    cfgr.use_hse(8.mhz()).sysclk(72.mhz()).pclk1(36.mhz()).freeze();

    let gpiob = board_peripherals.GPIOB.split();

    let scl = gpiob.pb8.into_alternate_af4_open_drain();
    let sda = gpiob.pb9.into_alternate_af4_open_drain();
    // Create an I2C connection for the SGP30. The speed 100 khz is taken
    // from code examples online, however the data sheet found at: 
    // https://cdn-learn.adafruit.com/assets/assets/000/050/058/original/Sensirion_Gas_Sensors_SGP30_Datasheet_EN.pdf
    // The datasheet shows the SCL clock frequency can be between 0 and 400 kHz.
    let i2c1 = I2c::i2c1(board_peripherals.I2C1, (scl, sda), 100.khz(), clocks);
    
    // Create a delay provider using the System Tick.
    let sgp30_delay = Delay::new(processor_peripherals.SYST, clocks);
    
    // The I2C address is specified as 0x58 in the datasheet.
    // https://cdn-learn.adafruit.com/assets/assets/000/050/058/original/Sensirion_Gas_Sensors_SGP30_Datasheet_EN.pdf
    let mut sgp30_device = Sgp30::new(i2c1, 0x58, sgp30_delay);
    match sgp30_device.init() {
        Ok(_) => {
            hprintln!("Device is intialized").unwrap();
        },
        Err(_) => {
            hprintln!("Device failed to intialized").unwrap();
            panic!();
        }
    }

    // Create a timer using TIM1. This will be used to pull
    // the SGP30's status every second. 
    let mut timer_source = Timer::tim1(board_peripherals.TIM1, 1.hz(), clocks);
    let mut count = 0;
    loop {
        // Wait till the timer is finished hitting 1 second.
        nb::block!(timer_source.wait()).unwrap();

        // The first 15 seconds are used to startup and calibrate the sensor.
        if count < 15 {
            if count == 0 {
                hprint!("Device is starting ").unwrap();
            }
            else if count == 14 {
                hprintln!("Done!").unwrap();
            }
            else {
                hprint!(".").unwrap();
            }
            count += 1;
            continue;
        }
        match sgp30_device.measure() {
            Ok(measurement) => {
                hprintln!("CO₂eq parts per million: {}", measurement.co2eq_ppm).unwrap();
                hprintln!("TVOC parts per billion: {}", measurement.tvoc_ppb).unwrap();
            },
            Err(_) => continue
        }
    }
}