#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[macro_use]
pub(crate) mod fmt;
mod adc_helpers;

use thiserror::Error;

device_driver::create_device!(device_name: AxpLowLevel, manifest: "device.yaml");
pub const AXP192_I2C_ADDRESS: u8 = 0x34;

#[derive(Debug, Error)]
pub enum AxpError<I2cErr> {
    #[error("I2C error")]
    I2c(I2cErr),
    #[error("Invalid voltage: {0}mV for setting")]
    InvalidVoltage(u16),
    #[error("Invalid current: {0}mA for setting")]
    InvalidCurrent(u16),
    #[error("Feature or specific mode not supported/implemented: {0}")]
    NotSupported(&'static str),
}

#[cfg(feature = "defmt")]
impl<I2cErr> defmt::Format for AxpError<I2cErr> {
    fn format(&self, f: defmt::Formatter) {
        match self {
            AxpError::I2c(_) => defmt::write!(f, "E:I2C"),
            AxpError::InvalidVoltage(v) => defmt::write!(f, "E:V_set({}mV)", v),
            AxpError::InvalidCurrent(c) => defmt::write!(f, "E:I_set({}mA)", c),
            AxpError::NotSupported(s) => defmt::write!(f, "E:NoSupp({=str})", s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcId {
    Dcdc1,
    Dcdc2,
    Dcdc3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LdoId {
    // Ldo1 is not configurable, set to 1.3V in hardware.
    Ldo2,
    Ldo3,
}

pub struct AxpInterface<I2CBus> {
    i2c_bus: I2CBus,
}

impl<I2CBus> AxpInterface<I2CBus> {
    pub fn new(i2c_bus: I2CBus) -> Self {
        Self { i2c_bus }
    }
}

#[path = "."]
mod asynchronous {
    use bisync::asynchronous::*;
    use device_driver::AsyncRegisterInterface as RegisterInterface;
    use embedded_hal_async::i2c::I2c;
    mod driver;
    pub use driver::*;
}
pub use asynchronous::Axp192 as Axp192Async;

#[path = "."]
mod blocking {
    use bisync::synchronous::*;
    use device_driver::RegisterInterface;
    use embedded_hal::i2c::I2c;
    #[allow(clippy::duplicate_mod)]
    mod driver;
    pub use driver::*;
}
pub use blocking::Axp192;
