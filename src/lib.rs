// src/lib.rs
#![cfg_attr(not(test), no_std)] // No_std for the library itself
#![cfg_attr(not(feature = "std"), no_std)] // Allow std for tests or specific features

// Common error type and helper enums can live at the crate root
// or be moved into driver_core.rs and re-exported. Let's define them here for now.

use thiserror::Error; // Assuming "std" feature for thiserror for now for simplicity.
// If strictly no-alloc, error definition will be simpler.

#[derive(Debug, Error)]
pub enum AxpError<I2cErr> {
    #[error("I2C communication error: {0:?}")]
    I2c(I2cErr),
    #[error("Invalid voltage value: {0}mV")]
    InvalidVoltage(u16),
    #[error("Invalid current value: {0}mA")]
    InvalidCurrent(u16),
    #[error("Invalid parameter for GPIO {pin:?}: mode {mode:?}")]
    InvalidGpioParameter { pin: GpioPin, mode: GpioMode },
    #[error("Feature not available for specified GPIO")]
    GpioFeatureUnavailable,
    #[error("Functionality not yet implemented: {0}")]
    NotImplemented(&'static str),
}

// Helper Enums (can also be in driver_core.rs and re-exported)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DcId {
    Dcdc1,
    Dcdc3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LdoId {
    Ldo2,
    Ldo3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioPin {
    Gpio0,
    Gpio1,
    Gpio2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GpioMode {
    NmosOpenDrainOutput = 0b000,
    UniversalInput = 0b001,
    PwmOutput = 0b010,
    LowNoiseLdo = 0b010, // For GPIO0
    AdcInput = 0b100,
    LowOutput = 0b101,
    Floating = 0b110,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChargeTargetVoltage {
    V4_10 = 0b00,
    V4_15 = 0b01,
    V4_20 = 0b10,
    V4_36 = 0b11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PekBootTime {
    S128ms = 0b00,
    S512ms = 0b01,
    S1 = 0b10,
    S2 = 0b11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PekLongPressTime {
    Ms1000 = 0b00,
    Ms1500 = 0b01,
    Ms2000 = 0b10,
    Ms2500 = 0b11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PekShutdownDuration {
    S4 = 0b00,
    S6 = 0b01,
    S8 = 0b10,
    S10 = 0b11,
}

// --- bisync module structure ---

// This defines the `asynchronous` module.
// Code using this path will use async/await.
#[path = "driver_core.rs"]
pub mod asynchronous {
    use bisync::asynchronous::*; // Pulls in async-specific bisync items
    // Re-export items from the inner module.
    // The `mod driver_core;` declaration makes items from driver_core.rs available under `driver_core::`.
    // `pub use driver_core::*;` makes them directly available under `asynchronous::`.
    mod driver_core;
    pub use driver_core::*;
}

// This defines the `blocking` module.
// Code using this path will have async/await stripped.
#[path = "driver_core.rs"]
pub mod blocking {
    use bisync::synchronous::*; // Pulls in sync-specific bisync items
    mod driver_core;
    pub use driver_core::*;
}

// Remove the placeholder function
// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

#[cfg(test)]
mod tests {
    // Tests will need to choose sync or async path.
    // Example for async test (needs a runtime like tokio::test):
    // #[tokio::test]
    // async fn async_it_works() {
    //     // use crate::asynchronous::Axp192;
    //     // ... setup mock async I2C ...
    //     // let mut pmu = Axp192::new(mock_i2c);
    //     // assert!(pmu.get_battery_voltage_mv().await.is_ok());
    // }

    // Example for blocking test:
    #[test]
    fn blocking_it_works() {
        // use crate::blocking::Axp192;
        // ... setup mock blocking I2C ...
        // let mut pmu = Axp192::new(mock_i2c);
        // assert!(pmu.get_battery_voltage_mv().is_ok());
        assert_eq!(2 + 2, 4); // Placeholder
    }
}
