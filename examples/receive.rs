#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::default_serial;
use embedded_nano_mesh::{ExactAddressType, Node, NodeConfig};
use panic_halt as _;

use platform_millis_arduino_nano::{init_timer, ms, Atmega328pMillis};
use platform_serial_arduino_nano::{init_serial, ArduinoNanoSerial};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    init_timer(dp.TC0);
    init_serial(default_serial!(dp, pins, 9600));

    let mut mesh_node = Node::new(NodeConfig {
        device_address: ExactAddressType::new(2).unwrap(),
        listen_period: 150 as ms,
    });

    loop {
        let _ = mesh_node.update::<Atmega328pMillis, ArduinoNanoSerial>();
        if let Some(packet) = mesh_node.receive() {
            ufmt::uwriteln!(
                &mut ArduinoNanoSerial::default(),
                "Packet from: {}",
                packet.source_device_identifier
            )
            .unwrap();

            for character in packet.data {
                let character = char::from(character);
                ufmt::uwrite!(&mut ArduinoNanoSerial::default(), "{}", character).unwrap();
            }
            ufmt::uwriteln!(&mut ArduinoNanoSerial::default(), "").unwrap();
        }
    }
}
