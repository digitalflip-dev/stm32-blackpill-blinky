#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m::delay::Delay;

use hal::{
    self,
    clocks::{Clocks},
    gpio::{
        Pin,
        PinMode,
        Port
    },
    pac
};

use defmt_rtt as _;
//use panic_halt as _;
use panic_probe as _;

#[entry]
fn main() -> ! {

    // Set up CPU peripherals
    let cp = cortex_m::Peripherals::take().unwrap();

    // Set up microcontroller peripherals
    let _dp = pac::Peripherals::take().unwrap();

    defmt::println!("Hello World");

    let clock_cfg = Clocks::default();
    clock_cfg.setup().unwrap();
    
    // Setup a delay based on Cortex-M systick
    let mut delay = Delay::new(cp.SYST,  clock_cfg.systick());

    let mut red_led = Pin::new(Port::A, 10, PinMode::Output);
    let mut green_led = Pin::new(Port::A, 11, PinMode::Output);
    let mut yellow_led = Pin::new(Port::A, 12, PinMode::Output);
    let mut blue_led = Pin::new(Port::A, 15, PinMode::Output);

    // create a mutable array of LEDs
    let mut leds = [&mut red_led, &mut green_led, &mut yellow_led, &mut blue_led];

    // create a mutable slice, that we can iterate over the array
    let led_array = &mut leds[0..4];

    // Setup leds
    for led in &mut *led_array {
        led.set_low();
    }
    
    loop {

        for led in &mut *led_array {
            led.set_high();
            delay.delay_ms(200);
            led.set_low();
            delay.delay_ms(200);
        }
    }
}

#[defmt::panic_handler]
fn panic() -> ! {
     cortex_m::asm::udf()
}
