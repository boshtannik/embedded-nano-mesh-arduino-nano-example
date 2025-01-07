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
        device_address: ExactAddressType::new(1).unwrap(),
        listen_period: 250 as ms,
    });

    let mut next_send = Atmega328pMillis::millis();
    let mut count: u64 = u64::default();

    loop {
        let current_time = Atmega328pMillis::millis();

        if current_time > next_send {
            let mut message = NodeString::new();
            let _ = message.push_str("Message # ");
            let _ = message.push_str(&NodeString::try_from(count).unwrap_or_default());
            let _ = message.push_str("\n");

            match mesh_node.send_to_exact(
                message.into_bytes(),              // Content.
                ExactAddressType::new(2).unwrap(), // Send to device with address 2.
                10 as LifeTimeType, // Let message travel 10 devices before being destroyed.
                true,
            ) {
                Ok(()) => (),
                Err(SendError::SendingQueueIsFull) => (),
            }
            next_send = current_time + 600 as ms;
            count += 1;
        }
        let _ = mesh_node.update(&mut interface_driver, current_time);
    }
}
