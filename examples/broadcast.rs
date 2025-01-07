#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(never_type)]

use arduino_hal::{
    hal::port,
    pac::USART0,
    port::{mode, Pin},
    Usart,
};
use embedded_nano_mesh::{ExactAddressType, LifeTimeType, Node, NodeConfig, NodeString, SendError};
use panic_halt as _;

use platform_millis_arduino_nano::{init_timer, ms, Atmega328pMillis, PlatformMillis};

use arduino_hal;

struct ArduinoNanoIO {
    usart: Usart<USART0, Pin<mode::Input, port::PD0>, Pin<mode::Output, port::PD1>>,
}

use embedded_serial::{MutBlockingTx, MutNonBlockingRx};

impl MutNonBlockingRx for ArduinoNanoIO {
    type Error = !;
    fn getc_try(&mut self) -> Result<Option<u8>, Self::Error> {
        match embedded_hal::serial::Read::read(&mut self.usart) {
            Ok(res) => Ok(Some(res)),
            Err(_) => Ok(None),
        }
    }
}

impl MutBlockingTx for ArduinoNanoIO {
    /// This shall be the blocking one
    type Error = !;
    fn putc(&mut self, ch: u8) -> Result<(), Self::Error> {
        let _ = self.usart.write_byte(ch);
        Ok(())
    }
}

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
        listen_period: 150 as ms,
    });

    let mut next_send = Atmega328pMillis::millis();

    loop {
        let current_time = Atmega328pMillis::millis();
        if current_time > next_send {
            let _ = match mesh_node.broadcast(
                NodeString::from_iter("This is the message to be broadcasted".chars()).into_bytes(),
                10 as LifeTimeType,
            ) {
                Ok(_) => interface_driver
                    .puts("Packet broadcasted\n")
                    .unwrap_or_default(),
                Err(SendError::SendingQueueIsFull) => interface_driver
                    .puts("SendingQueueIsFull\n")
                    .unwrap_or_default(),
            };

            next_send = current_time + 1000 as ms;
        }
        let _ = mesh_node.update(&mut interface_driver, current_time);
    }
}
