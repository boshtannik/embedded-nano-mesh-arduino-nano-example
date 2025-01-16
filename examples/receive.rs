#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(never_type)]

mod serial_driver;
use embedded_nano_mesh::{ExactAddressType, Node, NodeConfig, NodeString};

use panic_halt as _;
use serial_driver::*;

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
        device_address: ExactAddressType::new(2).unwrap(),
        listen_period: 200 as ms,
    });

    loop {
        if let Some(packet) = mesh_node.receive() {
            let mut msg = NodeString::from_iter("Sender: ".chars());
            let _ = msg.push_str(
                &NodeString::try_from(packet.source_device_identifier)
                    .unwrap_or(NodeString::from_iter("?".chars())),
            );
            let _ = msg.push(' ');

            let _ = interface_driver.puts(&msg);
            let _ = interface_driver.puts(&packet.data);
            let _ = interface_driver.puts("\n");
        }

        let _ = mesh_node.update(&mut interface_driver, Atmega328pMillis::millis());
    }
}
