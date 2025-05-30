// src/driver_core.rs

use super::*; // Pulls in bisync::asynchronous::* or ::synchronous::* items
use crate::{AxpError /* Other enums from lib.rs */};

// Logging macros (e.g., trace!, info!) are globally available from `crate::fmt`
// No `use crate::trace` etc. needed here.

// NO `use device_driver::[ll::]register::Register;` needed here for these interface impls.

device_driver::create_device!(device_name: Device, manifest: "device.yaml");

// --- I2C Bus Interface (AxpInterface) ---
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

// --- Synchronous RegisterInterface Implementation ---
#[only_sync]
impl<I2CBus, E> device_driver::RegisterInterface for AxpInterface<I2CBus>
where
    I2CBus: embedded_hal::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
{
    type AddressType = u8;
    type Error = AxpError<E>;

    fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        // `data.len()` implies the number of bytes to read for this register.
        // `_size_bits` could be used for an assertion: assert_eq!(_size_bits / 8, data.len() as u32);
        self.i2c_bus
            .write_read(self.device_address, &[address], data)
            .map_err(AxpError::I2c)
    }

    fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        // `data.len()` is the number of bytes for this register.
        // assert_eq!(_size_bits / 8, data.len() as u32);
        if data.len() > 2 {
            return Err(AxpError::NotImplemented(
                "Write data too large for AXP192 register fixed buffer",
            ));
        }
        let mut buffer = [0u8; 1 + 2]; // ADDR_BYTE + MAX_DATA_BYTES (2 for AXP192 registers)
        buffer[0] = address;
        buffer[1..1 + data.len()].copy_from_slice(data);
        self.i2c_bus
            .write(self.device_address, &buffer[..1 + data.len()])
            .map_err(AxpError::I2c)
    }
}

// --- Asynchronous AsyncRegisterInterface Implementation ---
#[only_async]
impl<I2CBus, E> device_driver::AsyncRegisterInterface for AxpInterface<I2CBus>
where
    I2CBus: embedded_hal_async::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
{
    type AddressType = u8;
    type Error = AxpError<E>;

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
        if data.len() > 2 {
            return Err(AxpError::NotImplemented(
                "Write data too large for AXP192 register fixed buffer",
            ));
        }
        let mut buffer = [0u8; 1 + 2];
        buffer[0] = address;
        buffer[1..1 + data.len()].copy_from_slice(data);
        self.i2c_bus
            .write(self.device_address, &buffer[..1 + data.len()])
            .await
            .map_err(AxpError::I2c)
    }
}

// --- High-Level Driver Struct ---
pub struct Axp192<I2CImpl> {
    ll: Device<I2CImpl>,
}

#[only_sync]
impl<I2CBus, E> Axp192<AxpInterface<I2CBus>>
where
    I2CBus: embedded_hal::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
    AxpInterface<I2CBus>: device_driver::RegisterInterface<Error = AxpError<E>, AddressType = u8>,
{
    pub fn new(i2c_bus: I2CBus) -> Self {
        Self {
            ll: Device::new(AxpInterface::new(i2c_bus, 0x34)),
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
    pub fn new(i2c_bus: I2CBus) -> Self {
        Self {
            ll: Device::new(AxpInterface::new(i2c_bus, 0x34)),
        }
    }
}

// --- Common High-Level Methods (transformed by bisync) ---
impl<I2CImpl, I2CBusErr> Axp192<I2CImpl>
where
    I2CImpl: device_driver::RegisterInterface<AddressType = u8, Error = AxpError<I2CBusErr>>
        + device_driver::AsyncRegisterInterface<AddressType = u8, Error = AxpError<I2CBusErr>>,
    I2CBusErr: core::fmt::Debug,
{
    #[bisync]
    pub async fn is_charging(&mut self) -> Result<bool, AxpError<I2CBusErr>> {
        trace!("Checking charging status..."); // Macro from crate::fmt

        // The generated `self.ll.power_status()` returns a `RegisterOperation`.
        // This `RegisterOperation` has `.read()` and `.read_async()` methods.
        // These methods internally call the `RegisterInterface::read_register` or
        // `AsyncRegisterInterface::read_register` that we implemented on `AxpInterface`.
        let power_status_fieldset = if ASYNC {
            // bisync constant
            self.ll.power_status().read_async().await?
        } else {
            // SYNC path
            self.ll.power_status().read()?
        };
        // `power_status_fieldset` is now the generated struct for the `PowerStatus` register's fields.
        Ok(power_status_fieldset.battery_current_direction()) // Accessing a field from the fieldset
    }

    #[bisync]
    pub async fn get_battery_voltage_mv(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        trace!("Reading battery voltage...");
        let adc_fieldset = if ASYNC {
            self.ll.battery_voltage_adc().read_async().await?
        } else {
            self.ll.battery_voltage_adc().read()?
        };
        let raw_adc = adc_fieldset.raw(); // Accessing the 'raw' field from BatteryVoltageAdc fieldset
        let val_12bit = raw_adc >> 4;
        Ok(val_12bit as f32 * 1.1)
    }
}
