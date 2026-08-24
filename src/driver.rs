use super::{I2c, RegisterInterface, bisync, only_async, only_sync};
use crate::{
    AXP192_I2C_ADDRESS, AxpError, AxpInterface, AxpLowLevel, DcId, GpioAdcRange, GpioId, LdoId,
    adc_helpers::*,
};
use device_driver::{FieldsetMetadata, RegisterInterfaceBase};

#[bisync]
impl<I2CBus, E> RegisterInterface for AxpInterface<I2CBus>
where
    I2CBus: I2c<Error = E>,
    E: core::fmt::Debug,
{
    async fn read_register(
        &mut self,
        address: u8,
        data: &mut [u8],
        _metadata: &FieldsetMetadata,
    ) -> Result<(), Self::Error> {
        self.i2c_bus
            .write_read(AXP192_I2C_ADDRESS, &[address], data)
            .await
            .map_err(AxpError::I2c)
    }
    async fn write_register(
        &mut self,
        address: u8,
        data: &mut [u8],
        _metadata: &FieldsetMetadata,
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
    I2CImpl: RegisterInterfaceBase<AddressType = u8, Error = AxpError<I2CBusErr>>,
    I2CBusErr: core::fmt::Debug,
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
    // ADC readings. The scale factors come from the channel table in datasheet
    // section 9.7; every channel is 12-bit except the two battery current ones,
    // which are 13-bit (REG7BH/REG7DH hold five low bits, not four).

    #[bisync]
    pub async fn get_battery_voltage_mv(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let fs = read_internal(self.ll.battery_voltage_adc()).await?;
        Ok(adc_12bit(fs.value_high(), fs.value_low()) as f32 * 1.1)
    }

    #[bisync]
    pub async fn get_battery_charge_current_ma(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let fs = read_internal(self.ll.battery_charge_current_adc()).await?;
        Ok(adc_13bit(fs.value_high(), fs.value_low()) as f32 * 0.5)
    }

    #[bisync]
    pub async fn get_battery_discharge_current_ma(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let fs = read_internal(self.ll.battery_discharge_current_adc()).await?;
        Ok(adc_13bit(fs.value_high(), fs.value_low()) as f32 * 0.5)
    }

    #[bisync]
    pub async fn get_battery_instantaneous_power_uw(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let fs = read_internal(self.ll.battery_instantaneous_power_adc()).await?;
        Ok(fs.value() as f32 * 0.55)
    }

    #[bisync]
    pub async fn get_acin_voltage_mv(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let fs = read_internal(self.ll.acin_voltage_adc()).await?;
        Ok(adc_12bit(fs.value_high(), fs.value_low()) as f32 * 1.7)
    }

    #[bisync]
    pub async fn get_acin_current_ma(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let fs = read_internal(self.ll.acin_current_adc()).await?;
        Ok(adc_12bit(fs.value_high(), fs.value_low()) as f32 * 0.625)
    }

    #[bisync]
    pub async fn get_vbus_voltage_mv(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let fs = read_internal(self.ll.vbus_voltage_adc()).await?;
        Ok(adc_12bit(fs.value_high(), fs.value_low()) as f32 * 1.7)
    }

    #[bisync]
    pub async fn get_vbus_current_ma(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let fs = read_internal(self.ll.vbus_current_adc()).await?;
        Ok(adc_12bit(fs.value_high(), fs.value_low()) as f32 * 0.375)
    }

    #[bisync]
    pub async fn get_aps_voltage_mv(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let fs = read_internal(self.ll.aps_voltage_adc()).await?;
        Ok(adc_12bit(fs.value_high(), fs.value_low()) as f32 * 1.4)
    }

    /// Voltage on the TS pin, which monitors battery temperature by default.
    #[bisync]
    pub async fn get_ts_pin_voltage_mv(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let fs = read_internal(self.ll.ts_pin_adc()).await?;
        Ok(adc_12bit(fs.value_high(), fs.value_low()) as f32 * 0.8)
    }

    /// Temperature of the AXP192 die itself, not the battery.
    #[bisync]
    pub async fn get_internal_temperature_c(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let fs = read_internal(self.ll.internal_temperature_adc()).await?;
        Ok(adc_12bit(fs.value_high(), fs.value_low()) as f32 * 0.1 - 144.7)
    }

    /// Voltage on a GPIO pin configured as an ADC input.
    ///
    /// The conversion depends on the input range configured in REG85H, so this
    /// reads that register first and therefore costs two I2C transactions. In a
    /// polling loop where the range is known and fixed, call
    /// [`Self::get_gpio_voltage_mv_with_range`] instead to halve the traffic.
    #[bisync]
    pub async fn get_gpio_voltage_mv(&mut self, gpio: GpioId) -> Result<f32, AxpError<I2CBusErr>> {
        let ranges = read_internal(self.ll.gpio_adc_input_range_setting()).await?;
        let range = match gpio {
            GpioId::Gpio0 => ranges.gpio_0_adc_input_range(),
            GpioId::Gpio1 => ranges.gpio_1_adc_input_range(),
            GpioId::Gpio2 => ranges.gpio_2_adc_input_range(),
            GpioId::Gpio3 => ranges.gpio_3_adc_input_range(),
        };
        self.get_gpio_voltage_mv_with_range(gpio, range).await
    }

    /// Voltage on a GPIO pin configured as an ADC input, skipping the REG85H read.
    ///
    /// `range` must match what REG85H is actually set to for this pin. Passing
    /// the wrong one silently shifts the result by 700mV, so prefer
    /// [`Self::get_gpio_voltage_mv`] unless the extra transaction matters.
    #[bisync]
    pub async fn get_gpio_voltage_mv_with_range(
        &mut self,
        gpio: GpioId,
        range: GpioAdcRange,
    ) -> Result<f32, AxpError<I2CBusErr>> {
        let fs = match gpio {
            GpioId::Gpio0 => read_internal(self.ll.gpio_0_voltage_adc()).await,
            GpioId::Gpio1 => read_internal(self.ll.gpio_1_voltage_adc()).await,
            GpioId::Gpio2 => read_internal(self.ll.gpio_2_voltage_adc()).await,
            GpioId::Gpio3 => read_internal(self.ll.gpio_3_voltage_adc()).await,
        }?;
        let offset = match range {
            GpioAdcRange::Range00To20475V => 0.0,
            GpioAdcRange::Range07To27475V => 700.0,
        };
        Ok(offset + adc_12bit(fs.value_high(), fs.value_low()) as f32 * 0.5)
    }

    #[bisync]
    pub async fn set_dcdc_enable(
        &mut self,
        dc: DcId,
        enable: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let op = self.ll.power_output_control();
        modify_internal(op, |r| match dc {
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
                let op = self.ll.dc_dc_1_voltage_setting();
                modify_internal(op, |r| r.set_voltage_setting(raw_setting)).await
            }
            DcId::Dcdc2 => {
                let op = self.ll.dc_dc_2_voltage_setting();
                modify_internal(op, |r| r.set_voltage_setting(raw_setting)).await
            }
            DcId::Dcdc3 => {
                let op = self.ll.dc_dc_3_voltage_setting();
                modify_internal(op, |r| r.set_voltage_setting(raw_setting)).await
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

        let op = self.ll.ldo_2_and_3_voltage_setting();
        modify_internal(op, |r| match ldo {
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

        let op = self.ll.gpio_0_ldo_voltage_setting();
        write_internal(op, |r| {
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

        let op = self.ll.battery_charge_high_temp_threshold();
        write_internal(op, |r| {
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

        let op = self.ll.battery_charge_low_temp_threshold();
        write_internal(op, |r| {
            r.set_threshold_setting_raw(raw_setting);
        })
        .await
    }
}
