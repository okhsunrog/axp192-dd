#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[macro_use]
pub(crate) mod fmt;

#[path = "."]
mod asynchronous {
    use bisync::asynchronous::*;
    use device_driver::AsyncRegisterInterface as RegisterInterface;
    use embedded_hal_async::i2c::I2c;
    mod driver;
    pub use driver::*;
}
// pub use asynchronous::Apa102Writer as Apa102WriterAsync;

#[path = "."]
mod blocking {
    use bisync::synchronous::*;
    use device_driver::RegisterInterface;
    use embedded_hal::i2c::I2c;
    #[allow(clippy::duplicate_mod)]
    mod driver;
    pub use driver::*;
}
// pub use blocking::Apa102Writer;
