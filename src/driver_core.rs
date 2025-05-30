// src/driver_core.rs

use super::*;
use crate::{
    AxpError, DcId, GpioMode, /* Other enums from lib.rs */ GpioPin, LdoId, PekBootTime,
    PekLongPressTime, PekShutdownDuration,
};

// trace!, info! etc. are globally available from `crate::fmt`

device_driver::create_device!(device_name: Device, manifest: "device.yaml");

// --- I2C Bus Interface (AxpInterface) ---
pub struct AxpInterface<I2CBus> {
    // Fields were missing here
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
        if data.len() > 2 {
            return Err(AxpError::NotImplemented("Write data too large"));
        }
        let mut buffer = [0u8; 1 + 2];
        buffer[0] = address;
        buffer[1..1 + data.len()].copy_from_slice(data);
        self.i2c_bus
            .write(self.device_address, &buffer[..1 + data.len()])
            .map_err(AxpError::I2c)
    }
}
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
            return Err(AxpError::NotImplemented("Write data too large"));
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

pub struct Axp192<I2CImpl> {
    ll: Device<I2CImpl>,
}

#[only_sync]
impl<I2CBus, E: core::fmt::Debug> Axp192<AxpInterface<I2CBus>>
where
    I2CBus: embedded_hal::i2c::I2c<Error = E>,
    AxpInterface<I2CBus>: device_driver::RegisterInterface<Error = AxpError<E>, AddressType = u8>,
{
    pub fn new(i2c: I2CBus) -> Self {
        Self {
            ll: Device::new(AxpInterface::new(i2c, 0x34)),
        }
    }
}

