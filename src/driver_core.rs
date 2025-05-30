// src/driver_core.rs
// no_std attribute is in lib.rs

use super::*; // Pulls in bisync::asynchronous::* or ::synchronous::* items
use crate::{
    AxpError, ChargeTargetVoltage, DcId, GpioMode, GpioPin, LdoId, PekBootTime, PekLongPressTime,
    PekShutdownDuration,
};

// Use crate-level logging macros (defined via #[macro_use] mod fmt; in lib.rs)
// No need for `use log::info;` if using `crate::info!` from fmt.rs
use crate::{debug, info, trace, warn};

// Corrected import for Register trait used as a bound
use device_driver::Register; // For R: Register bound, if needed by SYNC interface.
// For the async interface the compiler error points to, it's not used in the signature.

// Generate Axp192LowLevel from YAML
// Ensure "Axp192LowLevel" is used consistently as the device_name.
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

// Sync impl - using the signature style from device_driver 1.0.x RegisterInterface
#[only_sync]
impl<I2CBus, E> device_driver::RegisterInterface for AxpInterface<I2CBus>
where
    I2CBus: embedded_hal::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
{
    type AddressType = u8; // Matches device_driver 1.0.x RegisterInterface
    type Error = AxpError<E>;

    fn read_register<R: Register<AddressType = Self::AddressType>>(
        // Corrected generic bound
        &mut self,
        _offset: Option<u8>,
        out: &mut R::Word,
    ) -> Result<(), Self::Error>
    where
        R::Word: AsMut<[u8]> + Default, // Added Default bound, often needed
    {
        self.i2c_bus
            .write_read(self.device_address, &[R::ADDRESS], out.as_mut())
            .map_err(AxpError::I2c)
    }

    fn write_register<R: Register<AddressType = Self::AddressType>>(
        // Corrected generic bound
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

// Async impl - using the signature from the compiler error for device_driver 1.0.4 AsyncRegisterInterface
#[only_async]
impl<I2CBus, E> device_driver::AsyncRegisterInterface for AxpInterface<I2CBus>
where
    I2CBus: embedded_hal_async::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
{
    type AddressType = u8; // As per compiler error for this trait
    type Error = AxpError<E>;

    async fn read_register(
        // Signature from compiler error
        &mut self,
        address: Self::AddressType,
        _size_bits: u32, // This param is part of the trait, must be present
        data: &mut [u8], // Raw byte slice
    ) -> Result<(), Self::Error> {
        self.i2c_bus
            .write_read(self.device_address, &[address], data)
            .await
            .map_err(AxpError::I2c)
    }

    async fn write_register(
        // Signature from compiler error
        &mut self,
        address: Self::AddressType,
        _size_bits: u32, // This param is part of the trait
        data: &[u8],     // Raw byte slice
    ) -> Result<(), Self::Error> {
        let mut buffer = [0u8; 1 + 2]; // Max 2 bytes for AXP192 registers + address byte
        // This needs to be dynamic or large enough if data len varies.
        // For fixed size write, it's simpler.
        if data.len() > 2 { /* error or panic, AXP192 registers are small */ }
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
    ll: Axp192LowLevel<I2CImpl>, // This should now be found due to create_device! above
}

#[only_sync]
impl<I2CBus, E> Axp192<AxpInterface<I2CBus>>
where
    I2CBus: embedded_hal::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
    AxpInterface<I2CBus>: device_driver::RegisterInterface<Error = AxpError<E>, AddressType = u8>, // AddressType
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
        device_driver::AsyncRegisterInterface<Error = AxpError<E>, AddressType = u8>, // AddressType
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
        + // AddressType
        device_driver::AsyncRegisterInterface<AddressType = u8, Error = AxpError<I2CBusErr>>, // AddressType
    I2CBusErr: core::fmt::Debug,
{
    #[bisync]
    pub async fn get_chip_id_test(&mut self) -> Result<u8, AxpError<I2CBusErr>> {
        // The generated Axp192LowLevel uses the Register/AsyncRegisterInterface traits.
        // If AsyncRegisterInterface takes (address, size, data), then the generated
        // .read().await and .write().await calls from Axp192LowLevel will provide these.
        let reg_value = self.ll.power_status().read().await?; // This calls the appropriate trait method
        Ok(reg_value.into_bytes()[0])
    }

    // Minimal methods to test further
    #[bisync]
    pub async fn is_charging(&mut self) -> Result<bool, AxpError<I2CBusErr>> {
        Ok(self
            .ll
            .power_status()
            .read()
            .await?
            .battery_current_direction())
    }
}
