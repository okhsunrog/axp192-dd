use super::*;
use crate::{
    AxpError,
    ChargeTargetVoltage, // Using your existing ChargeTargetVoltage
    DcId,
    GpioMode,
    GpioPin,
    LdoId,
    PekBootTime,
    PekLongPressTime,
    PekShutdownDuration,
    // Add other enums you created for high-level API as needed:
    // Gpio0ModeSetting, BackupBatteryTargetVoltageSetting, BackupBatteryChargeCurrentSetting,
    // ChgLedPinFunctionSetting, NOeShutdownDelaySetting,
};

device_driver::create_device!(device_name: Device, manifest: "device.yaml");

// --- AxpInterface and its impls remain the same ---
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

#[only_sync]
impl<I2CBus, E: core::fmt::Debug> device_driver::RegisterInterface for AxpInterface<I2CBus>
where
    I2CBus: embedded_hal::i2c::I2c<Error = E>,
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
impl<I2CBus, E: core::fmt::Debug> device_driver::AsyncRegisterInterface for AxpInterface<I2CBus>
where
    I2CBus: embedded_hal_async::i2c::I2c<Error = E>,
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
    I2CImpl: CurrentAxpDriverInterface<AddressType = u8, Error = AxpError<I2CBusErr>>,
    I2CBusErr: core::fmt::Debug,
{
    #[bisync(async_suffix = "async")]
    pub async fn set_output_enable_dcdc(
        &mut self,
        dc: DcId,
        enable: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        self.ll
            .dcdc_13_ldo_23_control()
            .modify(|r| match dc {
                DcId::Dcdc1 => r.set_dcdc_1_enable(enable), // Corrected: dcdc_1_enable
                DcId::Dcdc3 => r.set_dcdc_3_enable(enable), // Corrected: dcdc_3_enable
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
        let setting = ((voltage_mv - 700) / 25) as u8;
        match dc {
            DcId::Dcdc1 => {
                self.ll
                    .dcdc_1_voltage()
                    .modify(|r| r.set_setting(setting & 0x7F))
                    .await?;
            }
            DcId::Dcdc3 => {
                self.ll
                    .dcdc_3_voltage()
                    .modify(|r| r.set_setting(setting & 0x7F))
                    .await?;
            }
        }
        Ok(())
    }

    #[bisync(async_suffix = "async")]
    pub async fn is_charging(&mut self) -> Result<bool, AxpError<I2CBusErr>> {
        let fieldset = self.ll.power_status().read().await?;
        Ok(fieldset.battery_current_direction())
    }

    #[bisync(async_suffix = "async")]
    pub async fn get_battery_voltage_mv(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let fieldset = self.ll.battery_voltage_adc().read().await?;
        Ok(fieldset.raw() as f32 * 1.1)
    }

    #[bisync(async_suffix = "async")]
    pub async fn set_output_enable_ldo(
        &mut self,
        ldo: LdoId,
        enable: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        self.ll
            .dcdc_13_ldo_23_control()
            .modify(|r| match ldo {
                LdoId::Ldo2 => r.set_ldo_2_enable(enable), // Corrected: ldo_2_enable
                LdoId::Ldo3 => r.set_ldo_3_enable(enable), // Corrected: ldo_3_enable
            })
            .await
    }

    #[bisync(async_suffix = "async")]
    pub async fn set_ldo_voltage(
        &mut self,
        ldo: LdoId,
        voltage_mv: u16,
    ) -> Result<(), AxpError<I2CBusErr>> {
        if !(1800..=3300).contains(&voltage_mv) {
            return Err(AxpError::InvalidVoltage(voltage_mv));
        }
        let setting = ((voltage_mv - 1800) / 100) as u8 & 0x0F;
        self.ll
            .ldo_23_voltage()
            .modify(|r| match ldo {
                LdoId::Ldo2 => r.set_ldo_2_setting(setting), // Corrected: ldo_2_setting
                LdoId::Ldo3 => r.set_ldo_3_setting(setting), // Corrected: ldo_3_setting
            })
            .await
    }

    #[bisync(async_suffix = "async")]
    pub async fn set_charge_current_ma(
        &mut self,
        current_ma: u16,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let current_bits = match current_ma {
            0..=100 => 0b0000,
            101..=190 => 0b0001,
            191..=280 => 0b0010,
            281..=360 => 0b0011,
            361..=450 => 0b0100,
            451..=550 => 0b0101,
            551..=630 => 0b0110,
            631..=700 => 0b0111,
            701..=780 => 0b1000,
            781..=880 => 0b1001,
            881..=960 => 0b1010,
            961..=1000 => 0b1011,
            1001..=1080 => 0b1100,
            1081..=1160 => 0b1101,
            1161..=1240 => 0b1110,
            1241..=1320 => 0b1111,
            _ => return Err(AxpError::InvalidCurrent(current_ma)),
        };
        self.ll
            .charge_control_1()
            .modify(|r| {
                r.set_charge_current_setting(current_bits);
            })
            .await
    }

    #[bisync(async_suffix = "async")]
    pub async fn get_acin_voltage_mv(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let fieldset = self.ll.acin_voltage_adc().read().await?;
        Ok(fieldset.raw() as f32 * 1.7)
    }

    #[bisync(async_suffix = "async")]
    pub async fn get_internal_temperature_c(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let fieldset = self.ll.internal_temperature_adc().read().await?;
        Ok((fieldset.raw() as f32 * 0.1) - 144.7)
    }

    #[bisync(async_suffix = "async")]
    pub async fn set_gpio_mode(
        &mut self,
        pin: GpioPin,
        mode: GpioMode,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let mode_val = mode as u8;
        match pin {
            GpioPin::Gpio0 => {
                self.ll
                    .gpio_0_control()
                    .modify(|r| r.set_mode(mode_val))
                    .await?;
            }
            GpioPin::Gpio1 => {
                self.ll
                    .gpio_1_control()
                    .modify(|r| r.set_mode(mode_val))
                    .await?;
            }
            GpioPin::Gpio2 => {
                self.ll
                    .gpio_2_control()
                    .modify(|r| r.set_mode(mode_val))
                    .await?;
            }
        };
        Ok(())
    }

    #[bisync(async_suffix = "async")]
    pub async fn set_gpio_output_level(
        &mut self,
        pin: GpioPin,
        level_high: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        match pin {
            GpioPin::Gpio0 => {
                self.ll
                    .gpio_20_signal_status()
                    .modify(|r| r.set_gpio_0_output_level_setting(level_high))
                    .await?; // Corrected
            }
            GpioPin::Gpio1 => {
                self.ll
                    .gpio_20_signal_status()
                    .modify(|r| r.set_gpio_1_output_level_setting(level_high))
                    .await?; // Corrected
            }
            GpioPin::Gpio2 => {
                self.ll
                    .gpio_20_signal_status()
                    .modify(|r| r.set_gpio_2_output_level_setting(level_high))
                    .await?; // Corrected
            }
        };
        Ok(())
    }

    #[bisync(async_suffix = "async")]
    pub async fn set_pek_settings(
        &mut self,
        boot_time: PekBootTime,
        long_press: PekLongPressTime,
        auto_shutdown_on_long_press: bool, // Changed name to match YAML
        pwrok_delay_64ms: bool,
        shutdown_duration: PekShutdownDuration,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let pwrok_delay_val_for_reg = if pwrok_delay_64ms { 1 } else { 0 }; // Assuming 1-bit field
        self.ll
            .pek_settings()
            .write(|w| {
                w.set_boot_time_setting(boot_time as u8);
                w.set_long_press_time_setting(long_press as u8);
                w.set_auto_shutdown_on_long_press_enable(auto_shutdown_on_long_press); // Corrected
                w.set_pwrok_signal_delay_after_power_on(pwrok_delay_val_for_reg); // Corrected
                w.set_shutdown_duration_setting(shutdown_duration as u8);
            })
            .await
    }

    #[bisync]
    pub async fn set_led_enable(&mut self, enable: bool) -> Result<(), AxpError<I2CBusErr>> {
        self.set_gpio_output_level(GpioPin::Gpio1, !enable).await
    }

    // --- NEW HIGH-LEVEL API METHODS FOR M5StickC-Plus init ---

    #[bisync(async_suffix = "async")]
    pub async fn enable_all_adcs_reg1(&mut self) -> Result<(), AxpError<I2CBusErr>> {
        // Renamed from enable_all_adcs
        self.ll
            .adc_enable_1()
            .write(|r| {
                r.set_battery_voltage_adc_enable(true);
                r.set_battery_current_adc_enable(true);
                r.set_acin_voltage_adc_enable(true);
                r.set_acin_current_adc_enable(true);
                r.set_vbus_voltage_adc_enable(true);
                r.set_vbus_current_adc_enable(true);
                r.set_aps_voltage_adc_enable(true);
                r.set_ts_pin_adc_enable(true);
            })
            .await
    }

    #[bisync(async_suffix = "async")]
    pub async fn enable_all_adcs_reg2(&mut self) -> Result<(), AxpError<I2CBusErr>> {
        self.ll
            .adc_enable_2()
            .write(|r| {
                r.set_internal_temp_adc_enable(true);
                r.set_gpio_0_adc_enable(true); // Corrected
                r.set_gpio_1_adc_enable(true); // Corrected
                r.set_gpio_2_adc_enable(true); // Corrected
                r.set_gpio_3_adc_enable(true); // Corrected
            })
            .await
    }

    #[bisync(async_suffix = "async")]
    pub async fn set_charge_control_basic(
        // Renamed from set_charging_parameters
        &mut self,
        enable_charging: bool,
        target_voltage: ChargeTargetVoltage, // Using your enum from lib.rs
        end_current_is_15_percent: bool,
        current_ma: u16,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let current_bits = match current_ma {
            0..=100 => 0b0000,
            101..=190 => 0b0001,
            191..=280 => 0b0010,
            281..=360 => 0b0011,
            361..=450 => 0b0100,
            451..=550 => 0b0101,
            551..=630 => 0b0110,
            631..=700 => 0b0111,
            701..=780 => 0b1000,
            781..=880 => 0b1001,
            881..=960 => 0b1010,
            961..=1000 => 0b1011,
            1001..=1080 => 0b1100,
            1081..=1160 => 0b1101,
            1161..=1240 => 0b1110,
            1241..=1320 => 0b1111,
            _ => return Err(AxpError::InvalidCurrent(current_ma)),
        };
        // Map your ChargeTargetVoltage enum to the 2-bit value AXP192 expects for REG33H[6:5]
        let target_voltage_bits = match target_voltage {
            ChargeTargetVoltage::V4_10 => 0b00,
            ChargeTargetVoltage::V4_15 => 0b01,
            ChargeTargetVoltage::V4_20 => 0b10,
            ChargeTargetVoltage::V4_36 => 0b11,
        };

        self.ll
            .charge_control_1()
            .write(|r| {
                r.set_charge_enable(enable_charging);
                r.set_charge_target_voltage_setting(target_voltage_bits);
                r.set_end_charge_current_select(end_current_is_15_percent);
                r.set_charge_current_setting(current_bits);
            })
            .await
    }

    #[bisync(async_suffix = "async")]
    pub async fn set_exten_enable(&mut self, enable: bool) -> Result<(), AxpError<I2CBusErr>> {
        self.ll
            .dcdc_13_ldo_23_control()
            .modify(|r| {
                r.set_exten_enable_alt(enable);
            })
            .await
    }

    #[bisync(async_suffix = "async")]
    pub async fn set_gpio0_ldo_mode_and_voltage(
        &mut self,
        voltage_mv: u16,
    ) -> Result<(), AxpError<I2CBusErr>> {
        // Assuming GpioMode::SpecialOutput010 is 0b010
        self.ll
            .gpio_0_control()
            .modify(|r| r.set_mode(0b010))
            .await?;

        if !(1800..=3300).contains(&voltage_mv) {
            return Err(AxpError::InvalidVoltage(voltage_mv));
        }
        let setting = ((voltage_mv.saturating_sub(1800)) / 100) as u8 & 0x0F;
        self.ll
            .gpio_0_ldoio_0_voltage()
            .write(|r| {
                r.set_voltage_setting(setting);
            })
            .await
    }

    #[bisync(async_suffix = "async")]
    pub async fn configure_vbus_settings(
        &mut self,
        path_override_nvbusen: bool,
        vhold_limit_disable: bool,
        current_limit_disable: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        self.ll
            .vbus_ipsout_channel()
            .write(|r| {
                r.set_vbus_ipsout_path_override_nvbusen(path_override_nvbusen);
                r.set_vbus_vhold_limit_enable(!vhold_limit_disable);
                if vhold_limit_disable {
                    r.set_vhold_voltage_setting(0b000); // 4.0V e.g.
                } else {
                    r.set_vhold_voltage_setting(0b100); // 4.4V default when enabled
                }
                r.set_vbus_current_limit_enable(!current_limit_disable);
                r.set_vbus_current_limit_value_select(false); // 500mA if enabled
            })
            .await
    }

    #[bisync(async_suffix = "async")]
    pub async fn set_battery_temp_protection_raw_n(
        &mut self,
        n_value_for_0x39: u8,
    ) -> Result<(), AxpError<I2CBusErr>> {
        self.ll
            .battery_charge_high_temp()
            .write(|r| {
                r.set_threshold_n_setting(n_value_for_0x39);
            })
            .await
    }

    #[bisync(async_suffix = "async")]
    pub async fn enable_backup_battery_charging(
        &mut self,
        enable: bool,
        target_3v0: bool,
        current_200ua: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let voltage_setting = if target_3v0 { 0b01 } else { 0b00 };
        let current_setting = if current_200ua { 0b10 } else { 0b01 };
        self.ll
            .battery_charge_control()
            .write(|r| {
                r.set_backup_battery_charge_enable(enable);
                r.set_backup_battery_target_voltage_setting(voltage_setting);
                r.set_backup_battery_charge_current_setting(current_setting);
            })
            .await
    }

    #[bisync(async_suffix = "async")]
    pub async fn configure_chgled_and_battery_detection(
        &mut self,
        battery_detection: bool,
        n_oe_delay_2s: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        self.ll
            .shutdown_battery_chgled_control()
            .modify(|r| {
                r.set_battery_detection_enable(battery_detection);
                r.set_chgled_pin_function_setting(0b00); // Hi-Z
                r.set_chgled_pin_source_control(false); // By Charger
                if n_oe_delay_2s {
                    r.set_n_oe_low_to_high_shutdown_delay(0b10); // 2S
                } else {
                    r.set_n_oe_low_to_high_shutdown_delay(0b00); // 0.5S
                }
            })
            .await
    }
    #[bisync(async_suffix = "async")]
    pub async fn power_off_chip(&mut self) -> Result<(), AxpError<I2CBusErr>> {
        self.ll
            .shutdown_battery_chgled_control()
            .modify(|r| {
                r.set_mode_a_shutdown_control(true);
            })
            .await
    }
}
