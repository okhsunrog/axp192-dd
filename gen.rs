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
        let reg = self.gpio_1_adc_irq_rising_threshold().read()?;
        callback(134 + 0 * 0, "gpio_1_adc_irq_rising_threshold", reg.into());
        let reg = self.gpio_1_adc_irq_falling_threshold().read()?;
        callback(135 + 0 * 0, "gpio_1_adc_irq_falling_threshold", reg.into());
        let reg = self.timer_control().read()?;
        callback(138 + 0 * 0, "timer_control", reg.into());
        let reg = self.vbus_srp_control().read()?;
        callback(139 + 0 * 0, "vbus_srp_control", reg.into());
        let reg = self.over_temp_shutdown_control().read()?;
        callback(143 + 0 * 0, "over_temp_shutdown_control", reg.into());
        let reg = self.gpio_0_control().read()?;
        callback(144 + 0 * 0, "gpio_0_control", reg.into());
        let reg = self.gpio_0_ldo_voltage_setting().read()?;
        callback(145 + 0 * 0, "gpio_0_ldo_voltage_setting", reg.into());
        let reg = self.gpio_1_control().read()?;
        callback(146 + 0 * 0, "gpio_1_control", reg.into());
        let reg = self.gpio_2_control().read()?;
        callback(147 + 0 * 0, "gpio_2_control", reg.into());
        let reg = self.gpio_0_to_2_signal_status_and_control().read()?;
        callback(148 + 0 * 0, "gpio_0_to_2_signal_status_and_control", reg.into());
        let reg = self.gpio_3_and_4_function_control().read()?;
        callback(149 + 0 * 0, "gpio_3_and_4_function_control", reg.into());
        let reg = self.gpio_3_and_4_signal_status_and_control().read()?;
        callback(150 + 0 * 0, "gpio_3_and_4_signal_status_and_control", reg.into());
        let reg = self.gpio_0_to_2_pulldown_control().read()?;
        callback(151 + 0 * 0, "gpio_0_to_2_pulldown_control", reg.into());
        let reg = self.pwm_1_frequency_setting().read()?;
        callback(152 + 0 * 0, "pwm_1_frequency_setting", reg.into());
        let reg = self.pwm_1_duty_cycle_setting_y_1().read()?;
        callback(153 + 0 * 0, "pwm_1_duty_cycle_setting_y_1", reg.into());
        let reg = self.pwm_1_duty_cycle_setting_y_2().read()?;
        callback(154 + 0 * 0, "pwm_1_duty_cycle_setting_y_2", reg.into());
        let reg = self.pwm_2_frequency_setting().read()?;
        callback(155 + 0 * 0, "pwm_2_frequency_setting", reg.into());
        let reg = self.pwm_2_duty_cycle_setting_y_1().read()?;
        callback(156 + 0 * 0, "pwm_2_duty_cycle_setting_y_1", reg.into());
        let reg = self.pwm_2_duty_cycle_setting_y_2().read()?;
        callback(157 + 0 * 0, "pwm_2_duty_cycle_setting_y_2", reg.into());
        let reg = self.nrsto_gpio_5_control().read()?;
        callback(158 + 0 * 0, "nrsto_gpio_5_control", reg.into());
        let reg = self.irq_enable_control_1().read()?;
        callback(64 + 0 * 0, "irq_enable_control_1", reg.into());
        let reg = self.irq_enable_control_2().read()?;
        callback(65 + 0 * 0, "irq_enable_control_2", reg.into());
        let reg = self.irq_enable_control_3().read()?;
        callback(66 + 0 * 0, "irq_enable_control_3", reg.into());
        let reg = self.irq_enable_control_4().read()?;
        callback(67 + 0 * 0, "irq_enable_control_4", reg.into());
        let reg = self.irq_enable_control_5().read()?;
        callback(74 + 0 * 0, "irq_enable_control_5", reg.into());
        let reg = self.irq_status_1().read()?;
        callback(68 + 0 * 0, "irq_status_1", reg.into());
        let reg = self.irq_status_2().read()?;
        callback(69 + 0 * 0, "irq_status_2", reg.into());
        let reg = self.irq_status_3().read()?;
        callback(70 + 0 * 0, "irq_status_3", reg.into());
        let reg = self.irq_status_4().read()?;
        callback(71 + 0 * 0, "irq_status_4", reg.into());
        let reg = self.irq_status_5().read()?;
        callback(77 + 0 * 0, "irq_status_5", reg.into());
        let reg = self.acin_voltage_adc().read()?;
        callback(86 + 0 * 0, "acin_voltage_adc", reg.into());
        let reg = self.acin_current_adc().read()?;
        callback(88 + 0 * 0, "acin_current_adc", reg.into());
        let reg = self.vbus_voltage_adc().read()?;
        callback(90 + 0 * 0, "vbus_voltage_adc", reg.into());
        let reg = self.vbus_current_adc().read()?;
        callback(92 + 0 * 0, "vbus_current_adc", reg.into());
        let reg = self.internal_temperature_adc().read()?;
        callback(94 + 0 * 0, "internal_temperature_adc", reg.into());
        let reg = self.ts_pin_adc().read()?;
        callback(98 + 0 * 0, "ts_pin_adc", reg.into());
        let reg = self.gpio_voltage_adc(0).read()?;
        callback(100 + 0 * 2, "gpio_voltage_adc[0]", reg.into());
        let reg = self.gpio_voltage_adc(1).read()?;
        callback(100 + 1 * 2, "gpio_voltage_adc[1]", reg.into());
        let reg = self.gpio_voltage_adc(2).read()?;
        callback(100 + 2 * 2, "gpio_voltage_adc[2]", reg.into());
        let reg = self.gpio_voltage_adc(3).read()?;
        callback(100 + 3 * 2, "gpio_voltage_adc[3]", reg.into());
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
        let reg = self.gpio_1_adc_irq_rising_threshold().read_async().await?;
        callback(134 + 0 * 0, "gpio_1_adc_irq_rising_threshold", reg.into());
        let reg = self.gpio_1_adc_irq_falling_threshold().read_async().await?;
        callback(135 + 0 * 0, "gpio_1_adc_irq_falling_threshold", reg.into());
        let reg = self.timer_control().read_async().await?;
        callback(138 + 0 * 0, "timer_control", reg.into());
        let reg = self.vbus_srp_control().read_async().await?;
        callback(139 + 0 * 0, "vbus_srp_control", reg.into());
        let reg = self.over_temp_shutdown_control().read_async().await?;
        callback(143 + 0 * 0, "over_temp_shutdown_control", reg.into());
        let reg = self.gpio_0_control().read_async().await?;
        callback(144 + 0 * 0, "gpio_0_control", reg.into());
        let reg = self.gpio_0_ldo_voltage_setting().read_async().await?;
        callback(145 + 0 * 0, "gpio_0_ldo_voltage_setting", reg.into());
        let reg = self.gpio_1_control().read_async().await?;
        callback(146 + 0 * 0, "gpio_1_control", reg.into());
        let reg = self.gpio_2_control().read_async().await?;
        callback(147 + 0 * 0, "gpio_2_control", reg.into());
        let reg = self.gpio_0_to_2_signal_status_and_control().read_async().await?;
        callback(148 + 0 * 0, "gpio_0_to_2_signal_status_and_control", reg.into());
        let reg = self.gpio_3_and_4_function_control().read_async().await?;
        callback(149 + 0 * 0, "gpio_3_and_4_function_control", reg.into());
        let reg = self.gpio_3_and_4_signal_status_and_control().read_async().await?;
        callback(150 + 0 * 0, "gpio_3_and_4_signal_status_and_control", reg.into());
        let reg = self.gpio_0_to_2_pulldown_control().read_async().await?;
        callback(151 + 0 * 0, "gpio_0_to_2_pulldown_control", reg.into());
        let reg = self.pwm_1_frequency_setting().read_async().await?;
        callback(152 + 0 * 0, "pwm_1_frequency_setting", reg.into());
        let reg = self.pwm_1_duty_cycle_setting_y_1().read_async().await?;
        callback(153 + 0 * 0, "pwm_1_duty_cycle_setting_y_1", reg.into());
        let reg = self.pwm_1_duty_cycle_setting_y_2().read_async().await?;
        callback(154 + 0 * 0, "pwm_1_duty_cycle_setting_y_2", reg.into());
        let reg = self.pwm_2_frequency_setting().read_async().await?;
        callback(155 + 0 * 0, "pwm_2_frequency_setting", reg.into());
        let reg = self.pwm_2_duty_cycle_setting_y_1().read_async().await?;
        callback(156 + 0 * 0, "pwm_2_duty_cycle_setting_y_1", reg.into());
        let reg = self.pwm_2_duty_cycle_setting_y_2().read_async().await?;
        callback(157 + 0 * 0, "pwm_2_duty_cycle_setting_y_2", reg.into());
        let reg = self.nrsto_gpio_5_control().read_async().await?;
        callback(158 + 0 * 0, "nrsto_gpio_5_control", reg.into());
        let reg = self.irq_enable_control_1().read_async().await?;
        callback(64 + 0 * 0, "irq_enable_control_1", reg.into());
        let reg = self.irq_enable_control_2().read_async().await?;
        callback(65 + 0 * 0, "irq_enable_control_2", reg.into());
        let reg = self.irq_enable_control_3().read_async().await?;
        callback(66 + 0 * 0, "irq_enable_control_3", reg.into());
        let reg = self.irq_enable_control_4().read_async().await?;
        callback(67 + 0 * 0, "irq_enable_control_4", reg.into());
        let reg = self.irq_enable_control_5().read_async().await?;
        callback(74 + 0 * 0, "irq_enable_control_5", reg.into());
        let reg = self.irq_status_1().read_async().await?;
        callback(68 + 0 * 0, "irq_status_1", reg.into());
        let reg = self.irq_status_2().read_async().await?;
        callback(69 + 0 * 0, "irq_status_2", reg.into());
        let reg = self.irq_status_3().read_async().await?;
        callback(70 + 0 * 0, "irq_status_3", reg.into());
        let reg = self.irq_status_4().read_async().await?;
        callback(71 + 0 * 0, "irq_status_4", reg.into());
        let reg = self.irq_status_5().read_async().await?;
        callback(77 + 0 * 0, "irq_status_5", reg.into());
        let reg = self.acin_voltage_adc().read_async().await?;
        callback(86 + 0 * 0, "acin_voltage_adc", reg.into());
        let reg = self.acin_current_adc().read_async().await?;
        callback(88 + 0 * 0, "acin_current_adc", reg.into());
        let reg = self.vbus_voltage_adc().read_async().await?;
        callback(90 + 0 * 0, "vbus_voltage_adc", reg.into());
        let reg = self.vbus_current_adc().read_async().await?;
        callback(92 + 0 * 0, "vbus_current_adc", reg.into());
        let reg = self.internal_temperature_adc().read_async().await?;
        callback(94 + 0 * 0, "internal_temperature_adc", reg.into());
        let reg = self.ts_pin_adc().read_async().await?;
        callback(98 + 0 * 0, "ts_pin_adc", reg.into());
        let reg = self.gpio_voltage_adc(0).read_async().await?;
        callback(100 + 0 * 2, "gpio_voltage_adc[0]", reg.into());
        let reg = self.gpio_voltage_adc(1).read_async().await?;
        callback(100 + 1 * 2, "gpio_voltage_adc[1]", reg.into());
        let reg = self.gpio_voltage_adc(2).read_async().await?;
        callback(100 + 2 * 2, "gpio_voltage_adc[2]", reg.into());
        let reg = self.gpio_voltage_adc(3).read_async().await?;
        callback(100 + 3 * 2, "gpio_voltage_adc[3]", reg.into());
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
    ///Sets the rising edge voltage threshold for GPIO1 ADC input to trigger an interrupt.
    ///An IRQ is generated when the GPIO1 ADC voltage rises above this set threshold.
    ///Formula: Threshold Voltage (V) = raw_value * 0.008.
    ///Range: 0V (raw 0x00) to 2.04V (raw 0xFF).
    pub fn gpio_1_adc_irq_rising_threshold(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Gpio1AdcIrqRisingThreshold,
        ::device_driver::RW,
    > {
        let address = self.base_address + 134;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Gpio1AdcIrqRisingThreshold,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::Gpio1AdcIrqRisingThreshold::new,
        )
    }
    ///Sets the falling edge voltage threshold for GPIO1 ADC input to trigger an interrupt.
    ///An IRQ is generated when the GPIO1 ADC voltage falls below this set threshold.
    ///Formula: Threshold Voltage (V) = raw_value * 0.008.
    ///Range: 0V (raw 0x00) to 2.04V (raw 0xFF).
    pub fn gpio_1_adc_irq_falling_threshold(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Gpio1AdcIrqFallingThreshold,
        ::device_driver::RW,
    > {
        let address = self.base_address + 135;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Gpio1AdcIrqFallingThreshold,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::Gpio1AdcIrqFallingThreshold::new,
        )
    }
    ///Controls an internal timer, sets its duration, and indicates timeout status.
    pub fn timer_control(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::TimerControl,
        ::device_driver::RW,
    > {
        let address = self.base_address + 138;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::TimerControl,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::TimerControl::new)
    }
    ///Controls VBUS pin monitoring for Session Request Protocol (SRP) related functions, including valid voltage threshold and SRP feature enables.
    pub fn vbus_srp_control(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::VbusSrpControl,
        ::device_driver::RW,
    > {
        let address = self.base_address + 139;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::VbusSrpControl,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::VbusSrpControl::new)
    }
    ///Controls the AXP192 internal over-temperature shutdown function. Other bits are reserved.
    pub fn over_temp_shutdown_control(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::OverTempShutdownControl,
        ::device_driver::RW,
    > {
        let address = self.base_address + 143;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::OverTempShutdownControl,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::OverTempShutdownControl::new)
    }
    ///Configures the function of the GPIO0 pin.
    pub fn gpio_0_control(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Gpio0Control,
        ::device_driver::RW,
    > {
        let address = self.base_address + 144;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Gpio0Control,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Gpio0Control::new)
    }
    ///Sets the output voltage for GPIO0 when it is configured in Low Noise LDO (LDOIO0) mode (via REG90H).
    pub fn gpio_0_ldo_voltage_setting(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Gpio0LdoVoltageSetting,
        ::device_driver::RW,
    > {
        let address = self.base_address + 145;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Gpio0LdoVoltageSetting,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Gpio0LdoVoltageSetting::new)
    }
    ///Configures the function of the GPIO1 pin.
    pub fn gpio_1_control(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Gpio1Control,
        ::device_driver::RW,
    > {
        let address = self.base_address + 146;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Gpio1Control,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Gpio1Control::new)
    }
    ///Configures the function of the GPIO2 pin.
    pub fn gpio_2_control(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Gpio2Control,
        ::device_driver::RW,
    > {
        let address = self.base_address + 147;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Gpio2Control,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Gpio2Control::new)
    }
    ///Monitors input status for GPIO0-GPIO2 and controls their output levels
    ///when configured as NMOS open-drain or output low.
    pub fn gpio_0_to_2_signal_status_and_control(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Gpio0To2SignalStatusAndControl,
        ::device_driver::RW,
    > {
        let address = self.base_address + 148;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Gpio0To2SignalStatusAndControl,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::Gpio0To2SignalStatusAndControl::new,
        )
    }
    ///Configures the function for GPIO3 and GPIO4 pins, and enables their GPIO mode.
    pub fn gpio_3_and_4_function_control(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Gpio3And4FunctionControl,
        ::device_driver::RW,
    > {
        let address = self.base_address + 149;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Gpio3And4FunctionControl,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::Gpio3And4FunctionControl::new,
        )
    }
    ///Monitors input status for GPIO3-GPIO4 and controls their output levels
    ///when configured as NMOS open-drain or output low.
    pub fn gpio_3_and_4_signal_status_and_control(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Gpio3And4SignalStatusAndControl,
        ::device_driver::RW,
    > {
        let address = self.base_address + 150;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Gpio3And4SignalStatusAndControl,
            ::device_driver::RW,
        >::new(
            self.interface(),
            address as u8,
            field_sets::Gpio3And4SignalStatusAndControl::new,
        )
    }
    ///Controls internal pull-down resistors for GPIO0, GPIO1, and GPIO2 when they are configured as inputs.
    pub fn gpio_0_to_2_pulldown_control(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Gpio0To2PulldownControl,
        ::device_driver::RW,
    > {
        let address = self.base_address + 151;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Gpio0To2PulldownControl,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Gpio0To2PulldownControl::new)
    }
    ///Sets the 'X' parameter for PWM1 output frequency calculation.
    ///Formula: F_pwm1 = 2.25MHz / (X_value + 1) / Y1_value (where Y1 is from REG99H).
    pub fn pwm_1_frequency_setting(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Pwm1FrequencySetting,
        ::device_driver::RW,
    > {
        let address = self.base_address + 152;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Pwm1FrequencySetting,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Pwm1FrequencySetting::new)
    }
    ///Sets the 'Y1' parameter for PWM1 duty cycle and frequency calculations.
    ///Y1 is the denominator for duty cycle (Duty = Y2/Y1) and also affects frequency.
    ///Y1 should not be set to 0.
    pub fn pwm_1_duty_cycle_setting_y_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Pwm1DutyCycleSettingY1,
        ::device_driver::RW,
    > {
        let address = self.base_address + 153;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Pwm1DutyCycleSettingY1,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Pwm1DutyCycleSettingY1::new)
    }
    ///Sets the 'Y2' parameter (numerator) for PWM1 duty cycle calculation (Duty = Y2/Y1).
    ///Only upper 5 bits (bits 7-3) are used for Y2.
    pub fn pwm_1_duty_cycle_setting_y_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Pwm1DutyCycleSettingY2,
        ::device_driver::RW,
    > {
        let address = self.base_address + 154;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Pwm1DutyCycleSettingY2,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Pwm1DutyCycleSettingY2::new)
    }
    ///Sets the 'X' parameter for PWM2 output frequency calculation.
    ///Formula: F_pwm2 = 2.25MHz / (X_value + 1) / Y1_value (where Y1 is from REG9CH).
    pub fn pwm_2_frequency_setting(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Pwm1FrequencySetting,
        ::device_driver::RW,
    > {
        let address = self.base_address + 155;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Pwm1FrequencySetting,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Pwm1FrequencySetting::new)
    }
    ///Sets the 'Y1' parameter for PWM2 duty cycle and frequency calculations.
    ///Y1 is the denominator for duty cycle (Duty = Y2/Y1) and also affects frequency.
    ///Y1 should not be set to 0.
    pub fn pwm_2_duty_cycle_setting_y_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Pwm1DutyCycleSettingY1,
        ::device_driver::RW,
    > {
        let address = self.base_address + 156;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Pwm1DutyCycleSettingY1,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Pwm1DutyCycleSettingY1::new)
    }
    ///Sets the 'Y2' parameter (numerator) for PWM2 duty cycle calculation (Duty = Y2/Y1).
    ///Only upper 5 bits (bits 7-3) are used for Y2.
    pub fn pwm_2_duty_cycle_setting_y_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Pwm1DutyCycleSettingY2,
        ::device_driver::RW,
    > {
        let address = self.base_address + 157;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Pwm1DutyCycleSettingY2,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Pwm1DutyCycleSettingY2::new)
    }
    ///Configures the N_RSTO/GPIO5 pin function. It can operate as N_RSTO (LDO1 status monitor)
    ///or as a general-purpose I/O pin (GPIO5) with configurable direction and output state.
    pub fn nrsto_gpio_5_control(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::NrstoGpio5Control,
        ::device_driver::RW,
    > {
        let address = self.base_address + 158;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::NrstoGpio5Control,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::NrstoGpio5Control::new)
    }
    ///Interrupt Enable Control Register 1 (ACIN and VBUS related IRQs).
    pub fn irq_enable_control_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::IrqEnableControl1,
        ::device_driver::RW,
    > {
        let address = self.base_address + 64;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::IrqEnableControl1,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::IrqEnableControl1::new)
    }
    ///Interrupt Enable Control Register 2 (Battery and Charge related IRQs).
    pub fn irq_enable_control_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::IrqEnableControl2,
        ::device_driver::RW,
    > {
        let address = self.base_address + 65;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::IrqEnableControl2,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::IrqEnableControl2::new)
    }
    ///Interrupt Enable Control Register 3 (IC Temp, Charge Current, DCDC VLow, PEK IRQs).
    pub fn irq_enable_control_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::IrqEnableControl3,
        ::device_driver::RW,
    > {
        let address = self.base_address + 66;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::IrqEnableControl3,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::IrqEnableControl3::new)
    }
    ///Interrupt Enable Control Register 4 (N_OE and VBUS session/status related IRQs, APS Low Voltage).
    pub fn irq_enable_control_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::IrqEnableControl4,
        ::device_driver::RW,
    > {
        let address = self.base_address + 67;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::IrqEnableControl4,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::IrqEnableControl4::new)
    }
    ///Interrupt Enable Control Register 5 (Timer and GPIO Input Edge Trigger IRQs).
    pub fn irq_enable_control_5(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::IrqEnableControl5,
        ::device_driver::RW,
    > {
        let address = self.base_address + 74;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::IrqEnableControl5,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::IrqEnableControl5::new)
    }
    ///Interrupt Status Register 1. Bits are set when corresponding IRQ occurs. Write 1 to a bit to clear it.
    pub fn irq_status_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::IrqStatus1,
        ::device_driver::RW,
    > {
        let address = self.base_address + 68;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::IrqStatus1,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::IrqStatus1::new)
    }
    ///Interrupt Status Register 2. Bits are set when corresponding IRQ occurs. Write 1 to a bit to clear it.
    pub fn irq_status_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::IrqStatus2,
        ::device_driver::RW,
    > {
        let address = self.base_address + 69;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::IrqStatus2,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::IrqStatus2::new)
    }
    ///Interrupt Status Register 3. Bits are set when corresponding IRQ occurs. Write 1 to a bit to clear it.
    pub fn irq_status_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::IrqStatus3,
        ::device_driver::RW,
    > {
        let address = self.base_address + 70;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::IrqStatus3,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::IrqStatus3::new)
    }
    ///Interrupt Status Register 4. Bits are set when corresponding IRQ occurs. Write 1 to a bit to clear it.
    pub fn irq_status_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::IrqStatus4,
        ::device_driver::RW,
    > {
        let address = self.base_address + 71;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::IrqStatus4,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::IrqStatus4::new)
    }
    ///Interrupt Status Register 5. Bits are set when corresponding IRQ occurs. Write 1 to a bit to clear it.
    pub fn irq_status_5(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::IrqStatus5,
        ::device_driver::RW,
    > {
        let address = self.base_address + 77;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::IrqStatus5,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::IrqStatus5::new)
    }
    ///ACIN Voltage ADC Data. This is a 12-bit value.
    ///The value is formed by (REG56H_byte << 4) | (REG57H_byte & 0x0F).
    ///Formula for conversion: Voltage (mV) = raw_12bit_adc_value * 1.7.
    pub fn acin_voltage_adc(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::AcinVoltageAdc,
        ::device_driver::RO,
    > {
        let address = self.base_address + 86;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::AcinVoltageAdc,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::AcinVoltageAdc::new)
    }
    ///ACIN Current ADC Data. This is a 12-bit value.
    ///The value is formed by (REG58H_byte << 4) | (REG59H_byte & 0x0F).
    ///Formula for conversion: Current (mA) = raw_12bit_adc_value * 0.625.
    pub fn acin_current_adc(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::AcinCurrentAdc,
        ::device_driver::RO,
    > {
        let address = self.base_address + 88;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::AcinCurrentAdc,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::AcinCurrentAdc::new)
    }
    ///VBUS Voltage ADC Data. This is a 12-bit value.
    ///The value is formed by (REG5AH_byte << 4) | (REG5BH_byte & 0x0F).
    ///Formula for conversion: Voltage (mV) = raw_12bit_adc_value * 1.7.
    pub fn vbus_voltage_adc(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::VbusVoltageAdc,
        ::device_driver::RO,
    > {
        let address = self.base_address + 90;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::VbusVoltageAdc,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::VbusVoltageAdc::new)
    }
    ///VBUS Current ADC Data. This is a 12-bit value.
    ///The value is formed by (REG5CH_byte << 4) | (REG5DH_byte & 0x0F).
    ///Formula for conversion: Current (mA) = raw_12bit_adc_value * 0.375.
    pub fn vbus_current_adc(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::VbusCurrentAdc,
        ::device_driver::RO,
    > {
        let address = self.base_address + 92;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::VbusCurrentAdc,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::VbusCurrentAdc::new)
    }
    ///AXP192 Internal Temperature ADC Data. This is a 12-bit value.
    ///The value is formed by (REG5EH_byte << 4) | (REG5FH_byte & 0x0F).
    ///Formula for conversion: Temperature (°C) = (raw_12bit_adc_value * 0.1) - 144.7.
    pub fn internal_temperature_adc(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::InternalTemperatureAdc,
        ::device_driver::RO,
    > {
        let address = self.base_address + 94;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::InternalTemperatureAdc,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::InternalTemperatureAdc::new)
    }
    ///TS (Temperature Sense) Pin ADC Data. This is a 12-bit value representing the voltage at the TS pin.
    ///The value is formed by (REG62H_byte << 4) | (REG63H_byte & 0x0F).
    ///Formula for TS pin voltage: Voltage (mV) = raw_12bit_adc_value * 0.8.
    ///This voltage is typically from an NTC thermistor circuit for battery temperature monitoring.
    pub fn ts_pin_adc(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::TsPinAdc,
        ::device_driver::RO,
    > {
        let address = self.base_address + 98;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::TsPinAdc,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::TsPinAdc::new)
    }
    ///Reads the 12-bit ADC value for a GPIO pin (indexed 0-3 for GPIO0-GPIO3).
    ///The value is formed by (MSB_byte_of_pair << 4) | (LSB_byte_of_pair & 0x0F).
    ///Formula for pin voltage: Voltage (mV) = raw_12bit_adc_value * 0.5.
    ///The measurable voltage input range for each GPIO ADC is set in REG85H.
    ///
    /// Valid index range: 0..4
    pub fn gpio_voltage_adc(
        &mut self,
        index: usize,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::GpioVoltageAdc,
        ::device_driver::RO,
    > {
        let address = {
            assert!(index < 4);
            self.base_address + 100 + index as u8 * 2
        };
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::GpioVoltageAdc,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::GpioVoltageAdc::new)
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
    ///Sets the rising edge voltage threshold for GPIO1 ADC input to trigger an interrupt.
    ///An IRQ is generated when the GPIO1 ADC voltage rises above this set threshold.
    ///Formula: Threshold Voltage (V) = raw_value * 0.008.
    ///Range: 0V (raw 0x00) to 2.04V (raw 0xFF).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpio1AdcIrqRisingThreshold {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Gpio1AdcIrqRisingThreshold {
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
    impl Gpio1AdcIrqRisingThreshold {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [255] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `threshold_setting_raw` field of the register.
        ///
        ///Raw 8-bit setting for the GPIO1 ADC IRQ rising edge threshold. 1 LSB = 8mV.
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
        ///Raw 8-bit setting for the GPIO1 ADC IRQ rising edge threshold. 1 LSB = 8mV.
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
    impl From<[u8; 1]> for Gpio1AdcIrqRisingThreshold {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Gpio1AdcIrqRisingThreshold> for [u8; 1] {
        fn from(val: Gpio1AdcIrqRisingThreshold) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Gpio1AdcIrqRisingThreshold {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Gpio1AdcIrqRisingThreshold");
            {
                d.field("threshold_setting_raw", &self.threshold_setting_raw());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpio1AdcIrqRisingThreshold {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gpio1AdcIrqRisingThreshold { ");
            defmt::write!(
                f,
                "threshold_setting_raw: {=u8}, ",
                &self.threshold_setting_raw(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Gpio1AdcIrqRisingThreshold {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Gpio1AdcIrqRisingThreshold {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Gpio1AdcIrqRisingThreshold {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Gpio1AdcIrqRisingThreshold {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Gpio1AdcIrqRisingThreshold {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Gpio1AdcIrqRisingThreshold {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Gpio1AdcIrqRisingThreshold {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Sets the falling edge voltage threshold for GPIO1 ADC input to trigger an interrupt.
    ///An IRQ is generated when the GPIO1 ADC voltage falls below this set threshold.
    ///Formula: Threshold Voltage (V) = raw_value * 0.008.
    ///Range: 0V (raw 0x00) to 2.04V (raw 0xFF).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpio1AdcIrqFallingThreshold {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Gpio1AdcIrqFallingThreshold {
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
    impl Gpio1AdcIrqFallingThreshold {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `threshold_setting_raw` field of the register.
        ///
        ///Raw 8-bit setting for the GPIO1 ADC IRQ falling edge threshold. 1 LSB = 8mV.
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
        ///Raw 8-bit setting for the GPIO1 ADC IRQ falling edge threshold. 1 LSB = 8mV.
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
    impl From<[u8; 1]> for Gpio1AdcIrqFallingThreshold {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Gpio1AdcIrqFallingThreshold> for [u8; 1] {
        fn from(val: Gpio1AdcIrqFallingThreshold) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Gpio1AdcIrqFallingThreshold {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Gpio1AdcIrqFallingThreshold");
            {
                d.field("threshold_setting_raw", &self.threshold_setting_raw());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpio1AdcIrqFallingThreshold {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gpio1AdcIrqFallingThreshold { ");
            defmt::write!(
                f,
                "threshold_setting_raw: {=u8}, ",
                &self.threshold_setting_raw(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Gpio1AdcIrqFallingThreshold {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Gpio1AdcIrqFallingThreshold {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Gpio1AdcIrqFallingThreshold {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Gpio1AdcIrqFallingThreshold {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Gpio1AdcIrqFallingThreshold {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Gpio1AdcIrqFallingThreshold {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Gpio1AdcIrqFallingThreshold {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Controls an internal timer, sets its duration, and indicates timeout status.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TimerControl {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for TimerControl {
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
    impl TimerControl {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `timeout_flag` field of the register.
        ///
        ///Timer timeout status flag (true: timed out, false: not timed out or cleared). Write 1 to clear this flag (while preserving other bits).
        pub fn timeout_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `duration_minutes` field of the register.
        ///
        ///Timer duration in minutes (0-127). Writing 0 disables the timer.
        pub fn duration_minutes(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 7)
            };
            raw
        }
        ///Write the `timeout_flag` field of the register.
        ///
        ///Timer timeout status flag (true: timed out, false: not timed out or cleared). Write 1 to clear this flag (while preserving other bits).
        pub fn set_timeout_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `duration_minutes` field of the register.
        ///
        ///Timer duration in minutes (0-127). Writing 0 disables the timer.
        pub fn set_duration_minutes(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 7, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for TimerControl {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<TimerControl> for [u8; 1] {
        fn from(val: TimerControl) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for TimerControl {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("TimerControl");
            {
                d.field("timeout_flag", &self.timeout_flag());
            }
            {
                d.field("duration_minutes", &self.duration_minutes());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TimerControl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TimerControl { ");
            defmt::write!(f, "timeout_flag: {=bool}, ", &self.timeout_flag());
            defmt::write!(f, "duration_minutes: {=u8}, ", &self.duration_minutes());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for TimerControl {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for TimerControl {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for TimerControl {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for TimerControl {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for TimerControl {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for TimerControl {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for TimerControl {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Controls VBUS pin monitoring for Session Request Protocol (SRP) related functions, including valid voltage threshold and SRP feature enables.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VbusSrpControl {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for VbusSrpControl {
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
    impl VbusSrpControl {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `vbus_valid_threshold` field of the register.
        ///
        ///Threshold voltage for considering VBUS as valid.
        pub fn vbus_valid_threshold(&self) -> super::VbusValidThresholdValue {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 6)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `vbus_valid_detection_enable` field of the register.
        ///
        ///VBUS Valid detection function (true: enabled, false: disabled).
        pub fn vbus_valid_detection_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `vbus_session_detection_enable` field of the register.
        ///
        ///VBUS Session detection function (true: enabled, false: disabled).
        pub fn vbus_session_detection_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `vbus_discharge_enable` field of the register.
        ///
        ///Enable VBUS discharge path (true: enabled, false: disabled).
        pub fn vbus_discharge_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `vbus_charge_srp_enable` field of the register.
        ///
        ///Enable VBUS charge path for SRP (true: enabled, false: disabled).
        pub fn vbus_charge_srp_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `vbus_valid_threshold` field of the register.
        ///
        ///Threshold voltage for considering VBUS as valid.
        pub fn set_vbus_valid_threshold(
            &mut self,
            value: super::VbusValidThresholdValue,
        ) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 6, &mut self.bits)
            };
        }
        ///Write the `vbus_valid_detection_enable` field of the register.
        ///
        ///VBUS Valid detection function (true: enabled, false: disabled).
        pub fn set_vbus_valid_detection_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `vbus_session_detection_enable` field of the register.
        ///
        ///VBUS Session detection function (true: enabled, false: disabled).
        pub fn set_vbus_session_detection_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `vbus_discharge_enable` field of the register.
        ///
        ///Enable VBUS discharge path (true: enabled, false: disabled).
        pub fn set_vbus_discharge_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `vbus_charge_srp_enable` field of the register.
        ///
        ///Enable VBUS charge path for SRP (true: enabled, false: disabled).
        pub fn set_vbus_charge_srp_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for VbusSrpControl {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<VbusSrpControl> for [u8; 1] {
        fn from(val: VbusSrpControl) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for VbusSrpControl {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("VbusSrpControl");
            {
                d.field("vbus_valid_threshold", &self.vbus_valid_threshold());
            }
            {
                d.field(
                    "vbus_valid_detection_enable",
                    &self.vbus_valid_detection_enable(),
                );
            }
            {
                d.field(
                    "vbus_session_detection_enable",
                    &self.vbus_session_detection_enable(),
                );
            }
            {
                d.field("vbus_discharge_enable", &self.vbus_discharge_enable());
            }
            {
                d.field("vbus_charge_srp_enable", &self.vbus_charge_srp_enable());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VbusSrpControl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "VbusSrpControl { ");
            defmt::write!(f, "vbus_valid_threshold: {}, ", &self.vbus_valid_threshold());
            defmt::write!(
                f,
                "vbus_valid_detection_enable: {=bool}, ",
                &self.vbus_valid_detection_enable(),
            );
            defmt::write!(
                f,
                "vbus_session_detection_enable: {=bool}, ",
                &self.vbus_session_detection_enable(),
            );
            defmt::write!(
                f,
                "vbus_discharge_enable: {=bool}, ",
                &self.vbus_discharge_enable(),
            );
            defmt::write!(
                f,
                "vbus_charge_srp_enable: {=bool}, ",
                &self.vbus_charge_srp_enable(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for VbusSrpControl {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for VbusSrpControl {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for VbusSrpControl {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for VbusSrpControl {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for VbusSrpControl {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for VbusSrpControl {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for VbusSrpControl {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Controls the AXP192 internal over-temperature shutdown function. Other bits are reserved.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OverTempShutdownControl {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for OverTempShutdownControl {
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
    impl OverTempShutdownControl {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [1] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `internal_over_temp_shutdown_enable` field of the register.
        ///
        ///AXP192 internal over-temperature shutdown function (true: enabled, device shuts down on OT; false: disabled).
        pub fn internal_over_temp_shutdown_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Write the `internal_over_temp_shutdown_enable` field of the register.
        ///
        ///AXP192 internal over-temperature shutdown function (true: enabled, device shuts down on OT; false: disabled).
        pub fn set_internal_over_temp_shutdown_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for OverTempShutdownControl {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<OverTempShutdownControl> for [u8; 1] {
        fn from(val: OverTempShutdownControl) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for OverTempShutdownControl {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("OverTempShutdownControl");
            {
                d.field(
                    "internal_over_temp_shutdown_enable",
                    &self.internal_over_temp_shutdown_enable(),
                );
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OverTempShutdownControl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OverTempShutdownControl { ");
            defmt::write!(
                f,
                "internal_over_temp_shutdown_enable: {=bool}, ",
                &self.internal_over_temp_shutdown_enable(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for OverTempShutdownControl {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for OverTempShutdownControl {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for OverTempShutdownControl {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for OverTempShutdownControl {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for OverTempShutdownControl {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for OverTempShutdownControl {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for OverTempShutdownControl {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Configures the function of the GPIO0 pin.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpio0Control {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Gpio0Control {
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
    impl Gpio0Control {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [7] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `function_select` field of the register.
        ///
        ///Selects the operating mode for the GPIO0 pin.
        pub fn function_select(&self) -> super::Gpio0FunctionSelect {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 3)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `function_select` field of the register.
        ///
        ///Selects the operating mode for the GPIO0 pin.
        pub fn set_function_select(&mut self, value: super::Gpio0FunctionSelect) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 3, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Gpio0Control {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Gpio0Control> for [u8; 1] {
        fn from(val: Gpio0Control) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Gpio0Control {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Gpio0Control");
            {
                d.field("function_select", &self.function_select());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpio0Control {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gpio0Control { ");
            defmt::write!(f, "function_select: {}, ", &self.function_select());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Gpio0Control {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Gpio0Control {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Gpio0Control {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Gpio0Control {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Gpio0Control {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Gpio0Control {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Gpio0Control {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Sets the output voltage for GPIO0 when it is configured in Low Noise LDO (LDOIO0) mode (via REG90H).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpio0LdoVoltageSetting {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Gpio0LdoVoltageSetting {
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
    impl Gpio0LdoVoltageSetting {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [160] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `voltage_setting_raw` field of the register.
        ///
        ///Raw 4-bit setting for LDOIO0 output voltage (1.8V to 3.3V, 100mV/step). Active when GPIO0 is in LDO mode.
        pub fn voltage_setting_raw(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 8)
            };
            raw
        }
        ///Write the `voltage_setting_raw` field of the register.
        ///
        ///Raw 4-bit setting for LDOIO0 output voltage (1.8V to 3.3V, 100mV/step). Active when GPIO0 is in LDO mode.
        pub fn set_voltage_setting_raw(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 8, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Gpio0LdoVoltageSetting {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Gpio0LdoVoltageSetting> for [u8; 1] {
        fn from(val: Gpio0LdoVoltageSetting) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Gpio0LdoVoltageSetting {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Gpio0LdoVoltageSetting");
            {
                d.field("voltage_setting_raw", &self.voltage_setting_raw());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpio0LdoVoltageSetting {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gpio0LdoVoltageSetting { ");
            defmt::write!(
                f,
                "voltage_setting_raw: {=u8}, ",
                &self.voltage_setting_raw(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Gpio0LdoVoltageSetting {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Gpio0LdoVoltageSetting {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Gpio0LdoVoltageSetting {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Gpio0LdoVoltageSetting {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Gpio0LdoVoltageSetting {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Gpio0LdoVoltageSetting {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Gpio0LdoVoltageSetting {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Configures the function of the GPIO1 pin.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpio1Control {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Gpio1Control {
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
    impl Gpio1Control {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [7] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `function_select` field of the register.
        ///
        ///Selects the operating mode for the GPIO1 pin.
        pub fn function_select(&self) -> super::Gpio1FunctionSelect {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 3)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `function_select` field of the register.
        ///
        ///Selects the operating mode for the GPIO1 pin.
        pub fn set_function_select(&mut self, value: super::Gpio1FunctionSelect) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 3, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Gpio1Control {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Gpio1Control> for [u8; 1] {
        fn from(val: Gpio1Control) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Gpio1Control {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Gpio1Control");
            {
                d.field("function_select", &self.function_select());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpio1Control {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gpio1Control { ");
            defmt::write!(f, "function_select: {}, ", &self.function_select());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Gpio1Control {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Gpio1Control {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Gpio1Control {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Gpio1Control {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Gpio1Control {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Gpio1Control {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Gpio1Control {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Configures the function of the GPIO2 pin.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpio2Control {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Gpio2Control {
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
    impl Gpio2Control {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [7] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `function_select` field of the register.
        ///
        ///Selects the operating mode for the GPIO2 pin.
        pub fn function_select(&self) -> super::Gpio2FunctionSelect {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 3)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `function_select` field of the register.
        ///
        ///Selects the operating mode for the GPIO2 pin.
        pub fn set_function_select(&mut self, value: super::Gpio2FunctionSelect) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 3, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Gpio2Control {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Gpio2Control> for [u8; 1] {
        fn from(val: Gpio2Control) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Gpio2Control {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Gpio2Control");
            {
                d.field("function_select", &self.function_select());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpio2Control {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gpio2Control { ");
            defmt::write!(f, "function_select: {}, ", &self.function_select());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Gpio2Control {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Gpio2Control {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Gpio2Control {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Gpio2Control {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Gpio2Control {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Gpio2Control {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Gpio2Control {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Monitors input status for GPIO0-GPIO2 and controls their output levels
    ///when configured as NMOS open-drain or output low.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpio0To2SignalStatusAndControl {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Gpio0To2SignalStatusAndControl {
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
    impl Gpio0To2SignalStatusAndControl {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `gpio_2_input_status` field of the register.
        ///
        ///Current input level of GPIO2 (true: high, false: low).
        pub fn gpio_2_input_status(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `gpio_1_input_status` field of the register.
        ///
        ///Current input level of GPIO1 (true: high, false: low).
        pub fn gpio_1_input_status(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `gpio_0_input_status` field of the register.
        ///
        ///Current input level of GPIO0 (true: high, false: low).
        pub fn gpio_0_input_status(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `gpio_2_output_set_floating` field of the register.
        ///
        ///GPIO2 output level control (true: set output floating/NMOS off, false: set output low/NMOS on). Effective when GPIO2 is in an output mode.
        pub fn gpio_2_output_set_floating(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `gpio_1_output_set_floating` field of the register.
        ///
        ///GPIO1 output level control (true: set output floating/NMOS off, false: set output low/NMOS on). Effective when GPIO1 is in an output mode.
        pub fn gpio_1_output_set_floating(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `gpio_0_output_set_floating` field of the register.
        ///
        ///GPIO0 output level control (true: set output floating/NMOS off, false: set output low/NMOS on). Effective when GPIO0 is in an output mode.
        pub fn gpio_0_output_set_floating(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `gpio_2_output_set_floating` field of the register.
        ///
        ///GPIO2 output level control (true: set output floating/NMOS off, false: set output low/NMOS on). Effective when GPIO2 is in an output mode.
        pub fn set_gpio_2_output_set_floating(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `gpio_1_output_set_floating` field of the register.
        ///
        ///GPIO1 output level control (true: set output floating/NMOS off, false: set output low/NMOS on). Effective when GPIO1 is in an output mode.
        pub fn set_gpio_1_output_set_floating(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `gpio_0_output_set_floating` field of the register.
        ///
        ///GPIO0 output level control (true: set output floating/NMOS off, false: set output low/NMOS on). Effective when GPIO0 is in an output mode.
        pub fn set_gpio_0_output_set_floating(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Gpio0To2SignalStatusAndControl {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Gpio0To2SignalStatusAndControl> for [u8; 1] {
        fn from(val: Gpio0To2SignalStatusAndControl) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Gpio0To2SignalStatusAndControl {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Gpio0To2SignalStatusAndControl");
            {
                d.field("gpio_2_input_status", &self.gpio_2_input_status());
            }
            {
                d.field("gpio_1_input_status", &self.gpio_1_input_status());
            }
            {
                d.field("gpio_0_input_status", &self.gpio_0_input_status());
            }
            {
                d.field(
                    "gpio_2_output_set_floating",
                    &self.gpio_2_output_set_floating(),
                );
            }
            {
                d.field(
                    "gpio_1_output_set_floating",
                    &self.gpio_1_output_set_floating(),
                );
            }
            {
                d.field(
                    "gpio_0_output_set_floating",
                    &self.gpio_0_output_set_floating(),
                );
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpio0To2SignalStatusAndControl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gpio0To2SignalStatusAndControl { ");
            defmt::write!(
                f,
                "gpio_2_input_status: {=bool}, ",
                &self.gpio_2_input_status(),
            );
            defmt::write!(
                f,
                "gpio_1_input_status: {=bool}, ",
                &self.gpio_1_input_status(),
            );
            defmt::write!(
                f,
                "gpio_0_input_status: {=bool}, ",
                &self.gpio_0_input_status(),
            );
            defmt::write!(
                f,
                "gpio_2_output_set_floating: {=bool}, ",
                &self.gpio_2_output_set_floating(),
            );
            defmt::write!(
                f,
                "gpio_1_output_set_floating: {=bool}, ",
                &self.gpio_1_output_set_floating(),
            );
            defmt::write!(
                f,
                "gpio_0_output_set_floating: {=bool}, ",
                &self.gpio_0_output_set_floating(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Gpio0To2SignalStatusAndControl {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Gpio0To2SignalStatusAndControl {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Gpio0To2SignalStatusAndControl {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Gpio0To2SignalStatusAndControl {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Gpio0To2SignalStatusAndControl {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Gpio0To2SignalStatusAndControl {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Gpio0To2SignalStatusAndControl {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Configures the function for GPIO3 and GPIO4 pins, and enables their GPIO mode.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpio3And4FunctionControl {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Gpio3And4FunctionControl {
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
    impl Gpio3And4FunctionControl {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `gpio_3_and_4_mode_enable` field of the register.
        ///
        ///Enable GPIO mode for GPIO3 and GPIO4 as set by their respective function_select fields (true: GPIO mode, false: alternative function/disabled).
        pub fn gpio_3_and_4_mode_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `gpio_4_function_select` field of the register.
        ///
        ///Selects function for GPIO4 when gpio3_and_4_mode_enable is true.
        pub fn gpio_4_function_select(
            &self,
        ) -> Result<
            super::Gpio4FunctionSetting,
            <super::Gpio4FunctionSetting as TryFrom<u8>>::Error,
        > {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 4)
            };
            raw.try_into()
        }
        ///Read the `gpio_3_function_select` field of the register.
        ///
        ///Selects function for GPIO3 when gpio3_and_4_mode_enable is true.
        pub fn gpio_3_function_select(&self) -> super::Gpio3FunctionSetting {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 2)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `gpio_3_and_4_mode_enable` field of the register.
        ///
        ///Enable GPIO mode for GPIO3 and GPIO4 as set by their respective function_select fields (true: GPIO mode, false: alternative function/disabled).
        pub fn set_gpio_3_and_4_mode_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `gpio_4_function_select` field of the register.
        ///
        ///Selects function for GPIO4 when gpio3_and_4_mode_enable is true.
        pub fn set_gpio_4_function_select(
            &mut self,
            value: super::Gpio4FunctionSetting,
        ) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 4, &mut self.bits)
            };
        }
        ///Write the `gpio_3_function_select` field of the register.
        ///
        ///Selects function for GPIO3 when gpio3_and_4_mode_enable is true.
        pub fn set_gpio_3_function_select(
            &mut self,
            value: super::Gpio3FunctionSetting,
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
    impl From<[u8; 1]> for Gpio3And4FunctionControl {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Gpio3And4FunctionControl> for [u8; 1] {
        fn from(val: Gpio3And4FunctionControl) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Gpio3And4FunctionControl {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Gpio3And4FunctionControl");
            {
                d.field("gpio_3_and_4_mode_enable", &self.gpio_3_and_4_mode_enable());
            }
            {
                d.field("gpio_4_function_select", &self.gpio_4_function_select());
            }
            {
                d.field("gpio_3_function_select", &self.gpio_3_function_select());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpio3And4FunctionControl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gpio3And4FunctionControl { ");
            defmt::write!(
                f,
                "gpio_3_and_4_mode_enable: {=bool}, ",
                &self.gpio_3_and_4_mode_enable(),
            );
            defmt::write!(
                f,
                "gpio_4_function_select: {}, ",
                &self.gpio_4_function_select(),
            );
            defmt::write!(
                f,
                "gpio_3_function_select: {}, ",
                &self.gpio_3_function_select(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Gpio3And4FunctionControl {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Gpio3And4FunctionControl {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Gpio3And4FunctionControl {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Gpio3And4FunctionControl {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Gpio3And4FunctionControl {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Gpio3And4FunctionControl {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Gpio3And4FunctionControl {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Monitors input status for GPIO3-GPIO4 and controls their output levels
    ///when configured as NMOS open-drain or output low.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpio3And4SignalStatusAndControl {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Gpio3And4SignalStatusAndControl {
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
    impl Gpio3And4SignalStatusAndControl {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `gpio_4_input_status` field of the register.
        ///
        ///Current input level of GPIO4 (true: high, false: low).
        pub fn gpio_4_input_status(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `gpio_3_input_status` field of the register.
        ///
        ///Current input level of GPIO3 (true: high, false: low).
        pub fn gpio_3_input_status(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `gpio_4_output_set_floating` field of the register.
        ///
        ///GPIO4 output level control (true: set output floating/NMOS off, false: set output low/NMOS on).
        pub fn gpio_4_output_set_floating(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `gpio_3_output_set_floating` field of the register.
        ///
        ///GPIO3 output level control (true: set output floating/NMOS off, false: set output low/NMOS on).
        pub fn gpio_3_output_set_floating(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `gpio_4_output_set_floating` field of the register.
        ///
        ///GPIO4 output level control (true: set output floating/NMOS off, false: set output low/NMOS on).
        pub fn set_gpio_4_output_set_floating(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `gpio_3_output_set_floating` field of the register.
        ///
        ///GPIO3 output level control (true: set output floating/NMOS off, false: set output low/NMOS on).
        pub fn set_gpio_3_output_set_floating(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Gpio3And4SignalStatusAndControl {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Gpio3And4SignalStatusAndControl> for [u8; 1] {
        fn from(val: Gpio3And4SignalStatusAndControl) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Gpio3And4SignalStatusAndControl {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Gpio3And4SignalStatusAndControl");
            {
                d.field("gpio_4_input_status", &self.gpio_4_input_status());
            }
            {
                d.field("gpio_3_input_status", &self.gpio_3_input_status());
            }
            {
                d.field(
                    "gpio_4_output_set_floating",
                    &self.gpio_4_output_set_floating(),
                );
            }
            {
                d.field(
                    "gpio_3_output_set_floating",
                    &self.gpio_3_output_set_floating(),
                );
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpio3And4SignalStatusAndControl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gpio3And4SignalStatusAndControl { ");
            defmt::write!(
                f,
                "gpio_4_input_status: {=bool}, ",
                &self.gpio_4_input_status(),
            );
            defmt::write!(
                f,
                "gpio_3_input_status: {=bool}, ",
                &self.gpio_3_input_status(),
            );
            defmt::write!(
                f,
                "gpio_4_output_set_floating: {=bool}, ",
                &self.gpio_4_output_set_floating(),
            );
            defmt::write!(
                f,
                "gpio_3_output_set_floating: {=bool}, ",
                &self.gpio_3_output_set_floating(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Gpio3And4SignalStatusAndControl {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Gpio3And4SignalStatusAndControl {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Gpio3And4SignalStatusAndControl {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Gpio3And4SignalStatusAndControl {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Gpio3And4SignalStatusAndControl {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Gpio3And4SignalStatusAndControl {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Gpio3And4SignalStatusAndControl {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Controls internal pull-down resistors for GPIO0, GPIO1, and GPIO2 when they are configured as inputs.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpio0To2PulldownControl {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Gpio0To2PulldownControl {
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
    impl Gpio0To2PulldownControl {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `gpio_2_pulldown_enable` field of the register.
        ///
        ///GPIO2 internal pull-down resistor (true: enabled, false: disabled). Effective when GPIO2 is an input.
        pub fn gpio_2_pulldown_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `gpio_1_pulldown_enable` field of the register.
        ///
        ///GPIO1 internal pull-down resistor (true: enabled, false: disabled). Effective when GPIO1 is an input.
        pub fn gpio_1_pulldown_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `gpio_0_pulldown_enable` field of the register.
        ///
        ///GPIO0 internal pull-down resistor (true: enabled, false: disabled). Effective when GPIO0 is an input.
        pub fn gpio_0_pulldown_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `gpio_2_pulldown_enable` field of the register.
        ///
        ///GPIO2 internal pull-down resistor (true: enabled, false: disabled). Effective when GPIO2 is an input.
        pub fn set_gpio_2_pulldown_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `gpio_1_pulldown_enable` field of the register.
        ///
        ///GPIO1 internal pull-down resistor (true: enabled, false: disabled). Effective when GPIO1 is an input.
        pub fn set_gpio_1_pulldown_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `gpio_0_pulldown_enable` field of the register.
        ///
        ///GPIO0 internal pull-down resistor (true: enabled, false: disabled). Effective when GPIO0 is an input.
        pub fn set_gpio_0_pulldown_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Gpio0To2PulldownControl {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Gpio0To2PulldownControl> for [u8; 1] {
        fn from(val: Gpio0To2PulldownControl) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Gpio0To2PulldownControl {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Gpio0To2PulldownControl");
            {
                d.field("gpio_2_pulldown_enable", &self.gpio_2_pulldown_enable());
            }
            {
                d.field("gpio_1_pulldown_enable", &self.gpio_1_pulldown_enable());
            }
            {
                d.field("gpio_0_pulldown_enable", &self.gpio_0_pulldown_enable());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpio0To2PulldownControl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gpio0To2PulldownControl { ");
            defmt::write!(
                f,
                "gpio_2_pulldown_enable: {=bool}, ",
                &self.gpio_2_pulldown_enable(),
            );
            defmt::write!(
                f,
                "gpio_1_pulldown_enable: {=bool}, ",
                &self.gpio_1_pulldown_enable(),
            );
            defmt::write!(
                f,
                "gpio_0_pulldown_enable: {=bool}, ",
                &self.gpio_0_pulldown_enable(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Gpio0To2PulldownControl {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Gpio0To2PulldownControl {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Gpio0To2PulldownControl {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Gpio0To2PulldownControl {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Gpio0To2PulldownControl {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Gpio0To2PulldownControl {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Gpio0To2PulldownControl {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Sets the 'X' parameter for PWM1 output frequency calculation.
    ///Formula: F_pwm1 = 2.25MHz / (X_value + 1) / Y1_value (where Y1 is from REG99H).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm1FrequencySetting {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Pwm1FrequencySetting {
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
    impl Pwm1FrequencySetting {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `parameter_x` field of the register.
        ///
        ///Parameter X for PWM1 frequency calculation (0-255).
        pub fn parameter_x(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 8)
            };
            raw
        }
        ///Write the `parameter_x` field of the register.
        ///
        ///Parameter X for PWM1 frequency calculation (0-255).
        pub fn set_parameter_x(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 8, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Pwm1FrequencySetting {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Pwm1FrequencySetting> for [u8; 1] {
        fn from(val: Pwm1FrequencySetting) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Pwm1FrequencySetting {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Pwm1FrequencySetting");
            {
                d.field("parameter_x", &self.parameter_x());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pwm1FrequencySetting {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pwm1FrequencySetting { ");
            defmt::write!(f, "parameter_x: {=u8}, ", &self.parameter_x());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Pwm1FrequencySetting {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Pwm1FrequencySetting {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Pwm1FrequencySetting {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Pwm1FrequencySetting {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Pwm1FrequencySetting {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Pwm1FrequencySetting {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Pwm1FrequencySetting {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Sets the 'Y1' parameter for PWM1 duty cycle and frequency calculations.
    ///Y1 is the denominator for duty cycle (Duty = Y2/Y1) and also affects frequency.
    ///Y1 should not be set to 0.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm1DutyCycleSettingY1 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Pwm1DutyCycleSettingY1 {
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
    impl Pwm1DutyCycleSettingY1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [22] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `parameter_y_1` field of the register.
        ///
        ///Parameter Y1 for PWM1 calculations (1-255). Do not set to 0.
        pub fn parameter_y_1(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 8)
            };
            raw
        }
        ///Write the `parameter_y_1` field of the register.
        ///
        ///Parameter Y1 for PWM1 calculations (1-255). Do not set to 0.
        pub fn set_parameter_y_1(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 8, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Pwm1DutyCycleSettingY1 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Pwm1DutyCycleSettingY1> for [u8; 1] {
        fn from(val: Pwm1DutyCycleSettingY1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Pwm1DutyCycleSettingY1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Pwm1DutyCycleSettingY1");
            {
                d.field("parameter_y_1", &self.parameter_y_1());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pwm1DutyCycleSettingY1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pwm1DutyCycleSettingY1 { ");
            defmt::write!(f, "parameter_y_1: {=u8}, ", &self.parameter_y_1());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Pwm1DutyCycleSettingY1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Pwm1DutyCycleSettingY1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Pwm1DutyCycleSettingY1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Pwm1DutyCycleSettingY1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Pwm1DutyCycleSettingY1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Pwm1DutyCycleSettingY1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Pwm1DutyCycleSettingY1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Sets the 'Y2' parameter (numerator) for PWM1 duty cycle calculation (Duty = Y2/Y1).
    ///Only upper 5 bits (bits 7-3) are used for Y2.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm1DutyCycleSettingY2 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Pwm1DutyCycleSettingY2 {
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
    impl Pwm1DutyCycleSettingY2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [11] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `parameter_y_2` field of the register.
        ///
        ///Parameter Y2 for PWM1 duty cycle calculation (0-31).
        pub fn parameter_y_2(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 8)
            };
            raw
        }
        ///Write the `parameter_y_2` field of the register.
        ///
        ///Parameter Y2 for PWM1 duty cycle calculation (0-31).
        pub fn set_parameter_y_2(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 8, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Pwm1DutyCycleSettingY2 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Pwm1DutyCycleSettingY2> for [u8; 1] {
        fn from(val: Pwm1DutyCycleSettingY2) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Pwm1DutyCycleSettingY2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Pwm1DutyCycleSettingY2");
            {
                d.field("parameter_y_2", &self.parameter_y_2());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pwm1DutyCycleSettingY2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pwm1DutyCycleSettingY2 { ");
            defmt::write!(f, "parameter_y_2: {=u8}, ", &self.parameter_y_2());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for Pwm1DutyCycleSettingY2 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Pwm1DutyCycleSettingY2 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Pwm1DutyCycleSettingY2 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Pwm1DutyCycleSettingY2 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Pwm1DutyCycleSettingY2 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Pwm1DutyCycleSettingY2 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Pwm1DutyCycleSettingY2 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Configures the N_RSTO/GPIO5 pin function. It can operate as N_RSTO (LDO1 status monitor)
    ///or as a general-purpose I/O pin (GPIO5) with configurable direction and output state.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NrstoGpio5Control {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for NrstoGpio5Control {
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
    impl NrstoGpio5Control {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [32] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `pin_is_gpio_5` field of the register.
        ///
        ///Selects the primary function of the N_RSTO/GPIO5 pin.
        pub fn pin_is_gpio_5(&self) -> super::NrstoPinFunction {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `gpio_5_is_input` field of the register.
        ///
        ///Sets GPIO5 direction when pin_is_gpio5 is Gpio5 (true: input, false: NMOS open-drain output).
        pub fn gpio_5_is_input(&self) -> super::Gpio5Direction {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `gpio_5_output_set_floating` field of the register.
        ///
        ///GPIO5 output level when in NMOS output mode (true: set floating/NMOS off, false: set output low/NMOS on).
        pub fn gpio_5_output_set_floating(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `gpio_5_input_status` field of the register.
        ///
        ///Current input level of GPIO5 when configured as input (true: high, false: low).
        pub fn gpio_5_input_status(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Write the `pin_is_gpio_5` field of the register.
        ///
        ///Selects the primary function of the N_RSTO/GPIO5 pin.
        pub fn set_pin_is_gpio_5(&mut self, value: super::NrstoPinFunction) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `gpio_5_is_input` field of the register.
        ///
        ///Sets GPIO5 direction when pin_is_gpio5 is Gpio5 (true: input, false: NMOS open-drain output).
        pub fn set_gpio_5_is_input(&mut self, value: super::Gpio5Direction) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `gpio_5_output_set_floating` field of the register.
        ///
        ///GPIO5 output level when in NMOS output mode (true: set floating/NMOS off, false: set output low/NMOS on).
        pub fn set_gpio_5_output_set_floating(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for NrstoGpio5Control {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<NrstoGpio5Control> for [u8; 1] {
        fn from(val: NrstoGpio5Control) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for NrstoGpio5Control {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("NrstoGpio5Control");
            {
                d.field("pin_is_gpio_5", &self.pin_is_gpio_5());
            }
            {
                d.field("gpio_5_is_input", &self.gpio_5_is_input());
            }
            {
                d.field(
                    "gpio_5_output_set_floating",
                    &self.gpio_5_output_set_floating(),
                );
            }
            {
                d.field("gpio_5_input_status", &self.gpio_5_input_status());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NrstoGpio5Control {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "NrstoGpio5Control { ");
            defmt::write!(f, "pin_is_gpio_5: {}, ", &self.pin_is_gpio_5());
            defmt::write!(f, "gpio_5_is_input: {}, ", &self.gpio_5_is_input());
            defmt::write!(
                f,
                "gpio_5_output_set_floating: {=bool}, ",
                &self.gpio_5_output_set_floating(),
            );
            defmt::write!(
                f,
                "gpio_5_input_status: {=bool}, ",
                &self.gpio_5_input_status(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for NrstoGpio5Control {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for NrstoGpio5Control {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for NrstoGpio5Control {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for NrstoGpio5Control {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for NrstoGpio5Control {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for NrstoGpio5Control {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for NrstoGpio5Control {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Interrupt Enable Control Register 1 (ACIN and VBUS related IRQs).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqEnableControl1 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for IrqEnableControl1 {
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
    impl IrqEnableControl1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [216] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `acin_over_voltage_irq_enable` field of the register.
        ///
        ///ACIN over-voltage interrupt.
        pub fn acin_over_voltage_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `acin_insertion_irq_enable` field of the register.
        ///
        ///ACIN insertion detection interrupt.
        pub fn acin_insertion_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `acin_removal_irq_enable` field of the register.
        ///
        ///ACIN removal detection interrupt.
        pub fn acin_removal_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `vbus_over_voltage_irq_enable` field of the register.
        ///
        ///VBUS over-voltage interrupt.
        pub fn vbus_over_voltage_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `vbus_insertion_irq_enable` field of the register.
        ///
        ///VBUS insertion detection interrupt.
        pub fn vbus_insertion_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `vbus_removal_irq_enable` field of the register.
        ///
        ///VBUS removal detection interrupt.
        pub fn vbus_removal_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `vbus_less_than_vhold_irq_enable` field of the register.
        ///
        ///VBUS voltage less than VHOLD interrupt.
        pub fn vbus_less_than_vhold_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Write the `acin_over_voltage_irq_enable` field of the register.
        ///
        ///ACIN over-voltage interrupt.
        pub fn set_acin_over_voltage_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `acin_insertion_irq_enable` field of the register.
        ///
        ///ACIN insertion detection interrupt.
        pub fn set_acin_insertion_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `acin_removal_irq_enable` field of the register.
        ///
        ///ACIN removal detection interrupt.
        pub fn set_acin_removal_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `vbus_over_voltage_irq_enable` field of the register.
        ///
        ///VBUS over-voltage interrupt.
        pub fn set_vbus_over_voltage_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `vbus_insertion_irq_enable` field of the register.
        ///
        ///VBUS insertion detection interrupt.
        pub fn set_vbus_insertion_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `vbus_removal_irq_enable` field of the register.
        ///
        ///VBUS removal detection interrupt.
        pub fn set_vbus_removal_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `vbus_less_than_vhold_irq_enable` field of the register.
        ///
        ///VBUS voltage less than VHOLD interrupt.
        pub fn set_vbus_less_than_vhold_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for IrqEnableControl1 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<IrqEnableControl1> for [u8; 1] {
        fn from(val: IrqEnableControl1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for IrqEnableControl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("IrqEnableControl1");
            {
                d.field(
                    "acin_over_voltage_irq_enable",
                    &self.acin_over_voltage_irq_enable(),
                );
            }
            {
                d.field("acin_insertion_irq_enable", &self.acin_insertion_irq_enable());
            }
            {
                d.field("acin_removal_irq_enable", &self.acin_removal_irq_enable());
            }
            {
                d.field(
                    "vbus_over_voltage_irq_enable",
                    &self.vbus_over_voltage_irq_enable(),
                );
            }
            {
                d.field("vbus_insertion_irq_enable", &self.vbus_insertion_irq_enable());
            }
            {
                d.field("vbus_removal_irq_enable", &self.vbus_removal_irq_enable());
            }
            {
                d.field(
                    "vbus_less_than_vhold_irq_enable",
                    &self.vbus_less_than_vhold_irq_enable(),
                );
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqEnableControl1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IrqEnableControl1 { ");
            defmt::write!(
                f,
                "acin_over_voltage_irq_enable: {=bool}, ",
                &self.acin_over_voltage_irq_enable(),
            );
            defmt::write!(
                f,
                "acin_insertion_irq_enable: {=bool}, ",
                &self.acin_insertion_irq_enable(),
            );
            defmt::write!(
                f,
                "acin_removal_irq_enable: {=bool}, ",
                &self.acin_removal_irq_enable(),
            );
            defmt::write!(
                f,
                "vbus_over_voltage_irq_enable: {=bool}, ",
                &self.vbus_over_voltage_irq_enable(),
            );
            defmt::write!(
                f,
                "vbus_insertion_irq_enable: {=bool}, ",
                &self.vbus_insertion_irq_enable(),
            );
            defmt::write!(
                f,
                "vbus_removal_irq_enable: {=bool}, ",
                &self.vbus_removal_irq_enable(),
            );
            defmt::write!(
                f,
                "vbus_less_than_vhold_irq_enable: {=bool}, ",
                &self.vbus_less_than_vhold_irq_enable(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for IrqEnableControl1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for IrqEnableControl1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for IrqEnableControl1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for IrqEnableControl1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for IrqEnableControl1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for IrqEnableControl1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for IrqEnableControl1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Interrupt Enable Control Register 2 (Battery and Charge related IRQs).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqEnableControl2 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for IrqEnableControl2 {
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
    impl IrqEnableControl2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [255] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `battery_insertion_irq_enable` field of the register.
        ///
        ///Battery insertion interrupt.
        pub fn battery_insertion_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `battery_removal_irq_enable` field of the register.
        ///
        ///Battery removal interrupt.
        pub fn battery_removal_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `battery_activation_mode_entry_irq_enable` field of the register.
        ///
        ///Battery enters activation mode interrupt.
        pub fn battery_activation_mode_entry_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `battery_activation_mode_exit_irq_enable` field of the register.
        ///
        ///Battery exits activation mode interrupt.
        pub fn battery_activation_mode_exit_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `charging_irq_enable` field of the register.
        ///
        ///Charging started interrupt.
        pub fn charging_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `charge_done_irq_enable` field of the register.
        ///
        ///Charge completion interrupt.
        pub fn charge_done_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `battery_over_temp_irq_enable` field of the register.
        ///
        ///Battery over-temperature interrupt.
        pub fn battery_over_temp_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `battery_under_temp_irq_enable` field of the register.
        ///
        ///Battery under-temperature interrupt.
        pub fn battery_under_temp_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `battery_insertion_irq_enable` field of the register.
        ///
        ///Battery insertion interrupt.
        pub fn set_battery_insertion_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `battery_removal_irq_enable` field of the register.
        ///
        ///Battery removal interrupt.
        pub fn set_battery_removal_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `battery_activation_mode_entry_irq_enable` field of the register.
        ///
        ///Battery enters activation mode interrupt.
        pub fn set_battery_activation_mode_entry_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `battery_activation_mode_exit_irq_enable` field of the register.
        ///
        ///Battery exits activation mode interrupt.
        pub fn set_battery_activation_mode_exit_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `charging_irq_enable` field of the register.
        ///
        ///Charging started interrupt.
        pub fn set_charging_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `charge_done_irq_enable` field of the register.
        ///
        ///Charge completion interrupt.
        pub fn set_charge_done_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `battery_over_temp_irq_enable` field of the register.
        ///
        ///Battery over-temperature interrupt.
        pub fn set_battery_over_temp_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `battery_under_temp_irq_enable` field of the register.
        ///
        ///Battery under-temperature interrupt.
        pub fn set_battery_under_temp_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for IrqEnableControl2 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<IrqEnableControl2> for [u8; 1] {
        fn from(val: IrqEnableControl2) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for IrqEnableControl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("IrqEnableControl2");
            {
                d.field(
                    "battery_insertion_irq_enable",
                    &self.battery_insertion_irq_enable(),
                );
            }
            {
                d.field(
                    "battery_removal_irq_enable",
                    &self.battery_removal_irq_enable(),
                );
            }
            {
                d.field(
                    "battery_activation_mode_entry_irq_enable",
                    &self.battery_activation_mode_entry_irq_enable(),
                );
            }
            {
                d.field(
                    "battery_activation_mode_exit_irq_enable",
                    &self.battery_activation_mode_exit_irq_enable(),
                );
            }
            {
                d.field("charging_irq_enable", &self.charging_irq_enable());
            }
            {
                d.field("charge_done_irq_enable", &self.charge_done_irq_enable());
            }
            {
                d.field(
                    "battery_over_temp_irq_enable",
                    &self.battery_over_temp_irq_enable(),
                );
            }
            {
                d.field(
                    "battery_under_temp_irq_enable",
                    &self.battery_under_temp_irq_enable(),
                );
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqEnableControl2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IrqEnableControl2 { ");
            defmt::write!(
                f,
                "battery_insertion_irq_enable: {=bool}, ",
                &self.battery_insertion_irq_enable(),
            );
            defmt::write!(
                f,
                "battery_removal_irq_enable: {=bool}, ",
                &self.battery_removal_irq_enable(),
            );
            defmt::write!(
                f,
                "battery_activation_mode_entry_irq_enable: {=bool}, ",
                &self.battery_activation_mode_entry_irq_enable(),
            );
            defmt::write!(
                f,
                "battery_activation_mode_exit_irq_enable: {=bool}, ",
                &self.battery_activation_mode_exit_irq_enable(),
            );
            defmt::write!(
                f,
                "charging_irq_enable: {=bool}, ",
                &self.charging_irq_enable(),
            );
            defmt::write!(
                f,
                "charge_done_irq_enable: {=bool}, ",
                &self.charge_done_irq_enable(),
            );
            defmt::write!(
                f,
                "battery_over_temp_irq_enable: {=bool}, ",
                &self.battery_over_temp_irq_enable(),
            );
            defmt::write!(
                f,
                "battery_under_temp_irq_enable: {=bool}, ",
                &self.battery_under_temp_irq_enable(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for IrqEnableControl2 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for IrqEnableControl2 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for IrqEnableControl2 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for IrqEnableControl2 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for IrqEnableControl2 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for IrqEnableControl2 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for IrqEnableControl2 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Interrupt Enable Control Register 3 (IC Temp, Charge Current, DCDC VLow, PEK IRQs).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqEnableControl3 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for IrqEnableControl3 {
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
    impl IrqEnableControl3 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [59] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `internal_over_temp_irq_enable` field of the register.
        ///
        ///IC internal over-temperature interrupt.
        pub fn internal_over_temp_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `charge_current_insufficient_irq_enable` field of the register.
        ///
        ///Charging current insufficient interrupt.
        pub fn charge_current_insufficient_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `dcdc_1_voltage_low_irq_enable` field of the register.
        ///
        ///DC-DC1 output voltage low interrupt.
        pub fn dcdc_1_voltage_low_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `dcdc_2_voltage_low_irq_enable` field of the register.
        ///
        ///DC-DC2 output voltage low interrupt.
        pub fn dcdc_2_voltage_low_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `dcdc_3_voltage_low_irq_enable` field of the register.
        ///
        ///DC-DC3 output voltage low interrupt.
        pub fn dcdc_3_voltage_low_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `pek_short_press_irq_enable` field of the register.
        ///
        ///PEK (Power Key) short press interrupt.
        pub fn pek_short_press_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `pek_long_press_irq_enable` field of the register.
        ///
        ///PEK (Power Key) long press interrupt.
        pub fn pek_long_press_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `internal_over_temp_irq_enable` field of the register.
        ///
        ///IC internal over-temperature interrupt.
        pub fn set_internal_over_temp_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `charge_current_insufficient_irq_enable` field of the register.
        ///
        ///Charging current insufficient interrupt.
        pub fn set_charge_current_insufficient_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `dcdc_1_voltage_low_irq_enable` field of the register.
        ///
        ///DC-DC1 output voltage low interrupt.
        pub fn set_dcdc_1_voltage_low_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `dcdc_2_voltage_low_irq_enable` field of the register.
        ///
        ///DC-DC2 output voltage low interrupt.
        pub fn set_dcdc_2_voltage_low_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `dcdc_3_voltage_low_irq_enable` field of the register.
        ///
        ///DC-DC3 output voltage low interrupt.
        pub fn set_dcdc_3_voltage_low_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `pek_short_press_irq_enable` field of the register.
        ///
        ///PEK (Power Key) short press interrupt.
        pub fn set_pek_short_press_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `pek_long_press_irq_enable` field of the register.
        ///
        ///PEK (Power Key) long press interrupt.
        pub fn set_pek_long_press_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for IrqEnableControl3 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<IrqEnableControl3> for [u8; 1] {
        fn from(val: IrqEnableControl3) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for IrqEnableControl3 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("IrqEnableControl3");
            {
                d.field(
                    "internal_over_temp_irq_enable",
                    &self.internal_over_temp_irq_enable(),
                );
            }
            {
                d.field(
                    "charge_current_insufficient_irq_enable",
                    &self.charge_current_insufficient_irq_enable(),
                );
            }
            {
                d.field(
                    "dcdc_1_voltage_low_irq_enable",
                    &self.dcdc_1_voltage_low_irq_enable(),
                );
            }
            {
                d.field(
                    "dcdc_2_voltage_low_irq_enable",
                    &self.dcdc_2_voltage_low_irq_enable(),
                );
            }
            {
                d.field(
                    "dcdc_3_voltage_low_irq_enable",
                    &self.dcdc_3_voltage_low_irq_enable(),
                );
            }
            {
                d.field(
                    "pek_short_press_irq_enable",
                    &self.pek_short_press_irq_enable(),
                );
            }
            {
                d.field("pek_long_press_irq_enable", &self.pek_long_press_irq_enable());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqEnableControl3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IrqEnableControl3 { ");
            defmt::write!(
                f,
                "internal_over_temp_irq_enable: {=bool}, ",
                &self.internal_over_temp_irq_enable(),
            );
            defmt::write!(
                f,
                "charge_current_insufficient_irq_enable: {=bool}, ",
                &self.charge_current_insufficient_irq_enable(),
            );
            defmt::write!(
                f,
                "dcdc_1_voltage_low_irq_enable: {=bool}, ",
                &self.dcdc_1_voltage_low_irq_enable(),
            );
            defmt::write!(
                f,
                "dcdc_2_voltage_low_irq_enable: {=bool}, ",
                &self.dcdc_2_voltage_low_irq_enable(),
            );
            defmt::write!(
                f,
                "dcdc_3_voltage_low_irq_enable: {=bool}, ",
                &self.dcdc_3_voltage_low_irq_enable(),
            );
            defmt::write!(
                f,
                "pek_short_press_irq_enable: {=bool}, ",
                &self.pek_short_press_irq_enable(),
            );
            defmt::write!(
                f,
                "pek_long_press_irq_enable: {=bool}, ",
                &self.pek_long_press_irq_enable(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for IrqEnableControl3 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for IrqEnableControl3 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for IrqEnableControl3 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for IrqEnableControl3 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for IrqEnableControl3 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for IrqEnableControl3 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for IrqEnableControl3 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Interrupt Enable Control Register 4 (N_OE and VBUS session/status related IRQs, APS Low Voltage).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqEnableControl4 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for IrqEnableControl4 {
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
    impl IrqEnableControl4 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [193] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `n_oe_power_on_irq_enable` field of the register.
        ///
        ///N_OE Power-on event interrupt.
        pub fn n_oe_power_on_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `n_oe_power_off_irq_enable` field of the register.
        ///
        ///N_OE Power-off event interrupt.
        pub fn n_oe_power_off_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `vbus_valid_irq_enable` field of the register.
        ///
        ///VBUS valid event interrupt.
        pub fn vbus_valid_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `vbus_invalid_irq_enable` field of the register.
        ///
        ///VBUS invalid event interrupt.
        pub fn vbus_invalid_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `vbus_session_ab_irq_enable` field of the register.
        ///
        ///VBUS Session A/B valid event interrupt.
        pub fn vbus_session_ab_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `vbus_session_end_irq_enable` field of the register.
        ///
        ///VBUS Session End event interrupt.
        pub fn vbus_session_end_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `aps_low_voltage_level_2_irq_enable` field of the register.
        ///
        ///APS low voltage (Warning Level 2) interrupt.
        pub fn aps_low_voltage_level_2_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `n_oe_power_on_irq_enable` field of the register.
        ///
        ///N_OE Power-on event interrupt.
        pub fn set_n_oe_power_on_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `n_oe_power_off_irq_enable` field of the register.
        ///
        ///N_OE Power-off event interrupt.
        pub fn set_n_oe_power_off_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `vbus_valid_irq_enable` field of the register.
        ///
        ///VBUS valid event interrupt.
        pub fn set_vbus_valid_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `vbus_invalid_irq_enable` field of the register.
        ///
        ///VBUS invalid event interrupt.
        pub fn set_vbus_invalid_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `vbus_session_ab_irq_enable` field of the register.
        ///
        ///VBUS Session A/B valid event interrupt.
        pub fn set_vbus_session_ab_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `vbus_session_end_irq_enable` field of the register.
        ///
        ///VBUS Session End event interrupt.
        pub fn set_vbus_session_end_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `aps_low_voltage_level_2_irq_enable` field of the register.
        ///
        ///APS low voltage (Warning Level 2) interrupt.
        pub fn set_aps_low_voltage_level_2_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for IrqEnableControl4 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<IrqEnableControl4> for [u8; 1] {
        fn from(val: IrqEnableControl4) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for IrqEnableControl4 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("IrqEnableControl4");
            {
                d.field("n_oe_power_on_irq_enable", &self.n_oe_power_on_irq_enable());
            }
            {
                d.field("n_oe_power_off_irq_enable", &self.n_oe_power_off_irq_enable());
            }
            {
                d.field("vbus_valid_irq_enable", &self.vbus_valid_irq_enable());
            }
            {
                d.field("vbus_invalid_irq_enable", &self.vbus_invalid_irq_enable());
            }
            {
                d.field(
                    "vbus_session_ab_irq_enable",
                    &self.vbus_session_ab_irq_enable(),
                );
            }
            {
                d.field(
                    "vbus_session_end_irq_enable",
                    &self.vbus_session_end_irq_enable(),
                );
            }
            {
                d.field(
                    "aps_low_voltage_level_2_irq_enable",
                    &self.aps_low_voltage_level_2_irq_enable(),
                );
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqEnableControl4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IrqEnableControl4 { ");
            defmt::write!(
                f,
                "n_oe_power_on_irq_enable: {=bool}, ",
                &self.n_oe_power_on_irq_enable(),
            );
            defmt::write!(
                f,
                "n_oe_power_off_irq_enable: {=bool}, ",
                &self.n_oe_power_off_irq_enable(),
            );
            defmt::write!(
                f,
                "vbus_valid_irq_enable: {=bool}, ",
                &self.vbus_valid_irq_enable(),
            );
            defmt::write!(
                f,
                "vbus_invalid_irq_enable: {=bool}, ",
                &self.vbus_invalid_irq_enable(),
            );
            defmt::write!(
                f,
                "vbus_session_ab_irq_enable: {=bool}, ",
                &self.vbus_session_ab_irq_enable(),
            );
            defmt::write!(
                f,
                "vbus_session_end_irq_enable: {=bool}, ",
                &self.vbus_session_end_irq_enable(),
            );
            defmt::write!(
                f,
                "aps_low_voltage_level_2_irq_enable: {=bool}, ",
                &self.aps_low_voltage_level_2_irq_enable(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for IrqEnableControl4 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for IrqEnableControl4 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for IrqEnableControl4 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for IrqEnableControl4 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for IrqEnableControl4 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for IrqEnableControl4 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for IrqEnableControl4 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Interrupt Enable Control Register 5 (Timer and GPIO Input Edge Trigger IRQs).
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqEnableControl5 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for IrqEnableControl5 {
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
    impl IrqEnableControl5 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `timer_timeout_irq_enable` field of the register.
        ///
        ///Timer timeout interrupt.
        pub fn timer_timeout_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `gpio_2_input_edge_trigger_irq_enable` field of the register.
        ///
        ///GPIO2 input edge trigger interrupt.
        pub fn gpio_2_input_edge_trigger_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `gpio_1_input_edge_trigger_irq_enable` field of the register.
        ///
        ///GPIO1 input edge trigger interrupt.
        pub fn gpio_1_input_edge_trigger_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `gpio_0_input_edge_trigger_irq_enable` field of the register.
        ///
        ///GPIO0 input edge trigger interrupt.
        pub fn gpio_0_input_edge_trigger_irq_enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `timer_timeout_irq_enable` field of the register.
        ///
        ///Timer timeout interrupt.
        pub fn set_timer_timeout_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `gpio_2_input_edge_trigger_irq_enable` field of the register.
        ///
        ///GPIO2 input edge trigger interrupt.
        pub fn set_gpio_2_input_edge_trigger_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `gpio_1_input_edge_trigger_irq_enable` field of the register.
        ///
        ///GPIO1 input edge trigger interrupt.
        pub fn set_gpio_1_input_edge_trigger_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `gpio_0_input_edge_trigger_irq_enable` field of the register.
        ///
        ///GPIO0 input edge trigger interrupt.
        pub fn set_gpio_0_input_edge_trigger_irq_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for IrqEnableControl5 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<IrqEnableControl5> for [u8; 1] {
        fn from(val: IrqEnableControl5) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for IrqEnableControl5 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("IrqEnableControl5");
            {
                d.field("timer_timeout_irq_enable", &self.timer_timeout_irq_enable());
            }
            {
                d.field(
                    "gpio_2_input_edge_trigger_irq_enable",
                    &self.gpio_2_input_edge_trigger_irq_enable(),
                );
            }
            {
                d.field(
                    "gpio_1_input_edge_trigger_irq_enable",
                    &self.gpio_1_input_edge_trigger_irq_enable(),
                );
            }
            {
                d.field(
                    "gpio_0_input_edge_trigger_irq_enable",
                    &self.gpio_0_input_edge_trigger_irq_enable(),
                );
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqEnableControl5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IrqEnableControl5 { ");
            defmt::write!(
                f,
                "timer_timeout_irq_enable: {=bool}, ",
                &self.timer_timeout_irq_enable(),
            );
            defmt::write!(
                f,
                "gpio_2_input_edge_trigger_irq_enable: {=bool}, ",
                &self.gpio_2_input_edge_trigger_irq_enable(),
            );
            defmt::write!(
                f,
                "gpio_1_input_edge_trigger_irq_enable: {=bool}, ",
                &self.gpio_1_input_edge_trigger_irq_enable(),
            );
            defmt::write!(
                f,
                "gpio_0_input_edge_trigger_irq_enable: {=bool}, ",
                &self.gpio_0_input_edge_trigger_irq_enable(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for IrqEnableControl5 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for IrqEnableControl5 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for IrqEnableControl5 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for IrqEnableControl5 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for IrqEnableControl5 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for IrqEnableControl5 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for IrqEnableControl5 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Interrupt Status Register 1. Bits are set when corresponding IRQ occurs. Write 1 to a bit to clear it.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqStatus1 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for IrqStatus1 {
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
    impl IrqStatus1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `acin_over_voltage_status_flag` field of the register.
        ///
        ///ACIN over-voltage IRQ status. Write 1 to clear.
        pub fn acin_over_voltage_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `acin_insertion_status_flag` field of the register.
        ///
        ///ACIN insertion IRQ status. Write 1 to clear.
        pub fn acin_insertion_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `acin_removal_status_flag` field of the register.
        ///
        ///ACIN removal IRQ status. Write 1 to clear.
        pub fn acin_removal_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `vbus_over_voltage_status_flag` field of the register.
        ///
        ///VBUS over-voltage IRQ status. Write 1 to clear.
        pub fn vbus_over_voltage_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `vbus_insertion_status_flag` field of the register.
        ///
        ///VBUS insertion IRQ status. Write 1 to clear.
        pub fn vbus_insertion_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `vbus_removal_status_flag` field of the register.
        ///
        ///VBUS removal IRQ status. Write 1 to clear.
        pub fn vbus_removal_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `vbus_less_than_vhold_status_flag` field of the register.
        ///
        ///VBUS < VHOLD IRQ status. Write 1 to clear.
        pub fn vbus_less_than_vhold_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Write the `acin_over_voltage_status_flag` field of the register.
        ///
        ///ACIN over-voltage IRQ status. Write 1 to clear.
        pub fn set_acin_over_voltage_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `acin_insertion_status_flag` field of the register.
        ///
        ///ACIN insertion IRQ status. Write 1 to clear.
        pub fn set_acin_insertion_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `acin_removal_status_flag` field of the register.
        ///
        ///ACIN removal IRQ status. Write 1 to clear.
        pub fn set_acin_removal_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `vbus_over_voltage_status_flag` field of the register.
        ///
        ///VBUS over-voltage IRQ status. Write 1 to clear.
        pub fn set_vbus_over_voltage_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `vbus_insertion_status_flag` field of the register.
        ///
        ///VBUS insertion IRQ status. Write 1 to clear.
        pub fn set_vbus_insertion_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `vbus_removal_status_flag` field of the register.
        ///
        ///VBUS removal IRQ status. Write 1 to clear.
        pub fn set_vbus_removal_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `vbus_less_than_vhold_status_flag` field of the register.
        ///
        ///VBUS < VHOLD IRQ status. Write 1 to clear.
        pub fn set_vbus_less_than_vhold_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for IrqStatus1 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<IrqStatus1> for [u8; 1] {
        fn from(val: IrqStatus1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for IrqStatus1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("IrqStatus1");
            {
                d.field(
                    "acin_over_voltage_status_flag",
                    &self.acin_over_voltage_status_flag(),
                );
            }
            {
                d.field(
                    "acin_insertion_status_flag",
                    &self.acin_insertion_status_flag(),
                );
            }
            {
                d.field("acin_removal_status_flag", &self.acin_removal_status_flag());
            }
            {
                d.field(
                    "vbus_over_voltage_status_flag",
                    &self.vbus_over_voltage_status_flag(),
                );
            }
            {
                d.field(
                    "vbus_insertion_status_flag",
                    &self.vbus_insertion_status_flag(),
                );
            }
            {
                d.field("vbus_removal_status_flag", &self.vbus_removal_status_flag());
            }
            {
                d.field(
                    "vbus_less_than_vhold_status_flag",
                    &self.vbus_less_than_vhold_status_flag(),
                );
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqStatus1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IrqStatus1 { ");
            defmt::write!(
                f,
                "acin_over_voltage_status_flag: {=bool}, ",
                &self.acin_over_voltage_status_flag(),
            );
            defmt::write!(
                f,
                "acin_insertion_status_flag: {=bool}, ",
                &self.acin_insertion_status_flag(),
            );
            defmt::write!(
                f,
                "acin_removal_status_flag: {=bool}, ",
                &self.acin_removal_status_flag(),
            );
            defmt::write!(
                f,
                "vbus_over_voltage_status_flag: {=bool}, ",
                &self.vbus_over_voltage_status_flag(),
            );
            defmt::write!(
                f,
                "vbus_insertion_status_flag: {=bool}, ",
                &self.vbus_insertion_status_flag(),
            );
            defmt::write!(
                f,
                "vbus_removal_status_flag: {=bool}, ",
                &self.vbus_removal_status_flag(),
            );
            defmt::write!(
                f,
                "vbus_less_than_vhold_status_flag: {=bool}, ",
                &self.vbus_less_than_vhold_status_flag(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for IrqStatus1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for IrqStatus1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for IrqStatus1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for IrqStatus1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for IrqStatus1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for IrqStatus1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for IrqStatus1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Interrupt Status Register 2. Bits are set when corresponding IRQ occurs. Write 1 to a bit to clear it.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqStatus2 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for IrqStatus2 {
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
    impl IrqStatus2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `battery_insertion_status_flag` field of the register.
        ///
        ///Battery insertion IRQ status. Write 1 to clear.
        pub fn battery_insertion_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `battery_removal_status_flag` field of the register.
        ///
        ///Battery removal IRQ status. Write 1 to clear.
        pub fn battery_removal_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `battery_activation_mode_entry_status_flag` field of the register.
        ///
        ///Battery enters activation mode IRQ status. Write 1 to clear.
        pub fn battery_activation_mode_entry_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `battery_activation_mode_exit_status_flag` field of the register.
        ///
        ///Battery exits activation mode IRQ status. Write 1 to clear.
        pub fn battery_activation_mode_exit_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `charging_status_flag` field of the register.
        ///
        ///Charging started/in progress IRQ status. Write 1 to clear.
        pub fn charging_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `charge_done_status_flag` field of the register.
        ///
        ///Charge completion IRQ status. Write 1 to clear.
        pub fn charge_done_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `battery_over_temp_status_flag` field of the register.
        ///
        ///Battery over-temperature IRQ status. Write 1 to clear.
        pub fn battery_over_temp_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `battery_under_temp_status_flag` field of the register.
        ///
        ///Battery under-temperature IRQ status. Write 1 to clear.
        pub fn battery_under_temp_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `battery_insertion_status_flag` field of the register.
        ///
        ///Battery insertion IRQ status. Write 1 to clear.
        pub fn set_battery_insertion_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `battery_removal_status_flag` field of the register.
        ///
        ///Battery removal IRQ status. Write 1 to clear.
        pub fn set_battery_removal_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `battery_activation_mode_entry_status_flag` field of the register.
        ///
        ///Battery enters activation mode IRQ status. Write 1 to clear.
        pub fn set_battery_activation_mode_entry_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `battery_activation_mode_exit_status_flag` field of the register.
        ///
        ///Battery exits activation mode IRQ status. Write 1 to clear.
        pub fn set_battery_activation_mode_exit_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `charging_status_flag` field of the register.
        ///
        ///Charging started/in progress IRQ status. Write 1 to clear.
        pub fn set_charging_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `charge_done_status_flag` field of the register.
        ///
        ///Charge completion IRQ status. Write 1 to clear.
        pub fn set_charge_done_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `battery_over_temp_status_flag` field of the register.
        ///
        ///Battery over-temperature IRQ status. Write 1 to clear.
        pub fn set_battery_over_temp_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `battery_under_temp_status_flag` field of the register.
        ///
        ///Battery under-temperature IRQ status. Write 1 to clear.
        pub fn set_battery_under_temp_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for IrqStatus2 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<IrqStatus2> for [u8; 1] {
        fn from(val: IrqStatus2) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for IrqStatus2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("IrqStatus2");
            {
                d.field(
                    "battery_insertion_status_flag",
                    &self.battery_insertion_status_flag(),
                );
            }
            {
                d.field(
                    "battery_removal_status_flag",
                    &self.battery_removal_status_flag(),
                );
            }
            {
                d.field(
                    "battery_activation_mode_entry_status_flag",
                    &self.battery_activation_mode_entry_status_flag(),
                );
            }
            {
                d.field(
                    "battery_activation_mode_exit_status_flag",
                    &self.battery_activation_mode_exit_status_flag(),
                );
            }
            {
                d.field("charging_status_flag", &self.charging_status_flag());
            }
            {
                d.field("charge_done_status_flag", &self.charge_done_status_flag());
            }
            {
                d.field(
                    "battery_over_temp_status_flag",
                    &self.battery_over_temp_status_flag(),
                );
            }
            {
                d.field(
                    "battery_under_temp_status_flag",
                    &self.battery_under_temp_status_flag(),
                );
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqStatus2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IrqStatus2 { ");
            defmt::write!(
                f,
                "battery_insertion_status_flag: {=bool}, ",
                &self.battery_insertion_status_flag(),
            );
            defmt::write!(
                f,
                "battery_removal_status_flag: {=bool}, ",
                &self.battery_removal_status_flag(),
            );
            defmt::write!(
                f,
                "battery_activation_mode_entry_status_flag: {=bool}, ",
                &self.battery_activation_mode_entry_status_flag(),
            );
            defmt::write!(
                f,
                "battery_activation_mode_exit_status_flag: {=bool}, ",
                &self.battery_activation_mode_exit_status_flag(),
            );
            defmt::write!(
                f,
                "charging_status_flag: {=bool}, ",
                &self.charging_status_flag(),
            );
            defmt::write!(
                f,
                "charge_done_status_flag: {=bool}, ",
                &self.charge_done_status_flag(),
            );
            defmt::write!(
                f,
                "battery_over_temp_status_flag: {=bool}, ",
                &self.battery_over_temp_status_flag(),
            );
            defmt::write!(
                f,
                "battery_under_temp_status_flag: {=bool}, ",
                &self.battery_under_temp_status_flag(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for IrqStatus2 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for IrqStatus2 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for IrqStatus2 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for IrqStatus2 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for IrqStatus2 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for IrqStatus2 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for IrqStatus2 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Interrupt Status Register 3. Bits are set when corresponding IRQ occurs. Write 1 to a bit to clear it.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqStatus3 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for IrqStatus3 {
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
    impl IrqStatus3 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `internal_over_temp_status_flag` field of the register.
        ///
        ///IC internal over-temperature IRQ status. Write 1 to clear.
        pub fn internal_over_temp_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `charge_current_insufficient_status_flag` field of the register.
        ///
        ///Charging current insufficient IRQ status. Write 1 to clear.
        pub fn charge_current_insufficient_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `dcdc_1_voltage_low_status_flag` field of the register.
        ///
        ///DC-DC1 output voltage low IRQ status. Write 1 to clear.
        pub fn dcdc_1_voltage_low_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `dcdc_2_voltage_low_status_flag` field of the register.
        ///
        ///DC-DC2 output voltage low IRQ status. Write 1 to clear.
        pub fn dcdc_2_voltage_low_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `dcdc_3_voltage_low_status_flag` field of the register.
        ///
        ///DC-DC3 output voltage low IRQ status. Write 1 to clear.
        pub fn dcdc_3_voltage_low_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `pek_short_press_status_flag` field of the register.
        ///
        ///PEK (Power Key) short press IRQ status. Write 1 to clear.
        pub fn pek_short_press_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `pek_long_press_status_flag` field of the register.
        ///
        ///PEK (Power Key) long press IRQ status. Write 1 to clear.
        pub fn pek_long_press_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `internal_over_temp_status_flag` field of the register.
        ///
        ///IC internal over-temperature IRQ status. Write 1 to clear.
        pub fn set_internal_over_temp_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `charge_current_insufficient_status_flag` field of the register.
        ///
        ///Charging current insufficient IRQ status. Write 1 to clear.
        pub fn set_charge_current_insufficient_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `dcdc_1_voltage_low_status_flag` field of the register.
        ///
        ///DC-DC1 output voltage low IRQ status. Write 1 to clear.
        pub fn set_dcdc_1_voltage_low_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `dcdc_2_voltage_low_status_flag` field of the register.
        ///
        ///DC-DC2 output voltage low IRQ status. Write 1 to clear.
        pub fn set_dcdc_2_voltage_low_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `dcdc_3_voltage_low_status_flag` field of the register.
        ///
        ///DC-DC3 output voltage low IRQ status. Write 1 to clear.
        pub fn set_dcdc_3_voltage_low_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `pek_short_press_status_flag` field of the register.
        ///
        ///PEK (Power Key) short press IRQ status. Write 1 to clear.
        pub fn set_pek_short_press_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `pek_long_press_status_flag` field of the register.
        ///
        ///PEK (Power Key) long press IRQ status. Write 1 to clear.
        pub fn set_pek_long_press_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for IrqStatus3 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<IrqStatus3> for [u8; 1] {
        fn from(val: IrqStatus3) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for IrqStatus3 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("IrqStatus3");
            {
                d.field(
                    "internal_over_temp_status_flag",
                    &self.internal_over_temp_status_flag(),
                );
            }
            {
                d.field(
                    "charge_current_insufficient_status_flag",
                    &self.charge_current_insufficient_status_flag(),
                );
            }
            {
                d.field(
                    "dcdc_1_voltage_low_status_flag",
                    &self.dcdc_1_voltage_low_status_flag(),
                );
            }
            {
                d.field(
                    "dcdc_2_voltage_low_status_flag",
                    &self.dcdc_2_voltage_low_status_flag(),
                );
            }
            {
                d.field(
                    "dcdc_3_voltage_low_status_flag",
                    &self.dcdc_3_voltage_low_status_flag(),
                );
            }
            {
                d.field(
                    "pek_short_press_status_flag",
                    &self.pek_short_press_status_flag(),
                );
            }
            {
                d.field(
                    "pek_long_press_status_flag",
                    &self.pek_long_press_status_flag(),
                );
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqStatus3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IrqStatus3 { ");
            defmt::write!(
                f,
                "internal_over_temp_status_flag: {=bool}, ",
                &self.internal_over_temp_status_flag(),
            );
            defmt::write!(
                f,
                "charge_current_insufficient_status_flag: {=bool}, ",
                &self.charge_current_insufficient_status_flag(),
            );
            defmt::write!(
                f,
                "dcdc_1_voltage_low_status_flag: {=bool}, ",
                &self.dcdc_1_voltage_low_status_flag(),
            );
            defmt::write!(
                f,
                "dcdc_2_voltage_low_status_flag: {=bool}, ",
                &self.dcdc_2_voltage_low_status_flag(),
            );
            defmt::write!(
                f,
                "dcdc_3_voltage_low_status_flag: {=bool}, ",
                &self.dcdc_3_voltage_low_status_flag(),
            );
            defmt::write!(
                f,
                "pek_short_press_status_flag: {=bool}, ",
                &self.pek_short_press_status_flag(),
            );
            defmt::write!(
                f,
                "pek_long_press_status_flag: {=bool}, ",
                &self.pek_long_press_status_flag(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for IrqStatus3 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for IrqStatus3 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for IrqStatus3 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for IrqStatus3 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for IrqStatus3 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for IrqStatus3 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for IrqStatus3 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Interrupt Status Register 4. Bits are set when corresponding IRQ occurs. Write 1 to a bit to clear it.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqStatus4 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for IrqStatus4 {
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
    impl IrqStatus4 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `n_oe_power_on_status_flag` field of the register.
        ///
        ///N_OE Power-on event IRQ status. Write 1 to clear.
        pub fn n_oe_power_on_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `n_oe_power_off_status_flag` field of the register.
        ///
        ///N_OE Power-off event IRQ status. Write 1 to clear.
        pub fn n_oe_power_off_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `vbus_valid_status_flag` field of the register.
        ///
        ///VBUS valid event IRQ status. Write 1 to clear.
        pub fn vbus_valid_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 5, 6)
            };
            raw > 0
        }
        ///Read the `vbus_invalid_status_flag` field of the register.
        ///
        ///VBUS invalid event IRQ status. Write 1 to clear.
        pub fn vbus_invalid_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `vbus_session_ab_status_flag` field of the register.
        ///
        ///VBUS Session A/B valid event IRQ status. Write 1 to clear.
        pub fn vbus_session_ab_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `vbus_session_end_status_flag` field of the register.
        ///
        ///VBUS Session End event IRQ status. Write 1 to clear.
        pub fn vbus_session_end_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `aps_low_voltage_level_2_status_flag` field of the register.
        ///
        ///APS low voltage (Warning Level 2) IRQ status. Write 1 to clear.
        pub fn aps_low_voltage_level_2_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `n_oe_power_on_status_flag` field of the register.
        ///
        ///N_OE Power-on event IRQ status. Write 1 to clear.
        pub fn set_n_oe_power_on_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `n_oe_power_off_status_flag` field of the register.
        ///
        ///N_OE Power-off event IRQ status. Write 1 to clear.
        pub fn set_n_oe_power_off_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `vbus_valid_status_flag` field of the register.
        ///
        ///VBUS valid event IRQ status. Write 1 to clear.
        pub fn set_vbus_valid_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 5, 6, &mut self.bits)
            };
        }
        ///Write the `vbus_invalid_status_flag` field of the register.
        ///
        ///VBUS invalid event IRQ status. Write 1 to clear.
        pub fn set_vbus_invalid_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `vbus_session_ab_status_flag` field of the register.
        ///
        ///VBUS Session A/B valid event IRQ status. Write 1 to clear.
        pub fn set_vbus_session_ab_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `vbus_session_end_status_flag` field of the register.
        ///
        ///VBUS Session End event IRQ status. Write 1 to clear.
        pub fn set_vbus_session_end_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `aps_low_voltage_level_2_status_flag` field of the register.
        ///
        ///APS low voltage (Warning Level 2) IRQ status. Write 1 to clear.
        pub fn set_aps_low_voltage_level_2_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for IrqStatus4 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<IrqStatus4> for [u8; 1] {
        fn from(val: IrqStatus4) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for IrqStatus4 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("IrqStatus4");
            {
                d.field("n_oe_power_on_status_flag", &self.n_oe_power_on_status_flag());
            }
            {
                d.field(
                    "n_oe_power_off_status_flag",
                    &self.n_oe_power_off_status_flag(),
                );
            }
            {
                d.field("vbus_valid_status_flag", &self.vbus_valid_status_flag());
            }
            {
                d.field("vbus_invalid_status_flag", &self.vbus_invalid_status_flag());
            }
            {
                d.field(
                    "vbus_session_ab_status_flag",
                    &self.vbus_session_ab_status_flag(),
                );
            }
            {
                d.field(
                    "vbus_session_end_status_flag",
                    &self.vbus_session_end_status_flag(),
                );
            }
            {
                d.field(
                    "aps_low_voltage_level_2_status_flag",
                    &self.aps_low_voltage_level_2_status_flag(),
                );
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqStatus4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IrqStatus4 { ");
            defmt::write!(
                f,
                "n_oe_power_on_status_flag: {=bool}, ",
                &self.n_oe_power_on_status_flag(),
            );
            defmt::write!(
                f,
                "n_oe_power_off_status_flag: {=bool}, ",
                &self.n_oe_power_off_status_flag(),
            );
            defmt::write!(
                f,
                "vbus_valid_status_flag: {=bool}, ",
                &self.vbus_valid_status_flag(),
            );
            defmt::write!(
                f,
                "vbus_invalid_status_flag: {=bool}, ",
                &self.vbus_invalid_status_flag(),
            );
            defmt::write!(
                f,
                "vbus_session_ab_status_flag: {=bool}, ",
                &self.vbus_session_ab_status_flag(),
            );
            defmt::write!(
                f,
                "vbus_session_end_status_flag: {=bool}, ",
                &self.vbus_session_end_status_flag(),
            );
            defmt::write!(
                f,
                "aps_low_voltage_level_2_status_flag: {=bool}, ",
                &self.aps_low_voltage_level_2_status_flag(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for IrqStatus4 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for IrqStatus4 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for IrqStatus4 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for IrqStatus4 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for IrqStatus4 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for IrqStatus4 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for IrqStatus4 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Interrupt Status Register 5. Bits are set when corresponding IRQ occurs. Write 1 to a bit to clear it.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqStatus5 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for IrqStatus5 {
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
    impl IrqStatus5 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `timer_timeout_status_flag` field of the register.
        ///
        ///Timer timeout IRQ status. Write 1 to clear.
        pub fn timer_timeout_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `gpio_2_input_edge_trigger_status_flag` field of the register.
        ///
        ///GPIO2 input edge trigger IRQ status. Write 1 to clear.
        pub fn gpio_2_input_edge_trigger_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `gpio_1_input_edge_trigger_status_flag` field of the register.
        ///
        ///GPIO1 input edge trigger IRQ status. Write 1 to clear.
        pub fn gpio_1_input_edge_trigger_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `gpio_0_input_edge_trigger_status_flag` field of the register.
        ///
        ///GPIO0 input edge trigger IRQ status. Write 1 to clear.
        pub fn gpio_0_input_edge_trigger_status_flag(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `timer_timeout_status_flag` field of the register.
        ///
        ///Timer timeout IRQ status. Write 1 to clear.
        pub fn set_timer_timeout_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `gpio_2_input_edge_trigger_status_flag` field of the register.
        ///
        ///GPIO2 input edge trigger IRQ status. Write 1 to clear.
        pub fn set_gpio_2_input_edge_trigger_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `gpio_1_input_edge_trigger_status_flag` field of the register.
        ///
        ///GPIO1 input edge trigger IRQ status. Write 1 to clear.
        pub fn set_gpio_1_input_edge_trigger_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `gpio_0_input_edge_trigger_status_flag` field of the register.
        ///
        ///GPIO0 input edge trigger IRQ status. Write 1 to clear.
        pub fn set_gpio_0_input_edge_trigger_status_flag(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for IrqStatus5 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<IrqStatus5> for [u8; 1] {
        fn from(val: IrqStatus5) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for IrqStatus5 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("IrqStatus5");
            {
                d.field("timer_timeout_status_flag", &self.timer_timeout_status_flag());
            }
            {
                d.field(
                    "gpio_2_input_edge_trigger_status_flag",
                    &self.gpio_2_input_edge_trigger_status_flag(),
                );
            }
            {
                d.field(
                    "gpio_1_input_edge_trigger_status_flag",
                    &self.gpio_1_input_edge_trigger_status_flag(),
                );
            }
            {
                d.field(
                    "gpio_0_input_edge_trigger_status_flag",
                    &self.gpio_0_input_edge_trigger_status_flag(),
                );
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqStatus5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IrqStatus5 { ");
            defmt::write!(
                f,
                "timer_timeout_status_flag: {=bool}, ",
                &self.timer_timeout_status_flag(),
            );
            defmt::write!(
                f,
                "gpio_2_input_edge_trigger_status_flag: {=bool}, ",
                &self.gpio_2_input_edge_trigger_status_flag(),
            );
            defmt::write!(
                f,
                "gpio_1_input_edge_trigger_status_flag: {=bool}, ",
                &self.gpio_1_input_edge_trigger_status_flag(),
            );
            defmt::write!(
                f,
                "gpio_0_input_edge_trigger_status_flag: {=bool}, ",
                &self.gpio_0_input_edge_trigger_status_flag(),
            );
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for IrqStatus5 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for IrqStatus5 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for IrqStatus5 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for IrqStatus5 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for IrqStatus5 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for IrqStatus5 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for IrqStatus5 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///ACIN Voltage ADC Data. This is a 12-bit value.
    ///The value is formed by (REG56H_byte << 4) | (REG57H_byte & 0x0F).
    ///Formula for conversion: Voltage (mV) = raw_12bit_adc_value * 1.7.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AcinVoltageAdc {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AcinVoltageAdc {
        const SIZE_BITS: u32 = 16;
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
    impl AcinVoltageAdc {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `value_raw` field of the register.
        ///
        ///Raw 12-bit ACIN voltage ADC reading. Multiply by 1.7 to get mV. Note: This field assumes REG57H[7:4] are zero or ignored for the 12-bit value composition.
        pub fn value_raw(&self) -> u16 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u16,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 12)
            };
            raw
        }
        ///Write the `value_raw` field of the register.
        ///
        ///Raw 12-bit ACIN voltage ADC reading. Multiply by 1.7 to get mV. Note: This field assumes REG57H[7:4] are zero or ignored for the 12-bit value composition.
        pub fn set_value_raw(&mut self, value: u16) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u16,
                    ::device_driver::ops::BE,
                >(raw, 0, 12, &mut self.bits)
            };
        }
    }
    impl From<[u8; 2]> for AcinVoltageAdc {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<AcinVoltageAdc> for [u8; 2] {
        fn from(val: AcinVoltageAdc) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AcinVoltageAdc {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AcinVoltageAdc");
            {
                d.field("value_raw", &self.value_raw());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AcinVoltageAdc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AcinVoltageAdc { ");
            defmt::write!(f, "value_raw: {=u16}, ", &self.value_raw());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for AcinVoltageAdc {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AcinVoltageAdc {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AcinVoltageAdc {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AcinVoltageAdc {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AcinVoltageAdc {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AcinVoltageAdc {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AcinVoltageAdc {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///ACIN Current ADC Data. This is a 12-bit value.
    ///The value is formed by (REG58H_byte << 4) | (REG59H_byte & 0x0F).
    ///Formula for conversion: Current (mA) = raw_12bit_adc_value * 0.625.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AcinCurrentAdc {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AcinCurrentAdc {
        const SIZE_BITS: u32 = 16;
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
    impl AcinCurrentAdc {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `value_raw` field of the register.
        ///
        ///Raw 12-bit ACIN current ADC reading. Multiply by 0.625 to get mA. Assumes REG59H[7:4] are zero/ignored.
        pub fn value_raw(&self) -> u16 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u16,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 12)
            };
            raw
        }
        ///Write the `value_raw` field of the register.
        ///
        ///Raw 12-bit ACIN current ADC reading. Multiply by 0.625 to get mA. Assumes REG59H[7:4] are zero/ignored.
        pub fn set_value_raw(&mut self, value: u16) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u16,
                    ::device_driver::ops::BE,
                >(raw, 0, 12, &mut self.bits)
            };
        }
    }
    impl From<[u8; 2]> for AcinCurrentAdc {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<AcinCurrentAdc> for [u8; 2] {
        fn from(val: AcinCurrentAdc) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AcinCurrentAdc {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AcinCurrentAdc");
            {
                d.field("value_raw", &self.value_raw());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AcinCurrentAdc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AcinCurrentAdc { ");
            defmt::write!(f, "value_raw: {=u16}, ", &self.value_raw());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for AcinCurrentAdc {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AcinCurrentAdc {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AcinCurrentAdc {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AcinCurrentAdc {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AcinCurrentAdc {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AcinCurrentAdc {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AcinCurrentAdc {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///VBUS Voltage ADC Data. This is a 12-bit value.
    ///The value is formed by (REG5AH_byte << 4) | (REG5BH_byte & 0x0F).
    ///Formula for conversion: Voltage (mV) = raw_12bit_adc_value * 1.7.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VbusVoltageAdc {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for VbusVoltageAdc {
        const SIZE_BITS: u32 = 16;
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
    impl VbusVoltageAdc {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `value_raw` field of the register.
        ///
        ///Raw 12-bit VBUS voltage ADC reading. Multiply by 1.7 to get mV. Assumes REG5BH[7:4] are zero/ignored.
        pub fn value_raw(&self) -> u16 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u16,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 12)
            };
            raw
        }
        ///Write the `value_raw` field of the register.
        ///
        ///Raw 12-bit VBUS voltage ADC reading. Multiply by 1.7 to get mV. Assumes REG5BH[7:4] are zero/ignored.
        pub fn set_value_raw(&mut self, value: u16) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u16,
                    ::device_driver::ops::BE,
                >(raw, 0, 12, &mut self.bits)
            };
        }
    }
    impl From<[u8; 2]> for VbusVoltageAdc {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<VbusVoltageAdc> for [u8; 2] {
        fn from(val: VbusVoltageAdc) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for VbusVoltageAdc {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("VbusVoltageAdc");
            {
                d.field("value_raw", &self.value_raw());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VbusVoltageAdc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "VbusVoltageAdc { ");
            defmt::write!(f, "value_raw: {=u16}, ", &self.value_raw());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for VbusVoltageAdc {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for VbusVoltageAdc {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for VbusVoltageAdc {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for VbusVoltageAdc {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for VbusVoltageAdc {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for VbusVoltageAdc {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for VbusVoltageAdc {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///VBUS Current ADC Data. This is a 12-bit value.
    ///The value is formed by (REG5CH_byte << 4) | (REG5DH_byte & 0x0F).
    ///Formula for conversion: Current (mA) = raw_12bit_adc_value * 0.375.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VbusCurrentAdc {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for VbusCurrentAdc {
        const SIZE_BITS: u32 = 16;
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
    impl VbusCurrentAdc {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `value_raw` field of the register.
        ///
        ///Raw 12-bit VBUS current ADC reading. Multiply by 0.375 to get mA. Assumes REG5DH[7:4] are zero/ignored.
        pub fn value_raw(&self) -> u16 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u16,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 12)
            };
            raw
        }
        ///Write the `value_raw` field of the register.
        ///
        ///Raw 12-bit VBUS current ADC reading. Multiply by 0.375 to get mA. Assumes REG5DH[7:4] are zero/ignored.
        pub fn set_value_raw(&mut self, value: u16) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u16,
                    ::device_driver::ops::BE,
                >(raw, 0, 12, &mut self.bits)
            };
        }
    }
    impl From<[u8; 2]> for VbusCurrentAdc {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<VbusCurrentAdc> for [u8; 2] {
        fn from(val: VbusCurrentAdc) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for VbusCurrentAdc {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("VbusCurrentAdc");
            {
                d.field("value_raw", &self.value_raw());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VbusCurrentAdc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "VbusCurrentAdc { ");
            defmt::write!(f, "value_raw: {=u16}, ", &self.value_raw());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for VbusCurrentAdc {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for VbusCurrentAdc {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for VbusCurrentAdc {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for VbusCurrentAdc {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for VbusCurrentAdc {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for VbusCurrentAdc {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for VbusCurrentAdc {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///AXP192 Internal Temperature ADC Data. This is a 12-bit value.
    ///The value is formed by (REG5EH_byte << 4) | (REG5FH_byte & 0x0F).
    ///Formula for conversion: Temperature (°C) = (raw_12bit_adc_value * 0.1) - 144.7.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InternalTemperatureAdc {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for InternalTemperatureAdc {
        const SIZE_BITS: u32 = 16;
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
    impl InternalTemperatureAdc {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `value_raw` field of the register.
        ///
        ///Raw 12-bit internal temperature ADC reading. Use formula (RAW * 0.1) - 144.7 to get °C. Assumes REG5FH[7:4] are zero/ignored.
        pub fn value_raw(&self) -> u16 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u16,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 12)
            };
            raw
        }
        ///Write the `value_raw` field of the register.
        ///
        ///Raw 12-bit internal temperature ADC reading. Use formula (RAW * 0.1) - 144.7 to get °C. Assumes REG5FH[7:4] are zero/ignored.
        pub fn set_value_raw(&mut self, value: u16) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u16,
                    ::device_driver::ops::BE,
                >(raw, 0, 12, &mut self.bits)
            };
        }
    }
    impl From<[u8; 2]> for InternalTemperatureAdc {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<InternalTemperatureAdc> for [u8; 2] {
        fn from(val: InternalTemperatureAdc) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for InternalTemperatureAdc {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("InternalTemperatureAdc");
            {
                d.field("value_raw", &self.value_raw());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for InternalTemperatureAdc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "InternalTemperatureAdc { ");
            defmt::write!(f, "value_raw: {=u16}, ", &self.value_raw());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for InternalTemperatureAdc {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for InternalTemperatureAdc {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for InternalTemperatureAdc {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for InternalTemperatureAdc {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for InternalTemperatureAdc {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for InternalTemperatureAdc {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for InternalTemperatureAdc {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///TS (Temperature Sense) Pin ADC Data. This is a 12-bit value representing the voltage at the TS pin.
    ///The value is formed by (REG62H_byte << 4) | (REG63H_byte & 0x0F).
    ///Formula for TS pin voltage: Voltage (mV) = raw_12bit_adc_value * 0.8.
    ///This voltage is typically from an NTC thermistor circuit for battery temperature monitoring.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsPinAdc {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for TsPinAdc {
        const SIZE_BITS: u32 = 16;
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
    impl TsPinAdc {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `value_raw` field of the register.
        ///
        ///Raw 12-bit TS pin ADC reading. Multiply by 0.8 to get mV. Assumes REG63H[7:4] are zero/ignored.
        pub fn value_raw(&self) -> u16 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u16,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 12)
            };
            raw
        }
        ///Write the `value_raw` field of the register.
        ///
        ///Raw 12-bit TS pin ADC reading. Multiply by 0.8 to get mV. Assumes REG63H[7:4] are zero/ignored.
        pub fn set_value_raw(&mut self, value: u16) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u16,
                    ::device_driver::ops::BE,
                >(raw, 0, 12, &mut self.bits)
            };
        }
    }
    impl From<[u8; 2]> for TsPinAdc {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<TsPinAdc> for [u8; 2] {
        fn from(val: TsPinAdc) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for TsPinAdc {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("TsPinAdc");
            {
                d.field("value_raw", &self.value_raw());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsPinAdc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TsPinAdc { ");
            defmt::write!(f, "value_raw: {=u16}, ", &self.value_raw());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for TsPinAdc {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for TsPinAdc {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for TsPinAdc {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for TsPinAdc {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for TsPinAdc {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for TsPinAdc {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for TsPinAdc {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///Reads the 12-bit ADC value for a GPIO pin (indexed 0-3 for GPIO0-GPIO3).
    ///The value is formed by (MSB_byte_of_pair << 4) | (LSB_byte_of_pair & 0x0F).
    ///Formula for pin voltage: Voltage (mV) = raw_12bit_adc_value * 0.5.
    ///The measurable voltage input range for each GPIO ADC is set in REG85H.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GpioVoltageAdc {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for GpioVoltageAdc {
        const SIZE_BITS: u32 = 16;
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
    impl GpioVoltageAdc {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `value_raw` field of the register.
        ///
        ///Raw 12-bit GPIO voltage ADC reading. Multiply by 0.5 to get mV.
        pub fn value_raw(&self) -> u16 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u16,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 12)
            };
            raw
        }
        ///Write the `value_raw` field of the register.
        ///
        ///Raw 12-bit GPIO voltage ADC reading. Multiply by 0.5 to get mV.
        pub fn set_value_raw(&mut self, value: u16) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u16,
                    ::device_driver::ops::BE,
                >(raw, 0, 12, &mut self.bits)
            };
        }
    }
    impl From<[u8; 2]> for GpioVoltageAdc {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<GpioVoltageAdc> for [u8; 2] {
        fn from(val: GpioVoltageAdc) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for GpioVoltageAdc {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("GpioVoltageAdc");
            {
                d.field("value_raw", &self.value_raw());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GpioVoltageAdc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GpioVoltageAdc { ");
            defmt::write!(f, "value_raw: {=u16}, ", &self.value_raw());
            defmt::write!(f, "}");
        }
    }
    impl core::ops::BitAnd for GpioVoltageAdc {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for GpioVoltageAdc {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for GpioVoltageAdc {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for GpioVoltageAdc {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for GpioVoltageAdc {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for GpioVoltageAdc {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for GpioVoltageAdc {
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
        ///Sets the rising edge voltage threshold for GPIO1 ADC input to trigger an interrupt.
        ///An IRQ is generated when the GPIO1 ADC voltage rises above this set threshold.
        ///Formula: Threshold Voltage (V) = raw_value * 0.008.
        ///Range: 0V (raw 0x00) to 2.04V (raw 0xFF).
        Gpio1AdcIrqRisingThreshold(Gpio1AdcIrqRisingThreshold),
        ///Sets the falling edge voltage threshold for GPIO1 ADC input to trigger an interrupt.
        ///An IRQ is generated when the GPIO1 ADC voltage falls below this set threshold.
        ///Formula: Threshold Voltage (V) = raw_value * 0.008.
        ///Range: 0V (raw 0x00) to 2.04V (raw 0xFF).
        Gpio1AdcIrqFallingThreshold(Gpio1AdcIrqFallingThreshold),
        ///Controls an internal timer, sets its duration, and indicates timeout status.
        TimerControl(TimerControl),
        ///Controls VBUS pin monitoring for Session Request Protocol (SRP) related functions, including valid voltage threshold and SRP feature enables.
        VbusSrpControl(VbusSrpControl),
        ///Controls the AXP192 internal over-temperature shutdown function. Other bits are reserved.
        OverTempShutdownControl(OverTempShutdownControl),
        ///Configures the function of the GPIO0 pin.
        Gpio0Control(Gpio0Control),
        ///Sets the output voltage for GPIO0 when it is configured in Low Noise LDO (LDOIO0) mode (via REG90H).
        Gpio0LdoVoltageSetting(Gpio0LdoVoltageSetting),
        ///Configures the function of the GPIO1 pin.
        Gpio1Control(Gpio1Control),
        ///Configures the function of the GPIO2 pin.
        Gpio2Control(Gpio2Control),
        ///Monitors input status for GPIO0-GPIO2 and controls their output levels
        ///when configured as NMOS open-drain or output low.
        Gpio0To2SignalStatusAndControl(Gpio0To2SignalStatusAndControl),
        ///Configures the function for GPIO3 and GPIO4 pins, and enables their GPIO mode.
        Gpio3And4FunctionControl(Gpio3And4FunctionControl),
        ///Monitors input status for GPIO3-GPIO4 and controls their output levels
        ///when configured as NMOS open-drain or output low.
        Gpio3And4SignalStatusAndControl(Gpio3And4SignalStatusAndControl),
        ///Controls internal pull-down resistors for GPIO0, GPIO1, and GPIO2 when they are configured as inputs.
        Gpio0To2PulldownControl(Gpio0To2PulldownControl),
        ///Sets the 'X' parameter for PWM1 output frequency calculation.
        ///Formula: F_pwm1 = 2.25MHz / (X_value + 1) / Y1_value (where Y1 is from REG99H).
        Pwm1FrequencySetting(Pwm1FrequencySetting),
        ///Sets the 'Y1' parameter for PWM1 duty cycle and frequency calculations.
        ///Y1 is the denominator for duty cycle (Duty = Y2/Y1) and also affects frequency.
        ///Y1 should not be set to 0.
        Pwm1DutyCycleSettingY1(Pwm1DutyCycleSettingY1),
        ///Sets the 'Y2' parameter (numerator) for PWM1 duty cycle calculation (Duty = Y2/Y1).
        ///Only upper 5 bits (bits 7-3) are used for Y2.
        Pwm1DutyCycleSettingY2(Pwm1DutyCycleSettingY2),
        ///Configures the N_RSTO/GPIO5 pin function. It can operate as N_RSTO (LDO1 status monitor)
        ///or as a general-purpose I/O pin (GPIO5) with configurable direction and output state.
        NrstoGpio5Control(NrstoGpio5Control),
        ///Interrupt Enable Control Register 1 (ACIN and VBUS related IRQs).
        IrqEnableControl1(IrqEnableControl1),
        ///Interrupt Enable Control Register 2 (Battery and Charge related IRQs).
        IrqEnableControl2(IrqEnableControl2),
        ///Interrupt Enable Control Register 3 (IC Temp, Charge Current, DCDC VLow, PEK IRQs).
        IrqEnableControl3(IrqEnableControl3),
        ///Interrupt Enable Control Register 4 (N_OE and VBUS session/status related IRQs, APS Low Voltage).
        IrqEnableControl4(IrqEnableControl4),
        ///Interrupt Enable Control Register 5 (Timer and GPIO Input Edge Trigger IRQs).
        IrqEnableControl5(IrqEnableControl5),
        ///Interrupt Status Register 1. Bits are set when corresponding IRQ occurs. Write 1 to a bit to clear it.
        IrqStatus1(IrqStatus1),
        ///Interrupt Status Register 2. Bits are set when corresponding IRQ occurs. Write 1 to a bit to clear it.
        IrqStatus2(IrqStatus2),
        ///Interrupt Status Register 3. Bits are set when corresponding IRQ occurs. Write 1 to a bit to clear it.
        IrqStatus3(IrqStatus3),
        ///Interrupt Status Register 4. Bits are set when corresponding IRQ occurs. Write 1 to a bit to clear it.
        IrqStatus4(IrqStatus4),
        ///Interrupt Status Register 5. Bits are set when corresponding IRQ occurs. Write 1 to a bit to clear it.
        IrqStatus5(IrqStatus5),
        ///ACIN Voltage ADC Data. This is a 12-bit value.
        ///The value is formed by (REG56H_byte << 4) | (REG57H_byte & 0x0F).
        ///Formula for conversion: Voltage (mV) = raw_12bit_adc_value * 1.7.
        AcinVoltageAdc(AcinVoltageAdc),
        ///ACIN Current ADC Data. This is a 12-bit value.
        ///The value is formed by (REG58H_byte << 4) | (REG59H_byte & 0x0F).
        ///Formula for conversion: Current (mA) = raw_12bit_adc_value * 0.625.
        AcinCurrentAdc(AcinCurrentAdc),
        ///VBUS Voltage ADC Data. This is a 12-bit value.
        ///The value is formed by (REG5AH_byte << 4) | (REG5BH_byte & 0x0F).
        ///Formula for conversion: Voltage (mV) = raw_12bit_adc_value * 1.7.
        VbusVoltageAdc(VbusVoltageAdc),
        ///VBUS Current ADC Data. This is a 12-bit value.
        ///The value is formed by (REG5CH_byte << 4) | (REG5DH_byte & 0x0F).
        ///Formula for conversion: Current (mA) = raw_12bit_adc_value * 0.375.
        VbusCurrentAdc(VbusCurrentAdc),
        ///AXP192 Internal Temperature ADC Data. This is a 12-bit value.
        ///The value is formed by (REG5EH_byte << 4) | (REG5FH_byte & 0x0F).
        ///Formula for conversion: Temperature (°C) = (raw_12bit_adc_value * 0.1) - 144.7.
        InternalTemperatureAdc(InternalTemperatureAdc),
        ///TS (Temperature Sense) Pin ADC Data. This is a 12-bit value representing the voltage at the TS pin.
        ///The value is formed by (REG62H_byte << 4) | (REG63H_byte & 0x0F).
        ///Formula for TS pin voltage: Voltage (mV) = raw_12bit_adc_value * 0.8.
        ///This voltage is typically from an NTC thermistor circuit for battery temperature monitoring.
        TsPinAdc(TsPinAdc),
        ///Reads the 12-bit ADC value for a GPIO pin (indexed 0-3 for GPIO0-GPIO3).
        ///The value is formed by (MSB_byte_of_pair << 4) | (LSB_byte_of_pair & 0x0F).
        ///Formula for pin voltage: Voltage (mV) = raw_12bit_adc_value * 0.5.
        ///The measurable voltage input range for each GPIO ADC is set in REG85H.
        GpioVoltageAdc(GpioVoltageAdc),
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
                Self::Gpio1AdcIrqRisingThreshold(val) => core::fmt::Debug::fmt(val, f),
                Self::Gpio1AdcIrqFallingThreshold(val) => core::fmt::Debug::fmt(val, f),
                Self::TimerControl(val) => core::fmt::Debug::fmt(val, f),
                Self::VbusSrpControl(val) => core::fmt::Debug::fmt(val, f),
                Self::OverTempShutdownControl(val) => core::fmt::Debug::fmt(val, f),
                Self::Gpio0Control(val) => core::fmt::Debug::fmt(val, f),
                Self::Gpio0LdoVoltageSetting(val) => core::fmt::Debug::fmt(val, f),
                Self::Gpio1Control(val) => core::fmt::Debug::fmt(val, f),
                Self::Gpio2Control(val) => core::fmt::Debug::fmt(val, f),
                Self::Gpio0To2SignalStatusAndControl(val) => {
                    core::fmt::Debug::fmt(val, f)
                }
                Self::Gpio3And4FunctionControl(val) => core::fmt::Debug::fmt(val, f),
                Self::Gpio3And4SignalStatusAndControl(val) => {
                    core::fmt::Debug::fmt(val, f)
                }
                Self::Gpio0To2PulldownControl(val) => core::fmt::Debug::fmt(val, f),
                Self::Pwm1FrequencySetting(val) => core::fmt::Debug::fmt(val, f),
                Self::Pwm1DutyCycleSettingY1(val) => core::fmt::Debug::fmt(val, f),
                Self::Pwm1DutyCycleSettingY2(val) => core::fmt::Debug::fmt(val, f),
                Self::NrstoGpio5Control(val) => core::fmt::Debug::fmt(val, f),
                Self::IrqEnableControl1(val) => core::fmt::Debug::fmt(val, f),
                Self::IrqEnableControl2(val) => core::fmt::Debug::fmt(val, f),
                Self::IrqEnableControl3(val) => core::fmt::Debug::fmt(val, f),
                Self::IrqEnableControl4(val) => core::fmt::Debug::fmt(val, f),
                Self::IrqEnableControl5(val) => core::fmt::Debug::fmt(val, f),
                Self::IrqStatus1(val) => core::fmt::Debug::fmt(val, f),
                Self::IrqStatus2(val) => core::fmt::Debug::fmt(val, f),
                Self::IrqStatus3(val) => core::fmt::Debug::fmt(val, f),
                Self::IrqStatus4(val) => core::fmt::Debug::fmt(val, f),
                Self::IrqStatus5(val) => core::fmt::Debug::fmt(val, f),
                Self::AcinVoltageAdc(val) => core::fmt::Debug::fmt(val, f),
                Self::AcinCurrentAdc(val) => core::fmt::Debug::fmt(val, f),
                Self::VbusVoltageAdc(val) => core::fmt::Debug::fmt(val, f),
                Self::VbusCurrentAdc(val) => core::fmt::Debug::fmt(val, f),
                Self::InternalTemperatureAdc(val) => core::fmt::Debug::fmt(val, f),
                Self::TsPinAdc(val) => core::fmt::Debug::fmt(val, f),
                Self::GpioVoltageAdc(val) => core::fmt::Debug::fmt(val, f),
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
                Self::Gpio1AdcIrqRisingThreshold(val) => defmt::Format::format(val, f),
                Self::Gpio1AdcIrqFallingThreshold(val) => defmt::Format::format(val, f),
                Self::TimerControl(val) => defmt::Format::format(val, f),
                Self::VbusSrpControl(val) => defmt::Format::format(val, f),
                Self::OverTempShutdownControl(val) => defmt::Format::format(val, f),
                Self::Gpio0Control(val) => defmt::Format::format(val, f),
                Self::Gpio0LdoVoltageSetting(val) => defmt::Format::format(val, f),
                Self::Gpio1Control(val) => defmt::Format::format(val, f),
                Self::Gpio2Control(val) => defmt::Format::format(val, f),
                Self::Gpio0To2SignalStatusAndControl(val) => {
                    defmt::Format::format(val, f)
                }
                Self::Gpio3And4FunctionControl(val) => defmt::Format::format(val, f),
                Self::Gpio3And4SignalStatusAndControl(val) => {
                    defmt::Format::format(val, f)
                }
                Self::Gpio0To2PulldownControl(val) => defmt::Format::format(val, f),
                Self::Pwm1FrequencySetting(val) => defmt::Format::format(val, f),
                Self::Pwm1DutyCycleSettingY1(val) => defmt::Format::format(val, f),
                Self::Pwm1DutyCycleSettingY2(val) => defmt::Format::format(val, f),
                Self::NrstoGpio5Control(val) => defmt::Format::format(val, f),
                Self::IrqEnableControl1(val) => defmt::Format::format(val, f),
                Self::IrqEnableControl2(val) => defmt::Format::format(val, f),
                Self::IrqEnableControl3(val) => defmt::Format::format(val, f),
                Self::IrqEnableControl4(val) => defmt::Format::format(val, f),
                Self::IrqEnableControl5(val) => defmt::Format::format(val, f),
                Self::IrqStatus1(val) => defmt::Format::format(val, f),
                Self::IrqStatus2(val) => defmt::Format::format(val, f),
                Self::IrqStatus3(val) => defmt::Format::format(val, f),
                Self::IrqStatus4(val) => defmt::Format::format(val, f),
                Self::IrqStatus5(val) => defmt::Format::format(val, f),
                Self::AcinVoltageAdc(val) => defmt::Format::format(val, f),
                Self::AcinCurrentAdc(val) => defmt::Format::format(val, f),
                Self::VbusVoltageAdc(val) => defmt::Format::format(val, f),
                Self::VbusCurrentAdc(val) => defmt::Format::format(val, f),
                Self::InternalTemperatureAdc(val) => defmt::Format::format(val, f),
                Self::TsPinAdc(val) => defmt::Format::format(val, f),
                Self::GpioVoltageAdc(val) => defmt::Format::format(val, f),
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
    impl From<Gpio1AdcIrqRisingThreshold> for FieldSetValue {
        fn from(val: Gpio1AdcIrqRisingThreshold) -> Self {
            Self::Gpio1AdcIrqRisingThreshold(val)
        }
    }
    impl From<Gpio1AdcIrqFallingThreshold> for FieldSetValue {
        fn from(val: Gpio1AdcIrqFallingThreshold) -> Self {
            Self::Gpio1AdcIrqFallingThreshold(val)
        }
    }
    impl From<TimerControl> for FieldSetValue {
        fn from(val: TimerControl) -> Self {
            Self::TimerControl(val)
        }
    }
    impl From<VbusSrpControl> for FieldSetValue {
        fn from(val: VbusSrpControl) -> Self {
            Self::VbusSrpControl(val)
        }
    }
    impl From<OverTempShutdownControl> for FieldSetValue {
        fn from(val: OverTempShutdownControl) -> Self {
            Self::OverTempShutdownControl(val)
        }
    }
    impl From<Gpio0Control> for FieldSetValue {
        fn from(val: Gpio0Control) -> Self {
            Self::Gpio0Control(val)
        }
    }
    impl From<Gpio0LdoVoltageSetting> for FieldSetValue {
        fn from(val: Gpio0LdoVoltageSetting) -> Self {
            Self::Gpio0LdoVoltageSetting(val)
        }
    }
    impl From<Gpio1Control> for FieldSetValue {
        fn from(val: Gpio1Control) -> Self {
            Self::Gpio1Control(val)
        }
    }
    impl From<Gpio2Control> for FieldSetValue {
        fn from(val: Gpio2Control) -> Self {
            Self::Gpio2Control(val)
        }
    }
    impl From<Gpio0To2SignalStatusAndControl> for FieldSetValue {
        fn from(val: Gpio0To2SignalStatusAndControl) -> Self {
            Self::Gpio0To2SignalStatusAndControl(val)
        }
    }
    impl From<Gpio3And4FunctionControl> for FieldSetValue {
        fn from(val: Gpio3And4FunctionControl) -> Self {
            Self::Gpio3And4FunctionControl(val)
        }
    }
    impl From<Gpio3And4SignalStatusAndControl> for FieldSetValue {
        fn from(val: Gpio3And4SignalStatusAndControl) -> Self {
            Self::Gpio3And4SignalStatusAndControl(val)
        }
    }
    impl From<Gpio0To2PulldownControl> for FieldSetValue {
        fn from(val: Gpio0To2PulldownControl) -> Self {
            Self::Gpio0To2PulldownControl(val)
        }
    }
    impl From<Pwm1FrequencySetting> for FieldSetValue {
        fn from(val: Pwm1FrequencySetting) -> Self {
            Self::Pwm1FrequencySetting(val)
        }
    }
    impl From<Pwm1DutyCycleSettingY1> for FieldSetValue {
        fn from(val: Pwm1DutyCycleSettingY1) -> Self {
            Self::Pwm1DutyCycleSettingY1(val)
        }
    }
    impl From<Pwm1DutyCycleSettingY2> for FieldSetValue {
        fn from(val: Pwm1DutyCycleSettingY2) -> Self {
            Self::Pwm1DutyCycleSettingY2(val)
        }
    }
    impl From<NrstoGpio5Control> for FieldSetValue {
        fn from(val: NrstoGpio5Control) -> Self {
            Self::NrstoGpio5Control(val)
        }
    }
    impl From<IrqEnableControl1> for FieldSetValue {
        fn from(val: IrqEnableControl1) -> Self {
            Self::IrqEnableControl1(val)
        }
    }
    impl From<IrqEnableControl2> for FieldSetValue {
        fn from(val: IrqEnableControl2) -> Self {
            Self::IrqEnableControl2(val)
        }
    }
    impl From<IrqEnableControl3> for FieldSetValue {
        fn from(val: IrqEnableControl3) -> Self {
            Self::IrqEnableControl3(val)
        }
    }
    impl From<IrqEnableControl4> for FieldSetValue {
        fn from(val: IrqEnableControl4) -> Self {
            Self::IrqEnableControl4(val)
        }
    }
    impl From<IrqEnableControl5> for FieldSetValue {
        fn from(val: IrqEnableControl5) -> Self {
            Self::IrqEnableControl5(val)
        }
    }
    impl From<IrqStatus1> for FieldSetValue {
        fn from(val: IrqStatus1) -> Self {
            Self::IrqStatus1(val)
        }
    }
    impl From<IrqStatus2> for FieldSetValue {
        fn from(val: IrqStatus2) -> Self {
            Self::IrqStatus2(val)
        }
    }
    impl From<IrqStatus3> for FieldSetValue {
        fn from(val: IrqStatus3) -> Self {
            Self::IrqStatus3(val)
        }
    }
    impl From<IrqStatus4> for FieldSetValue {
        fn from(val: IrqStatus4) -> Self {
            Self::IrqStatus4(val)
        }
    }
    impl From<IrqStatus5> for FieldSetValue {
        fn from(val: IrqStatus5) -> Self {
            Self::IrqStatus5(val)
        }
    }
    impl From<AcinVoltageAdc> for FieldSetValue {
        fn from(val: AcinVoltageAdc) -> Self {
            Self::AcinVoltageAdc(val)
        }
    }
    impl From<AcinCurrentAdc> for FieldSetValue {
        fn from(val: AcinCurrentAdc) -> Self {
            Self::AcinCurrentAdc(val)
        }
    }
    impl From<VbusVoltageAdc> for FieldSetValue {
        fn from(val: VbusVoltageAdc) -> Self {
            Self::VbusVoltageAdc(val)
        }
    }
    impl From<VbusCurrentAdc> for FieldSetValue {
        fn from(val: VbusCurrentAdc) -> Self {
            Self::VbusCurrentAdc(val)
        }
    }
    impl From<InternalTemperatureAdc> for FieldSetValue {
        fn from(val: InternalTemperatureAdc) -> Self {
            Self::InternalTemperatureAdc(val)
        }
    }
    impl From<TsPinAdc> for FieldSetValue {
        fn from(val: TsPinAdc) -> Self {
            Self::TsPinAdc(val)
        }
    }
    impl From<GpioVoltageAdc> for FieldSetValue {
        fn from(val: GpioVoltageAdc) -> Self {
            Self::GpioVoltageAdc(val)
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusValidThresholdValue {
    ///4.00V threshold.
    V400 = 0,
    ///4.15V threshold.
    V415 = 1,
    ///4.45V threshold.
    V445 = 2,
    ///4.55V threshold.
    V455 = 3,
}
impl core::convert::TryFrom<u8> for VbusValidThresholdValue {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::V400),
            1 => Ok(Self::V415),
            2 => Ok(Self::V445),
            3 => Ok(Self::V455),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "VbusValidThresholdValue",
                })
            }
        }
    }
}
impl From<VbusValidThresholdValue> for u8 {
    fn from(val: VbusValidThresholdValue) -> Self {
        match val {
            VbusValidThresholdValue::V400 => 0,
            VbusValidThresholdValue::V415 => 1,
            VbusValidThresholdValue::V445 => 2,
            VbusValidThresholdValue::V455 => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0FunctionSelect {
    ///NMOS Open-Drain Output.
    NmosOpenDrainOutput = 0,
    ///Universal Input.
    UniversalInput = 1,
    ///Low Noise LDO (LDOIO0) Output.
    LowNoiseLdoOutput = 2,
    ///ADC Input.
    AdcInput = 4,
    ///Output Driven Low.
    OutputLow = 5,
    ///Floating (High-Impedance).
    Floating = 6,
}
impl Default for Gpio0FunctionSelect {
    fn default() -> Self {
        Self::Floating
    }
}
impl From<u8> for Gpio0FunctionSelect {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NmosOpenDrainOutput,
            1 => Self::UniversalInput,
            2 => Self::LowNoiseLdoOutput,
            4 => Self::AdcInput,
            5 => Self::OutputLow,
            _ => Self::default(),
        }
    }
}
impl From<Gpio0FunctionSelect> for u8 {
    fn from(val: Gpio0FunctionSelect) -> Self {
        match val {
            Gpio0FunctionSelect::NmosOpenDrainOutput => 0,
            Gpio0FunctionSelect::UniversalInput => 1,
            Gpio0FunctionSelect::LowNoiseLdoOutput => 2,
            Gpio0FunctionSelect::AdcInput => 4,
            Gpio0FunctionSelect::OutputLow => 5,
            Gpio0FunctionSelect::Floating => 6,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1FunctionSelect {
    ///NMOS Open-Drain Output.
    NmosOpenDrainOutput = 0,
    ///Universal Input.
    UniversalInput = 1,
    ///PWM1 Output.
    Pwm1Output = 2,
    ///ADC Input.
    AdcInput = 4,
    ///Output Driven Low.
    OutputLow = 5,
    ///Floating (High-Impedance).
    Floating = 6,
}
impl Default for Gpio1FunctionSelect {
    fn default() -> Self {
        Self::Floating
    }
}
impl From<u8> for Gpio1FunctionSelect {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NmosOpenDrainOutput,
            1 => Self::UniversalInput,
            2 => Self::Pwm1Output,
            4 => Self::AdcInput,
            5 => Self::OutputLow,
            _ => Self::default(),
        }
    }
}
impl From<Gpio1FunctionSelect> for u8 {
    fn from(val: Gpio1FunctionSelect) -> Self {
        match val {
            Gpio1FunctionSelect::NmosOpenDrainOutput => 0,
            Gpio1FunctionSelect::UniversalInput => 1,
            Gpio1FunctionSelect::Pwm1Output => 2,
            Gpio1FunctionSelect::AdcInput => 4,
            Gpio1FunctionSelect::OutputLow => 5,
            Gpio1FunctionSelect::Floating => 6,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio2FunctionSelect {
    ///NMOS Open-Drain Output.
    NmosOpenDrainOutput = 0,
    ///Universal Input.
    UniversalInput = 1,
    ///PWM2 Output.
    Pwm2Output = 2,
    ///ADC Input.
    AdcInput = 4,
    ///Output Driven Low.
    OutputLow = 5,
    ///Floating (High-Impedance).
    Floating = 6,
}
impl Default for Gpio2FunctionSelect {
    fn default() -> Self {
        Self::Floating
    }
}
impl From<u8> for Gpio2FunctionSelect {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NmosOpenDrainOutput,
            1 => Self::UniversalInput,
            2 => Self::Pwm2Output,
            4 => Self::AdcInput,
            5 => Self::OutputLow,
            _ => Self::default(),
        }
    }
}
impl From<Gpio2FunctionSelect> for u8 {
    fn from(val: Gpio2FunctionSelect) -> Self {
        match val {
            Gpio2FunctionSelect::NmosOpenDrainOutput => 0,
            Gpio2FunctionSelect::UniversalInput => 1,
            Gpio2FunctionSelect::Pwm2Output => 2,
            Gpio2FunctionSelect::AdcInput => 4,
            Gpio2FunctionSelect::OutputLow => 5,
            Gpio2FunctionSelect::Floating => 6,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio4FunctionSetting {
    ///External Charge Control.
    ExternalChargeControl = 0,
    ///NMOS Open-Drain Output.
    NmosOpenDrainOutput = 1,
    ///Universal Input.
    UniversalInput = 2,
}
impl core::convert::TryFrom<u8> for Gpio4FunctionSetting {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::ExternalChargeControl),
            1 => Ok(Self::NmosOpenDrainOutput),
            2 => Ok(Self::UniversalInput),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "Gpio4FunctionSetting",
                })
            }
        }
    }
}
impl From<Gpio4FunctionSetting> for u8 {
    fn from(val: Gpio4FunctionSetting) -> Self {
        match val {
            Gpio4FunctionSetting::ExternalChargeControl => 0,
            Gpio4FunctionSetting::NmosOpenDrainOutput => 1,
            Gpio4FunctionSetting::UniversalInput => 2,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio3FunctionSetting {
    ///External Charge Control.
    ExternalChargeControl = 0,
    ///NMOS Open-Drain Output.
    NmosOpenDrainOutput = 1,
    ///Universal Input.
    UniversalInput = 2,
    ///ADC Input.
    AdcInput = 3,
}
impl core::convert::TryFrom<u8> for Gpio3FunctionSetting {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::ExternalChargeControl),
            1 => Ok(Self::NmosOpenDrainOutput),
            2 => Ok(Self::UniversalInput),
            3 => Ok(Self::AdcInput),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "Gpio3FunctionSetting",
                })
            }
        }
    }
}
impl From<Gpio3FunctionSetting> for u8 {
    fn from(val: Gpio3FunctionSetting) -> Self {
        match val {
            Gpio3FunctionSetting::ExternalChargeControl => 0,
            Gpio3FunctionSetting::NmosOpenDrainOutput => 1,
            Gpio3FunctionSetting::UniversalInput => 2,
            Gpio3FunctionSetting::AdcInput => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NrstoPinFunction {
    ///Pin functions as N_RSTO (LDO1 status monitor).
    NrstoLdo1Monitor = 0,
    ///Pin functions as GPIO5 (universal I/O).
    Gpio5 = 1,
}
impl core::convert::TryFrom<u8> for NrstoPinFunction {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NrstoLdo1Monitor),
            1 => Ok(Self::Gpio5),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "NrstoPinFunction",
                })
            }
        }
    }
}
impl From<NrstoPinFunction> for u8 {
    fn from(val: NrstoPinFunction) -> Self {
        match val {
            NrstoPinFunction::NrstoLdo1Monitor => 0,
            NrstoPinFunction::Gpio5 => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio5Direction {
    ///GPIO5 configured as NMOS Open-Drain Output.
    NmosOpenDrainOutput = 0,
    ///GPIO5 configured as Universal Input.
    UniversalInput = 1,
}
impl core::convert::TryFrom<u8> for Gpio5Direction {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NmosOpenDrainOutput),
            1 => Ok(Self::UniversalInput),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "Gpio5Direction",
                })
            }
        }
    }
}
impl From<Gpio5Direction> for u8 {
    fn from(val: Gpio5Direction) -> Self {
        match val {
            Gpio5Direction::NmosOpenDrainOutput => 0,
            Gpio5Direction::UniversalInput => 1,
        }
    }
}
