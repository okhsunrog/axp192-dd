use super::{I2c, RegisterInterface, bisync, only_async, only_sync};
use crate::{AXP192_I2C_ADDRESS, AxpError, AxpInterface, AxpLowLevel, DcId, LdoId, adc_helpers::*};

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

include!("bisync_helpers.rs");

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

    #[bisync]
    pub async fn get_battery_charge_current_ma(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let mut op = self.ll.battery_charge_current_adc();
        let raw_fieldset = read_internal(&mut op).await?;
        let adc_val = adc_13bit_from_raw_u16(raw_fieldset.raw());
        Ok(adc_val as f32 * 0.5)
    }

    #[bisync]
    pub async fn get_battery_instantaneous_power_uw(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let mut op = self.ll.battery_instantaneous_power_adc();
        let raw_fieldset = read_internal(&mut op).await?;
        let adc_val = adc_24bit_from_raw_u32(raw_fieldset.raw());
        Ok(adc_val as f32 * 0.55)
    }

    #[bisync]
    pub async fn set_dcdc_enable(
        &mut self,
        dc: DcId,
        enable: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let mut op = self.ll.power_output_control();
        modify_internal(&mut op, |r| match dc {
            DcId::Dcdc1 => r.set_dcdc_1_output_enable(enable),
            DcId::Dcdc2 => r.set_dcdc_2_output_enable(enable),
            DcId::Dcdc3 => r.set_dcdc_3_output_enable(enable),
        })
        .await
    }

    #[bisync]
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
                let mut op = self.ll.dc_dc_1_voltage_setting();
                modify_internal(&mut op, |r| r.set_voltage_setting(raw_setting)).await
            }
            DcId::Dcdc2 => {
                let mut op = self.ll.dc_dc_2_voltage_setting();
                modify_internal(&mut op, |r| r.set_voltage_setting(raw_setting)).await
            }
            DcId::Dcdc3 => {
                let mut op = self.ll.dc_dc_3_voltage_setting();
                modify_internal(&mut op, |r| r.set_voltage_setting(raw_setting)).await
            }
        }
    }

    #[bisync]
    pub async fn set_ldo_voltage_mv(
        &mut self,
        ldo: LdoId,
        voltage_mv: u16,
    ) -> Result<(), AxpError<I2CBusErr>> {
        if !(1800..=3300).contains(&voltage_mv) {
            return Err(AxpError::InvalidVoltage(voltage_mv));
        }
        let raw_setting = ((voltage_mv.saturating_sub(1800)) / 100) as u8;
        if raw_setting > 0x0F {
            return Err(AxpError::InvalidVoltage(voltage_mv));
        }

        let mut op = self.ll.ldo_2_and_3_voltage_setting();
        modify_internal(&mut op, |r| match ldo {
            LdoId::Ldo2 => r.set_ldo_2_voltage_setting(raw_setting),
            LdoId::Ldo3 => r.set_ldo_3_voltage_setting(raw_setting),
        })
        .await
    }

    #[bisync]
    pub async fn set_gpio0_ldo_voltage_mv(
        &mut self,
        voltage_mv: u16,
    ) -> Result<(), AxpError<I2CBusErr>> {
        if !(1800..=3300).contains(&voltage_mv) {
            return Err(AxpError::InvalidVoltage(voltage_mv));
        }
        let raw_4bit_setting = ((voltage_mv.saturating_sub(1800)) / 100) as u8;
        if raw_4bit_setting > 0x0F {
            return Err(AxpError::InvalidVoltage(voltage_mv));
        }

        let mut op = self.ll.gpio_0_ldo_voltage_setting();
        write_internal(&mut op, |r| {
            r.set_voltage_setting_raw(raw_4bit_setting);
        })
        .await
    }

    #[bisync]
    pub async fn set_battery_charge_high_temp_threshold_mv(
        &mut self,
        threshold_mv: u16,
    ) -> Result<(), AxpError<I2CBusErr>> {
        if threshold_mv > 3264 {
            return Err(AxpError::InvalidVoltage(threshold_mv));
        }
        let raw_setting_u16 = (threshold_mv * 10 + 64) / 128;
        let raw_setting = raw_setting_u16 as u8;

        let mut op = self.ll.battery_charge_high_temp_threshold();
        write_internal(&mut op, |r| {
            r.set_threshold_setting_raw(raw_setting);
        })
        .await
    }

    #[bisync]
    pub async fn set_battery_charge_low_temp_threshold_mv(
        &mut self,
        threshold_mv: u16,
    ) -> Result<(), AxpError<I2CBusErr>> {
        if threshold_mv > 3264 {
            return Err(AxpError::InvalidVoltage(threshold_mv));
        }
        let raw_setting_u16 = (threshold_mv * 10 + 64) / 128;
        let raw_setting = raw_setting_u16 as u8;

        let mut op = self.ll.battery_charge_low_temp_threshold();
        write_internal(&mut op, |r| {
            r.set_threshold_setting_raw(raw_setting);
        })
        .await
    }
}
