#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::default_serial;
use embedded_nano_mesh::{
    ExactAddressType, LifeTimeType, Node, NodeConfig, NodeString, SendError,
};
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
        device_address: ExactAddressType::new(1).unwrap(),
        listen_period: 150 as ms,
    });

    match mesh_node.send_to_exact(
        NodeString::from("This is the message to be sent").into_bytes(),
        ExactAddressType::new(2).unwrap(),
        10 as LifeTimeType,
        true,
    ) {
        Ok(()) => {
            ufmt::uwriteln!(&mut ArduinoNanoSerial::default(), "Packet sent").unwrap();
        }
        Err(SendError::SendingQueueIsFull) => {
            ufmt::uwriteln!(&mut ArduinoNanoSerial::default(), "SendingQueueIsFull").unwrap();
        }
    }

    loop {
        let _ = mesh_node.update::<Atmega328pMillis, ArduinoNanoSerial>();
    }
}
