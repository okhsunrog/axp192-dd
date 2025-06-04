#![cfg_attr(not(any(test, feature = "std")), no_std)]
//! # AXP192 Power Management Chip Interface
//!
//! This crate provides a bisync-based driver for the AXP192 power management IC,
//! built upon the `device-driver` crate for robust, declarative register
//! definitions via a YAML manifest. It supports both asynchronous (`async`)
//! and blocking operation through a unified API, using the [`bisync`](https://docs.rs/bisync) crate
//! for seamless compatibility with both `embedded-hal` and `embedded-hal-async` traits.
//!
//! ## Features
//!
//! *   **Declarative Register Map:** Full device configuration defined in `device.yaml`.
//! *   **Unified Async/Blocking Support:** Write your code once and use it in both async and blocking contexts via bisync.
//! *   **Type-Safe API:** High-level functions for common operations (e.g., setting voltages)
//!     and a generated low-level API (`ll`) for direct register access.
//! *   **Comprehensive Register Coverage:** Aims to support the full feature set of the AXP192.
//! *   **`defmt` and `log` Integration:** Optional support for logging and debugging.
//!
//! ## Getting Started
//!
//! To use the driver, instantiate `Axp192` (blocking) or `Axp192Async` (async) with your I2C bus implementation:
//!
//! ```rust,no_run
//! # use embedded_hal::i2c::I2c;
//! # use axp192_dd::{Axp192, DcId};
//! let i2c_bus = todo!();
//! let mut axp = Axp192::new(i2c_bus);
//!
//! axp.set_dcdc_voltage(DcId::Dcdc1, 3300)?;
//! # Ok(())
//! ```
//!
//! For async environments, use `Axp192Async` (re-exported from the `asynchronous` module):
//!
//! ```rust,no_run
//! # use embedded_hal_async::i2c::I2c;
//! # use axp192_dd::{Axp192Async, DcId};
//! let i2c_bus = todo!();
//! let mut axp = Axp192Async::new(i2c_bus);
//!
//! axp.set_dcdc_voltage(DcId::Dcdc1, 3300).await?;
//! # Ok(())
//! ```
//!
//! For a detailed register map, please refer to the `device.yaml` file in the
//! [repository](https://github.com/okhsunrog/axp192-dd).
//!
//! ## Supported Devices
//!
//! The AXP192 is found in various embedded devices, including but not limited to:
//!
//!  * [M5Stack Core 2](https://docs.m5stack.com/en/core/core2) (including the [Core 2 for
//!    AWS](https://docs.m5stack.com/en/core/core2_for_aws) variant)
//!  * [M5Stack Tough](https://docs.m5stack.com/en/core/tough)
//!  * [M5StickC](https://docs.m5stack.com/en/core/m5stickc)
//!  * [M5StickC PLUS](https://docs.m5stack.com/en/core/m5stickc_plus)
//!
//! ## Warning!
//!
//! ***Caution!*** This chip controls power to the microcontroller and other critical
//! components. Incorrect configuration can potentially damage or brick your device.
//! Proceed with care and always consult the AXP192 datasheet.
//!
//! ## Datasheet
//!
//! [AXP192 Datasheet v1.1 (English Draft)](https://github.com/m5stack/M5-Schematic/blob/master/Core/AXP192%20Datasheet_v1.1_en_draft_2211.pdf)
//!

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
