// src/lib.rs
#![cfg_attr(not(any(test, feature = "std")), no_std)]

// If fmt.rs defines exported macros, it should be early.
// If it's just for `format_args!` used locally, its exact position is less critical.
#[macro_use]
pub(crate) mod fmt;

use thiserror::Error;

// HAL trait aliases for clarity
#[cfg(feature = "blocking")]
use embedded_hal::i2c::I2c as HalI2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as HalAsyncI2c;

// Conditionally import the correct bisync attribute macro
#[cfg(all(feature = "async", not(feature = "blocking")))]
use bisync::asynchronous::bisync;
#[cfg(all(feature = "blocking", not(feature = "async")))]
use bisync::synchronous::bisync;
// If both or neither are selected, bisync won't be in scope, leading to errors.
// Your Cargo.toml should ideally enforce mutual exclusivity or have a default.

// --- Generated Device Code ---
// This creates AxpLowLevel struct and all associated enums from device.yaml
device_driver::create_device!(device_name: AxpLowLevel, manifest: "device.yaml");

// --- Public Error Type ---
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

// --- AxpInterface (I2C HAL Abstraction) ---
pub struct AxpInterface<I2CBus> {
    i2c_bus: I2CBus,
    device_address: u8,
}

impl<I2CBus> AxpInterface<I2CBus> {
    pub fn new(i2c_bus: I2CBus, device_address: u8) -> Self {
        Self { i2c_bus, device_address }
    }
}

// --- Synchronous RegisterInterface Implementation ---
#[cfg(feature = "blocking")]
impl<I2CBus, E> device_driver::RegisterInterface for AxpInterface<I2CBus>
where
    I2CBus: HalI2c<Error = E>,
    E: core::fmt::Debug,
{
    type AddressType = u8;
    type Error = AxpError<E>;

    fn read_register(&mut self, address: u8, _size_bits: u32, data: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c_bus.write_read(self.device_address, &[address], data).map_err(AxpError::I2c)
    }

    fn write_register(&mut self, address: u8, _size_bits: u32, data: &[u8]) -> Result<(), Self::Error> {
        let mut buffer = [0u8; 5]; 
        if (1 + data.len()) > buffer.len() {
            return Err(AxpError::NotSupported("Write data length exceeds buffer"));
        }
        buffer[0] = address;
        buffer[1..1 + data.len()].copy_from_slice(data);
        self.i2c_bus.write(self.device_address, &buffer[..1 + data.len()]).map_err(AxpError::I2c)
    }
}

#[cfg(feature = "async")]
impl<I2CBus, E> device_driver::AsyncRegisterInterface for AxpInterface<I2CBus>
where
    I2CBus: HalAsyncI2c<Error = E>,
    E: core::fmt::Debug,
{
    type AddressType = u8;
    type Error = AxpError<E>;

    // Remove explicit 'a lifetime here
    async fn read_register(&mut self, address: u8, _size_bits: u32, data: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c_bus.write_read(self.device_address, &[address], data).await.map_err(AxpError::I2c)
    }

    // Remove explicit 'a lifetime here
    async fn write_register(&mut self, address: u8, _size_bits: u32, data: &[u8]) -> Result<(), Self::Error> {
        let mut buffer = [0u8; 5];
        if (1 + data.len()) > buffer.len() {
            return Err(AxpError::NotSupported("Write data length exceeds buffer"));
        }
        buffer[0] = address;
        buffer[1..1 + data.len()].copy_from_slice(data);
        self.i2c_bus.write(self.device_address, &buffer[..1 + data.len()]).await.map_err(AxpError::I2c)
    }
}

// --- Main Driver Struct ---
// We need to define the struct differently based on the feature to satisfy trait bounds
#[cfg(feature = "blocking")]
pub struct Axp192<I2CImpl: device_driver::RegisterInterface<AddressType = u8, Error = AxpError<I2CBusErr>>, I2CBusErr: core::fmt::Debug = <I2CImpl as device_driver::RegisterInterface>::Error> {
    pub ll: AxpLowLevel<I2CImpl>,
    _marker: core::marker::PhantomData<I2CBusErr>, // To use I2CBusErr in struct definition
}

#[cfg(feature = "async")]
pub struct Axp192<I2CImpl: device_driver::AsyncRegisterInterface<AddressType = u8, Error = AxpError<I2CBusErr>>, I2CBusErr: core::fmt::Debug = <I2CImpl as device_driver::AsyncRegisterInterface>::Error> {
    pub ll: AxpLowLevel<I2CImpl>,
    _marker: core::marker::PhantomData<I2CBusErr>, // To use I2CBusErr in struct definition
}


// --- Constructor(s) ---
#[cfg(feature = "blocking")]
impl<I2CBus, E> Axp192<AxpInterface<I2CBus>, E> // Add E here
where
    I2CBus: HalI2c<Error = E>,
    E: core::fmt::Debug,
{
    pub fn new(i2c: I2CBus) -> Self {
        Self {
            ll: AxpLowLevel::new(AxpInterface::new(i2c, 0x34)),
            _marker: core::marker::PhantomData,
        }
    }
}

#[cfg(feature = "async")]
impl<I2CBus, E> Axp192<AxpInterface<I2CBus>, E> // Add E here
where
    I2CBus: HalAsyncI2c<Error = E>,
    E: core::fmt::Debug,
{
    pub fn new(i2c: I2CBus) -> Self {
        Self {
            ll: AxpLowLevel::new(AxpInterface::new(i2c, 0x34)),
            _marker: core::marker::PhantomData,
        }
    }
}

// --- Trait alias for CurrentAxpDriverInterface ---
// This will be used in the `impl` block for high-level methods
#[cfg(feature = "blocking")]
pub trait CurrentAxpDriverInterface<E>: device_driver::RegisterInterface<AddressType = u8, Error = AxpError<E>> {} // Added pub
#[cfg(feature = "blocking")]
impl<T, E> CurrentAxpDriverInterface<E> for T where T: device_driver::RegisterInterface<AddressType = u8, Error = AxpError<E>>, E: core::fmt::Debug {}

#[cfg(feature = "async")]
pub trait CurrentAxpDriverInterface<E>: device_driver::AsyncRegisterInterface<AddressType = u8, Error = AxpError<E>> {} // Added pub
#[cfg(feature = "async")]
impl<T, E> CurrentAxpDriverInterface<E> for T where T: device_driver::AsyncRegisterInterface<AddressType = u8, Error = AxpError<E>>, E: core::fmt::Debug {}

// --- High-Level API Implementation (Initially Empty) ---
impl<I2CImpl, I2CBusErr> Axp192<I2CImpl, I2CBusErr> // Add I2CBusErr here
where
    I2CImpl: CurrentAxpDriverInterface<I2CBusErr>, // Use the alias
    I2CBusErr: core::fmt::Debug,
{
    // We will add high-level methods here one by one.
    // For example:
    // #[bisync]
    // pub async fn get_chip_id(&mut self) -> Result<u8, AxpError<I2CBusErr>> {
    //     let status = self.ll.chip_id().read_fieldset().await?; // Assuming ChipId register exists
    //     Ok(status.value()) // Assuming a value field
    // }
}

// Ensure features are mutually exclusive in Cargo.toml or provide a default
#[cfg(all(feature = "async", feature = "blocking"))]
compile_error!("Features 'async' and 'blocking' should be mutually exclusive.");

#[cfg(not(any(feature = "async", feature = "blocking")))]
compile_error!("Either the 'async' or 'blocking' feature must be enabled.");