#[only_async]
impl<I2CBus, E: core::fmt::Debug> Axp192<AxpInterface<I2CBus>>
where
    I2CBus: embedded_hal_async::i2c::I2c<Error = E>,
    AxpInterface<I2CBus>:
        device_driver::AsyncRegisterInterface<Error = AxpError<E>, AddressType = u8>,
{
    pub fn new(i2c: I2CBus) -> Self {
        Self {
            ll: Device::new(AxpInterface::new(i2c, 0x34)),
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
    pub async fn set_output_enable_dcdc(
        &mut self,
        dc: DcId,
        enable: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let mut op = self.ll.dcdc_13_ldo_23_control();
        if ASYNC {
            op.modify_async(|r| match dc {
                DcId::Dcdc1 => r.set_dcdc_1_enable(enable),
                DcId::Dcdc3 => r.set_dcdc_3_enable(enable),
            })
            .await?; // Apply ? after await
        } else {
            op.modify(|r| match dc {
                DcId::Dcdc1 => r.set_dcdc_1_enable(enable),
                DcId::Dcdc3 => r.set_dcdc_3_enable(enable),
            })?; // Apply ? directly
        }
        Ok(())
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
        let setting = ((voltage_mv - 700) / 25) as u8 & 0x7F;
        match dc {
            DcId::Dcdc1 => {
                let mut op = self.ll.dcdc_1_voltage();
                if ASYNC {
                    op.modify_async(|r| r.set_setting(setting)).await?
                } else {
                    op.modify(|r| r.set_setting(setting))?
                }
            }
            DcId::Dcdc3 => {
                let mut op = self.ll.dcdc_3_voltage();
                if ASYNC {
                    op.modify_async(|r| r.set_setting(setting)).await?
                } else {
                    op.modify(|r| r.set_setting(setting))?
                }
            }
        }
        Ok(())
    }

    // ... (Apply the same `if ASYNC { op.method_async(...).await? } else { op.method_sync(...)? }`
    //      pattern for all other methods that return Result<(), E> like set_output_enable_ldo,
    //      set_ldo_voltage, set_charge_current_ma, set_gpio_mode, set_pek_settings) ...

    // For methods returning Result<T, E> (like reads):
    #[bisync]
    pub async fn is_charging(&mut self) -> Result<bool, AxpError<I2CBusErr>> {
        trace!("Checking charging status...");
        let power_status_fieldset = if ASYNC {
            self.ll.power_status().read_async().await
        } else {
            self.ll.power_status().read()
        }?; // Note: ? is outside the if/else
        Ok(power_status_fieldset.battery_current_direction())
    }

    #[bisync]
    pub async fn get_battery_voltage_mv(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        trace!("Reading battery voltage...");
        let adc_fieldset = if ASYNC {
            self.ll.battery_voltage_adc().read_async().await
        } else {
            self.ll.battery_voltage_adc().read()
        }?;
        let raw_adc = adc_fieldset.raw();
        Ok((raw_adc >> 4) as f32 * 1.1)
    }

    // --- Re-implement other setters with the corrected bisync pattern ---

    #[bisync]
    pub async fn set_output_enable_ldo(
        &mut self,
        ldo: LdoId,
        enable: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let mut op = self.ll.dcdc_13_ldo_23_control();
        if ASYNC {
            op.modify_async(|r| match ldo {
                LdoId::Ldo2 => r.set_ldo_2_enable(enable),
                LdoId::Ldo3 => r.set_ldo_3_enable(enable),
            })
            .await?;
        } else {
            op.modify(|r| match ldo {
                LdoId::Ldo2 => r.set_ldo_2_enable(enable),
                LdoId::Ldo3 => r.set_ldo_3_enable(enable),
            })?;
        }
        Ok(())
    }

    #[bisync]
    pub async fn set_ldo_voltage(
        &mut self,
        ldo: LdoId,
        voltage_mv: u16,
    ) -> Result<(), AxpError<I2CBusErr>> {
        if !(1800..=3300).contains(&voltage_mv) {
            return Err(AxpError::InvalidVoltage(voltage_mv));
        }
        let setting = ((voltage_mv - 1800) / 100) as u8 & 0x0F;
        let mut op = self.ll.ldo_23_voltage();
        if ASYNC {
            op.modify_async(|r| match ldo {
                LdoId::Ldo2 => r.set_ldo_2_setting(setting),
                LdoId::Ldo3 => r.set_ldo_3_setting(setting),
            })
            .await?;
        } else {
            op.modify(|r| match ldo {
                LdoId::Ldo2 => r.set_ldo_2_setting(setting),
                LdoId::Ldo3 => r.set_ldo_3_setting(setting),
            })?;
        }
        Ok(())
    }

    #[bisync]
    pub async fn set_charge_current_ma(
        &mut self,
        current_ma: u16,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let current_bits = match current_ma {
            0..=100 => 0b0000,
            101..=190 => 0b0001,
            191..=280 => 0b0010,
            281..=370 => 0b0011,
            371..=460 => 0b0100,
            461..=550 => 0b0101,
            551..=640 => 0b0110,
            641..=780 => 0b0111,
            _ => return Err(AxpError::InvalidCurrent(current_ma)),
        };
        let mut op = self.ll.charge_control_1();
        if ASYNC {
            op.modify_async(|r| r.set_charge_current_setting(current_bits))
                .await?;
        } else {
            op.modify(|r| r.set_charge_current_setting(current_bits))?;
        }
        Ok(())
    }

    #[bisync]
    pub async fn get_acin_voltage_mv(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let adc_fieldset = if ASYNC {
            self.ll.acin_voltage_adc().read_async().await
        } else {
            self.ll.acin_voltage_adc().read()
        }?;
        let raw = adc_fieldset.raw();
        Ok((raw >> 4) as f32 * 1.7)
    }

    #[bisync]
    pub async fn get_internal_temperature_c(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let temp_fieldset = if ASYNC {
            self.ll.internal_temperature_adc().read_async().await
        } else {
            self.ll.internal_temperature_adc().read()
        }?;
        let raw = temp_fieldset.raw();
        Ok(((raw >> 4) as f32 * 0.1) - 144.7)
    }

    #[bisync]
    pub async fn set_gpio_mode(
        &mut self,
        pin: GpioPin,
        mode: GpioMode,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let mode_val = mode as u8;
        match pin {
            GpioPin::Gpio0 => {
                let mut op = self.ll.gpio_0_control();
                if ASYNC {
                    op.modify_async(|r| r.set_mode(mode_val)).await?
                } else {
                    op.modify(|r| r.set_mode(mode_val))?
                }
            }
            GpioPin::Gpio1 => {
                let mut op = self.ll.gpio_1_control();
                if ASYNC {
                    op.modify_async(|r| r.set_mode(mode_val)).await?
                } else {
                    op.modify(|r| r.set_mode(mode_val))?
                }
            }
            GpioPin::Gpio2 => {
                let mut op = self.ll.gpio_2_control();
                if ASYNC {
                    op.modify_async(|r| r.set_mode(mode_val)).await?
                } else {
                    op.modify(|r| r.set_mode(mode_val))?
                }
            }
        }
        Ok(())
    }

    #[bisync]
    pub async fn set_gpio_output_level(
        &mut self,
        pin: GpioPin,
        level_high: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let mut op = self.ll.gpio_20_signal_status();
        if ASYNC {
            op.modify_async(|r| match pin {
                GpioPin::Gpio0 => r.set_gpio_0_out_level(level_high),
                GpioPin::Gpio1 => r.set_gpio_1_out_level(level_high),
                GpioPin::Gpio2 => r.set_gpio_2_out_level(level_high),
            })
            .await?;
        } else {
            op.modify(|r| match pin {
                GpioPin::Gpio0 => r.set_gpio_0_out_level(level_high),
                GpioPin::Gpio1 => r.set_gpio_1_out_level(level_high),
                GpioPin::Gpio2 => r.set_gpio_2_out_level(level_high),
            })?;
        }
        Ok(())
    }

    #[bisync]
    pub async fn set_pek_settings(
        &mut self,
        boot_time: PekBootTime,
        long_press: PekLongPressTime,
        auto_shutdown_by_pwrok_en: bool,
        pwrok_signal_delay_64ms: bool,
        shutdown_duration: PekShutdownDuration,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let mut op = self.ll.pek_settings();
        let pwrok_delay_val_for_reg = if pwrok_signal_delay_64ms { 1 } else { 0 };
        if ASYNC {
            op.write_async(|w| {
                w.set_boot_time_setting(boot_time as u8);
                w.set_long_press_time_setting(long_press as u8);
                w.set_auto_shutdown_by_pwrok_en(auto_shutdown_by_pwrok_en);
                w.set_pwrok_signal_delay(pwrok_delay_val_for_reg);
                w.set_shutdown_duration_setting(shutdown_duration as u8);
            })
            .await?;
        } else {
            op.write(|w| {
                w.set_boot_time_setting(boot_time as u8);
                w.set_long_press_time_setting(long_press as u8);
                w.set_auto_shutdown_by_pwrok_en(auto_shutdown_by_pwrok_en);
                w.set_pwrok_signal_delay(pwrok_delay_val_for_reg);
                w.set_shutdown_duration_setting(shutdown_duration as u8);
            })?;
        }
        Ok(())
    }

    #[bisync]
    pub async fn set_led_enable(&mut self, enable: bool) -> Result<(), AxpError<I2CBusErr>> {
        self.set_gpio_output_level(GpioPin::Gpio1, !enable).await
    }
}
