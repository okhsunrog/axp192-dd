// src/lib.rs
#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[macro_use]
mod fmt; // For info!, debug! etc.

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

/// Asynchronous version of the AXP192 driver.
#[cfg(feature = "async")]
#[path = "."] // Indicates that submodules declared here look for files in `src/`
pub mod asynchronous {
    #[doc(hidden)]
    pub use bisync::asynchronous::*;

    // This will look for `src/driver_core.rs` because of `#[path = "."]` on the parent `asynchronous` module.
    mod driver_core;
    pub use driver_core::*;
}
// Optional: Re-export the main driver struct for convenience if desired
#[cfg(feature = "async")]
pub use asynchronous::Axp192 as Axp192Async;

/// Blocking (synchronous) version of the AXP192 driver.
#[cfg(feature = "blocking")]
#[path = "."] // Indicates that submodules declared here look for files in `src/`
pub mod blocking {
    #[doc(hidden)]
    pub use bisync::synchronous::*;

    // This will also look for `src/driver_core.rs`.
    // `bisync` handles the fact that the same file is compiled under two different `super` contexts.
    #[allow(clippy::duplicate_mod)] // Allow the same module name if clippy complains
    mod driver_core;
    pub use driver_core::*;
}
// Optional: Re-export the main driver struct for convenience
#[cfg(feature = "blocking")]
pub use blocking::Axp192 as Axp192Blocking;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works_placeholder() {
        assert_eq!(2 + 2, 4);
    }
}
