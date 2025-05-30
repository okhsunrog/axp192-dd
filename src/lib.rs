// src/lib.rs
#![cfg_attr(not(any(test, feature = "std")), no_std)]

// This mod MUST go first, so that the others see its macros.
#[macro_use]
pub(crate) mod fmt; // Changed to pub(crate) as per usbpd example

use thiserror::Error;

// --- Public Error Type ---
#[derive(Debug, Error)]
pub enum AxpError<I2cErr> {
    #[error("I2C error")]
    I2c(I2cErr),
    #[error("Invalid voltage")]
    InvalidVoltage(u16),
    #[error("Invalid current")]
    InvalidCurrent(u16),
    #[error("Invalid GPIO param")]
    InvalidGpioParameter { pin: GpioPin, mode: GpioMode },
    #[error("GPIO unavailable")]
    GpioFeatureUnavailable,
    #[error("Not implemented")]
    NotImplemented(&'static str),
}

#[cfg(feature = "defmt")]
impl<I2cErr> defmt::Format for AxpError<I2cErr> {
    fn format(&self, f: defmt::Formatter) {
        match self {
            AxpError::I2c(_) => defmt::write!(f, "E:I2C"),
            AxpError::InvalidVoltage(v) => defmt::write!(f, "E:V({}mV)", v),
            AxpError::InvalidCurrent(c) => defmt::write!(f, "E:I({}mA)", c),
            AxpError::InvalidGpioParameter { .. } => defmt::write!(f, "E:GPIOParam"),
            AxpError::GpioFeatureUnavailable => defmt::write!(f, "E:GPIOFeat"),
            AxpError::NotImplemented(s) => defmt::write!(f, "E:NoImpl({=str})", s),
        }
    }
}

// --- Public Helper Enums ---
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcId {
    Dcdc1,
    Dcdc3,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LdoId {
    Ldo2,
    Ldo3,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioPin {
    Gpio0,
    Gpio1,
    Gpio2,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum GpioMode {
    NmosOpenDrainOutput = 0,
    UniversalInput = 1,
    SpecialOutput010 = 2,
    AdcInput = 4,
    LowOutput = 5,
    Floating = 6,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum ChargeTargetVoltage {
    V4_10 = 0,
    V4_15 = 1,
    V4_20 = 2,
    V4_36 = 3,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum PekBootTime {
    S128ms = 0,
    S512ms = 1,
    S1 = 2,
    S2 = 3,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum PekLongPressTime {
    Ms1000 = 0,
    Ms1500 = 1,
    Ms2000 = 2,
    Ms2500 = 3,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum PekShutdownDuration {
    S4 = 0,
    S6 = 1,
    S8 = 2,
    S10 = 3,
}

// --- bisync Modules ---
#[cfg(feature = "async")]
#[path = "."]
pub mod asynchronous {
    #[doc(hidden)]
    pub use bisync::asynchronous::*;
    mod driver_core; // Will find src/driver_core.rs
    pub use driver_core::*;
}
#[cfg(feature = "async")]
pub use asynchronous::Axp192 as Axp192Async; // Assuming Axp192 is pub in driver_core

#[cfg(feature = "blocking")]
#[path = "."]
pub mod blocking {
    #[doc(hidden)]
    pub use bisync::synchronous::*;
    #[allow(clippy::duplicate_mod)]
    mod driver_core; // Will find src/driver_core.rs
    pub use driver_core::*;
}
#[cfg(feature = "blocking")]
pub use blocking::Axp192 as Axp192Blocking; // Assuming Axp192 is pub in driver_core

#[cfg(test)]
mod tests {
    #[test]
    fn it_works_placeholder() {
        assert_eq!(2 + 2, 4);
    }
}
