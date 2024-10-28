#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32_hal2::{
    clocks::Clocks,
    gpio::{Pin, PinMode, Port}
};

#[entry]
fn main() -> ! {
    // CPU Core peripherals 
    let cp = cortex_m::Peripherals::take().unwrap();

    // Configure clocks
    let clock_cfg = Clocks::default();
    clock_cfg.setup().unwrap();

    // System timer as a delay provider
    let mut delay = Delay::new(cp.SYST, clock_cfg.systick());

    // RGB LED GPIOs
    let mut led_red = Pin::new(Port::C, 6, PinMode::Output);
    let mut led_green = Pin::new(Port::C, 8, PinMode::Output);
    let mut led_blue = Pin::new(Port::E, 0, PinMode::Output);

    // Loop the led show as long as the device is powered on
    loop {
        // Start the show
        led_red.set_low();
        led_green.set_low();
        led_blue.set_low();

        delay.delay_ms(1_000);

        // Turn on red
        led_red.set_high();
        led_green.set_low();
        led_blue.set_low();

        delay.delay_ms(1_000);

        // Turn on green
        led_red.set_low();
        led_green.set_high();
        led_blue.set_low();

        delay.delay_ms(1_000);

        // Turn on blue
        led_red.set_low();
        led_green.set_low();
        led_blue.set_high();

        delay.delay_ms(1_000);

        // Turn on yellow (red + green)
        led_red.set_high();
        led_green.set_high();
        led_blue.set_low();

        delay.delay_ms(1_000);

        // Turn on cyan (green + blue)
        led_red.set_low();
        led_green.set_high();
        led_blue.set_high();

        delay.delay_ms(1_000);

        // Turn on magenta (red + blue)
        led_red.set_high();
        led_green.set_low();
        led_blue.set_high();

        delay.delay_ms(1_000);

        // Turn on white (red + green + blue)
        led_red.set_high();
        led_green.set_high();
        led_blue.set_high();

        delay.delay_ms(1_000);
    }
}