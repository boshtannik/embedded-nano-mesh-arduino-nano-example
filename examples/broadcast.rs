#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use embedded_nano_mesh::{ExactAddressType, LifeTimeType, Node, NodeConfig, NodeString, SendError};
use panic_halt as _;
mod serial_driver;

use platform_millis_arduino_nano::{init_timer, ms, Atmega328pMillis, PlatformMillis};

use arduino_hal;
use serial_driver::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    init_timer(dp.TC0);
    let usart =
        arduino_hal::usart::Usart::new(dp.USART0, pins.d0, pins.d1.into_output(), 9600.into());

    let mut interface_driver = serial_driver::ArduinoNanoIO { usart };

    let mut mesh_node = Node::new(NodeConfig {
        device_address: ExactAddressType::new(2).unwrap(),
        listen_period: 150 as ms,
    });

    match mesh_node.broadcast(
        NodeString::from_iter("This is the message to be broadcasted".chars()).into_bytes(),
        10 as LifeTimeType,
    ) {
        Ok(_) => interface_driver.puts("Packet broadcasted\n").unwrap(),
        Err(SendError::SendingQueueIsFull) => {
            interface_driver.puts("SendingQueueIsFull\n").unwrap()
        }
    };
    loop {
        let _ = mesh_node.update(&mut interface_driver, Atmega328pMillis::millis());
    }
}
