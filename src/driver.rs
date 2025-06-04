use super::{I2c, RegisterInterface, bisync};
use crate::{AXP192_I2C_ADDRESS, AxpError, AxpInterface, AxpLowLevel};

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

#[bisync]
pub struct Axp192<
    I2CImpl: RegisterInterface<AddressType = u8, Error = AxpError<I2CBusErr>>,
    I2CBusErr: core::fmt::Debug = <I2CImpl as RegisterInterface>::Error,
> {
    pub ll: AxpLowLevel<I2CImpl>,
    _marker: core::marker::PhantomData<I2CBusErr>,
}

#[bisync]
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
