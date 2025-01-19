#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(never_type)]

use embedded_nano_mesh::{ExactAddressType, Node, NodeConfig, NodeString, PacketState};
use embedded_nano_mesh_arduino_nano_io::*;

use panic_halt as _;

use platform_millis_arduino_nano::{init_timer, ms, Atmega328pMillis, PlatformMillis};

use arduino_hal;
use ufmt::uwriteln;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    init_timer(dp.TC0);
    let usart =
        arduino_hal::usart::Usart::new(dp.USART0, pins.d0, pins.d1.into_output(), 9600.into());

    let mut interface_driver = ArduinoNanoIO::new(usart);

    let mut mesh_node = Node::new(NodeConfig {
        device_address: ExactAddressType::new(2).unwrap(),
        listen_period: 200 as ms,
    });

    loop {
        if let Some(packet) = mesh_node.receive() {
            uwriteln!(
                &mut interface_driver,
                "Packet from: {}",
                packet.source_device_identifier
            )
            .unwrap();
            let msg_type = match packet.get_spec_state() {
                PacketState::Normal => "Normal",
                PacketState::Ping => "Ping",
                PacketState::Pong => "Pong",
                PacketState::SendTransaction => "SendTransaction",
                PacketState::InitTransaction => "InitTransaction",
                PacketState::AcceptTransaction => "AcceptTransaction",
                PacketState::FinishTransaction => "FinishTransaction",
            };
            uwriteln!(&mut interface_driver, "Type: {}", msg_type).unwrap();
            uwriteln!(
                &mut interface_driver,
                "Data: {}",
                NodeString::from(packet.data.iter().map(|b| *b as char).collect()).as_str()
            )
            .unwrap();
        }

        let _ = mesh_node.update(&mut interface_driver, Atmega328pMillis::millis());
    }
}
