// src/lib.rs
#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[macro_use]
pub(crate) mod fmt;

use thiserror::Error;

#[cfg(feature = "blocking")]
use embedded_hal::i2c::I2c as HalI2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as HalAsyncI2c;

#[cfg(all(feature = "async", not(feature = "blocking")))]
use bisync::asynchronous::bisync;
#[cfg(all(feature = "blocking", not(feature = "async")))]
use bisync::synchronous::bisync;

device_driver::create_device!(device_name: AxpLowLevel, manifest: "device.yaml");

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

#[cfg(feature = "blocking")]
impl<I2CBus, E> device_driver::RegisterInterface for AxpInterface<I2CBus>
where
    I2CBus: HalI2c<Error = E>,
    E: core::fmt::Debug,
{
    type AddressType = u8;
    type Error = AxpError<E>;
    fn read_register(
        &mut self,
        address: u8,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.i2c_bus
            .write_read(self.device_address, &[address], data)
            .map_err(AxpError::I2c)
    }
    fn write_register(
        &mut self,
        address: u8,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        let mut buffer = [0u8; 5];
        if (1 + data.len()) > buffer.len() {
            return Err(AxpError::NotSupported("Write data length exceeds buffer"));
        }
        buffer[0] = address;
        buffer[1..1 + data.len()].copy_from_slice(data);
        self.i2c_bus
            .write(self.device_address, &buffer[..1 + data.len()])
            .map_err(AxpError::I2c)
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
    async fn read_register(
        &mut self,
        address: u8,
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
        address: u8,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        let mut buffer = [0u8; 5];
        if (1 + data.len()) > buffer.len() {
            return Err(AxpError::NotSupported("Write data length exceeds buffer"));
        }
        buffer[0] = address;
        buffer[1..1 + data.len()].copy_from_slice(data);
        self.i2c_bus
            .write(self.device_address, &buffer[..1 + data.len()])
            .await
            .map_err(AxpError::I2c)
    }
}

#[cfg(feature = "blocking")]
pub struct Axp192<
    I2CImpl: device_driver::RegisterInterface<AddressType = u8, Error = AxpError<I2CBusErr>>,
    I2CBusErr: core::fmt::Debug = <I2CImpl as device_driver::RegisterInterface>::Error,
> {
    pub ll: AxpLowLevel<I2CImpl>,
    _marker: core::marker::PhantomData<I2CBusErr>,
}

#[cfg(feature = "async")]
pub struct Axp192<
    I2CImpl: device_driver::AsyncRegisterInterface<AddressType = u8, Error = AxpError<I2CBusErr>>,
    I2CBusErr: core::fmt::Debug = <I2CImpl as device_driver::AsyncRegisterInterface>::Error,
> {
    pub ll: AxpLowLevel<I2CImpl>,
    _marker: core::marker::PhantomData<I2CBusErr>,
}

#[cfg(feature = "blocking")]
impl<I2CBus, E> Axp192<AxpInterface<I2CBus>, E>
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
impl<I2CBus, E> Axp192<AxpInterface<I2CBus>, E>
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

#[cfg(feature = "blocking")]
pub trait CurrentAxpDriverInterface<E>:
    device_driver::RegisterInterface<AddressType = u8, Error = AxpError<E>>
{
}
#[cfg(feature = "blocking")]
impl<T, E> CurrentAxpDriverInterface<E> for T
where
    T: device_driver::RegisterInterface<AddressType = u8, Error = AxpError<E>>,
    E: core::fmt::Debug,
{
}

#[cfg(feature = "async")]
pub trait CurrentAxpDriverInterface<E>:
    device_driver::AsyncRegisterInterface<AddressType = u8, Error = AxpError<E>>
{
}
#[cfg(feature = "async")]
impl<T, E> CurrentAxpDriverInterface<E> for T
where
    T: device_driver::AsyncRegisterInterface<AddressType = u8, Error = AxpError<E>>,
    E: core::fmt::Debug,
{
}

impl<I2CImpl, I2CBusErr> Axp192<I2CImpl, I2CBusErr>
where
    I2CImpl: CurrentAxpDriverInterface<I2CBusErr>,
    I2CBusErr: core::fmt::Debug,
{
    #[bisync(async_suffix = "async")]
    pub async fn get_battery_voltage_mv(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let raw_fieldset = self.ll.battery_voltage_adc().read().await?;
        let adc_val = adc_12bit_from_raw_u16(raw_fieldset.raw());
        Ok(adc_val as f32 * 1.1)
    }

    #[bisync(async_suffix = "async")]
    pub async fn get_battery_charge_current_ma(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let raw_fieldset = self.ll.battery_charge_current_adc().read().await?;
        let adc_val = adc_13bit_from_raw_u16(raw_fieldset.raw());
        Ok(adc_val as f32 * 0.5)
    }

    #[bisync(async_suffix = "async")]
    pub async fn get_battery_instantaneous_power_uw(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let raw_fieldset = self.ll.battery_instantaneous_power_adc().read().await?;
        // device-driver for a 24-bit field "raw" will return a u32.
        let adc_val = adc_24bit_from_raw_u32(raw_fieldset.raw());
        Ok(adc_val as f32 * 0.55)
    }

    #[bisync(async_suffix = "async")]
    pub async fn set_dcdc_enable(
        &mut self,
        dc: DcId,
        enable: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        self.ll
            .power_output_control()
            .modify(|r| match dc {
                DcId::Dcdc1 => r.set_dcdc_1_output_enable(enable),
                DcId::Dcdc2 => r.set_dcdc_2_output_enable(enable),
                DcId::Dcdc3 => r.set_dcdc_3_output_enable(enable),
            })
            .await
    }

    #[bisync(async_suffix = "async")]
    pub async fn set_dcdc_voltage(
        &mut self,
        dc: DcId,
        voltage_mv: u16,
    ) -> Result<(), AxpError<I2CBusErr>> {
        if !(700..=3500).contains(&voltage_mv) {
            return Err(AxpError::InvalidVoltage(voltage_mv));
        }
        let raw_setting = ((voltage_mv.saturating_sub(700)) / 25) as u8;

        match dc {
            DcId::Dcdc1 => {
                self.ll
                    .dc_dc_1_voltage_setting()
                    .modify(|r| r.set_voltage_setting(raw_setting))
                    .await
            }
            DcId::Dcdc2 => {
                self.ll
                    .dc_dc_2_voltage_setting()
                    .modify(|r| r.set_voltage_setting(raw_setting))
                    .await
            }
            DcId::Dcdc3 => {
                self.ll
                    .dc_dc_3_voltage_setting()
                    .modify(|r| r.set_voltage_setting(raw_setting))
                    .await
            }
        }
    }
}

/// Helper to extract a 12-bit ADC value from a raw u16 read over two 8-bit registers.
/// Assumes Big Endian read: raw_u16 = (MSB_register_byte << 8) | LSB_register_byte.
/// MSB_register_byte contains ADC[11:4].
/// LSB_register_byte contains ADC[3:0] in its lower nibble.
fn adc_12bit_from_raw_u16(raw_be_u16: u16) -> u16 {
    let msb_reg_val = (raw_be_u16 >> 8) as u8; // Content of the first physical register (e.g., 0x78)
    let lsb_reg_val = (raw_be_u16 & 0xFF) as u8; // Content of the second physical register (e.g., 0x79)
    ((msb_reg_val as u16) << 4) | ((lsb_reg_val & 0x0F) as u16)
}

/// Helper to extract a 13-bit ADC value from a raw u16 read over two 8-bit registers.
/// Assumes Big Endian read: raw_u16 = (MSB_register_byte << 8) | LSB_register_byte.
/// MSB_register_byte contains ADC[12:5].
/// LSB_register_byte contains ADC[4:0] in its bits 4:0.
fn adc_13bit_from_raw_u16(raw_be_u16: u16) -> u16 {
    let msb_reg_val = (raw_be_u16 >> 8) as u8; // Content of the first physical register (e.g., 0x7A)
    let lsb_reg_val = (raw_be_u16 & 0xFF) as u8; // Content of the second physical register (e.g., 0x7B)
    ((msb_reg_val as u16) << 5) | ((lsb_reg_val & 0x1F) as u16)
}

/// Helper to extract a 24-bit ADC value from a raw u32 read over three 8-bit registers.
/// Assumes Big Endian read: raw_u32 = (MSB_reg << 16) | (MID_reg << 8) | LSB_reg.
/// The device-driver field for a 24-bit uint will likely yield a u32 with data in lower 24 bits.
fn adc_24bit_from_raw_u32(raw_be_u32: u32) -> u32 {
    raw_be_u32 & 0x00FFFFFF // Ensure only the lower 24 bits are used
}

#[cfg(all(feature = "async", feature = "blocking"))]
compile_error!("Features 'async' and 'blocking' should be mutually exclusive.");

#[cfg(not(any(feature = "async", feature = "blocking")))]
compile_error!("Either the 'async' or 'blocking' feature must be enabled.");
