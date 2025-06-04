use super::{I2c, RegisterInterface, bisync};
use thiserror::Error;

device_driver::create_device!(device_name: AxpLowLevel, manifest: "device.yaml");
const AXP192_I2C_ADDRESS: u8 = 0x34;

// add optional defmt here later
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

pub struct AxpInterface<I2CBus> {
    i2c_bus: I2CBus,
}

impl<I2CBus> AxpInterface<I2CBus> {
    pub fn new(i2c_bus: I2CBus) -> Self {
        Self { i2c_bus }
    }
}

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
