#![no_std]
#![no_main]
use arduino_hal::{
    hal::port,
    pac::USART0,
    port::{mode, Pin},
    Usart,
};
pub use embedded_serial::{MutBlockingTx, MutNonBlockingRx};

pub struct ArduinoNanoIO {
    pub usart: Usart<USART0, Pin<mode::Input, port::PD0>, Pin<mode::Output, port::PD1>>,
}

impl MutNonBlockingRx for ArduinoNanoIO {
    type Error = ();
    fn getc_try(&mut self) -> Result<Option<u8>, Self::Error> {
        match embedded_hal::serial::Read::read(&mut self.usart) {
            Ok(res) => Ok(Some(res)),
            Err(_) => Ok(None),
        }
    }
}

impl MutBlockingTx for ArduinoNanoIO {
    type Error = ();
    fn putc(&mut self, ch: u8) -> Result<(), Self::Error> {
        let _ = self.usart.write_byte(ch);
        Ok(())
    }
}
