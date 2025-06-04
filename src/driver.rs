use super::{ASYNC, I2c, RegisterInterface, SYNC, bisync, only_async, only_sync};
use crate::{AXP192_I2C_ADDRESS, AxpError, AxpInterface, AxpLowLevel, helpers::*};
use device_driver::RegisterOperation;

#[bisync]
impl<I2CBus, E> RegisterInterface for AxpInterface<I2CBus>
where
    I2CBus: I2c<Error = E>,
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
            .write_read(AXP192_I2C_ADDRESS, &[address], data)
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
            .write(AXP192_I2C_ADDRESS, &buffer[..1 + data.len()])
            .await
            .map_err(AxpError::I2c)
    }
}

pub struct Axp192<
    I2CImpl: RegisterInterface<AddressType = u8, Error = AxpError<I2CBusErr>>,
    I2CBusErr: core::fmt::Debug = <I2CImpl as RegisterInterface>::Error,
> {
    pub ll: AxpLowLevel<I2CImpl>,
    _marker: core::marker::PhantomData<I2CBusErr>,
}

impl<I2CBus, E> Axp192<AxpInterface<I2CBus>, E>
where
    I2CBus: I2c<Error = E>,
    E: core::fmt::Debug,
{
    pub fn new(i2c: I2CBus) -> Self {
        Self {
            ll: AxpLowLevel::new(AxpInterface::new(i2c)),
            _marker: core::marker::PhantomData,
        }
    }
}

pub trait CurrentAxpDriverInterface<E>:
    RegisterInterface<AddressType = u8, Error = AxpError<E>>
{
}

impl<T, E> CurrentAxpDriverInterface<E> for T
where
    T: RegisterInterface<AddressType = u8, Error = AxpError<E>>,
    E: core::fmt::Debug,
{
}

#[only_sync]
fn read_internal<'a, Interface, Register, Access>(
    op: &mut RegisterOperation<'a, Interface, u8, Register, Access>
) -> Result<Register, Interface::Error>
where
    Interface: RegisterInterface<AddressType = u8>,
    Register: device_driver::FieldSet,
    Access: device_driver::ReadCapability,
{
    op.read()
}

#[only_async]
async fn read_internal<'a, Interface, Register, Access>(
    op: &mut RegisterOperation<'a, Interface, u8, Register, Access>
) -> Result<Register, Interface::Error>
where
    Interface: RegisterInterface<AddressType = u8>,
    Register: device_driver::FieldSet,
    Access: device_driver::ReadCapability,
{
    op.read_async().await
}

#[only_sync]
fn write_internal<'a, Interface, Register, Access, R>(
    op: &mut RegisterOperation<'a, Interface, u8, Register, Access>,
    f: impl FnOnce(&mut Register) -> R,
) -> Result<R, Interface::Error>
where
    Interface: RegisterInterface<AddressType = u8>,
    Register: device_driver::FieldSet,
    Access: device_driver::WriteCapability,
{
    op.write(f)
}

#[only_async]
async fn write_internal<'a, Interface, Register, Access, R>(
    op: &mut RegisterOperation<'a, Interface, u8, Register, Access>,
    f: impl FnOnce(&mut Register) -> R,
) -> Result<R, Interface::Error>
where
    Interface: RegisterInterface<AddressType = u8>,
    Register: device_driver::FieldSet,
    Access: device_driver::WriteCapability,
{
    op.write_async(f).await
}

#[only_sync]
fn modify_internal<'a, Interface, Register, Access, R>(
    op: &mut RegisterOperation<'a, Interface, u8, Register, Access>,
    f: impl FnOnce(&mut Register) -> R,
) -> Result<R, Interface::Error>
where
    Interface: RegisterInterface<AddressType = u8>,
    Register: device_driver::FieldSet,
    Access: device_driver::ReadCapability + device_driver::WriteCapability,
{
    op.modify(f)
}

#[only_async]
async fn modify_internal<'a, Interface, Register, Access, R>(
    op: &mut RegisterOperation<'a, Interface, u8, Register, Access>,
    f: impl FnOnce(&mut Register) -> R,
) -> Result<R, Interface::Error>
where
    Interface: RegisterInterface<AddressType = u8>,
    Register: device_driver::FieldSet,
    Access: device_driver::ReadCapability + device_driver::WriteCapability,
{
    op.modify_async(f).await
}

impl<I2CImpl, I2CBusErr> Axp192<I2CImpl, I2CBusErr>
where
    I2CImpl: CurrentAxpDriverInterface<I2CBusErr>,
    I2CBusErr: core::fmt::Debug,
{
    #[bisync]
    pub async fn get_battery_voltage_mv(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let mut op = self.ll.battery_voltage_adc();
        let raw_fieldset = read_internal(&mut op).await?;
        let adc_val = adc_12bit_from_raw_u16(raw_fieldset.raw());
        Ok(adc_val as f32 * 1.1)
    }
}
