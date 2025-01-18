use arduino_hal::{
    hal::port,
    pac::USART0,
    port::{mode, Pin},
    Usart,
};

pub struct ArduinoNanoIO {
    pub usart: Usart<USART0, Pin<mode::Input, port::PD0>, Pin<mode::Output, port::PD1>>,
}

#[derive(Debug)]
pub struct Error;

impl embedded_hal_nb::serial::Error for Error {
    fn kind(&self) -> embedded_hal_nb::serial::ErrorKind {
        embedded_hal_nb::serial::ErrorKind::Other
    }
}

impl embedded_hal_nb::serial::ErrorType for ArduinoNanoIO {
    type Error = Error;
}

impl embedded_hal_nb::serial::Read<u8> for ArduinoNanoIO {
    fn read(&mut self) -> embedded_hal_nb::nb::Result<u8, Self::Error> {
        match embedded_hal::serial::Read::read(&mut self.usart) {
            Ok(b) => embedded_hal_nb::nb::Result::Ok(b),
            Err(_) => embedded_hal_nb::nb::Result::Err(embedded_hal_nb::nb::Error::Other(Error)),
        }
    }
}

impl embedded_hal_nb::serial::Write<u8> for ArduinoNanoIO {
    fn write(&mut self, byte: u8) -> embedded_hal_nb::nb::Result<(), Self::Error> {
        match embedded_hal::serial::Write::write(&mut self.usart, byte) {
            Ok(()) => embedded_hal_nb::nb::Result::Ok(()),
            Err(_) => embedded_hal_nb::nb::Result::Err(embedded_hal_nb::nb::Error::Other(Error)),
        }
    }

    fn flush(&mut self) -> embedded_hal_nb::nb::Result<(), Self::Error> {
        match embedded_hal::serial::Write::flush(&mut self.usart) {
            Ok(()) => embedded_hal_nb::nb::Result::Ok(()),
            Err(_) => embedded_hal_nb::nb::Result::Err(embedded_hal_nb::nb::Error::Other(Error)),
        }
    }
}

impl ufmt::uWrite for ArduinoNanoIO {
    type Error = Error;

    /// Writes a string slice into this writer, returning whether the write succeeded.
    ///
    /// This method can only succeed if the entire string slice was successfully written, and this
    /// method will not return until all data has been written or an error occurs.
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        for c in s.chars() {
            if let Err(_) = embedded_hal_nb::serial::Write::write(self, c as u8) {
                return Err(Error);
            }
        }
        Ok(())
    }
}
