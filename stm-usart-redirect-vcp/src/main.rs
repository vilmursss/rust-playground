#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32_hal2::{
    clocks::Clocks,
    gpio::{Pin, PinMode, Port},
    usart::{Usart, UsartConfig},
    pac,
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

    let dp = pac::Peripherals::take().unwrap();

    // Configure GPIO for USART3 (PC10)
    let _tx_pin = Pin::new(Port::C, 10, PinMode::Alt(7));

    // Configure USART3 with default settings
    let usart_config = UsartConfig::default();
    let mut usart3 = Usart::new(dp.USART3, 115200, usart_config, &clock_cfg);

    // Function to write a string to USART3
    fn usart_write_str(usart: &mut Usart<pac::USART3>, s: &str) {
        for byte in s.bytes() {
            usart.write(&[byte]).unwrap();
        }
    }

    // Send initial debug message
    usart_write_str(&mut usart3, "USART3 initialized.\r\n");

    // Main loop
    loop {
        usart_write_str(&mut usart3, "Hello, USART3 message over VCP!\r\n");
        delay.delay_ms(1000);
    }
}