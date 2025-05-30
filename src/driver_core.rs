// src/driver_core.rs
// no_std attribute is primarily for lib.rs. If this file also needs it:
#![cfg_attr(not(any(test, feature = "std")), no_std)]

use super::*; // This is KEY for bisync. Pulls in bisync::asynchronous::* or ::synchronous::*
use crate::{
    AxpError, GpioMode, /* ... other common enums from lib.rs if needed directly here ... */
    GpioPin,
};

// Logging macros (info!, debug!, etc.) are now globally available from `crate::fmt`
// No explicit `use log::info;` or `use crate::info;` needed if `#[macro_use]` is on `mod fmt`.

use device_driver::Register; // For the R: Register bound

// Generate Axp192LowLevel. This struct will be defined in *this* module's scope.
// Make sure your YAML path is correct relative to the crate root.
device_driver::create_device!(device_name: Axp192LowLevel, manifest: "src/axp192_registers.yaml");

// --- I2C Bus Interface ---
pub struct AxpInterface<I2CBus> {
    i2c_bus: I2CBus,
    device_address: u8,
}

impl<I2CBus> AxpInterface<I2CBus> {
    pub fn new(i2c_bus: I2CBus, device_address: u8) -> Self {
        Self {
            i2c_bus,
            device_address,
        }
    }
}

// Sync impl
#[only_sync]
impl<I2CBus, E> device_driver::RegisterInterface for AxpInterface<I2CBus>
// Corrected trait path
where
    I2CBus: embedded_hal::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
{
    type AddressType = u8;
    type Error = AxpError<E>;

    fn read_register<R: Register<AddressType = Self::AddressType>>(
        &mut self,
        _offset: Option<u8>,
        out: &mut R::Word,
    ) -> Result<(), Self::Error>
    where
        R::Word: AsMut<[u8]> + Default,
    {
        self.i2c_bus
            .write_read(self.device_address, &[R::ADDRESS], out.as_mut())
            .map_err(AxpError::I2c)
    }

    fn write_register<R: Register<AddressType = Self::AddressType>>(
        &mut self,
        _offset: Option<u8>,
        value: &R::Word,
    ) -> Result<(), Self::Error>
    where
        R::Word: AsRef<[u8]>,
    {
        let word_bytes = value.as_ref();
        let mut buffer = [0u8; 1 + R::WORD_SIZE_BYTES];
        buffer[0] = R::ADDRESS;
        buffer[1..1 + word_bytes.len()].copy_from_slice(word_bytes);
        self.i2c_bus
            .write(self.device_address, &buffer[..1 + word_bytes.len()])
            .map_err(AxpError::I2c)
    }
}

// Async impl
#[only_async]
impl<I2CBus, E> device_driver::AsyncRegisterInterface for AxpInterface<I2CBus>
// Corrected trait path
where
    I2CBus: embedded_hal_async::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
{
    type AddressType = u8; // Matches the trait definition
    type Error = AxpError<E>;

    // Use the signature expected by device_driver v1.0.4 AsyncRegisterInterface
    async fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.i2c_bus
            .write_read(self.device_address, &[address], data)
            .await
            .map_err(AxpError::I2c)
    }

    async fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        let mut buffer = [0u8; 1 + 2]; // Max 2 data bytes for AXP192 + address byte
        if data.len() > buffer.len() - 1 {
            // This case should ideally not happen if device-driver passes correct length
            // based on register size_bits. For safety, or return an error.
            return Err(AxpError::NotImplemented(
                "Write data too large for fixed buffer",
            ));
        }
        buffer[0] = address;
        buffer[1..1 + data.len()].copy_from_slice(data);
        self.i2c_bus
            .write(self.device_address, &buffer[..1 + data.len()])
            .await
            .map_err(AxpError::I2c)
    }
}

// --- High-Level Driver ---
pub struct Axp192<I2CImpl> {
    ll: Axp192LowLevel<I2CImpl>, // Axp192LowLevel is defined in this module by create_device!
}

#[only_sync]
impl<I2CBus, E> Axp192<AxpInterface<I2CBus>>
where
    I2CBus: embedded_hal::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
    AxpInterface<I2CBus>: device_driver::RegisterInterface<Error = AxpError<E>, AddressType = u8>,
{
    pub fn new(i2c: I2CBus) -> Self {
        Self {
            ll: Axp192LowLevel::new(AxpInterface::new(i2c, 0x34)),
        }
    }
}

#[only_async]
impl<I2CBus, E> Axp192<AxpInterface<I2CBus>>
where
    I2CBus: embedded_hal_async::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
    AxpInterface<I2CBus>:
        device_driver::AsyncRegisterInterface<Error = AxpError<E>, AddressType = u8>,
{
    pub fn new(i2c: I2CBus) -> Self {
        Self {
            ll: Axp192LowLevel::new(AxpInterface::new(i2c, 0x34)),
        }
    }
}

impl<I2CImpl, I2CBusErr> Axp192<I2CImpl>
where
    I2CImpl: device_driver::RegisterInterface<AddressType = u8, Error = AxpError<I2CBusErr>>
        + device_driver::AsyncRegisterInterface<AddressType = u8, Error = AxpError<I2CBusErr>>,
    I2CBusErr: core::fmt::Debug,
{
    #[bisync]
    pub async fn is_charging(&mut self) -> Result<bool, AxpError<I2CBusErr>> {
        // Use the global logging macros
        trace!("Checking charging status...");
        // The .read().await? will call the appropriate sync/async method on Axp192LowLevel
        // which in turn calls the correct trait method on AxpInterface.
        let power_status_reg = self.ll.power_status().read().await?;
        Ok(power_status_reg.battery_current_direction())
    }
}
