#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(never_type)]

mod serial_driver;
use serial_driver::*;

use embedded_nano_mesh::{
    ExactAddressType, LifeTimeType, Node, NodeConfig, NodeString, SpecialSendError,
};
use panic_halt as _;

use platform_millis_arduino_nano::{init_timer, ms, Atmega328pMillis, PlatformMillis};

use arduino_hal;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    init_timer(dp.TC0);
    let usart =
        arduino_hal::usart::Usart::new(dp.USART0, pins.d0, pins.d1.into_output(), 9600.into());

    let mut interface_driver = ArduinoNanoIO { usart };

    let mut mesh_node = Node::new(NodeConfig {
        device_address: ExactAddressType::new(1).unwrap(),
        listen_period: 250 as ms,
    });

    match mesh_node.send_ping_pong(
        NodeString::from_iter("This is the message to be sent".chars()).into_bytes(), // Content.
        ExactAddressType::new(2).unwrap(), // Send to device with address 2.
        10 as LifeTimeType,                // Let message travel 10 devices before being destroyed.
        2000 as ms,
        || Atmega328pMillis::millis(),
        &mut interface_driver,
    ) {
        Ok(()) => interface_driver.puts("Packet sent, pong caught.").unwrap(),
        Err(SpecialSendError::SendingQueueIsFull) => {
            interface_driver.puts("SendingQueueIsFull").unwrap()
        }
        Err(SpecialSendError::Timeout) => interface_driver.puts("Timeout").unwrap(),
    };
    loop {
        let _ = mesh_node.update(&mut interface_driver, Atmega328pMillis::millis());
    }
}
