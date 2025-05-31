///Root block of the Device driver
#[derive(Debug)]
pub struct Device<I> {
    pub(crate) interface: I,
    #[doc(hidden)]
    base_address: u8,
}
impl<I> Device<I> {
    /// Create a new instance of the block based on device interface
    pub const fn new(interface: I) -> Self {
        Self { interface, base_address: 0 }
    }
    /// A reference to the interface used to communicate with the device
    pub(crate) fn interface(&mut self) -> &mut I {
        &mut self.interface
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    pub fn read_all_registers(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::RegisterInterface<AddressType = u8>,
    {
        let reg = self.power_status().read()?;
        callback(0 + 0 * 0, "power_status", reg.into());
        let reg = self.charge_status().read()?;
        callback(1 + 0 * 0, "charge_status", reg.into());
        let reg = self.otg_vbus_status().read()?;
        callback(4 + 0 * 0, "otg_vbus_status", reg.into());
        let reg = self.data_buffer_0().read()?;
        callback(6 + 0 * 0, "data_buffer_0", reg.into());
        let reg = self.data_buffer_1().read()?;
        callback(7 + 0 * 0, "data_buffer_1", reg.into());
        let reg = self.data_buffer_2().read()?;
        callback(8 + 0 * 0, "data_buffer_2", reg.into());
        let reg = self.data_buffer_3().read()?;
        callback(9 + 0 * 0, "data_buffer_3", reg.into());
        let reg = self.data_buffer_4().read()?;
        callback(10 + 0 * 0, "data_buffer_4", reg.into());
        let reg = self.data_buffer_5().read()?;
        callback(11 + 0 * 0, "data_buffer_5", reg.into());
        let reg = self.exten_dc_dc_2_control().read()?;
        callback(16 + 0 * 0, "exten_dc_dc_2_control", reg.into());
        let reg = self.power_output_control().read()?;
        callback(18 + 0 * 0, "power_output_control", reg.into());
        let reg = self.dc_dc_2_voltage_setting().read()?;
        callback(35 + 0 * 0, "dc_dc_2_voltage_setting", reg.into());
        let reg = self.dc_dc_2_vrc_parameter().read()?;
        callback(37 + 0 * 0, "dc_dc_2_vrc_parameter", reg.into());
        let reg = self.dc_dc_1_voltage_setting().read()?;
        callback(38 + 0 * 0, "dc_dc_1_voltage_setting", reg.into());
        let reg = self.dc_dc_3_voltage_setting().read()?;
        callback(39 + 0 * 0, "dc_dc_3_voltage_setting", reg.into());
        let reg = self.ldo_2_and_3_voltage_setting().read()?;
        callback(40 + 0 * 0, "ldo_2_and_3_voltage_setting", reg.into());
        let reg = self.vbus_ipsout_path_management().read()?;
        callback(48 + 0 * 0, "vbus_ipsout_path_management", reg.into());
        let reg = self.shutdown_voltage_setting().read()?;
        callback(49 + 0 * 0, "shutdown_voltage_setting", reg.into());
        let reg = self.shutdown_bat_chg_led_control().read()?;
        callback(50 + 0 * 0, "shutdown_bat_chg_led_control", reg.into());
        let reg = self.charge_control_1().read()?;
        callback(51 + 0 * 0, "charge_control_1", reg.into());
        let reg = self.charge_control_2().read()?;
        callback(52 + 0 * 0, "charge_control_2", reg.into());
        let reg = self.backup_battery_charge_control().read()?;
        callback(53 + 0 * 0, "backup_battery_charge_control", reg.into());
        let reg = self.pek_key_parameters().read()?;
        callback(54 + 0 * 0, "pek_key_parameters", reg.into());
        let reg = self.dc_dc_operating_frequency().read()?;
        callback(55 + 0 * 0, "dc_dc_operating_frequency", reg.into());
        let reg = self.battery_charge_low_temp_threshold().read()?;
        callback(56 + 0 * 0, "battery_charge_low_temp_threshold", reg.into());
        let reg = self.battery_charge_high_temp_threshold().read()?;
        callback(57 + 0 * 0, "battery_charge_high_temp_threshold", reg.into());
        let reg = self.aps_low_power_level_1_setting().read()?;
        callback(58 + 0 * 0, "aps_low_power_level_1_setting", reg.into());
        let reg = self.aps_low_power_level_2_setting().read()?;
        callback(59 + 0 * 0, "aps_low_power_level_2_setting", reg.into());
        let reg = self.battery_discharge_low_temp_threshold().read()?;
        callback(60 + 0 * 0, "battery_discharge_low_temp_threshold", reg.into());
        let reg = self.battery_discharge_high_temp_threshold().read()?;
        callback(61 + 0 * 0, "battery_discharge_high_temp_threshold", reg.into());
        let reg = self.dc_dc_operating_mode().read()?;
        callback(128 + 0 * 0, "dc_dc_operating_mode", reg.into());
        let reg = self.adc_enable_1().read()?;
        callback(130 + 0 * 0, "adc_enable_1", reg.into());
        let reg = self.adc_enable_2().read()?;
        callback(131 + 0 * 0, "adc_enable_2", reg.into());
        let reg = self.adc_sample_rate_ts_pin_control().read()?;
        callback(132 + 0 * 0, "adc_sample_rate_ts_pin_control", reg.into());
        let reg = self.gpio_adc_input_range_setting().read()?;
        callback(133 + 0 * 0, "gpio_adc_input_range_setting", reg.into());
        Ok(())
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    pub async fn read_all_registers_async(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::AsyncRegisterInterface<AddressType = u8>,
    {
        let reg = self.power_status().read_async().await?;
        callback(0 + 0 * 0, "power_status", reg.into());
        let reg = self.charge_status().read_async().await?;
        callback(1 + 0 * 0, "charge_status", reg.into());
        let reg = self.otg_vbus_status().read_async().await?;
        callback(4 + 0 * 0, "otg_vbus_status", reg.into());
        let reg = self.data_buffer_0().read_async().await?;
        callback(6 + 0 * 0, "data_buffer_0", reg.into());
        let reg = self.data_buffer_1().read_async().await?;
        callback(7 + 0 * 0, "data_buffer_1", reg.into());
        let reg = self.data_buffer_2().read_async().await?;
        callback(8 + 0 * 0, "data_buffer_2", reg.into());
        let reg = self.data_buffer_3().read_async().await?;
        callback(9 + 0 * 0, "data_buffer_3", reg.into());
        let reg = self.data_buffer_4().read_async().await?;
        callback(10 + 0 * 0, "data_buffer_4", reg.into());
        let reg = self.data_buffer_5().read_async().await?;
        callback(11 + 0 * 0, "data_buffer_5", reg.into());
        let reg = self.exten_dc_dc_2_control().read_async().await?;
        callback(16 + 0 * 0, "exten_dc_dc_2_control", reg.into());
        let reg = self.power_output_control().read_async().await?;
        callback(18 + 0 * 0, "power_output_control", reg.into());
        let reg = self.dc_dc_2_voltage_setting().read_async().await?;
        callback(35 + 0 * 0, "dc_dc_2_voltage_setting", reg.into());
        let reg = self.dc_dc_2_vrc_parameter().read_async().await?;
        callback(37 + 0 * 0, "dc_dc_2_vrc_parameter", reg.into());
        let reg = self.dc_dc_1_voltage_setting().read_async().await?;
        callback(38 + 0 * 0, "dc_dc_1_voltage_setting", reg.into());
        let reg = self.dc_dc_3_voltage_setting().read_async().await?;
        callback(39 + 0 * 0, "dc_dc_3_voltage_setting", reg.into());
        let reg = self.ldo_2_and_3_voltage_setting().read_async().await?;
        callback(40 + 0 * 0, "ldo_2_and_3_voltage_setting", reg.into());
        let reg = self.vbus_ipsout_path_management().read_async().await?;
        callback(48 + 0 * 0, "vbus_ipsout_path_management", reg.into());
        let reg = self.shutdown_voltage_setting().read_async().await?;
        callback(49 + 0 * 0, "shutdown_voltage_setting", reg.into());
        let reg = self.shutdown_bat_chg_led_control().read_async().await?;
        callback(50 + 0 * 0, "shutdown_bat_chg_led_control", reg.into());
        let reg = self.charge_control_1().read_async().await?;
        callback(51 + 0 * 0, "charge_control_1", reg.into());
        let reg = self.charge_control_2().read_async().await?;
        callback(52 + 0 * 0, "charge_control_2", reg.into());
        let reg = self.backup_battery_charge_control().read_async().await?;
        callback(53 + 0 * 0, "backup_battery_charge_control", reg.into());
        let reg = self.pek_key_parameters().read_async().await?;
        callback(54 + 0 * 0, "pek_key_parameters", reg.into());
        let reg = self.dc_dc_operating_frequency().read_async().await?;
        callback(55 + 0 * 0, "dc_dc_operating_frequency", reg.into());
        let reg = self.battery_charge_low_temp_threshold().read_async().await?;
        callback(56 + 0 * 0, "battery_charge_low_temp_threshold", reg.into());
        let reg = self.battery_charge_high_temp_threshold().read_async().await?;
        callback(57 + 0 * 0, "battery_charge_high_temp_threshold", reg.into());
        let reg = self.aps_low_power_level_1_setting().read_async().await?;
        callback(58 + 0 * 0, "aps_low_power_level_1_setting", reg.into());
        let reg = self.aps_low_power_level_2_setting().read_async().await?;
        callback(59 + 0 * 0, "aps_low_power_level_2_setting", reg.into());
        let reg = self.battery_discharge_low_temp_threshold().read_async().await?;
        callback(60 + 0 * 0, "battery_discharge_low_temp_threshold", reg.into());
        let reg = self.battery_discharge_high_temp_threshold().read_async().await?;
        callback(61 + 0 * 0, "battery_discharge_high_temp_threshold", reg.into());
        let reg = self.dc_dc_operating_mode().read_async().await?;
        callback(128 + 0 * 0, "dc_dc_operating_mode", reg.into());
        let reg = self.adc_enable_1().read_async().await?;
        callback(130 + 0 * 0, "adc_enable_1", reg.into());
        let reg = self.adc_enable_2().read_async().await?;
        callback(131 + 0 * 0, "adc_enable_2", reg.into());
        let reg = self.adc_sample_rate_ts_pin_control().read_async().await?;
        callback(132 + 0 * 0, "adc_sample_rate_ts_pin_control", reg.into());
        let reg = self.gpio_adc_input_range_setting().read_async().await?;
        callback(133 + 0 * 0, "gpio_adc_input_range_setting", reg.into());
        Ok(())
    }
    ///Indicates the input power source status (ACIN, VBUS), battery current direction,
    ///and other power-related conditions.
    pub fn power_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::PowerStatus,
        ::device_driver::RO,
    > {
        let address = self.base_address + 0;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::PowerStatus,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::PowerStatus::new)
    }
    ///Indicates various operational modes and charging statuses, including AXP192 temperature,
    ///battery presence, and charging progress.
    pub fn charge_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::ChargeStatus,
        ::device_driver::RO,
    > {
        let address = self.base_address + 1;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::ChargeStatus,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::ChargeStatus::new)
    }
    ///Indicates the status of VBUS when operating in USB OTG (On-The-Go) mode.
    pub fn otg_vbus_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::OtgVbusStatus,
        ::device_driver::RO,
    > {
        let address = self.base_address + 4;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::OtgVbusStatus,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::OtgVbusStatus::new)
    }
    ///General purpose data buffer for system data storage.
    ///Note: Retained as long as any AXP192 power source is present (not affected by host on/off).
    pub fn data_buffer_0(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::DataBuffer0,
        ::device_driver::RW,
    > {
        let address = self.base_address + 6;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::DataBuffer0,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::DataBuffer0::new)
    }
    pub fn data_buffer_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::DataBuffer0,
        ::device_driver::RW,
    > {
        let address = self.base_address + 7;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::DataBuffer0,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::DataBuffer0::new_as_data_buffer_1,
        )
    }
    pub fn data_buffer_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::DataBuffer0,
        ::device_driver::RW,
    > {
        let address = self.base_address + 8;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::DataBuffer0,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::DataBuffer0::new_as_data_buffer_2,
        )
    }
    pub fn data_buffer_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::DataBuffer0,
        ::device_driver::RW,
    > {
        let address = self.base_address + 9;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::DataBuffer0,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::DataBuffer0::new_as_data_buffer_3,
        )
    }
    pub fn data_buffer_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::DataBuffer0,
        ::device_driver::RW,
    > {
        let address = self.base_address + 10;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::DataBuffer0,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::DataBuffer0::new_as_data_buffer_4,
        )
    }
    pub fn data_buffer_5(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::DataBuffer0,
        ::device_driver::RW,
    > {
        let address = self.base_address + 11;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::DataBuffer0,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::DataBuffer0::new_as_data_buffer_5,
        )
    }
    ///Controls the output enable state for the EXTEN pin and the DC-DC2 converter.
    pub fn exten_dc_dc_2_control(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::ExtenDcDc2Control,
        ::device_driver::RW,
    > {
        let address = self.base_address + 16;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::ExtenDcDc2Control,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::ExtenDcDc2Control::new)
    }
    ///Controls output enable for DC-DC1, DC-DC3, LDO2, LDO3, EXTEN, and DC-DC2.
    ///Power-on default for M5StickC Plus is 0x5F.
    ///IMPORTANT: REG12H bit 6 (exten_output_enable) is linked to REG10H bit 2.
    ///             REG12H bit 4 (dcdc2_output_enable) is linked to REG10H bit 0.
    ///             These pairs control the same underlying hardware function.
    pub fn power_output_control(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::PowerOutputControl,
        ::device_driver::RW,
    > {
        let address = self.base_address + 18;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::PowerOutputControl,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::PowerOutputControl::new)
    }
    ///Sets the output voltage for the DC-DC2 converter.
    ///Formula: Output Voltage (V) = 0.7 + (value * 0.025). Range: 0.7V to 2.275V (raw 0-63).
    pub fn dc_dc_2_voltage_setting(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::DcDc2VoltageSetting,
        ::device_driver::RW,
    > {
        let address = self.base_address + 35;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::DcDc2VoltageSetting,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::DcDc2VoltageSetting::new)
    }
    ///Configures DC-DC2 Dynamic Voltage Scaling / Voltage Ramp Control (VRC).
    pub fn dc_dc_2_vrc_parameter(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::DcDc2VrcParameter,
        ::device_driver::RW,
    > {
        let address = self.base_address + 37;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::DcDc2VrcParameter,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::DcDc2VrcParameter::new)
    }
    ///Sets the output voltage for a DC-DC converter.
    ///Formula: V_out = 0.7V + (value * 25mV). Range 0.7V-3.5V (raw 0-112).
    pub fn dc_dc_1_voltage_setting(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::DcDc1VoltageSetting,
        ::device_driver::RW,
    > {
        let address = self.base_address + 38;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::DcDc1VoltageSetting,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::DcDc1VoltageSetting::new)
    }
    pub fn dc_dc_3_voltage_setting(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::DcDc1VoltageSetting,
        ::device_driver::RW,
    > {
        let address = self.base_address + 39;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::DcDc1VoltageSetting,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::DcDc1VoltageSetting::new_as_dc_dc_3_voltage_setting,
        )
    }
    ///Sets the output voltage for both LDO2 and LDO3.
    ///LDO2 (bits 7-4) & LDO3 (bits 3-0): V_out = 1.8V + (setting * 100mV). Range: 1.8V-3.3V.
    pub fn ldo_2_and_3_voltage_setting(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Ldo2And3VoltageSetting,
        ::device_driver::RW,
    > {
        let address = self.base_address + 40;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Ldo2And3VoltageSetting,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Ldo2And3VoltageSetting::new)
    }
    ///Manages VBUS to IPSOUT path, VHOLD voltage limiting, and VBUS current limiting.
    pub fn vbus_ipsout_path_management(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::VbusIpsoutPathManagement,
        ::device_driver::RW,
    > {
        let address = self.base_address + 48;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::VbusIpsoutPathManagement,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::VbusIpsoutPathManagement::new,
        )
    }
    ///Sets VOFF (shutdown) voltage and PWRON short-press wakeup from sleep.
    pub fn shutdown_voltage_setting(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::ShutdownVoltageSetting,
        ::device_driver::RW,
    > {
        let address = self.base_address + 49;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::ShutdownVoltageSetting,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::ShutdownVoltageSetting::new)
    }
    ///Controls shutdown, battery detection, CHGLED pin, and N_OE shutdown delay.
    pub fn shutdown_bat_chg_led_control(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::ShutdownBatChgLedControl,
        ::device_driver::RW,
    > {
        let address = self.base_address + 50;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::ShutdownBatChgLedControl,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::ShutdownBatChgLedControl::new,
        )
    }
    ///Controls primary charging parameters.
    pub fn charge_control_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::ChargeControl1,
        ::device_driver::RW,
    > {
        let address = self.base_address + 51;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::ChargeControl1,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::ChargeControl1::new)
    }
    ///Controls secondary charging parameters including pre-charge timeout,
    ///external path charge current and enable state, and constant current mode timeout.
    pub fn charge_control_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::ChargeControl2,
        ::device_driver::RW,
    > {
        let address = self.base_address + 52;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::ChargeControl2,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::ChargeControl2::new)
    }
    ///Controls the charging parameters for the backup battery (RTC battery).
    pub fn backup_battery_charge_control(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::BackupBatteryChargeControl,
        ::device_driver::RW,
    > {
        let address = self.base_address + 53;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::BackupBatteryChargeControl,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::BackupBatteryChargeControl::new,
        )
    }
    ///Configures parameters related to the PEK (Power Enable Key/Button) operations.
    pub fn pek_key_parameters(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::PekKeyParameters,
        ::device_driver::RW,
    > {
        let address = self.base_address + 54;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::PekKeyParameters,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::PekKeyParameters::new)
    }
    ///Sets the operating (switching) frequency for the DC-DC converters.
    ///The 4-bit value (bits 0-3) adjusts the frequency around a default of 1.5MHz.
    ///The raw value 0b1000 (decimal 8) corresponds to 1.5MHz.
    ///Each step away from this default value changes the frequency by approximately 5% of 1.5MHz (75kHz).
    ///Range approx 0.9MHz (raw 0) to 2.025MHz (raw 15).
    pub fn dc_dc_operating_frequency(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::DcDcOperatingFrequency,
        ::device_driver::RW,
    > {
        let address = self.base_address + 55;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::DcDcOperatingFrequency,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::DcDcOperatingFrequency::new)
    }
    ///Sets the low-temperature threshold (V_LTF_charge) for battery charging, typically read from an NTC thermistor.
    ///If the NTC voltage rises above this threshold (indicating low temperature), charging may be suspended or modified.
    ///Formula: Threshold Voltage (V) = raw_value * 0.0128.
    ///Range: 0V (raw 0x00) to 3.264V (raw 0xFF).
    pub fn battery_charge_low_temp_threshold(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::BatteryChargeLowTempThreshold,
        ::device_driver::RW,
    > {
        let address = self.base_address + 56;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::BatteryChargeLowTempThreshold,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::BatteryChargeLowTempThreshold::new,
        )
    }
    ///Sets the high-temperature threshold (V_HTF_charge) for battery charging, typically read from an NTC thermistor.
    ///If the NTC voltage falls below this threshold (indicating high temperature), charging may be suspended or modified.
    ///Formula: Threshold Voltage (V) = raw_value * 0.0128.
    ///Range: 0V (raw 0x00) to 3.264V (raw 0xFF).
    pub fn battery_charge_high_temp_threshold(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::BatteryChargeHighTempThreshold,
        ::device_driver::RW,
    > {
        let address = self.base_address + 57;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::BatteryChargeHighTempThreshold,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::BatteryChargeHighTempThreshold::new,
        )
    }
    ///Sets the APS (Average Power Source voltage) low power warning Level 1 threshold.
    ///An IRQ may be triggered if APS voltage drops below this level.
    ///Formula: V_WARNING1 (V) = 2.8672 + (raw_value * 0.0014 * 4).
    ///The raw_value is the 8-bit content of this register.
    pub fn aps_low_power_level_1_setting(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::ApsLowPowerLevel1Setting,
        ::device_driver::RW,
    > {
        let address = self.base_address + 58;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::ApsLowPowerLevel1Setting,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::ApsLowPowerLevel1Setting::new,
        )
    }
    ///Sets the APS (Average Power Source voltage) low power warning Level 2 threshold.
    ///Typically set lower than Level 1. An IRQ may be triggered if APS voltage drops below this level.
    ///Formula: V_WARNING2 (V) = 2.8672 + (raw_value * 0.0014 * 4).
    ///The raw_value is the 8-bit content of this register.
    pub fn aps_low_power_level_2_setting(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::ApsLowPowerLevel2Setting,
        ::device_driver::RW,
    > {
        let address = self.base_address + 59;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::ApsLowPowerLevel2Setting,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::ApsLowPowerLevel2Setting::new,
        )
    }
    ///Sets the low-temperature threshold (V_LTF_discharge) for battery discharging, typically read from an NTC thermistor.
    ///If the NTC voltage rises above this threshold (indicating low temperature), system behavior may be modified.
    ///Formula: Threshold Voltage (V) = raw_value * 0.0128.
    ///Range: 0V (raw 0x00) to 3.264V (raw 0xFF).
    pub fn battery_discharge_low_temp_threshold(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::BatteryDischargeLowTempThreshold,
        ::device_driver::RW,
    > {
        let address = self.base_address + 60;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::BatteryDischargeLowTempThreshold,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::BatteryDischargeLowTempThreshold::new,
        )
    }
    ///Sets the high-temperature threshold (V_HTF_discharge) for battery discharging, typically read from an NTC thermistor.
    ///If the NTC voltage falls below this threshold (indicating high temperature), system behavior may be modified.
    ///Formula: Threshold Voltage (V) = raw_value * 0.0128.
    ///Range: 0V (raw 0x00) to 3.264V (raw 0xFF).
    pub fn battery_discharge_high_temp_threshold(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::BatteryDischargeHighTempThreshold,
        ::device_driver::RW,
    > {
        let address = self.base_address + 61;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::BatteryDischargeHighTempThreshold,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::BatteryDischargeHighTempThreshold::new,
        )
    }
    ///Selects the operating mode (PFM/PWM Auto or Fixed PWM) for DC-DC1, DC-DC2, and DC-DC3.
    pub fn dc_dc_operating_mode(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::DcDcOperatingMode,
        ::device_driver::RW,
    > {
        let address = self.base_address + 128;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::DcDcOperatingMode,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::DcDcOperatingMode::new)
    }
    ///Controls the enable state for various ADC channels (Set 1).
    pub fn adc_enable_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::AdcEnable1,
        ::device_driver::RW,
    > {
        let address = self.base_address + 130;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::AdcEnable1,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::AdcEnable1::new)
    }
    ///Controls the enable state for internal temperature ADC and GPIO ADCs (Set 2).
    pub fn adc_enable_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::AdcEnable2,
        ::device_driver::RW,
    > {
        let address = self.base_address + 131;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::AdcEnable2,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::AdcEnable2::new)
    }
    ///Configures ADC sample rate and TS (Temperature Sense) pin functionality, current, and output mode.
    pub fn adc_sample_rate_ts_pin_control(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::AdcSampleRateTsPinControl,
        ::device_driver::RW,
    > {
        let address = self.base_address + 132;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::AdcSampleRateTsPinControl,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::AdcSampleRateTsPinControl::new,
        )
    }
    ///Sets the ADC input voltage range for GPIO0, GPIO1, GPIO2, and GPIO3.
    pub fn gpio_adc_input_range_setting(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::GpioAdcInputRangeSetting,
        ::device_driver::RW,
    > {
        let address = self.base_address + 133;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::GpioAdcInputRangeSetting,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::GpioAdcInputRangeSetting::new,
        )
    }
}
/// Module containing the generated fieldsets of the registers and commands
pub mod field_sets {
    use super::*;
    ///Indicates the input power source status (ACIN, VBUS), battery current direction,
    ///and other power-related conditions.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PowerStatus {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for PowerStatus {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl PowerStatus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `acin_present` field of the register.
        ///
        ///ACIN physical presence status (true: voltage detected, false: no voltage detected).
        pub fn acin_present(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `acin_usable` field of the register.
        ///
        ///ACIN usability status (true: ACIN voltage is valid for system use, false: ACIN voltage is not valid/sufficient).
        pub fn acin_usable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `vbus_present` field of the register.
        ///
        ///VBUS physical presence status (true: voltage detected, false: no voltage detected).
        pub fn vbus_present(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `vbus_usable` field of the register.
        ///
        ///VBUS usability status (true: VBUS voltage is valid for system use, false: VBUS voltage is not valid/sufficient).
        pub fn vbus_usable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `vbus_voltage_above_vhold_when_connected` field of the register.
        ///
        ///Indicates if VBUS voltage was greater than VHOLD when VBUS was connected (true: VBUS > VHOLD, false: VBUS <= VHOLD or not applicable).
        pub fn vbus_voltage_above_vhold_when_connected(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `battery_flow` field of the register.
        ///
        ///Battery current direction.
        pub fn battery_flow(&self) -> super::BatteryFlowDirection {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `acin_and_vbus_inputs_shorted_on_pcb` field of the register.
        ///
        ///Indicates if ACIN and VBUS inputs are short-circuited on the PCB (true: short-circuited detected, false: no short-circuit).
        pub fn acin_and_vbus_inputs_shorted_on_pcb(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `boot_source_was_acin_or_vbus` field of the register.
        ///
        ///Indicates if the boot-up power source was ACIN or VBUS (true: boot from ACIN/VBUS, false: boot from Battery or other).
        pub fn boot_source_was_acin_or_vbus(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `acin_present` field of the register.
        ///
        ///ACIN physical presence status (true: voltage detected, false: no voltage detected).
        pub fn set_acin_present(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `acin_usable` field of the register.
        ///
        ///ACIN usability status (true: ACIN voltage is valid for system use, false: ACIN voltage is not valid/sufficient).
        pub fn set_acin_usable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `vbus_present` field of the register.
        ///
        ///VBUS physical presence status (true: voltage detected, false: no voltage detected).
        pub fn set_vbus_present(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `vbus_usable` field of the register.
        ///
        ///VBUS usability status (true: VBUS voltage is valid for system use, false: VBUS voltage is not valid/sufficient).
        pub fn set_vbus_usable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `vbus_voltage_above_vhold_when_connected` field of the register.
        ///
        ///Indicates if VBUS voltage was greater than VHOLD when VBUS was connected (true: VBUS > VHOLD, false: VBUS <= VHOLD or not applicable).
        pub fn set_vbus_voltage_above_vhold_when_connected(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `battery_flow` field of the register.
        ///
        ///Battery current direction.
        pub fn set_battery_flow(&mut self, value: super::BatteryFlowDirection) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `acin_and_vbus_inputs_shorted_on_pcb` field of the register.
        ///
        ///Indicates if ACIN and VBUS inputs are short-circuited on the PCB (true: short-circuited detected, false: no short-circuit).
        pub fn set_acin_and_vbus_inputs_shorted_on_pcb(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `boot_source_was_acin_or_vbus` field of the register.
        ///
        ///Indicates if the boot-up power source was ACIN or VBUS (true: boot from ACIN/VBUS, false: boot from Battery or other).
        pub fn set_boot_source_was_acin_or_vbus(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for PowerStatus {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<PowerStatus> for [u8; 1] {
        fn from(val: PowerStatus) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for PowerStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("PowerStatus");
            {
                d.field("acin_present", &self.acin_present());
            }
            {
                d.field("acin_usable", &self.acin_usable());
            }
            {
                d.field("vbus_present", &self.vbus_present());
            }
            {
                d.field("vbus_usable", &self.vbus_usable());
            }
            {
                d.field(
                    "vbus_voltage_above_vhold_when_connected",
                    &self.vbus_voltage_above_vhold_when_connected(),
                );
            }
            {
                d.field("battery_flow", &self.battery_flow());
            }
            {
                d.field(
                    "acin_and_vbus_inputs_shorted_on_pcb",
                    &self.acin_and_vbus_inputs_shorted_on_pcb(),
                );
            }
            {
                d.field(
                    "boot_source_was_acin_or_vbus",
                    &self.boot_source_was_acin_or_vbus(),
                );
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PowerStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PowerStatus { ");
            defmt::write!(f, "acin_present: {=bool}, ", &self.acin_present());
            defmt::write!(f, "acin_usable: {=bool}, ", &self.acin_usable());
            defmt::write!(f, "vbus_present: {=bool}, ", &self.vbus_present());
            defmt::write!(f, "vbus_usable: {=bool}, ", &self.vbus_usable());
            defmt::write!(
                f,
                "vbus_voltage_above_vhold_when_connected: {=bool}, ",
                &self.vbus_voltage_above_vhold_when_connected(),
            );
            defmt::write!(f, "battery_flow: {}, ", &self.battery_flow());
            defmt::write!(
                f,
                "acin_and_vbus_inputs_shorted_on_pcb: {=bool}, ",
                &self.acin_and_vbus_inputs_shorted_on_pcb(),
            );
            defmt::write!(
                f,
                "boot_source_was_acin_or_vbus: {=bool}, ",
                &self.boot_source_was_acin_or_vbus(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for PowerStatus {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for PowerStatus {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for PowerStatus {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for PowerStatus {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for PowerStatus {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for PowerStatus {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for PowerStatus {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Indicates various operational modes and charging statuses, including AXP192 temperature,
    ///battery presence, and charging progress.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeStatus {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeStatus {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeStatus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `axp_192_over_temperature` field of the register.
        ///
        ///AXP192 over-temperature status (true: over-temperature, false: normal temperature).
        pub fn axp_192_over_temperature(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `charging_in_progress` field of the register.
        ///
        ///Charging progress status (true: charging is in progress, false: not charging or charge complete).
        pub fn charging_in_progress(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `battery_connected` field of the register.
        ///
        ///Battery connection status (true: battery is connected, false: no battery connected).
        pub fn battery_connected(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `battery_in_activation_mode` field of the register.
        ///
        ///Battery activation mode status (true: battery is in activation mode, false: not in activation mode).
        pub fn battery_in_activation_mode(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `charge_current_less_than_expected` field of the register.
        ///
        ///Indicates if actual charging current is less than the set/expected current (true: actual < expected, false: actual >= expected).
        pub fn charge_current_less_than_expected(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `startup_mode` field of the register.
        ///
        ///AXP192 startup mode.
        pub fn startup_mode(&self) -> super::AxpStartupMode {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `axp_192_over_temperature` field of the register.
        ///
        ///AXP192 over-temperature status (true: over-temperature, false: normal temperature).
        pub fn set_axp_192_over_temperature(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `charging_in_progress` field of the register.
        ///
        ///Charging progress status (true: charging is in progress, false: not charging or charge complete).
        pub fn set_charging_in_progress(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `battery_connected` field of the register.
        ///
        ///Battery connection status (true: battery is connected, false: no battery connected).
        pub fn set_battery_connected(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `battery_in_activation_mode` field of the register.
        ///
        ///Battery activation mode status (true: battery is in activation mode, false: not in activation mode).
        pub fn set_battery_in_activation_mode(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `charge_current_less_than_expected` field of the register.
        ///
        ///Indicates if actual charging current is less than the set/expected current (true: actual < expected, false: actual >= expected).
        pub fn set_charge_current_less_than_expected(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `startup_mode` field of the register.
        ///
        ///AXP192 startup mode.
        pub fn set_startup_mode(&mut self, value: super::AxpStartupMode) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for ChargeStatus {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeStatus> for [u8; 1] {
        fn from(val: ChargeStatus) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeStatus");
            {
                d.field("axp_192_over_temperature", &self.axp_192_over_temperature());
            }
            {
                d.field("charging_in_progress", &self.charging_in_progress());
            }
            {
                d.field("battery_connected", &self.battery_connected());
            }
            {
                d.field(
                    "battery_in_activation_mode",
                    &self.battery_in_activation_mode(),
                );
            }
            {
                d.field(
                    "charge_current_less_than_expected",
                    &self.charge_current_less_than_expected(),
                );
            }
            {
                d.field("startup_mode", &self.startup_mode());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ChargeStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeStatus { ");
            defmt::write!(
                f,
                "axp_192_over_temperature: {=bool}, ",
                &self.axp_192_over_temperature(),
            );
            defmt::write!(
                f,
                "charging_in_progress: {=bool}, ",
                &self.charging_in_progress(),
            );
            defmt::write!(f, "battery_connected: {=bool}, ", &self.battery_connected());
            defmt::write!(
                f,
                "battery_in_activation_mode: {=bool}, ",
                &self.battery_in_activation_mode(),
            );
            defmt::write!(
                f,
                "charge_current_less_than_expected: {=bool}, ",
                &self.charge_current_less_than_expected(),
            );
            defmt::write!(f, "startup_mode: {}, ", &self.startup_mode());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for ChargeStatus {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeStatus {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeStatus {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeStatus {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeStatus {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeStatus {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeStatus {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Indicates the status of VBUS when operating in USB OTG (On-The-Go) mode.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OtgVbusStatus {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for OtgVbusStatus {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl OtgVbusStatus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `vbus_is_valid` field of the register.
        ///
        ///VBUS validity status in OTG mode (true: VBUS is valid, false: VBUS is not valid).
        pub fn vbus_is_valid(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `vbus_session_ab_is_valid` field of the register.
        ///
        ///VBUS Session A/B validity status (true: Session A/B is valid, false: Session A/B is not valid).
        pub fn vbus_session_ab_is_valid(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `vbus_session_ended` field of the register.
        ///
        ///VBUS Session End status (true: Session has ended, false: Session is ongoing or not started).
        pub fn vbus_session_ended(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `vbus_is_valid` field of the register.
        ///
        ///VBUS validity status in OTG mode (true: VBUS is valid, false: VBUS is not valid).
        pub fn set_vbus_is_valid(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `vbus_session_ab_is_valid` field of the register.
        ///
        ///VBUS Session A/B validity status (true: Session A/B is valid, false: Session A/B is not valid).
        pub fn set_vbus_session_ab_is_valid(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `vbus_session_ended` field of the register.
        ///
        ///VBUS Session End status (true: Session has ended, false: Session is ongoing or not started).
        pub fn set_vbus_session_ended(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for OtgVbusStatus {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<OtgVbusStatus> for [u8; 1] {
        fn from(val: OtgVbusStatus) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for OtgVbusStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("OtgVbusStatus");
            {
                d.field("vbus_is_valid", &self.vbus_is_valid());
            }
            {
                d.field("vbus_session_ab_is_valid", &self.vbus_session_ab_is_valid());
            }
            {
                d.field("vbus_session_ended", &self.vbus_session_ended());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OtgVbusStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OtgVbusStatus { ");
            defmt::write!(f, "vbus_is_valid: {=bool}, ", &self.vbus_is_valid());
            defmt::write!(
                f,
                "vbus_session_ab_is_valid: {=bool}, ",
                &self.vbus_session_ab_is_valid(),
            );
            defmt::write!(
                f,
                "vbus_session_ended: {=bool}, ",
                &self.vbus_session_ended(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for OtgVbusStatus {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for OtgVbusStatus {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for OtgVbusStatus {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for OtgVbusStatus {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for OtgVbusStatus {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for OtgVbusStatus {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for OtgVbusStatus {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///General purpose data buffer for system data storage.
    ///Note: Retained as long as any AXP192 power source is present (not affected by host on/off).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DataBuffer0 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for DataBuffer0 {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl DataBuffer0 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [240] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Create a new instance, loaded with the reset value of the `DataBuffer1` ref
        pub const fn new_as_data_buffer_1() -> Self {
            Self { bits: [15] }
        }
        ///Create a new instance, loaded with the reset value of the `DataBuffer2` ref
        pub const fn new_as_data_buffer_2() -> Self {
            Self { bits: [0] }
        }
        ///Create a new instance, loaded with the reset value of the `DataBuffer3` ref
        pub const fn new_as_data_buffer_3() -> Self {
            Self { bits: [255] }
        }
        ///Create a new instance, loaded with the reset value of the `DataBuffer4` ref
        pub const fn new_as_data_buffer_4() -> Self {
            Self { bits: [0] }
        }
        ///Create a new instance, loaded with the reset value of the `DataBuffer5` ref
        pub const fn new_as_data_buffer_5() -> Self {
            Self { bits: [0] }
        }
        ///Read the `value` field of the register.
        ///
        ///8-bit data value.
        pub fn value(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 8)
            };
            raw
        }
        ///Write the `value` field of the register.
        ///
        ///8-bit data value.
        pub fn set_value(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 8, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for DataBuffer0 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<DataBuffer0> for [u8; 1] {
        fn from(val: DataBuffer0) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for DataBuffer0 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("DataBuffer0");
            {
                d.field("value", &self.value());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DataBuffer0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DataBuffer0 { ");
            defmt::write!(f, "value: {=u8}, ", &self.value());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for DataBuffer0 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for DataBuffer0 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for DataBuffer0 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for DataBuffer0 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for DataBuffer0 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for DataBuffer0 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for DataBuffer0 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Controls the output enable state for the EXTEN pin and the DC-DC2 converter.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExtenDcDc2Control {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ExtenDcDc2Control {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ExtenDcDc2Control {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [5] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `exten_output_enable` field of the register.
        ///
        ///EXTEN pin output switch control (true: enabled/on, false: disabled/off).
        pub fn exten_output_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `dcdc_2_output_enable` field of the register.
        ///
        ///DC-DC2 converter output switch control (true: enabled/on, false: disabled/off).
        pub fn dcdc_2_output_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `exten_output_enable` field of the register.
        ///
        ///EXTEN pin output switch control (true: enabled/on, false: disabled/off).
        pub fn set_exten_output_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `dcdc_2_output_enable` field of the register.
        ///
        ///DC-DC2 converter output switch control (true: enabled/on, false: disabled/off).
        pub fn set_dcdc_2_output_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for ExtenDcDc2Control {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ExtenDcDc2Control> for [u8; 1] {
        fn from(val: ExtenDcDc2Control) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ExtenDcDc2Control {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ExtenDcDc2Control");
            {
                d.field("exten_output_enable", &self.exten_output_enable());
            }
            {
                d.field("dcdc_2_output_enable", &self.dcdc_2_output_enable());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ExtenDcDc2Control {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ExtenDcDc2Control { ");
            defmt::write!(
                f,
                "exten_output_enable: {=bool}, ",
                &self.exten_output_enable(),
            );
            defmt::write!(
                f,
                "dcdc_2_output_enable: {=bool}, ",
                &self.dcdc_2_output_enable(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for ExtenDcDc2Control {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ExtenDcDc2Control {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ExtenDcDc2Control {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ExtenDcDc2Control {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ExtenDcDc2Control {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ExtenDcDc2Control {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ExtenDcDc2Control {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Controls output enable for DC-DC1, DC-DC3, LDO2, LDO3, EXTEN, and DC-DC2.
    ///Power-on default for M5StickC Plus is 0x5F.
    ///IMPORTANT: REG12H bit 6 (exten_output_enable) is linked to REG10H bit 2.
    ///             REG12H bit 4 (dcdc2_output_enable) is linked to REG10H bit 0.
    ///             These pairs control the same underlying hardware function.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PowerOutputControl {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for PowerOutputControl {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl PowerOutputControl {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [95] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `exten_output_enable` field of the register.
        ///
        ///EXTEN pin output switch control (true: enabled, false: disabled). Linked to REG10H[2].
        pub fn exten_output_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `dcdc_2_output_enable` field of the register.
        ///
        ///DC-DC2 converter output switch control (true: enabled, false: disabled). Linked to REG10H[0].
        pub fn dcdc_2_output_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `ldo_3_output_enable` field of the register.
        ///
        ///LDO3 output switch control (true: enabled, false: disabled).
        pub fn ldo_3_output_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `ldo_2_output_enable` field of the register.
        ///
        ///LDO2 output switch control (true: enabled, false: disabled).
        pub fn ldo_2_output_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `dcdc_3_output_enable` field of the register.
        ///
        ///DC-DC3 converter output switch control (true: enabled, false: disabled).
        pub fn dcdc_3_output_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `dcdc_1_output_enable` field of the register.
        ///
        ///DC-DC1 converter output switch control (true: enabled, false: disabled).
        pub fn dcdc_1_output_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `exten_output_enable` field of the register.
        ///
        ///EXTEN pin output switch control (true: enabled, false: disabled). Linked to REG10H[2].
        pub fn set_exten_output_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `dcdc_2_output_enable` field of the register.
        ///
        ///DC-DC2 converter output switch control (true: enabled, false: disabled). Linked to REG10H[0].
        pub fn set_dcdc_2_output_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `ldo_3_output_enable` field of the register.
        ///
        ///LDO3 output switch control (true: enabled, false: disabled).
        pub fn set_ldo_3_output_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `ldo_2_output_enable` field of the register.
        ///
        ///LDO2 output switch control (true: enabled, false: disabled).
        pub fn set_ldo_2_output_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `dcdc_3_output_enable` field of the register.
        ///
        ///DC-DC3 converter output switch control (true: enabled, false: disabled).
        pub fn set_dcdc_3_output_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `dcdc_1_output_enable` field of the register.
        ///
        ///DC-DC1 converter output switch control (true: enabled, false: disabled).
        pub fn set_dcdc_1_output_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for PowerOutputControl {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<PowerOutputControl> for [u8; 1] {
        fn from(val: PowerOutputControl) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for PowerOutputControl {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("PowerOutputControl");
            {
                d.field("exten_output_enable", &self.exten_output_enable());
            }
            {
                d.field("dcdc_2_output_enable", &self.dcdc_2_output_enable());
            }
            {
                d.field("ldo_3_output_enable", &self.ldo_3_output_enable());
            }
            {
                d.field("ldo_2_output_enable", &self.ldo_2_output_enable());
            }
            {
                d.field("dcdc_3_output_enable", &self.dcdc_3_output_enable());
            }
            {
                d.field("dcdc_1_output_enable", &self.dcdc_1_output_enable());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PowerOutputControl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PowerOutputControl { ");
            defmt::write!(
                f,
                "exten_output_enable: {=bool}, ",
                &self.exten_output_enable(),
            );
            defmt::write!(
                f,
                "dcdc_2_output_enable: {=bool}, ",
                &self.dcdc_2_output_enable(),
            );
            defmt::write!(
                f,
                "ldo_3_output_enable: {=bool}, ",
                &self.ldo_3_output_enable(),
            );
            defmt::write!(
                f,
                "ldo_2_output_enable: {=bool}, ",
                &self.ldo_2_output_enable(),
            );
            defmt::write!(
                f,
                "dcdc_3_output_enable: {=bool}, ",
                &self.dcdc_3_output_enable(),
            );
            defmt::write!(
                f,
                "dcdc_1_output_enable: {=bool}, ",
                &self.dcdc_1_output_enable(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for PowerOutputControl {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for PowerOutputControl {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for PowerOutputControl {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for PowerOutputControl {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for PowerOutputControl {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for PowerOutputControl {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for PowerOutputControl {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Sets the output voltage for the DC-DC2 converter.
    ///Formula: Output Voltage (V) = 0.7 + (value * 0.025). Range: 0.7V to 2.275V (raw 0-63).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DcDc2VoltageSetting {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for DcDc2VoltageSetting {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl DcDc2VoltageSetting {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [22] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `voltage_setting` field of the register.
        ///
        pub fn voltage_setting(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 6)
            };
            raw
        }
        ///Write the `voltage_setting` field of the register.
        ///
        pub fn set_voltage_setting(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 6, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for DcDc2VoltageSetting {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<DcDc2VoltageSetting> for [u8; 1] {
        fn from(val: DcDc2VoltageSetting) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for DcDc2VoltageSetting {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("DcDc2VoltageSetting");
            {
                d.field("voltage_setting", &self.voltage_setting());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DcDc2VoltageSetting {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DcDc2VoltageSetting { ");
            defmt::write!(f, "voltage_setting: {=u8}, ", &self.voltage_setting());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for DcDc2VoltageSetting {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for DcDc2VoltageSetting {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for DcDc2VoltageSetting {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for DcDc2VoltageSetting {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for DcDc2VoltageSetting {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for DcDc2VoltageSetting {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for DcDc2VoltageSetting {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Configures DC-DC2 Dynamic Voltage Scaling / Voltage Ramp Control (VRC).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DcDc2VrcParameter {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for DcDc2VrcParameter {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl DcDc2VrcParameter {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `vrc_disabled` field of the register.
        ///
        ///DC-DC2 VRC disable control (true: VRC disabled, false: VRC enabled). Note inverted logic.
        pub fn vrc_disabled(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `vrc_rise_slope` field of the register.
        ///
        ///DC-DC2 VRC voltage rise slope.
        pub fn vrc_rise_slope(&self) -> super::VrcRiseSlope {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `vrc_disabled` field of the register.
        ///
        ///DC-DC2 VRC disable control (true: VRC disabled, false: VRC enabled). Note inverted logic.
        pub fn set_vrc_disabled(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `vrc_rise_slope` field of the register.
        ///
        ///DC-DC2 VRC voltage rise slope.
        pub fn set_vrc_rise_slope(&mut self, value: super::VrcRiseSlope) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for DcDc2VrcParameter {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<DcDc2VrcParameter> for [u8; 1] {
        fn from(val: DcDc2VrcParameter) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for DcDc2VrcParameter {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("DcDc2VrcParameter");
            {
                d.field("vrc_disabled", &self.vrc_disabled());
            }
            {
                d.field("vrc_rise_slope", &self.vrc_rise_slope());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DcDc2VrcParameter {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DcDc2VrcParameter { ");
            defmt::write!(f, "vrc_disabled: {=bool}, ", &self.vrc_disabled());
            defmt::write!(f, "vrc_rise_slope: {}, ", &self.vrc_rise_slope());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for DcDc2VrcParameter {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for DcDc2VrcParameter {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for DcDc2VrcParameter {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for DcDc2VrcParameter {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for DcDc2VrcParameter {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for DcDc2VrcParameter {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for DcDc2VrcParameter {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Sets the output voltage for a DC-DC converter.
    ///Formula: V_out = 0.7V + (value * 25mV). Range 0.7V-3.5V (raw 0-112).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DcDc1VoltageSetting {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for DcDc1VoltageSetting {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl DcDc1VoltageSetting {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [104] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Create a new instance, loaded with the reset value of the `DcDc3VoltageSetting` ref
        pub const fn new_as_dc_dc_3_voltage_setting() -> Self {
            Self { bits: [72] }
        }
        ///Read the `voltage_setting` field of the register.
        ///
        pub fn voltage_setting(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 7)
            };
            raw
        }
        ///Write the `voltage_setting` field of the register.
        ///
        pub fn set_voltage_setting(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 7, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for DcDc1VoltageSetting {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<DcDc1VoltageSetting> for [u8; 1] {
        fn from(val: DcDc1VoltageSetting) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for DcDc1VoltageSetting {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("DcDc1VoltageSetting");
            {
                d.field("voltage_setting", &self.voltage_setting());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DcDc1VoltageSetting {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DcDc1VoltageSetting { ");
            defmt::write!(f, "voltage_setting: {=u8}, ", &self.voltage_setting());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for DcDc1VoltageSetting {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for DcDc1VoltageSetting {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for DcDc1VoltageSetting {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for DcDc1VoltageSetting {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for DcDc1VoltageSetting {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for DcDc1VoltageSetting {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for DcDc1VoltageSetting {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Sets the output voltage for both LDO2 and LDO3.
    ///LDO2 (bits 7-4) & LDO3 (bits 3-0): V_out = 1.8V + (setting * 100mV). Range: 1.8V-3.3V.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ldo2And3VoltageSetting {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Ldo2And3VoltageSetting {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Ldo2And3VoltageSetting {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [207] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `ldo_2_voltage_setting` field of the register.
        ///
        ///4-bit setting for LDO2 output voltage (1.8V to 3.3V, 100mV/step).
        pub fn ldo_2_voltage_setting(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 8)
            };
            raw
        }
        ///Read the `ldo_3_voltage_setting` field of the register.
        ///
        ///4-bit setting for LDO3 output voltage (1.8V to 3.3V, 100mV/step).
        pub fn ldo_3_voltage_setting(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 4)
            };
            raw
        }
        ///Write the `ldo_2_voltage_setting` field of the register.
        ///
        ///4-bit setting for LDO2 output voltage (1.8V to 3.3V, 100mV/step).
        pub fn set_ldo_2_voltage_setting(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 8, &mut self.bits)
            };
        }
        ///Write the `ldo_3_voltage_setting` field of the register.
        ///
        ///4-bit setting for LDO3 output voltage (1.8V to 3.3V, 100mV/step).
        pub fn set_ldo_3_voltage_setting(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 4, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Ldo2And3VoltageSetting {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Ldo2And3VoltageSetting> for [u8; 1] {
        fn from(val: Ldo2And3VoltageSetting) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Ldo2And3VoltageSetting {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Ldo2And3VoltageSetting");
            {
                d.field("ldo_2_voltage_setting", &self.ldo_2_voltage_setting());
            }
            {
                d.field("ldo_3_voltage_setting", &self.ldo_3_voltage_setting());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ldo2And3VoltageSetting {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ldo2And3VoltageSetting { ");
            defmt::write!(
                f,
                "ldo_2_voltage_setting: {=u8}, ",
                &self.ldo_2_voltage_setting(),
            );
            defmt::write!(
                f,
                "ldo_3_voltage_setting: {=u8}, ",
                &self.ldo_3_voltage_setting(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Ldo2And3VoltageSetting {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Ldo2And3VoltageSetting {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Ldo2And3VoltageSetting {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Ldo2And3VoltageSetting {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Ldo2And3VoltageSetting {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Ldo2And3VoltageSetting {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Ldo2And3VoltageSetting {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Manages VBUS to IPSOUT path, VHOLD voltage limiting, and VBUS current limiting.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VbusIpsoutPathManagement {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for VbusIpsoutPathManagement {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl VbusIpsoutPathManagement {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [128] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `path_selection_override` field of the register.
        ///
        ///VBUS-IPSOUT path selection control.
        pub fn path_selection_override(&self) -> super::VbusPathSelectionControl {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `vhold_limit_enabled` field of the register.
        ///
        ///VBUS VHOLD voltage limiting control (true: limit enabled, false: no limit).
        pub fn vhold_limit_enabled(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `vhold_voltage` field of the register.
        ///
        ///VHOLD voltage setting when vhold_limit_enabled is true.
        pub fn vhold_voltage(&self) -> super::VholdVoltageValue {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 6)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `vbus_current_limit_enabled` field of the register.
        ///
        ///VBUS current limit control (true: enabled, false: disabled).
        pub fn vbus_current_limit_enabled(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `vbus_current_limit` field of the register.
        ///
        ///VBUS current limit selection when vbus_current_limit_enabled is true.
        pub fn vbus_current_limit(&self) -> super::VbusCurrentLimitValue {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `path_selection_override` field of the register.
        ///
        ///VBUS-IPSOUT path selection control.
        pub fn set_path_selection_override(
            &mut self,
            value: super::VbusPathSelectionControl,
        ) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `vhold_limit_enabled` field of the register.
        ///
        ///VBUS VHOLD voltage limiting control (true: limit enabled, false: no limit).
        pub fn set_vhold_limit_enabled(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `vhold_voltage` field of the register.
        ///
        ///VHOLD voltage setting when vhold_limit_enabled is true.
        pub fn set_vhold_voltage(&mut self, value: super::VholdVoltageValue) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 6, &mut self.bits)
            };
        }
        ///Write the `vbus_current_limit_enabled` field of the register.
        ///
        ///VBUS current limit control (true: enabled, false: disabled).
        pub fn set_vbus_current_limit_enabled(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `vbus_current_limit` field of the register.
        ///
        ///VBUS current limit selection when vbus_current_limit_enabled is true.
        pub fn set_vbus_current_limit(&mut self, value: super::VbusCurrentLimitValue) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for VbusIpsoutPathManagement {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<VbusIpsoutPathManagement> for [u8; 1] {
        fn from(val: VbusIpsoutPathManagement) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for VbusIpsoutPathManagement {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("VbusIpsoutPathManagement");
            {
                d.field("path_selection_override", &self.path_selection_override());
            }
            {
                d.field("vhold_limit_enabled", &self.vhold_limit_enabled());
            }
            {
                d.field("vhold_voltage", &self.vhold_voltage());
            }
            {
                d.field(
                    "vbus_current_limit_enabled",
                    &self.vbus_current_limit_enabled(),
                );
            }
            {
                d.field("vbus_current_limit", &self.vbus_current_limit());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VbusIpsoutPathManagement {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "VbusIpsoutPathManagement { ");
            defmt::write!(
                f,
                "path_selection_override: {}, ",
                &self.path_selection_override(),
            );
            defmt::write!(
                f,
                "vhold_limit_enabled: {=bool}, ",
                &self.vhold_limit_enabled(),
            );
            defmt::write!(f, "vhold_voltage: {}, ", &self.vhold_voltage());
            defmt::write!(
                f,
                "vbus_current_limit_enabled: {=bool}, ",
                &self.vbus_current_limit_enabled(),
            );
            defmt::write!(f, "vbus_current_limit: {}, ", &self.vbus_current_limit());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for VbusIpsoutPathManagement {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for VbusIpsoutPathManagement {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for VbusIpsoutPathManagement {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for VbusIpsoutPathManagement {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for VbusIpsoutPathManagement {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for VbusIpsoutPathManagement {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for VbusIpsoutPathManagement {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Sets VOFF (shutdown) voltage and PWRON short-press wakeup from sleep.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ShutdownVoltageSetting {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ShutdownVoltageSetting {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ShutdownVoltageSetting {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [3] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `sleep_pwron_short_press_wakeup_enable` field of the register.
        ///
        ///Enable PWRON short press to wake from sleep (true: enabled, false: disabled). Auto-clears.
        pub fn sleep_pwron_short_press_wakeup_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `voff_voltage` field of the register.
        ///
        ///VOFF (shutdown) voltage threshold.
        pub fn voff_voltage(&self) -> super::VoffVoltageValue {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 3)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `sleep_pwron_short_press_wakeup_enable` field of the register.
        ///
        ///Enable PWRON short press to wake from sleep (true: enabled, false: disabled). Auto-clears.
        pub fn set_sleep_pwron_short_press_wakeup_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `voff_voltage` field of the register.
        ///
        ///VOFF (shutdown) voltage threshold.
        pub fn set_voff_voltage(&mut self, value: super::VoffVoltageValue) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 3, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for ShutdownVoltageSetting {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ShutdownVoltageSetting> for [u8; 1] {
        fn from(val: ShutdownVoltageSetting) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ShutdownVoltageSetting {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ShutdownVoltageSetting");
            {
                d.field(
                    "sleep_pwron_short_press_wakeup_enable",
                    &self.sleep_pwron_short_press_wakeup_enable(),
                );
            }
            {
                d.field("voff_voltage", &self.voff_voltage());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ShutdownVoltageSetting {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ShutdownVoltageSetting { ");
            defmt::write!(
                f,
                "sleep_pwron_short_press_wakeup_enable: {=bool}, ",
                &self.sleep_pwron_short_press_wakeup_enable(),
            );
            defmt::write!(f, "voff_voltage: {}, ", &self.voff_voltage());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for ShutdownVoltageSetting {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ShutdownVoltageSetting {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ShutdownVoltageSetting {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ShutdownVoltageSetting {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ShutdownVoltageSetting {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ShutdownVoltageSetting {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ShutdownVoltageSetting {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Controls shutdown, battery detection, CHGLED pin, and N_OE shutdown delay.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ShutdownBatChgLedControl {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ShutdownBatChgLedControl {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ShutdownBatChgLedControl {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [70] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `request_shutdown_mode_a` field of the register.
        ///
        ///Initiate AXP192 shutdown in Mode A (true: request shutdown).
        pub fn request_shutdown_mode_a(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `battery_monitoring_enable` field of the register.
        ///
        ///Battery monitoring function (true: enabled, false: disabled).
        pub fn battery_monitoring_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `chgled_function` field of the register.
        ///
        ///CHGLED pin function when chgled_control_source is ByRegister.
        pub fn chgled_function(&self) -> super::ChgLedFunctionSetting {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 6)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `chgled_control_source` field of the register.
        ///
        ///CHGLED pin control source.
        pub fn chgled_control_source(&self) -> super::ChgLedControlSourceSelect {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `n_oe_shutdown_delay` field of the register.
        ///
        ///Shutdown delay after N_OE pin transitions from low to high.
        pub fn n_oe_shutdown_delay(&self) -> super::NoeShutdownDelayValue {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 2)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `request_shutdown_mode_a` field of the register.
        ///
        ///Initiate AXP192 shutdown in Mode A (true: request shutdown).
        pub fn set_request_shutdown_mode_a(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `battery_monitoring_enable` field of the register.
        ///
        ///Battery monitoring function (true: enabled, false: disabled).
        pub fn set_battery_monitoring_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `chgled_function` field of the register.
        ///
        ///CHGLED pin function when chgled_control_source is ByRegister.
        pub fn set_chgled_function(&mut self, value: super::ChgLedFunctionSetting) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 6, &mut self.bits)
            };
        }
        ///Write the `chgled_control_source` field of the register.
        ///
        ///CHGLED pin control source.
        pub fn set_chgled_control_source(
            &mut self,
            value: super::ChgLedControlSourceSelect,
        ) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `n_oe_shutdown_delay` field of the register.
        ///
        ///Shutdown delay after N_OE pin transitions from low to high.
        pub fn set_n_oe_shutdown_delay(&mut self, value: super::NoeShutdownDelayValue) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 2, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for ShutdownBatChgLedControl {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ShutdownBatChgLedControl> for [u8; 1] {
        fn from(val: ShutdownBatChgLedControl) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ShutdownBatChgLedControl {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ShutdownBatChgLedControl");
            {
                d.field("request_shutdown_mode_a", &self.request_shutdown_mode_a());
            }
            {
                d.field("battery_monitoring_enable", &self.battery_monitoring_enable());
            }
            {
                d.field("chgled_function", &self.chgled_function());
            }
            {
                d.field("chgled_control_source", &self.chgled_control_source());
            }
            {
                d.field("n_oe_shutdown_delay", &self.n_oe_shutdown_delay());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ShutdownBatChgLedControl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ShutdownBatChgLedControl { ");
            defmt::write!(
                f,
                "request_shutdown_mode_a: {=bool}, ",
                &self.request_shutdown_mode_a(),
            );
            defmt::write!(
                f,
                "battery_monitoring_enable: {=bool}, ",
                &self.battery_monitoring_enable(),
            );
            defmt::write!(f, "chgled_function: {}, ", &self.chgled_function());
            defmt::write!(
                f,
                "chgled_control_source: {}, ",
                &self.chgled_control_source(),
            );
            defmt::write!(f, "n_oe_shutdown_delay: {}, ", &self.n_oe_shutdown_delay());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for ShutdownBatChgLedControl {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ShutdownBatChgLedControl {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ShutdownBatChgLedControl {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ShutdownBatChgLedControl {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ShutdownBatChgLedControl {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ShutdownBatChgLedControl {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ShutdownBatChgLedControl {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Controls primary charging parameters.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeControl1 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeControl1 {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeControl1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [200] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `charge_enable` field of the register.
        ///
        ///Charging function enable (true: enabled, false: disabled).
        pub fn charge_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `target_voltage` field of the register.
        ///
        ///Charging target voltage setting.
        pub fn target_voltage(&self) -> super::ChargeTargetVoltageValue {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 7)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `end_current_threshold` field of the register.
        ///
        ///Charge termination current threshold.
        pub fn end_current_threshold(&self) -> super::ChargeEndCurrentThresholdValue {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `charge_current` field of the register.
        ///
        ///Internal path charging current setting.
        pub fn charge_current(&self) -> super::ChargeCurrentValue {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 4)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `charge_enable` field of the register.
        ///
        ///Charging function enable (true: enabled, false: disabled).
        pub fn set_charge_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `target_voltage` field of the register.
        ///
        ///Charging target voltage setting.
        pub fn set_target_voltage(&mut self, value: super::ChargeTargetVoltageValue) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 7, &mut self.bits)
            };
        }
        ///Write the `end_current_threshold` field of the register.
        ///
        ///Charge termination current threshold.
        pub fn set_end_current_threshold(
            &mut self,
            value: super::ChargeEndCurrentThresholdValue,
        ) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `charge_current` field of the register.
        ///
        ///Internal path charging current setting.
        pub fn set_charge_current(&mut self, value: super::ChargeCurrentValue) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 4, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for ChargeControl1 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeControl1> for [u8; 1] {
        fn from(val: ChargeControl1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeControl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeControl1");
            {
                d.field("charge_enable", &self.charge_enable());
            }
            {
                d.field("target_voltage", &self.target_voltage());
            }
            {
                d.field("end_current_threshold", &self.end_current_threshold());
            }
            {
                d.field("charge_current", &self.charge_current());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ChargeControl1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeControl1 { ");
            defmt::write!(f, "charge_enable: {=bool}, ", &self.charge_enable());
            defmt::write!(f, "target_voltage: {}, ", &self.target_voltage());
            defmt::write!(
                f,
                "end_current_threshold: {}, ",
                &self.end_current_threshold(),
            );
            defmt::write!(f, "charge_current: {}, ", &self.charge_current());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for ChargeControl1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeControl1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeControl1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeControl1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeControl1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeControl1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeControl1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Controls secondary charging parameters including pre-charge timeout,
    ///external path charge current and enable state, and constant current mode timeout.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeControl2 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeControl2 {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeControl2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [65] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `precharge_timeout` field of the register.
        ///
        ///Pre-charge phase timeout duration.
        pub fn precharge_timeout(&self) -> super::PrechargeTimeoutValue {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 8)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `external_path_charge_current` field of the register.
        ///
        ///External path charging current setting (300mA to 1000mA, 100mA/step).
        pub fn external_path_charge_current(
            &self,
        ) -> super::ExternalPathChargeCurrentValue {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 6)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `external_path_charge_enable` field of the register.
        ///
        ///External path charging enable (true: enabled, false: disabled).
        pub fn external_path_charge_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `constant_current_timeout` field of the register.
        ///
        ///Constant Current (CC) charging mode timeout duration.
        pub fn constant_current_timeout(&self) -> super::ConstantCurrentTimeoutValue {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 2)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `precharge_timeout` field of the register.
        ///
        ///Pre-charge phase timeout duration.
        pub fn set_precharge_timeout(&mut self, value: super::PrechargeTimeoutValue) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 8, &mut self.bits)
            };
        }
        ///Write the `external_path_charge_current` field of the register.
        ///
        ///External path charging current setting (300mA to 1000mA, 100mA/step).
        pub fn set_external_path_charge_current(
            &mut self,
            value: super::ExternalPathChargeCurrentValue,
        ) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 6, &mut self.bits)
            };
        }
        ///Write the `external_path_charge_enable` field of the register.
        ///
        ///External path charging enable (true: enabled, false: disabled).
        pub fn set_external_path_charge_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `constant_current_timeout` field of the register.
        ///
        ///Constant Current (CC) charging mode timeout duration.
        pub fn set_constant_current_timeout(
            &mut self,
            value: super::ConstantCurrentTimeoutValue,
        ) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 2, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for ChargeControl2 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeControl2> for [u8; 1] {
        fn from(val: ChargeControl2) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeControl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeControl2");
            {
                d.field("precharge_timeout", &self.precharge_timeout());
            }
            {
                d.field(
                    "external_path_charge_current",
                    &self.external_path_charge_current(),
                );
            }
            {
                d.field(
                    "external_path_charge_enable",
                    &self.external_path_charge_enable(),
                );
            }
            {
                d.field("constant_current_timeout", &self.constant_current_timeout());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ChargeControl2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeControl2 { ");
            defmt::write!(f, "precharge_timeout: {}, ", &self.precharge_timeout());
            defmt::write!(
                f,
                "external_path_charge_current: {}, ",
                &self.external_path_charge_current(),
            );
            defmt::write!(
                f,
                "external_path_charge_enable: {=bool}, ",
                &self.external_path_charge_enable(),
            );
            defmt::write!(
                f,
                "constant_current_timeout: {}, ",
                &self.constant_current_timeout(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for ChargeControl2 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeControl2 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeControl2 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeControl2 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeControl2 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeControl2 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeControl2 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Controls the charging parameters for the backup battery (RTC battery).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BackupBatteryChargeControl {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for BackupBatteryChargeControl {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl BackupBatteryChargeControl {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [34] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `backup_charge_enable` field of the register.
        ///
        ///Backup battery charging function (true: enabled, false: disabled).
        pub fn backup_charge_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `backup_target_voltage` field of the register.
        ///
        ///Target voltage for backup battery charging.
        pub fn backup_target_voltage(&self) -> super::BackupTargetVoltageValue {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 7)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `backup_charge_current` field of the register.
        ///
        ///Charging current for backup battery.
        pub fn backup_charge_current(&self) -> super::BackupChargeCurrentValue {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 2)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `backup_charge_enable` field of the register.
        ///
        ///Backup battery charging function (true: enabled, false: disabled).
        pub fn set_backup_charge_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `backup_target_voltage` field of the register.
        ///
        ///Target voltage for backup battery charging.
        pub fn set_backup_target_voltage(
            &mut self,
            value: super::BackupTargetVoltageValue,
        ) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 7, &mut self.bits)
            };
        }
        ///Write the `backup_charge_current` field of the register.
        ///
        ///Charging current for backup battery.
        pub fn set_backup_charge_current(
            &mut self,
            value: super::BackupChargeCurrentValue,
        ) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 2, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for BackupBatteryChargeControl {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<BackupBatteryChargeControl> for [u8; 1] {
        fn from(val: BackupBatteryChargeControl) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for BackupBatteryChargeControl {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("BackupBatteryChargeControl");
            {
                d.field("backup_charge_enable", &self.backup_charge_enable());
            }
            {
                d.field("backup_target_voltage", &self.backup_target_voltage());
            }
            {
                d.field("backup_charge_current", &self.backup_charge_current());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BackupBatteryChargeControl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BackupBatteryChargeControl { ");
            defmt::write!(
                f,
                "backup_charge_enable: {=bool}, ",
                &self.backup_charge_enable(),
            );
            defmt::write!(
                f,
                "backup_target_voltage: {}, ",
                &self.backup_target_voltage(),
            );
            defmt::write!(
                f,
                "backup_charge_current: {}, ",
                &self.backup_charge_current(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for BackupBatteryChargeControl {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for BackupBatteryChargeControl {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for BackupBatteryChargeControl {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for BackupBatteryChargeControl {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for BackupBatteryChargeControl {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for BackupBatteryChargeControl {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for BackupBatteryChargeControl {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Configures parameters related to the PEK (Power Enable Key/Button) operations.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PekKeyParameters {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for PekKeyParameters {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl PekKeyParameters {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [93] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `power_on_time` field of the register.
        ///
        ///PEK press duration required for power-on.
        pub fn power_on_time(&self) -> super::PekPowerOnTime {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 8)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `long_press_time` field of the register.
        ///
        ///PEK long press detection time.
        pub fn long_press_time(&self) -> super::PekLongPressTime {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 6)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `auto_shutdown_if_pek_held_longer_than_shutdown_time` field of the register.
        ///
        ///Auto-shutdown if PEK is held longer than the configured shutdown time (true: enabled, false: disabled).
        pub fn auto_shutdown_if_pek_held_longer_than_shutdown_time(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `pwrok_signal_delay` field of the register.
        ///
        ///Delay for the PWROK signal after power-up sequence completion.
        pub fn pwrok_signal_delay(&self) -> super::PwrokSignalDelay {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `shutdown_time` field of the register.
        ///
        ///PEK press duration required for shutdown.
        pub fn shutdown_time(&self) -> super::PekShutdownTime {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 2)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `power_on_time` field of the register.
        ///
        ///PEK press duration required for power-on.
        pub fn set_power_on_time(&mut self, value: super::PekPowerOnTime) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 8, &mut self.bits)
            };
        }
        ///Write the `long_press_time` field of the register.
        ///
        ///PEK long press detection time.
        pub fn set_long_press_time(&mut self, value: super::PekLongPressTime) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 6, &mut self.bits)
            };
        }
        ///Write the `auto_shutdown_if_pek_held_longer_than_shutdown_time` field of the register.
        ///
        ///Auto-shutdown if PEK is held longer than the configured shutdown time (true: enabled, false: disabled).
        pub fn set_auto_shutdown_if_pek_held_longer_than_shutdown_time(
            &mut self,
            value: bool,
        ) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `pwrok_signal_delay` field of the register.
        ///
        ///Delay for the PWROK signal after power-up sequence completion.
        pub fn set_pwrok_signal_delay(&mut self, value: super::PwrokSignalDelay) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `shutdown_time` field of the register.
        ///
        ///PEK press duration required for shutdown.
        pub fn set_shutdown_time(&mut self, value: super::PekShutdownTime) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 2, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for PekKeyParameters {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<PekKeyParameters> for [u8; 1] {
        fn from(val: PekKeyParameters) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for PekKeyParameters {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("PekKeyParameters");
            {
                d.field("power_on_time", &self.power_on_time());
            }
            {
                d.field("long_press_time", &self.long_press_time());
            }
            {
                d.field(
                    "auto_shutdown_if_pek_held_longer_than_shutdown_time",
                    &self.auto_shutdown_if_pek_held_longer_than_shutdown_time(),
                );
            }
            {
                d.field("pwrok_signal_delay", &self.pwrok_signal_delay());
            }
            {
                d.field("shutdown_time", &self.shutdown_time());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PekKeyParameters {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PekKeyParameters { ");
            defmt::write!(f, "power_on_time: {}, ", &self.power_on_time());
            defmt::write!(f, "long_press_time: {}, ", &self.long_press_time());
            defmt::write!(
                f,
                "auto_shutdown_if_pek_held_longer_than_shutdown_time: {=bool}, ",
                &self.auto_shutdown_if_pek_held_longer_than_shutdown_time(),
            );
            defmt::write!(f, "pwrok_signal_delay: {}, ", &self.pwrok_signal_delay());
            defmt::write!(f, "shutdown_time: {}, ", &self.shutdown_time());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for PekKeyParameters {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for PekKeyParameters {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for PekKeyParameters {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for PekKeyParameters {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for PekKeyParameters {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for PekKeyParameters {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for PekKeyParameters {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Sets the operating (switching) frequency for the DC-DC converters.
    ///The 4-bit value (bits 0-3) adjusts the frequency around a default of 1.5MHz.
    ///The raw value 0b1000 (decimal 8) corresponds to 1.5MHz.
    ///Each step away from this default value changes the frequency by approximately 5% of 1.5MHz (75kHz).
    ///Range approx 0.9MHz (raw 0) to 2.025MHz (raw 15).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DcDcOperatingFrequency {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for DcDcOperatingFrequency {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl DcDcOperatingFrequency {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [8] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `frequency_setting_raw` field of the register.
        ///
        ///Raw 4-bit setting for DC-DC switching frequency. 0b1000 = 1.5MHz. Each LSB step is approx. +/- 5% of 1.5MHz.
        pub fn frequency_setting_raw(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 4)
            };
            raw
        }
        ///Write the `frequency_setting_raw` field of the register.
        ///
        ///Raw 4-bit setting for DC-DC switching frequency. 0b1000 = 1.5MHz. Each LSB step is approx. +/- 5% of 1.5MHz.
        pub fn set_frequency_setting_raw(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 4, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for DcDcOperatingFrequency {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<DcDcOperatingFrequency> for [u8; 1] {
        fn from(val: DcDcOperatingFrequency) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for DcDcOperatingFrequency {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("DcDcOperatingFrequency");
            {
                d.field("frequency_setting_raw", &self.frequency_setting_raw());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DcDcOperatingFrequency {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DcDcOperatingFrequency { ");
            defmt::write!(
                f,
                "frequency_setting_raw: {=u8}, ",
                &self.frequency_setting_raw(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for DcDcOperatingFrequency {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for DcDcOperatingFrequency {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for DcDcOperatingFrequency {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for DcDcOperatingFrequency {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for DcDcOperatingFrequency {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for DcDcOperatingFrequency {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for DcDcOperatingFrequency {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Sets the low-temperature threshold (V_LTF_charge) for battery charging, typically read from an NTC thermistor.
    ///If the NTC voltage rises above this threshold (indicating low temperature), charging may be suspended or modified.
    ///Formula: Threshold Voltage (V) = raw_value * 0.0128.
    ///Range: 0V (raw 0x00) to 3.264V (raw 0xFF).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BatteryChargeLowTempThreshold {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for BatteryChargeLowTempThreshold {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl BatteryChargeLowTempThreshold {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [165] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `threshold_setting_raw` field of the register.
        ///
        ///Raw 8-bit setting for the low-temperature charge threshold voltage. See register description for formula.
        pub fn threshold_setting_raw(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 8)
            };
            raw
        }
        ///Write the `threshold_setting_raw` field of the register.
        ///
        ///Raw 8-bit setting for the low-temperature charge threshold voltage. See register description for formula.
        pub fn set_threshold_setting_raw(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 8, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for BatteryChargeLowTempThreshold {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<BatteryChargeLowTempThreshold> for [u8; 1] {
        fn from(val: BatteryChargeLowTempThreshold) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for BatteryChargeLowTempThreshold {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("BatteryChargeLowTempThreshold");
            {
                d.field("threshold_setting_raw", &self.threshold_setting_raw());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BatteryChargeLowTempThreshold {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BatteryChargeLowTempThreshold { ");
            defmt::write!(
                f,
                "threshold_setting_raw: {=u8}, ",
                &self.threshold_setting_raw(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for BatteryChargeLowTempThreshold {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for BatteryChargeLowTempThreshold {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for BatteryChargeLowTempThreshold {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for BatteryChargeLowTempThreshold {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for BatteryChargeLowTempThreshold {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for BatteryChargeLowTempThreshold {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for BatteryChargeLowTempThreshold {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Sets the high-temperature threshold (V_HTF_charge) for battery charging, typically read from an NTC thermistor.
    ///If the NTC voltage falls below this threshold (indicating high temperature), charging may be suspended or modified.
    ///Formula: Threshold Voltage (V) = raw_value * 0.0128.
    ///Range: 0V (raw 0x00) to 3.264V (raw 0xFF).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BatteryChargeHighTempThreshold {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for BatteryChargeHighTempThreshold {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl BatteryChargeHighTempThreshold {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [31] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `threshold_setting_raw` field of the register.
        ///
        ///Raw 8-bit setting for the high-temperature charge threshold voltage. See register description for formula.
        pub fn threshold_setting_raw(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 8)
            };
            raw
        }
        ///Write the `threshold_setting_raw` field of the register.
        ///
        ///Raw 8-bit setting for the high-temperature charge threshold voltage. See register description for formula.
        pub fn set_threshold_setting_raw(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 8, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for BatteryChargeHighTempThreshold {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<BatteryChargeHighTempThreshold> for [u8; 1] {
        fn from(val: BatteryChargeHighTempThreshold) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for BatteryChargeHighTempThreshold {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("BatteryChargeHighTempThreshold");
            {
                d.field("threshold_setting_raw", &self.threshold_setting_raw());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BatteryChargeHighTempThreshold {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BatteryChargeHighTempThreshold { ");
            defmt::write!(
                f,
                "threshold_setting_raw: {=u8}, ",
                &self.threshold_setting_raw(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for BatteryChargeHighTempThreshold {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for BatteryChargeHighTempThreshold {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for BatteryChargeHighTempThreshold {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for BatteryChargeHighTempThreshold {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for BatteryChargeHighTempThreshold {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for BatteryChargeHighTempThreshold {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for BatteryChargeHighTempThreshold {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Sets the APS (Average Power Source voltage) low power warning Level 1 threshold.
    ///An IRQ may be triggered if APS voltage drops below this level.
    ///Formula: V_WARNING1 (V) = 2.8672 + (raw_value * 0.0014 * 4).
    ///The raw_value is the 8-bit content of this register.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ApsLowPowerLevel1Setting {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ApsLowPowerLevel1Setting {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ApsLowPowerLevel1Setting {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [104] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `level_setting_raw` field of the register.
        ///
        ///Raw 8-bit setting for APS low power Level 1 threshold. See register description for formula.
        pub fn level_setting_raw(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 8)
            };
            raw
        }
        ///Write the `level_setting_raw` field of the register.
        ///
        ///Raw 8-bit setting for APS low power Level 1 threshold. See register description for formula.
        pub fn set_level_setting_raw(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 8, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for ApsLowPowerLevel1Setting {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ApsLowPowerLevel1Setting> for [u8; 1] {
        fn from(val: ApsLowPowerLevel1Setting) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ApsLowPowerLevel1Setting {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ApsLowPowerLevel1Setting");
            {
                d.field("level_setting_raw", &self.level_setting_raw());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ApsLowPowerLevel1Setting {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ApsLowPowerLevel1Setting { ");
            defmt::write!(f, "level_setting_raw: {=u8}, ", &self.level_setting_raw());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for ApsLowPowerLevel1Setting {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ApsLowPowerLevel1Setting {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ApsLowPowerLevel1Setting {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ApsLowPowerLevel1Setting {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ApsLowPowerLevel1Setting {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ApsLowPowerLevel1Setting {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ApsLowPowerLevel1Setting {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Sets the APS (Average Power Source voltage) low power warning Level 2 threshold.
    ///Typically set lower than Level 1. An IRQ may be triggered if APS voltage drops below this level.
    ///Formula: V_WARNING2 (V) = 2.8672 + (raw_value * 0.0014 * 4).
    ///The raw_value is the 8-bit content of this register.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ApsLowPowerLevel2Setting {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ApsLowPowerLevel2Setting {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ApsLowPowerLevel2Setting {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [95] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `level_setting_raw` field of the register.
        ///
        ///Raw 8-bit setting for APS low power Level 2 threshold. See register description for formula.
        pub fn level_setting_raw(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 8)
            };
            raw
        }
        ///Write the `level_setting_raw` field of the register.
        ///
        ///Raw 8-bit setting for APS low power Level 2 threshold. See register description for formula.
        pub fn set_level_setting_raw(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 8, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for ApsLowPowerLevel2Setting {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ApsLowPowerLevel2Setting> for [u8; 1] {
        fn from(val: ApsLowPowerLevel2Setting) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ApsLowPowerLevel2Setting {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ApsLowPowerLevel2Setting");
            {
                d.field("level_setting_raw", &self.level_setting_raw());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ApsLowPowerLevel2Setting {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ApsLowPowerLevel2Setting { ");
            defmt::write!(f, "level_setting_raw: {=u8}, ", &self.level_setting_raw());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for ApsLowPowerLevel2Setting {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ApsLowPowerLevel2Setting {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ApsLowPowerLevel2Setting {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ApsLowPowerLevel2Setting {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ApsLowPowerLevel2Setting {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ApsLowPowerLevel2Setting {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ApsLowPowerLevel2Setting {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Sets the low-temperature threshold (V_LTF_discharge) for battery discharging, typically read from an NTC thermistor.
    ///If the NTC voltage rises above this threshold (indicating low temperature), system behavior may be modified.
    ///Formula: Threshold Voltage (V) = raw_value * 0.0128.
    ///Range: 0V (raw 0x00) to 3.264V (raw 0xFF).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BatteryDischargeLowTempThreshold {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for BatteryDischargeLowTempThreshold {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl BatteryDischargeLowTempThreshold {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [252] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `threshold_setting_raw` field of the register.
        ///
        ///Raw 8-bit setting for the low-temperature discharge threshold voltage. See register description for formula.
        pub fn threshold_setting_raw(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 8)
            };
            raw
        }
        ///Write the `threshold_setting_raw` field of the register.
        ///
        ///Raw 8-bit setting for the low-temperature discharge threshold voltage. See register description for formula.
        pub fn set_threshold_setting_raw(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 8, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for BatteryDischargeLowTempThreshold {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<BatteryDischargeLowTempThreshold> for [u8; 1] {
        fn from(val: BatteryDischargeLowTempThreshold) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for BatteryDischargeLowTempThreshold {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("BatteryDischargeLowTempThreshold");
            {
                d.field("threshold_setting_raw", &self.threshold_setting_raw());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BatteryDischargeLowTempThreshold {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BatteryDischargeLowTempThreshold { ");
            defmt::write!(
                f,
                "threshold_setting_raw: {=u8}, ",
                &self.threshold_setting_raw(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for BatteryDischargeLowTempThreshold {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for BatteryDischargeLowTempThreshold {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for BatteryDischargeLowTempThreshold {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for BatteryDischargeLowTempThreshold {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for BatteryDischargeLowTempThreshold {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for BatteryDischargeLowTempThreshold {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for BatteryDischargeLowTempThreshold {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Sets the high-temperature threshold (V_HTF_discharge) for battery discharging, typically read from an NTC thermistor.
    ///If the NTC voltage falls below this threshold (indicating high temperature), system behavior may be modified.
    ///Formula: Threshold Voltage (V) = raw_value * 0.0128.
    ///Range: 0V (raw 0x00) to 3.264V (raw 0xFF).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BatteryDischargeHighTempThreshold {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for BatteryDischargeHighTempThreshold {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl BatteryDischargeHighTempThreshold {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [22] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `threshold_setting_raw` field of the register.
        ///
        ///Raw 8-bit setting for the high-temperature discharge threshold voltage. See register description for formula.
        pub fn threshold_setting_raw(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 8)
            };
            raw
        }
        ///Write the `threshold_setting_raw` field of the register.
        ///
        ///Raw 8-bit setting for the high-temperature discharge threshold voltage. See register description for formula.
        pub fn set_threshold_setting_raw(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 8, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for BatteryDischargeHighTempThreshold {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<BatteryDischargeHighTempThreshold> for [u8; 1] {
        fn from(val: BatteryDischargeHighTempThreshold) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for BatteryDischargeHighTempThreshold {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("BatteryDischargeHighTempThreshold");
            {
                d.field("threshold_setting_raw", &self.threshold_setting_raw());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BatteryDischargeHighTempThreshold {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BatteryDischargeHighTempThreshold { ");
            defmt::write!(
                f,
                "threshold_setting_raw: {=u8}, ",
                &self.threshold_setting_raw(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for BatteryDischargeHighTempThreshold {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for BatteryDischargeHighTempThreshold {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for BatteryDischargeHighTempThreshold {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for BatteryDischargeHighTempThreshold {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for BatteryDischargeHighTempThreshold {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for BatteryDischargeHighTempThreshold {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for BatteryDischargeHighTempThreshold {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Selects the operating mode (PFM/PWM Auto or Fixed PWM) for DC-DC1, DC-DC2, and DC-DC3.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DcDcOperatingMode {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for DcDcOperatingMode {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl DcDcOperatingMode {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [224] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `dcdc_1_operating_mode` field of the register.
        ///
        ///DC-DC1 operating mode.
        pub fn dcdc_1_operating_mode(&self) -> super::DcDcModeSelection {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `dcdc_2_operating_mode` field of the register.
        ///
        ///DC-DC2 operating mode.
        pub fn dcdc_2_operating_mode(&self) -> super::DcDcModeSelection {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `dcdc_3_operating_mode` field of the register.
        ///
        ///DC-DC3 operating mode.
        pub fn dcdc_3_operating_mode(&self) -> super::DcDcModeSelection {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `dcdc_1_operating_mode` field of the register.
        ///
        ///DC-DC1 operating mode.
        pub fn set_dcdc_1_operating_mode(&mut self, value: super::DcDcModeSelection) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `dcdc_2_operating_mode` field of the register.
        ///
        ///DC-DC2 operating mode.
        pub fn set_dcdc_2_operating_mode(&mut self, value: super::DcDcModeSelection) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `dcdc_3_operating_mode` field of the register.
        ///
        ///DC-DC3 operating mode.
        pub fn set_dcdc_3_operating_mode(&mut self, value: super::DcDcModeSelection) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for DcDcOperatingMode {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<DcDcOperatingMode> for [u8; 1] {
        fn from(val: DcDcOperatingMode) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for DcDcOperatingMode {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("DcDcOperatingMode");
            {
                d.field("dcdc_1_operating_mode", &self.dcdc_1_operating_mode());
            }
            {
                d.field("dcdc_2_operating_mode", &self.dcdc_2_operating_mode());
            }
            {
                d.field("dcdc_3_operating_mode", &self.dcdc_3_operating_mode());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DcDcOperatingMode {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DcDcOperatingMode { ");
            defmt::write!(
                f,
                "dcdc_1_operating_mode: {}, ",
                &self.dcdc_1_operating_mode(),
            );
            defmt::write!(
                f,
                "dcdc_2_operating_mode: {}, ",
                &self.dcdc_2_operating_mode(),
            );
            defmt::write!(
                f,
                "dcdc_3_operating_mode: {}, ",
                &self.dcdc_3_operating_mode(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for DcDcOperatingMode {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for DcDcOperatingMode {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for DcDcOperatingMode {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for DcDcOperatingMode {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for DcDcOperatingMode {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for DcDcOperatingMode {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for DcDcOperatingMode {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Controls the enable state for various ADC channels (Set 1).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcEnable1 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AdcEnable1 {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AdcEnable1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [131] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `battery_voltage_adc_enable` field of the register.
        ///
        ///Battery voltage ADC (true: enabled, false: disabled).
        pub fn battery_voltage_adc_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `battery_current_adc_enable` field of the register.
        ///
        ///Battery charge/discharge current ADC (true: enabled, false: disabled).
        pub fn battery_current_adc_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `acin_voltage_adc_enable` field of the register.
        ///
        ///ACIN voltage ADC (true: enabled, false: disabled).
        pub fn acin_voltage_adc_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `acin_current_adc_enable` field of the register.
        ///
        ///ACIN current ADC (true: enabled, false: disabled).
        pub fn acin_current_adc_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `vbus_voltage_adc_enable` field of the register.
        ///
        ///VBUS voltage ADC (true: enabled, false: disabled).
        pub fn vbus_voltage_adc_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `vbus_current_adc_enable` field of the register.
        ///
        ///VBUS current ADC (true: enabled, false: disabled).
        pub fn vbus_current_adc_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `aps_voltage_adc_enable` field of the register.
        ///
        ///APS (Average Power Source) voltage ADC (true: enabled, false: disabled).
        pub fn aps_voltage_adc_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `ts_pin_adc_enable` field of the register.
        ///
        ///TS (Temperature Sense) pin ADC function (true: enabled, false: disabled).
        pub fn ts_pin_adc_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `battery_voltage_adc_enable` field of the register.
        ///
        ///Battery voltage ADC (true: enabled, false: disabled).
        pub fn set_battery_voltage_adc_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `battery_current_adc_enable` field of the register.
        ///
        ///Battery charge/discharge current ADC (true: enabled, false: disabled).
        pub fn set_battery_current_adc_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `acin_voltage_adc_enable` field of the register.
        ///
        ///ACIN voltage ADC (true: enabled, false: disabled).
        pub fn set_acin_voltage_adc_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `acin_current_adc_enable` field of the register.
        ///
        ///ACIN current ADC (true: enabled, false: disabled).
        pub fn set_acin_current_adc_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `vbus_voltage_adc_enable` field of the register.
        ///
        ///VBUS voltage ADC (true: enabled, false: disabled).
        pub fn set_vbus_voltage_adc_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `vbus_current_adc_enable` field of the register.
        ///
        ///VBUS current ADC (true: enabled, false: disabled).
        pub fn set_vbus_current_adc_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `aps_voltage_adc_enable` field of the register.
        ///
        ///APS (Average Power Source) voltage ADC (true: enabled, false: disabled).
        pub fn set_aps_voltage_adc_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `ts_pin_adc_enable` field of the register.
        ///
        ///TS (Temperature Sense) pin ADC function (true: enabled, false: disabled).
        pub fn set_ts_pin_adc_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for AdcEnable1 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AdcEnable1> for [u8; 1] {
        fn from(val: AdcEnable1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AdcEnable1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AdcEnable1");
            {
                d.field(
                    "battery_voltage_adc_enable",
                    &self.battery_voltage_adc_enable(),
                );
            }
            {
                d.field(
                    "battery_current_adc_enable",
                    &self.battery_current_adc_enable(),
                );
            }
            {
                d.field("acin_voltage_adc_enable", &self.acin_voltage_adc_enable());
            }
            {
                d.field("acin_current_adc_enable", &self.acin_current_adc_enable());
            }
            {
                d.field("vbus_voltage_adc_enable", &self.vbus_voltage_adc_enable());
            }
            {
                d.field("vbus_current_adc_enable", &self.vbus_current_adc_enable());
            }
            {
                d.field("aps_voltage_adc_enable", &self.aps_voltage_adc_enable());
            }
            {
                d.field("ts_pin_adc_enable", &self.ts_pin_adc_enable());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AdcEnable1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AdcEnable1 { ");
            defmt::write!(
                f,
                "battery_voltage_adc_enable: {=bool}, ",
                &self.battery_voltage_adc_enable(),
            );
            defmt::write!(
                f,
                "battery_current_adc_enable: {=bool}, ",
                &self.battery_current_adc_enable(),
            );
            defmt::write!(
                f,
                "acin_voltage_adc_enable: {=bool}, ",
                &self.acin_voltage_adc_enable(),
            );
            defmt::write!(
                f,
                "acin_current_adc_enable: {=bool}, ",
                &self.acin_current_adc_enable(),
            );
            defmt::write!(
                f,
                "vbus_voltage_adc_enable: {=bool}, ",
                &self.vbus_voltage_adc_enable(),
            );
            defmt::write!(
                f,
                "vbus_current_adc_enable: {=bool}, ",
                &self.vbus_current_adc_enable(),
            );
            defmt::write!(
                f,
                "aps_voltage_adc_enable: {=bool}, ",
                &self.aps_voltage_adc_enable(),
            );
            defmt::write!(f, "ts_pin_adc_enable: {=bool}, ", &self.ts_pin_adc_enable());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for AdcEnable1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AdcEnable1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AdcEnable1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AdcEnable1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AdcEnable1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AdcEnable1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AdcEnable1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Controls the enable state for internal temperature ADC and GPIO ADCs (Set 2).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcEnable2 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AdcEnable2 {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AdcEnable2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [128] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `internal_temperature_adc_enable` field of the register.
        ///
        ///AXP192 internal temperature monitoring ADC (true: enabled, false: disabled).
        pub fn internal_temperature_adc_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `gpio_0_adc_enable` field of the register.
        ///
        ///GPIO0 ADC function (true: enabled, false: disabled).
        pub fn gpio_0_adc_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `gpio_1_adc_enable` field of the register.
        ///
        ///GPIO1 ADC function (true: enabled, false: disabled).
        pub fn gpio_1_adc_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `gpio_2_adc_enable` field of the register.
        ///
        ///GPIO2 ADC function (true: enabled, false: disabled).
        pub fn gpio_2_adc_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `gpio_3_adc_enable` field of the register.
        ///
        ///GPIO3 ADC function (true: enabled, false: disabled).
        pub fn gpio_3_adc_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `internal_temperature_adc_enable` field of the register.
        ///
        ///AXP192 internal temperature monitoring ADC (true: enabled, false: disabled).
        pub fn set_internal_temperature_adc_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `gpio_0_adc_enable` field of the register.
        ///
        ///GPIO0 ADC function (true: enabled, false: disabled).
        pub fn set_gpio_0_adc_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `gpio_1_adc_enable` field of the register.
        ///
        ///GPIO1 ADC function (true: enabled, false: disabled).
        pub fn set_gpio_1_adc_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `gpio_2_adc_enable` field of the register.
        ///
        ///GPIO2 ADC function (true: enabled, false: disabled).
        pub fn set_gpio_2_adc_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `gpio_3_adc_enable` field of the register.
        ///
        ///GPIO3 ADC function (true: enabled, false: disabled).
        pub fn set_gpio_3_adc_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for AdcEnable2 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AdcEnable2> for [u8; 1] {
        fn from(val: AdcEnable2) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AdcEnable2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AdcEnable2");
            {
                d.field(
                    "internal_temperature_adc_enable",
                    &self.internal_temperature_adc_enable(),
                );
            }
            {
                d.field("gpio_0_adc_enable", &self.gpio_0_adc_enable());
            }
            {
                d.field("gpio_1_adc_enable", &self.gpio_1_adc_enable());
            }
            {
                d.field("gpio_2_adc_enable", &self.gpio_2_adc_enable());
            }
            {
                d.field("gpio_3_adc_enable", &self.gpio_3_adc_enable());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AdcEnable2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AdcEnable2 { ");
            defmt::write!(
                f,
                "internal_temperature_adc_enable: {=bool}, ",
                &self.internal_temperature_adc_enable(),
            );
            defmt::write!(f, "gpio_0_adc_enable: {=bool}, ", &self.gpio_0_adc_enable());
            defmt::write!(f, "gpio_1_adc_enable: {=bool}, ", &self.gpio_1_adc_enable());
            defmt::write!(f, "gpio_2_adc_enable: {=bool}, ", &self.gpio_2_adc_enable());
            defmt::write!(f, "gpio_3_adc_enable: {=bool}, ", &self.gpio_3_adc_enable());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for AdcEnable2 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AdcEnable2 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AdcEnable2 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AdcEnable2 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AdcEnable2 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AdcEnable2 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AdcEnable2 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Configures ADC sample rate and TS (Temperature Sense) pin functionality, current, and output mode.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcSampleRateTsPinControl {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AdcSampleRateTsPinControl {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AdcSampleRateTsPinControl {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [50] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `adc_sample_rate` field of the register.
        ///
        ///ADC sampling rate for enabled channels.
        pub fn adc_sample_rate(&self) -> super::AdcSampleRateValue {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 8)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `ts_pin_output_current` field of the register.
        ///
        ///Output current for the TS pin (typically for NTC biasing).
        pub fn ts_pin_output_current(&self) -> super::TsPinCurrentValue {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 6)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `ts_pin_function_is_external_adc` field of the register.
        ///
        ///TS pin function (true: External ADC input, false: Battery Temperature Monitor).
        pub fn ts_pin_function_is_external_adc(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `ts_pin_current_output_mode` field of the register.
        ///
        ///Controls when current is output on the TS pin.
        pub fn ts_pin_current_output_mode(&self) -> super::TsPinOutputMode {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 2)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `adc_sample_rate` field of the register.
        ///
        ///ADC sampling rate for enabled channels.
        pub fn set_adc_sample_rate(&mut self, value: super::AdcSampleRateValue) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 8, &mut self.bits)
            };
        }
        ///Write the `ts_pin_output_current` field of the register.
        ///
        ///Output current for the TS pin (typically for NTC biasing).
        pub fn set_ts_pin_output_current(&mut self, value: super::TsPinCurrentValue) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 6, &mut self.bits)
            };
        }
        ///Write the `ts_pin_function_is_external_adc` field of the register.
        ///
        ///TS pin function (true: External ADC input, false: Battery Temperature Monitor).
        pub fn set_ts_pin_function_is_external_adc(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `ts_pin_current_output_mode` field of the register.
        ///
        ///Controls when current is output on the TS pin.
        pub fn set_ts_pin_current_output_mode(&mut self, value: super::TsPinOutputMode) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 2, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for AdcSampleRateTsPinControl {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AdcSampleRateTsPinControl> for [u8; 1] {
        fn from(val: AdcSampleRateTsPinControl) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AdcSampleRateTsPinControl {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AdcSampleRateTsPinControl");
            {
                d.field("adc_sample_rate", &self.adc_sample_rate());
            }
            {
                d.field("ts_pin_output_current", &self.ts_pin_output_current());
            }
            {
                d.field(
                    "ts_pin_function_is_external_adc",
                    &self.ts_pin_function_is_external_adc(),
                );
            }
            {
                d.field(
                    "ts_pin_current_output_mode",
                    &self.ts_pin_current_output_mode(),
                );
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AdcSampleRateTsPinControl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AdcSampleRateTsPinControl { ");
            defmt::write!(f, "adc_sample_rate: {}, ", &self.adc_sample_rate());
            defmt::write!(
                f,
                "ts_pin_output_current: {}, ",
                &self.ts_pin_output_current(),
            );
            defmt::write!(
                f,
                "ts_pin_function_is_external_adc: {=bool}, ",
                &self.ts_pin_function_is_external_adc(),
            );
            defmt::write!(
                f,
                "ts_pin_current_output_mode: {}, ",
                &self.ts_pin_current_output_mode(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for AdcSampleRateTsPinControl {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AdcSampleRateTsPinControl {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AdcSampleRateTsPinControl {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AdcSampleRateTsPinControl {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AdcSampleRateTsPinControl {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AdcSampleRateTsPinControl {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AdcSampleRateTsPinControl {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Sets the ADC input voltage range for GPIO0, GPIO1, GPIO2, and GPIO3.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GpioAdcInputRangeSetting {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for GpioAdcInputRangeSetting {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl GpioAdcInputRangeSetting {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `gpio_3_adc_input_range` field of the register.
        ///
        ///ADC input voltage range for GPIO3.
        pub fn gpio_3_adc_input_range(&self) -> super::GpioAdcRange {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `gpio_2_adc_input_range` field of the register.
        ///
        ///ADC input voltage range for GPIO2.
        pub fn gpio_2_adc_input_range(&self) -> super::GpioAdcRange {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `gpio_1_adc_input_range` field of the register.
        ///
        ///ADC input voltage range for GPIO1.
        pub fn gpio_1_adc_input_range(&self) -> super::GpioAdcRange {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `gpio_0_adc_input_range` field of the register.
        ///
        ///ADC input voltage range for GPIO0.
        pub fn gpio_0_adc_input_range(&self) -> super::GpioAdcRange {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `gpio_3_adc_input_range` field of the register.
        ///
        ///ADC input voltage range for GPIO3.
        pub fn set_gpio_3_adc_input_range(&mut self, value: super::GpioAdcRange) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `gpio_2_adc_input_range` field of the register.
        ///
        ///ADC input voltage range for GPIO2.
        pub fn set_gpio_2_adc_input_range(&mut self, value: super::GpioAdcRange) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `gpio_1_adc_input_range` field of the register.
        ///
        ///ADC input voltage range for GPIO1.
        pub fn set_gpio_1_adc_input_range(&mut self, value: super::GpioAdcRange) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `gpio_0_adc_input_range` field of the register.
        ///
        ///ADC input voltage range for GPIO0.
        pub fn set_gpio_0_adc_input_range(&mut self, value: super::GpioAdcRange) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for GpioAdcInputRangeSetting {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<GpioAdcInputRangeSetting> for [u8; 1] {
        fn from(val: GpioAdcInputRangeSetting) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for GpioAdcInputRangeSetting {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("GpioAdcInputRangeSetting");
            {
                d.field("gpio_3_adc_input_range", &self.gpio_3_adc_input_range());
            }
            {
                d.field("gpio_2_adc_input_range", &self.gpio_2_adc_input_range());
            }
            {
                d.field("gpio_1_adc_input_range", &self.gpio_1_adc_input_range());
            }
            {
                d.field("gpio_0_adc_input_range", &self.gpio_0_adc_input_range());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GpioAdcInputRangeSetting {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GpioAdcInputRangeSetting { ");
            defmt::write!(
                f,
                "gpio_3_adc_input_range: {}, ",
                &self.gpio_3_adc_input_range(),
            );
            defmt::write!(
                f,
                "gpio_2_adc_input_range: {}, ",
                &self.gpio_2_adc_input_range(),
            );
            defmt::write!(
                f,
                "gpio_1_adc_input_range: {}, ",
                &self.gpio_1_adc_input_range(),
            );
            defmt::write!(
                f,
                "gpio_0_adc_input_range: {}, ",
                &self.gpio_0_adc_input_range(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for GpioAdcInputRangeSetting {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for GpioAdcInputRangeSetting {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for GpioAdcInputRangeSetting {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for GpioAdcInputRangeSetting {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for GpioAdcInputRangeSetting {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for GpioAdcInputRangeSetting {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for GpioAdcInputRangeSetting {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Enum containing all possible field set types
    pub enum FieldSetValue {
        ///Indicates the input power source status (ACIN, VBUS), battery current direction,
        ///and other power-related conditions.
        PowerStatus(PowerStatus),
        ///Indicates various operational modes and charging statuses, including AXP192 temperature,
        ///battery presence, and charging progress.
        ChargeStatus(ChargeStatus),
        ///Indicates the status of VBUS when operating in USB OTG (On-The-Go) mode.
        OtgVbusStatus(OtgVbusStatus),
        ///General purpose data buffer for system data storage.
        ///Note: Retained as long as any AXP192 power source is present (not affected by host on/off).
        DataBuffer0(DataBuffer0),
        ///Controls the output enable state for the EXTEN pin and the DC-DC2 converter.
        ExtenDcDc2Control(ExtenDcDc2Control),
        ///Controls output enable for DC-DC1, DC-DC3, LDO2, LDO3, EXTEN, and DC-DC2.
        ///Power-on default for M5StickC Plus is 0x5F.
        ///IMPORTANT: REG12H bit 6 (exten_output_enable) is linked to REG10H bit 2.
        ///             REG12H bit 4 (dcdc2_output_enable) is linked to REG10H bit 0.
        ///             These pairs control the same underlying hardware function.
        PowerOutputControl(PowerOutputControl),
        ///Sets the output voltage for the DC-DC2 converter.
        ///Formula: Output Voltage (V) = 0.7 + (value * 0.025). Range: 0.7V to 2.275V (raw 0-63).
        DcDc2VoltageSetting(DcDc2VoltageSetting),
        ///Configures DC-DC2 Dynamic Voltage Scaling / Voltage Ramp Control (VRC).
        DcDc2VrcParameter(DcDc2VrcParameter),
        ///Sets the output voltage for a DC-DC converter.
        ///Formula: V_out = 0.7V + (value * 25mV). Range 0.7V-3.5V (raw 0-112).
        DcDc1VoltageSetting(DcDc1VoltageSetting),
        ///Sets the output voltage for both LDO2 and LDO3.
        ///LDO2 (bits 7-4) & LDO3 (bits 3-0): V_out = 1.8V + (setting * 100mV). Range: 1.8V-3.3V.
        Ldo2And3VoltageSetting(Ldo2And3VoltageSetting),
        ///Manages VBUS to IPSOUT path, VHOLD voltage limiting, and VBUS current limiting.
        VbusIpsoutPathManagement(VbusIpsoutPathManagement),
        ///Sets VOFF (shutdown) voltage and PWRON short-press wakeup from sleep.
        ShutdownVoltageSetting(ShutdownVoltageSetting),
        ///Controls shutdown, battery detection, CHGLED pin, and N_OE shutdown delay.
        ShutdownBatChgLedControl(ShutdownBatChgLedControl),
        ///Controls primary charging parameters.
        ChargeControl1(ChargeControl1),
        ///Controls secondary charging parameters including pre-charge timeout,
        ///external path charge current and enable state, and constant current mode timeout.
        ChargeControl2(ChargeControl2),
        ///Controls the charging parameters for the backup battery (RTC battery).
        BackupBatteryChargeControl(BackupBatteryChargeControl),
        ///Configures parameters related to the PEK (Power Enable Key/Button) operations.
        PekKeyParameters(PekKeyParameters),
        ///Sets the operating (switching) frequency for the DC-DC converters.
        ///The 4-bit value (bits 0-3) adjusts the frequency around a default of 1.5MHz.
        ///The raw value 0b1000 (decimal 8) corresponds to 1.5MHz.
        ///Each step away from this default value changes the frequency by approximately 5% of 1.5MHz (75kHz).
        ///Range approx 0.9MHz (raw 0) to 2.025MHz (raw 15).
        DcDcOperatingFrequency(DcDcOperatingFrequency),
        ///Sets the low-temperature threshold (V_LTF_charge) for battery charging, typically read from an NTC thermistor.
        ///If the NTC voltage rises above this threshold (indicating low temperature), charging may be suspended or modified.
        ///Formula: Threshold Voltage (V) = raw_value * 0.0128.
        ///Range: 0V (raw 0x00) to 3.264V (raw 0xFF).
        BatteryChargeLowTempThreshold(BatteryChargeLowTempThreshold),
        ///Sets the high-temperature threshold (V_HTF_charge) for battery charging, typically read from an NTC thermistor.
        ///If the NTC voltage falls below this threshold (indicating high temperature), charging may be suspended or modified.
        ///Formula: Threshold Voltage (V) = raw_value * 0.0128.
        ///Range: 0V (raw 0x00) to 3.264V (raw 0xFF).
        BatteryChargeHighTempThreshold(BatteryChargeHighTempThreshold),
        ///Sets the APS (Average Power Source voltage) low power warning Level 1 threshold.
        ///An IRQ may be triggered if APS voltage drops below this level.
        ///Formula: V_WARNING1 (V) = 2.8672 + (raw_value * 0.0014 * 4).
        ///The raw_value is the 8-bit content of this register.
        ApsLowPowerLevel1Setting(ApsLowPowerLevel1Setting),
        ///Sets the APS (Average Power Source voltage) low power warning Level 2 threshold.
        ///Typically set lower than Level 1. An IRQ may be triggered if APS voltage drops below this level.
        ///Formula: V_WARNING2 (V) = 2.8672 + (raw_value * 0.0014 * 4).
        ///The raw_value is the 8-bit content of this register.
        ApsLowPowerLevel2Setting(ApsLowPowerLevel2Setting),
        ///Sets the low-temperature threshold (V_LTF_discharge) for battery discharging, typically read from an NTC thermistor.
        ///If the NTC voltage rises above this threshold (indicating low temperature), system behavior may be modified.
        ///Formula: Threshold Voltage (V) = raw_value * 0.0128.
        ///Range: 0V (raw 0x00) to 3.264V (raw 0xFF).
        BatteryDischargeLowTempThreshold(BatteryDischargeLowTempThreshold),
        ///Sets the high-temperature threshold (V_HTF_discharge) for battery discharging, typically read from an NTC thermistor.
        ///If the NTC voltage falls below this threshold (indicating high temperature), system behavior may be modified.
        ///Formula: Threshold Voltage (V) = raw_value * 0.0128.
        ///Range: 0V (raw 0x00) to 3.264V (raw 0xFF).
        BatteryDischargeHighTempThreshold(BatteryDischargeHighTempThreshold),
        ///Selects the operating mode (PFM/PWM Auto or Fixed PWM) for DC-DC1, DC-DC2, and DC-DC3.
        DcDcOperatingMode(DcDcOperatingMode),
        ///Controls the enable state for various ADC channels (Set 1).
        AdcEnable1(AdcEnable1),
        ///Controls the enable state for internal temperature ADC and GPIO ADCs (Set 2).
        AdcEnable2(AdcEnable2),
        ///Configures ADC sample rate and TS (Temperature Sense) pin functionality, current, and output mode.
        AdcSampleRateTsPinControl(AdcSampleRateTsPinControl),
        ///Sets the ADC input voltage range for GPIO0, GPIO1, GPIO2, and GPIO3.
        GpioAdcInputRangeSetting(GpioAdcInputRangeSetting),
    }
    impl core::fmt::Debug for FieldSetValue {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::PowerStatus(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeStatus(val) => core::fmt::Debug::fmt(val, f),
                Self::OtgVbusStatus(val) => core::fmt::Debug::fmt(val, f),
                Self::DataBuffer0(val) => core::fmt::Debug::fmt(val, f),
                Self::ExtenDcDc2Control(val) => core::fmt::Debug::fmt(val, f),
                Self::PowerOutputControl(val) => core::fmt::Debug::fmt(val, f),
                Self::DcDc2VoltageSetting(val) => core::fmt::Debug::fmt(val, f),
                Self::DcDc2VrcParameter(val) => core::fmt::Debug::fmt(val, f),
                Self::DcDc1VoltageSetting(val) => core::fmt::Debug::fmt(val, f),
                Self::Ldo2And3VoltageSetting(val) => core::fmt::Debug::fmt(val, f),
                Self::VbusIpsoutPathManagement(val) => core::fmt::Debug::fmt(val, f),
                Self::ShutdownVoltageSetting(val) => core::fmt::Debug::fmt(val, f),
                Self::ShutdownBatChgLedControl(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeControl1(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeControl2(val) => core::fmt::Debug::fmt(val, f),
                Self::BackupBatteryChargeControl(val) => core::fmt::Debug::fmt(val, f),
                Self::PekKeyParameters(val) => core::fmt::Debug::fmt(val, f),
                Self::DcDcOperatingFrequency(val) => core::fmt::Debug::fmt(val, f),
                Self::BatteryChargeLowTempThreshold(val) => core::fmt::Debug::fmt(val, f),
                Self::BatteryChargeHighTempThreshold(val) => {
                    core::fmt::Debug::fmt(val, f)
                }
                Self::ApsLowPowerLevel1Setting(val) => core::fmt::Debug::fmt(val, f),
                Self::ApsLowPowerLevel2Setting(val) => core::fmt::Debug::fmt(val, f),
                Self::BatteryDischargeLowTempThreshold(val) => {
                    core::fmt::Debug::fmt(val, f)
                }
                Self::BatteryDischargeHighTempThreshold(val) => {
                    core::fmt::Debug::fmt(val, f)
                }
                Self::DcDcOperatingMode(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcEnable1(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcEnable2(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcSampleRateTsPinControl(val) => core::fmt::Debug::fmt(val, f),
                Self::GpioAdcInputRangeSetting(val) => core::fmt::Debug::fmt(val, f),
                _ => unreachable!(),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FieldSetValue {
        fn format(&self, f: defmt::Formatter) {
            match self {
                Self::PowerStatus(val) => defmt::Format::format(val, f),
                Self::ChargeStatus(val) => defmt::Format::format(val, f),
                Self::OtgVbusStatus(val) => defmt::Format::format(val, f),
                Self::DataBuffer0(val) => defmt::Format::format(val, f),
                Self::ExtenDcDc2Control(val) => defmt::Format::format(val, f),
                Self::PowerOutputControl(val) => defmt::Format::format(val, f),
                Self::DcDc2VoltageSetting(val) => defmt::Format::format(val, f),
                Self::DcDc2VrcParameter(val) => defmt::Format::format(val, f),
                Self::DcDc1VoltageSetting(val) => defmt::Format::format(val, f),
                Self::Ldo2And3VoltageSetting(val) => defmt::Format::format(val, f),
                Self::VbusIpsoutPathManagement(val) => defmt::Format::format(val, f),
                Self::ShutdownVoltageSetting(val) => defmt::Format::format(val, f),
                Self::ShutdownBatChgLedControl(val) => defmt::Format::format(val, f),
                Self::ChargeControl1(val) => defmt::Format::format(val, f),
                Self::ChargeControl2(val) => defmt::Format::format(val, f),
                Self::BackupBatteryChargeControl(val) => defmt::Format::format(val, f),
                Self::PekKeyParameters(val) => defmt::Format::format(val, f),
                Self::DcDcOperatingFrequency(val) => defmt::Format::format(val, f),
                Self::BatteryChargeLowTempThreshold(val) => defmt::Format::format(val, f),
                Self::BatteryChargeHighTempThreshold(val) => {
                    defmt::Format::format(val, f)
                }
                Self::ApsLowPowerLevel1Setting(val) => defmt::Format::format(val, f),
                Self::ApsLowPowerLevel2Setting(val) => defmt::Format::format(val, f),
                Self::BatteryDischargeLowTempThreshold(val) => {
                    defmt::Format::format(val, f)
                }
                Self::BatteryDischargeHighTempThreshold(val) => {
                    defmt::Format::format(val, f)
                }
                Self::DcDcOperatingMode(val) => defmt::Format::format(val, f),
                Self::AdcEnable1(val) => defmt::Format::format(val, f),
                Self::AdcEnable2(val) => defmt::Format::format(val, f),
                Self::AdcSampleRateTsPinControl(val) => defmt::Format::format(val, f),
                Self::GpioAdcInputRangeSetting(val) => defmt::Format::format(val, f),
            }
        }
    }
    impl From<PowerStatus> for FieldSetValue {
        fn from(val: PowerStatus) -> Self {
            Self::PowerStatus(val)
        }
    }
    impl From<ChargeStatus> for FieldSetValue {
        fn from(val: ChargeStatus) -> Self {
            Self::ChargeStatus(val)
        }
    }
    impl From<OtgVbusStatus> for FieldSetValue {
        fn from(val: OtgVbusStatus) -> Self {
            Self::OtgVbusStatus(val)
        }
    }
    impl From<DataBuffer0> for FieldSetValue {
        fn from(val: DataBuffer0) -> Self {
            Self::DataBuffer0(val)
        }
    }
    impl From<ExtenDcDc2Control> for FieldSetValue {
        fn from(val: ExtenDcDc2Control) -> Self {
            Self::ExtenDcDc2Control(val)
        }
    }
    impl From<PowerOutputControl> for FieldSetValue {
        fn from(val: PowerOutputControl) -> Self {
            Self::PowerOutputControl(val)
        }
    }
    impl From<DcDc2VoltageSetting> for FieldSetValue {
        fn from(val: DcDc2VoltageSetting) -> Self {
            Self::DcDc2VoltageSetting(val)
        }
    }
    impl From<DcDc2VrcParameter> for FieldSetValue {
        fn from(val: DcDc2VrcParameter) -> Self {
            Self::DcDc2VrcParameter(val)
        }
    }
    impl From<DcDc1VoltageSetting> for FieldSetValue {
        fn from(val: DcDc1VoltageSetting) -> Self {
            Self::DcDc1VoltageSetting(val)
        }
    }
    impl From<Ldo2And3VoltageSetting> for FieldSetValue {
        fn from(val: Ldo2And3VoltageSetting) -> Self {
            Self::Ldo2And3VoltageSetting(val)
        }
    }
    impl From<VbusIpsoutPathManagement> for FieldSetValue {
        fn from(val: VbusIpsoutPathManagement) -> Self {
            Self::VbusIpsoutPathManagement(val)
        }
    }
    impl From<ShutdownVoltageSetting> for FieldSetValue {
        fn from(val: ShutdownVoltageSetting) -> Self {
            Self::ShutdownVoltageSetting(val)
        }
    }
    impl From<ShutdownBatChgLedControl> for FieldSetValue {
        fn from(val: ShutdownBatChgLedControl) -> Self {
            Self::ShutdownBatChgLedControl(val)
        }
    }
    impl From<ChargeControl1> for FieldSetValue {
        fn from(val: ChargeControl1) -> Self {
            Self::ChargeControl1(val)
        }
    }
    impl From<ChargeControl2> for FieldSetValue {
        fn from(val: ChargeControl2) -> Self {
            Self::ChargeControl2(val)
        }
    }
    impl From<BackupBatteryChargeControl> for FieldSetValue {
        fn from(val: BackupBatteryChargeControl) -> Self {
            Self::BackupBatteryChargeControl(val)
        }
    }
    impl From<PekKeyParameters> for FieldSetValue {
        fn from(val: PekKeyParameters) -> Self {
            Self::PekKeyParameters(val)
        }
    }
    impl From<DcDcOperatingFrequency> for FieldSetValue {
        fn from(val: DcDcOperatingFrequency) -> Self {
            Self::DcDcOperatingFrequency(val)
        }
    }
    impl From<BatteryChargeLowTempThreshold> for FieldSetValue {
        fn from(val: BatteryChargeLowTempThreshold) -> Self {
            Self::BatteryChargeLowTempThreshold(val)
        }
    }
    impl From<BatteryChargeHighTempThreshold> for FieldSetValue {
        fn from(val: BatteryChargeHighTempThreshold) -> Self {
            Self::BatteryChargeHighTempThreshold(val)
        }
    }
    impl From<ApsLowPowerLevel1Setting> for FieldSetValue {
        fn from(val: ApsLowPowerLevel1Setting) -> Self {
            Self::ApsLowPowerLevel1Setting(val)
        }
    }
    impl From<ApsLowPowerLevel2Setting> for FieldSetValue {
        fn from(val: ApsLowPowerLevel2Setting) -> Self {
            Self::ApsLowPowerLevel2Setting(val)
        }
    }
    impl From<BatteryDischargeLowTempThreshold> for FieldSetValue {
        fn from(val: BatteryDischargeLowTempThreshold) -> Self {
            Self::BatteryDischargeLowTempThreshold(val)
        }
    }
    impl From<BatteryDischargeHighTempThreshold> for FieldSetValue {
        fn from(val: BatteryDischargeHighTempThreshold) -> Self {
            Self::BatteryDischargeHighTempThreshold(val)
        }
    }
    impl From<DcDcOperatingMode> for FieldSetValue {
        fn from(val: DcDcOperatingMode) -> Self {
            Self::DcDcOperatingMode(val)
        }
    }
    impl From<AdcEnable1> for FieldSetValue {
        fn from(val: AdcEnable1) -> Self {
            Self::AdcEnable1(val)
        }
    }
    impl From<AdcEnable2> for FieldSetValue {
        fn from(val: AdcEnable2) -> Self {
            Self::AdcEnable2(val)
        }
    }
    impl From<AdcSampleRateTsPinControl> for FieldSetValue {
        fn from(val: AdcSampleRateTsPinControl) -> Self {
            Self::AdcSampleRateTsPinControl(val)
        }
    }
    impl From<GpioAdcInputRangeSetting> for FieldSetValue {
        fn from(val: GpioAdcInputRangeSetting) -> Self {
            Self::GpioAdcInputRangeSetting(val)
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BatteryFlowDirection {
    Discharging = 0,
    Charging = 1,
}
impl core::convert::TryFrom<u8> for BatteryFlowDirection {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Discharging),
            1 => Ok(Self::Charging),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "BatteryFlowDirection",
                })
            }
        }
    }
}
impl From<BatteryFlowDirection> for u8 {
    fn from(val: BatteryFlowDirection) -> Self {
        match val {
            BatteryFlowDirection::Discharging => 0,
            BatteryFlowDirection::Charging => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AxpStartupMode {
    ///Startup Mode A.
    ModeA = 0,
    ///Startup Mode B.
    ModeB = 1,
}
impl core::convert::TryFrom<u8> for AxpStartupMode {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::ModeA),
            1 => Ok(Self::ModeB),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "AxpStartupMode",
                })
            }
        }
    }
}
impl From<AxpStartupMode> for u8 {
    fn from(val: AxpStartupMode) -> Self {
        match val {
            AxpStartupMode::ModeA => 0,
            AxpStartupMode::ModeB => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VrcRiseSlope {
    ///1.6mV/us slope.
    Slope16MVUs = 0,
    ///0.8mV/us slope.
    Slope08MVUs = 1,
}
impl core::convert::TryFrom<u8> for VrcRiseSlope {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Slope16MVUs),
            1 => Ok(Self::Slope08MVUs),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "VrcRiseSlope",
                })
            }
        }
    }
}
impl From<VrcRiseSlope> for u8 {
    fn from(val: VrcRiseSlope) -> Self {
        match val {
            VrcRiseSlope::Slope16MVUs => 0,
            VrcRiseSlope::Slope08MVUs => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusPathSelectionControl {
    ///Path controlled by N_VBUSEN pin.
    ControlledByNvbusenPin = 0,
    ///Path forced open (ignores N_VBUSEN).
    ForcedOpen = 1,
}
impl core::convert::TryFrom<u8> for VbusPathSelectionControl {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::ControlledByNvbusenPin),
            1 => Ok(Self::ForcedOpen),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "VbusPathSelectionControl",
                })
            }
        }
    }
}
impl From<VbusPathSelectionControl> for u8 {
    fn from(val: VbusPathSelectionControl) -> Self {
        match val {
            VbusPathSelectionControl::ControlledByNvbusenPin => 0,
            VbusPathSelectionControl::ForcedOpen => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VholdVoltageValue {
    ///4.0V
    V40 = 0,
    ///4.1V
    V41 = 1,
    ///4.2V
    V42 = 2,
    ///4.3V
    V43 = 3,
    ///4.4V
    V44 = 4,
    ///4.5V
    V45 = 5,
    ///4.6V
    V46 = 6,
    ///4.7V
    V47 = 7,
}
impl core::convert::TryFrom<u8> for VholdVoltageValue {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::V40),
            1 => Ok(Self::V41),
            2 => Ok(Self::V42),
            3 => Ok(Self::V43),
            4 => Ok(Self::V44),
            5 => Ok(Self::V45),
            6 => Ok(Self::V46),
            7 => Ok(Self::V47),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "VholdVoltageValue",
                })
            }
        }
    }
}
impl From<VholdVoltageValue> for u8 {
    fn from(val: VholdVoltageValue) -> Self {
        match val {
            VholdVoltageValue::V40 => 0,
            VholdVoltageValue::V41 => 1,
            VholdVoltageValue::V42 => 2,
            VholdVoltageValue::V43 => 3,
            VholdVoltageValue::V44 => 4,
            VholdVoltageValue::V45 => 5,
            VholdVoltageValue::V46 => 6,
            VholdVoltageValue::V47 => 7,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusCurrentLimitValue {
    ///500mA limit.
    Ma500 = 0,
    ///100mA limit.
    Ma100 = 1,
}
impl core::convert::TryFrom<u8> for VbusCurrentLimitValue {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Ma500),
            1 => Ok(Self::Ma100),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "VbusCurrentLimitValue",
                })
            }
        }
    }
}
impl From<VbusCurrentLimitValue> for u8 {
    fn from(val: VbusCurrentLimitValue) -> Self {
        match val {
            VbusCurrentLimitValue::Ma500 => 0,
            VbusCurrentLimitValue::Ma100 => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VoffVoltageValue {
    ///2.6V
    V26 = 0,
    ///2.7V
    V27 = 1,
    ///2.8V
    V28 = 2,
    ///2.9V
    V29 = 3,
    ///3.0V
    V30 = 4,
    ///3.1V
    V31 = 5,
    ///3.2V
    V32 = 6,
    ///3.3V
    V33 = 7,
}
impl core::convert::TryFrom<u8> for VoffVoltageValue {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::V26),
            1 => Ok(Self::V27),
            2 => Ok(Self::V28),
            3 => Ok(Self::V29),
            4 => Ok(Self::V30),
            5 => Ok(Self::V31),
            6 => Ok(Self::V32),
            7 => Ok(Self::V33),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "VoffVoltageValue",
                })
            }
        }
    }
}
impl From<VoffVoltageValue> for u8 {
    fn from(val: VoffVoltageValue) -> Self {
        match val {
            VoffVoltageValue::V26 => 0,
            VoffVoltageValue::V27 => 1,
            VoffVoltageValue::V28 => 2,
            VoffVoltageValue::V29 => 3,
            VoffVoltageValue::V30 => 4,
            VoffVoltageValue::V31 => 5,
            VoffVoltageValue::V32 => 6,
            VoffVoltageValue::V33 => 7,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChgLedFunctionSetting {
    ///High Impedance.
    HighZ = 0,
    ///1Hz blink (25% duty).
    Blink1Hz = 1,
    ///4Hz blink (25% duty).
    Blink4Hz = 2,
    ///Output Low.
    OutputLow = 3,
}
impl core::convert::TryFrom<u8> for ChgLedFunctionSetting {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::HighZ),
            1 => Ok(Self::Blink1Hz),
            2 => Ok(Self::Blink4Hz),
            3 => Ok(Self::OutputLow),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "ChgLedFunctionSetting",
                })
            }
        }
    }
}
impl From<ChgLedFunctionSetting> for u8 {
    fn from(val: ChgLedFunctionSetting) -> Self {
        match val {
            ChgLedFunctionSetting::HighZ => 0,
            ChgLedFunctionSetting::Blink1Hz => 1,
            ChgLedFunctionSetting::Blink4Hz => 2,
            ChgLedFunctionSetting::OutputLow => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChgLedControlSourceSelect {
    ///Controlled by charging function.
    ByChargeLogic = 0,
    ///Controlled by this register's chgled_function field.
    ByRegisterSetting = 1,
}
impl core::convert::TryFrom<u8> for ChgLedControlSourceSelect {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::ByChargeLogic),
            1 => Ok(Self::ByRegisterSetting),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "ChgLedControlSourceSelect",
                })
            }
        }
    }
}
impl From<ChgLedControlSourceSelect> for u8 {
    fn from(val: ChgLedControlSourceSelect) -> Self {
        match val {
            ChgLedControlSourceSelect::ByChargeLogic => 0,
            ChgLedControlSourceSelect::ByRegisterSetting => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NoeShutdownDelayValue {
    ///0.5S delay.
    Ms500 = 0,
    ///1S delay.
    S1 = 1,
    ///2S delay.
    S2 = 2,
    ///3S delay.
    S3 = 3,
}
impl core::convert::TryFrom<u8> for NoeShutdownDelayValue {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Ms500),
            1 => Ok(Self::S1),
            2 => Ok(Self::S2),
            3 => Ok(Self::S3),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "NoeShutdownDelayValue",
                })
            }
        }
    }
}
impl From<NoeShutdownDelayValue> for u8 {
    fn from(val: NoeShutdownDelayValue) -> Self {
        match val {
            NoeShutdownDelayValue::Ms500 => 0,
            NoeShutdownDelayValue::S1 => 1,
            NoeShutdownDelayValue::S2 => 2,
            NoeShutdownDelayValue::S3 => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChargeTargetVoltageValue {
    ///4.10V
    V410 = 0,
    ///4.15V
    V415 = 1,
    ///4.20V
    V420 = 2,
    ///4.36V
    V436 = 3,
}
impl core::convert::TryFrom<u8> for ChargeTargetVoltageValue {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::V410),
            1 => Ok(Self::V415),
            2 => Ok(Self::V420),
            3 => Ok(Self::V436),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "ChargeTargetVoltageValue",
                })
            }
        }
    }
}
impl From<ChargeTargetVoltageValue> for u8 {
    fn from(val: ChargeTargetVoltageValue) -> Self {
        match val {
            ChargeTargetVoltageValue::V410 => 0,
            ChargeTargetVoltageValue::V415 => 1,
            ChargeTargetVoltageValue::V420 => 2,
            ChargeTargetVoltageValue::V436 => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChargeEndCurrentThresholdValue {
    ///Terminate at <10% of set current.
    Percent10 = 0,
    ///Terminate at <15% of set current.
    Percent15 = 1,
}
impl core::convert::TryFrom<u8> for ChargeEndCurrentThresholdValue {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Percent10),
            1 => Ok(Self::Percent15),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "ChargeEndCurrentThresholdValue",
                })
            }
        }
    }
}
impl From<ChargeEndCurrentThresholdValue> for u8 {
    fn from(val: ChargeEndCurrentThresholdValue) -> Self {
        match val {
            ChargeEndCurrentThresholdValue::Percent10 => 0,
            ChargeEndCurrentThresholdValue::Percent15 => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChargeCurrentValue {
    ///100mA
    Ma100 = 0,
    ///190mA
    Ma190 = 1,
    ///280mA
    Ma280 = 2,
    ///360mA
    Ma360 = 3,
    ///450mA
    Ma450 = 4,
    ///550mA
    Ma550 = 5,
    ///630mA
    Ma630 = 6,
    ///700mA
    Ma700 = 7,
    ///780mA
    Ma780 = 8,
    ///880mA
    Ma880 = 9,
    ///960mA
    Ma960 = 10,
    ///1000mA
    Ma1000 = 11,
    ///1080mA
    Ma1080 = 12,
    ///1160mA
    Ma1160 = 13,
    ///1240mA
    Ma1240 = 14,
    ///1320mA
    Ma1320 = 15,
}
impl core::convert::TryFrom<u8> for ChargeCurrentValue {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Ma100),
            1 => Ok(Self::Ma190),
            2 => Ok(Self::Ma280),
            3 => Ok(Self::Ma360),
            4 => Ok(Self::Ma450),
            5 => Ok(Self::Ma550),
            6 => Ok(Self::Ma630),
            7 => Ok(Self::Ma700),
            8 => Ok(Self::Ma780),
            9 => Ok(Self::Ma880),
            10 => Ok(Self::Ma960),
            11 => Ok(Self::Ma1000),
            12 => Ok(Self::Ma1080),
            13 => Ok(Self::Ma1160),
            14 => Ok(Self::Ma1240),
            15 => Ok(Self::Ma1320),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "ChargeCurrentValue",
                })
            }
        }
    }
}
impl From<ChargeCurrentValue> for u8 {
    fn from(val: ChargeCurrentValue) -> Self {
        match val {
            ChargeCurrentValue::Ma100 => 0,
            ChargeCurrentValue::Ma190 => 1,
            ChargeCurrentValue::Ma280 => 2,
            ChargeCurrentValue::Ma360 => 3,
            ChargeCurrentValue::Ma450 => 4,
            ChargeCurrentValue::Ma550 => 5,
            ChargeCurrentValue::Ma630 => 6,
            ChargeCurrentValue::Ma700 => 7,
            ChargeCurrentValue::Ma780 => 8,
            ChargeCurrentValue::Ma880 => 9,
            ChargeCurrentValue::Ma960 => 10,
            ChargeCurrentValue::Ma1000 => 11,
            ChargeCurrentValue::Ma1080 => 12,
            ChargeCurrentValue::Ma1160 => 13,
            ChargeCurrentValue::Ma1240 => 14,
            ChargeCurrentValue::Ma1320 => 15,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PrechargeTimeoutValue {
    ///30 minutes timeout.
    Min30 = 0,
    ///40 minutes timeout.
    Min40 = 1,
    ///50 minutes timeout.
    Min50 = 2,
    ///60 minutes timeout.
    Min60 = 3,
}
impl core::convert::TryFrom<u8> for PrechargeTimeoutValue {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Min30),
            1 => Ok(Self::Min40),
            2 => Ok(Self::Min50),
            3 => Ok(Self::Min60),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "PrechargeTimeoutValue",
                })
            }
        }
    }
}
impl From<PrechargeTimeoutValue> for u8 {
    fn from(val: PrechargeTimeoutValue) -> Self {
        match val {
            PrechargeTimeoutValue::Min30 => 0,
            PrechargeTimeoutValue::Min40 => 1,
            PrechargeTimeoutValue::Min50 => 2,
            PrechargeTimeoutValue::Min60 => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExternalPathChargeCurrentValue {
    ///300mA
    Ma300 = 0,
    ///400mA
    Ma400 = 1,
    ///500mA
    Ma500 = 2,
    ///600mA
    Ma600 = 3,
    ///700mA
    Ma700 = 4,
    ///800mA
    Ma800 = 5,
    ///900mA
    Ma900 = 6,
    ///1000mA
    Ma1000 = 7,
}
impl core::convert::TryFrom<u8> for ExternalPathChargeCurrentValue {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Ma300),
            1 => Ok(Self::Ma400),
            2 => Ok(Self::Ma500),
            3 => Ok(Self::Ma600),
            4 => Ok(Self::Ma700),
            5 => Ok(Self::Ma800),
            6 => Ok(Self::Ma900),
            7 => Ok(Self::Ma1000),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "ExternalPathChargeCurrentValue",
                })
            }
        }
    }
}
impl From<ExternalPathChargeCurrentValue> for u8 {
    fn from(val: ExternalPathChargeCurrentValue) -> Self {
        match val {
            ExternalPathChargeCurrentValue::Ma300 => 0,
            ExternalPathChargeCurrentValue::Ma400 => 1,
            ExternalPathChargeCurrentValue::Ma500 => 2,
            ExternalPathChargeCurrentValue::Ma600 => 3,
            ExternalPathChargeCurrentValue::Ma700 => 4,
            ExternalPathChargeCurrentValue::Ma800 => 5,
            ExternalPathChargeCurrentValue::Ma900 => 6,
            ExternalPathChargeCurrentValue::Ma1000 => 7,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConstantCurrentTimeoutValue {
    ///7 hours timeout.
    Hours7 = 0,
    ///8 hours timeout.
    Hours8 = 1,
    ///9 hours timeout.
    Hours9 = 2,
    ///10 hours timeout.
    Hours10 = 3,
}
impl core::convert::TryFrom<u8> for ConstantCurrentTimeoutValue {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Hours7),
            1 => Ok(Self::Hours8),
            2 => Ok(Self::Hours9),
            3 => Ok(Self::Hours10),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "ConstantCurrentTimeoutValue",
                })
            }
        }
    }
}
impl From<ConstantCurrentTimeoutValue> for u8 {
    fn from(val: ConstantCurrentTimeoutValue) -> Self {
        match val {
            ConstantCurrentTimeoutValue::Hours7 => 0,
            ConstantCurrentTimeoutValue::Hours8 => 1,
            ConstantCurrentTimeoutValue::Hours9 => 2,
            ConstantCurrentTimeoutValue::Hours10 => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BackupTargetVoltageValue {
    ///3.1V target.
    V31 = 0,
    ///3.0V target.
    V30 = 1,
    ///3.0V target (alternative?).
    V30Alt = 2,
    ///2.5V target.
    V25 = 3,
}
impl core::convert::TryFrom<u8> for BackupTargetVoltageValue {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::V31),
            1 => Ok(Self::V30),
            2 => Ok(Self::V30Alt),
            3 => Ok(Self::V25),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "BackupTargetVoltageValue",
                })
            }
        }
    }
}
impl From<BackupTargetVoltageValue> for u8 {
    fn from(val: BackupTargetVoltageValue) -> Self {
        match val {
            BackupTargetVoltageValue::V31 => 0,
            BackupTargetVoltageValue::V30 => 1,
            BackupTargetVoltageValue::V30Alt => 2,
            BackupTargetVoltageValue::V25 => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BackupChargeCurrentValue {
    ///50uA charge current.
    Ua50 = 0,
    ///100uA charge current.
    Ua100 = 1,
    ///200uA charge current.
    Ua200 = 2,
    ///400uA charge current.
    Ua400 = 3,
}
impl core::convert::TryFrom<u8> for BackupChargeCurrentValue {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Ua50),
            1 => Ok(Self::Ua100),
            2 => Ok(Self::Ua200),
            3 => Ok(Self::Ua400),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "BackupChargeCurrentValue",
                })
            }
        }
    }
}
impl From<BackupChargeCurrentValue> for u8 {
    fn from(val: BackupChargeCurrentValue) -> Self {
        match val {
            BackupChargeCurrentValue::Ua50 => 0,
            BackupChargeCurrentValue::Ua100 => 1,
            BackupChargeCurrentValue::Ua200 => 2,
            BackupChargeCurrentValue::Ua400 => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PekPowerOnTime {
    ///128ms
    Ms128 = 0,
    ///512ms
    Ms512 = 1,
    ///1 Second
    S1 = 2,
    ///2 Seconds
    S2 = 3,
}
impl core::convert::TryFrom<u8> for PekPowerOnTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Ms128),
            1 => Ok(Self::Ms512),
            2 => Ok(Self::S1),
            3 => Ok(Self::S2),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "PekPowerOnTime",
                })
            }
        }
    }
}
impl From<PekPowerOnTime> for u8 {
    fn from(val: PekPowerOnTime) -> Self {
        match val {
            PekPowerOnTime::Ms128 => 0,
            PekPowerOnTime::Ms512 => 1,
            PekPowerOnTime::S1 => 2,
            PekPowerOnTime::S2 => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PekLongPressTime {
    ///1.0 Second
    S10 = 0,
    ///1.5 Seconds
    S15 = 1,
    ///2.0 Seconds
    S20 = 2,
    ///2.5 Seconds
    S25 = 3,
}
impl core::convert::TryFrom<u8> for PekLongPressTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::S10),
            1 => Ok(Self::S15),
            2 => Ok(Self::S20),
            3 => Ok(Self::S25),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "PekLongPressTime",
                })
            }
        }
    }
}
impl From<PekLongPressTime> for u8 {
    fn from(val: PekLongPressTime) -> Self {
        match val {
            PekLongPressTime::S10 => 0,
            PekLongPressTime::S15 => 1,
            PekLongPressTime::S20 => 2,
            PekLongPressTime::S25 => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwrokSignalDelay {
    ///32ms delay.
    Ms32 = 0,
    ///64ms delay.
    Ms64 = 1,
}
impl core::convert::TryFrom<u8> for PwrokSignalDelay {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Ms32),
            1 => Ok(Self::Ms64),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "PwrokSignalDelay",
                })
            }
        }
    }
}
impl From<PwrokSignalDelay> for u8 {
    fn from(val: PwrokSignalDelay) -> Self {
        match val {
            PwrokSignalDelay::Ms32 => 0,
            PwrokSignalDelay::Ms64 => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PekShutdownTime {
    ///4 Seconds
    S4 = 0,
    ///6 Seconds
    S6 = 1,
    ///8 Seconds
    S8 = 2,
    ///10 Seconds
    S10 = 3,
}
impl core::convert::TryFrom<u8> for PekShutdownTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::S4),
            1 => Ok(Self::S6),
            2 => Ok(Self::S8),
            3 => Ok(Self::S10),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "PekShutdownTime",
                })
            }
        }
    }
}
impl From<PekShutdownTime> for u8 {
    fn from(val: PekShutdownTime) -> Self {
        match val {
            PekShutdownTime::S4 => 0,
            PekShutdownTime::S6 => 1,
            PekShutdownTime::S8 => 2,
            PekShutdownTime::S10 => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcDcModeSelection {
    ///Automatic PFM/PWM switching.
    AutoPfmPwm = 0,
    ///Fixed PWM mode.
    FixedPwm = 1,
}
impl core::convert::TryFrom<u8> for DcDcModeSelection {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::AutoPfmPwm),
            1 => Ok(Self::FixedPwm),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "DcDcModeSelection",
                })
            }
        }
    }
}
impl From<DcDcModeSelection> for u8 {
    fn from(val: DcDcModeSelection) -> Self {
        match val {
            DcDcModeSelection::AutoPfmPwm => 0,
            DcDcModeSelection::FixedPwm => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcSampleRateValue {
    ///25Hz sampling rate.
    Hz25 = 0,
    ///50Hz sampling rate.
    Hz50 = 1,
    ///100Hz sampling rate.
    Hz100 = 2,
    ///200Hz sampling rate.
    Hz200 = 3,
}
impl core::convert::TryFrom<u8> for AdcSampleRateValue {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Hz25),
            1 => Ok(Self::Hz50),
            2 => Ok(Self::Hz100),
            3 => Ok(Self::Hz200),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "AdcSampleRateValue",
                })
            }
        }
    }
}
impl From<AdcSampleRateValue> for u8 {
    fn from(val: AdcSampleRateValue) -> Self {
        match val {
            AdcSampleRateValue::Hz25 => 0,
            AdcSampleRateValue::Hz50 => 1,
            AdcSampleRateValue::Hz100 => 2,
            AdcSampleRateValue::Hz200 => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsPinCurrentValue {
    ///20uA output current.
    Ua20 = 0,
    ///40uA output current.
    Ua40 = 1,
    ///60uA output current.
    Ua60 = 2,
    ///80uA output current.
    Ua80 = 3,
}
impl core::convert::TryFrom<u8> for TsPinCurrentValue {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Ua20),
            1 => Ok(Self::Ua40),
            2 => Ok(Self::Ua60),
            3 => Ok(Self::Ua80),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "TsPinCurrentValue",
                })
            }
        }
    }
}
impl From<TsPinCurrentValue> for u8 {
    fn from(val: TsPinCurrentValue) -> Self {
        match val {
            TsPinCurrentValue::Ua20 => 0,
            TsPinCurrentValue::Ua40 => 1,
            TsPinCurrentValue::Ua60 => 2,
            TsPinCurrentValue::Ua80 => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsPinOutputMode {
    ///TS pin current output is disabled.
    Disabled = 0,
    ///TS pin current output enabled during charging.
    DuringCharging = 1,
    ///TS pin current output enabled only during ADC sampling (power saving).
    DuringAdcSampling = 2,
    ///TS pin current output is always enabled.
    AlwaysEnabled = 3,
}
impl core::convert::TryFrom<u8> for TsPinOutputMode {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Disabled),
            1 => Ok(Self::DuringCharging),
            2 => Ok(Self::DuringAdcSampling),
            3 => Ok(Self::AlwaysEnabled),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "TsPinOutputMode",
                })
            }
        }
    }
}
impl From<TsPinOutputMode> for u8 {
    fn from(val: TsPinOutputMode) -> Self {
        match val {
            TsPinOutputMode::Disabled => 0,
            TsPinOutputMode::DuringCharging => 1,
            TsPinOutputMode::DuringAdcSampling => 2,
            TsPinOutputMode::AlwaysEnabled => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioAdcRange {
    ///0V to 2.0475V input range.
    Range00To20475V = 0,
    ///0.7V to 2.7475V input range.
    Range07To27475V = 1,
}
impl core::convert::TryFrom<u8> for GpioAdcRange {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Range00To20475V),
            1 => Ok(Self::Range07To27475V),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "GpioAdcRange",
                })
            }
        }
    }
}
impl From<GpioAdcRange> for u8 {
    fn from(val: GpioAdcRange) -> Self {
        match val {
            GpioAdcRange::Range00To20475V => 0,
            GpioAdcRange::Range07To27475V => 1,
        }
    }
}
