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
    ///Controls output enable for DC-DC1, DC-DC3, LDO2, LDO3, and provides alternative controls for EXTEN and DC-DC2.
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
    ///Formula: Output Voltage (V) = 0.7 + (voltage_setting_raw * 0.025).
    ///Effective range for voltage_setting_raw (bits 0-5): 0-63.
    ///Resulting voltage range: 0.7V to 2.275V, in 25mV steps.
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
    ///Configures parameters for DC-DC2 Dynamic Voltage Scaling / Voltage Ramp Control (VRC),
    ///including VRC enable state and voltage rise slope.
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
    ///Sets the output voltage for the DC-DC converter.
    ///Formula: Output Voltage (V) = 0.7 + (voltage_setting_raw * 0.025).
    ///Effective range for voltage_setting_raw (bits 0-6): 0-112 (decimal).
    ///Resulting voltage range: 0.7V to 3.5V, in 25mV steps.
    ///Values for voltage_setting_raw from 113 to 127 may be reserved or have undefined behavior.
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
    ///LDO2 (bits 7-4): Formula: V_out = 1.8V + (setting * 100mV). Range: 1.8V-3.3V.
    ///LDO3 (bits 3-0): Formula: V_out = 1.8V + (setting * 100mV). Range: 1.8V-3.3V.
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
    ///Manages the VBUS to IPSOUT path, VHOLD voltage limiting, and VBUS current limiting.
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
        ///Read the `battery_is_charging` field of the register.
        ///
        ///Battery current direction (true: Battery is Charging, false: Battery is Discharging).
        pub fn battery_is_charging(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
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
        ///Write the `battery_is_charging` field of the register.
        ///
        ///Battery current direction (true: Battery is Charging, false: Battery is Discharging).
        pub fn set_battery_is_charging(&mut self, value: bool) {
            let raw = value as _;
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
                d.field("battery_is_charging", &self.battery_is_charging());
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
            defmt::write!(
                f,
                "battery_is_charging: {=bool}, ",
                &self.battery_is_charging(),
            );
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
        ///Read the `axp_192_startup_mode_is_b` field of the register.
        ///
        ///AXP192 startup mode (true: Mode B, false: Mode A).
        pub fn axp_192_startup_mode_is_b(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 1, 2)
            };
            raw > 0
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
        ///Write the `axp_192_startup_mode_is_b` field of the register.
        ///
        ///AXP192 startup mode (true: Mode B, false: Mode A).
        pub fn set_axp_192_startup_mode_is_b(&mut self, value: bool) {
            let raw = value as _;
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
                d.field("axp_192_startup_mode_is_b", &self.axp_192_startup_mode_is_b());
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
            defmt::write!(
                f,
                "axp_192_startup_mode_is_b: {=bool}, ",
                &self.axp_192_startup_mode_is_b(),
            );
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
    ///Controls output enable for DC-DC1, DC-DC3, LDO2, LDO3, and provides alternative controls for EXTEN and DC-DC2.
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
    ///Formula: Output Voltage (V) = 0.7 + (voltage_setting_raw * 0.025).
    ///Effective range for voltage_setting_raw (bits 0-5): 0-63.
    ///Resulting voltage range: 0.7V to 2.275V, in 25mV steps.
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
        ///Read the `voltage_setting_raw` field of the register.
        ///
        ///Raw 6-bit value for DC-DC2 output voltage setting. See register description for conversion formula.
        pub fn voltage_setting_raw(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 6)
            };
            raw
        }
        ///Write the `voltage_setting_raw` field of the register.
        ///
        ///Raw 6-bit value for DC-DC2 output voltage setting. See register description for conversion formula.
        pub fn set_voltage_setting_raw(&mut self, value: u8) {
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
                d.field("voltage_setting_raw", &self.voltage_setting_raw());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DcDc2VoltageSetting {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DcDc2VoltageSetting { ");
            defmt::write!(
                f,
                "voltage_setting_raw: {=u8}, ",
                &self.voltage_setting_raw(),
            );
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
    ///Configures parameters for DC-DC2 Dynamic Voltage Scaling / Voltage Ramp Control (VRC),
    ///including VRC enable state and voltage rise slope.
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
        ///DC-DC2 VRC (Voltage Ramp Control) disable control (true: VRC disabled, false: VRC enabled). Note inverted logic.
        pub fn vrc_disabled(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `vrc_rise_slope_is_0_8_mv_us` field of the register.
        ///
        ///DC-DC2 VRC voltage rise slope (true: 0.8mV/us (25mV/31.25us), false: 1.6mV/us (25mV/15.625us)).
        pub fn vrc_rise_slope_is_0_8_mv_us(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `vrc_disabled` field of the register.
        ///
        ///DC-DC2 VRC (Voltage Ramp Control) disable control (true: VRC disabled, false: VRC enabled). Note inverted logic.
        pub fn set_vrc_disabled(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `vrc_rise_slope_is_0_8_mv_us` field of the register.
        ///
        ///DC-DC2 VRC voltage rise slope (true: 0.8mV/us (25mV/31.25us), false: 1.6mV/us (25mV/15.625us)).
        pub fn set_vrc_rise_slope_is_0_8_mv_us(&mut self, value: bool) {
            let raw = value as _;
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
                d.field(
                    "vrc_rise_slope_is_0_8_mv_us",
                    &self.vrc_rise_slope_is_0_8_mv_us(),
                );
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DcDc2VrcParameter {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DcDc2VrcParameter { ");
            defmt::write!(f, "vrc_disabled: {=bool}, ", &self.vrc_disabled());
            defmt::write!(
                f,
                "vrc_rise_slope_is_0_8_mv_us: {=bool}, ",
                &self.vrc_rise_slope_is_0_8_mv_us(),
            );
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
    ///Sets the output voltage for the DC-DC converter.
    ///Formula: Output Voltage (V) = 0.7 + (voltage_setting_raw * 0.025).
    ///Effective range for voltage_setting_raw (bits 0-6): 0-112 (decimal).
    ///Resulting voltage range: 0.7V to 3.5V, in 25mV steps.
    ///Values for voltage_setting_raw from 113 to 127 may be reserved or have undefined behavior.
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
        ///Read the `voltage_setting_raw` field of the register.
        ///
        ///Raw 7-bit value for DC-DC1 output voltage setting. See register description for conversion formula and valid range.
        pub fn voltage_setting_raw(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 7)
            };
            raw
        }
        ///Write the `voltage_setting_raw` field of the register.
        ///
        ///Raw 7-bit value for DC-DC1 output voltage setting. See register description for conversion formula and valid range.
        pub fn set_voltage_setting_raw(&mut self, value: u8) {
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
                d.field("voltage_setting_raw", &self.voltage_setting_raw());
            }
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DcDc1VoltageSetting {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DcDc1VoltageSetting { ");
            defmt::write!(
                f,
                "voltage_setting_raw: {=u8}, ",
                &self.voltage_setting_raw(),
            );
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
    ///LDO2 (bits 7-4): Formula: V_out = 1.8V + (setting * 100mV). Range: 1.8V-3.3V.
    ///LDO3 (bits 3-0): Formula: V_out = 1.8V + (setting * 100mV). Range: 1.8V-3.3V.
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
        ///Read the `ldo_2_voltage_setting_raw` field of the register.
        ///
        ///Raw 4-bit setting for LDO2 output voltage (1.8V to 3.3V, 100mV/step).
        pub fn ldo_2_voltage_setting_raw(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 4, 8)
            };
            raw
        }
        ///Read the `ldo_3_voltage_setting_raw` field of the register.
        ///
        ///Raw 4-bit setting for LDO3 output voltage (1.8V to 3.3V, 100mV/step).
        pub fn ldo_3_voltage_setting_raw(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 4)
            };
            raw
        }
        ///Write the `ldo_2_voltage_setting_raw` field of the register.
        ///
        ///Raw 4-bit setting for LDO2 output voltage (1.8V to 3.3V, 100mV/step).
        pub fn set_ldo_2_voltage_setting_raw(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 4, 8, &mut self.bits)
            };
        }
        ///Write the `ldo_3_voltage_setting_raw` field of the register.
        ///
        ///Raw 4-bit setting for LDO3 output voltage (1.8V to 3.3V, 100mV/step).
        pub fn set_ldo_3_voltage_setting_raw(&mut self, value: u8) {
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
                d.field("ldo_2_voltage_setting_raw", &self.ldo_2_voltage_setting_raw());
            }
            {
                d.field("ldo_3_voltage_setting_raw", &self.ldo_3_voltage_setting_raw());
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
                "ldo_2_voltage_setting_raw: {=u8}, ",
                &self.ldo_2_voltage_setting_raw(),
            );
            defmt::write!(
                f,
                "ldo_3_voltage_setting_raw: {=u8}, ",
                &self.ldo_3_voltage_setting_raw(),
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
    ///Manages the VBUS to IPSOUT path, VHOLD voltage limiting, and VBUS current limiting.
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
            Self { bits: [96] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `path_control_by_n_vbusen_pin` field of the register.
        ///
        ///VBUS-IPSOUT path selection control (true: path open regardless of N_VBUSEN, false: path controlled by N_VBUSEN pin).
        pub fn path_control_by_n_vbusen_pin(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `vhold_voltage_limit_enabled` field of the register.
        ///
        ///VBUS VHOLD voltage limiting control (true: limit enabled, false: no limit).
        pub fn vhold_voltage_limit_enabled(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 6, 7)
            };
            raw > 0
        }
        ///Read the `vhold_voltage_setting_raw` field of the register.
        ///
        ///VHOLD voltage setting (000:4.0V, 001:4.1V, 010:4.2V, 011:4.3V, 100:4.4V, 101:4.5V, 110:4.6V, 111:4.7V).
        ///Raw value.
        pub fn vhold_voltage_setting_raw(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 3, 6)
            };
            raw
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
        ///Read the `vbus_current_limit_is_100_ma` field of the register.
        ///
        ///VBUS current limit selection when enabled (true: 100mA, false: 500mA).
        pub fn vbus_current_limit_is_100_ma(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `path_control_by_n_vbusen_pin` field of the register.
        ///
        ///VBUS-IPSOUT path selection control (true: path open regardless of N_VBUSEN, false: path controlled by N_VBUSEN pin).
        pub fn set_path_control_by_n_vbusen_pin(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `vhold_voltage_limit_enabled` field of the register.
        ///
        ///VBUS VHOLD voltage limiting control (true: limit enabled, false: no limit).
        pub fn set_vhold_voltage_limit_enabled(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::BE,
                >(raw, 6, 7, &mut self.bits)
            };
        }
        ///Write the `vhold_voltage_setting_raw` field of the register.
        ///
        ///VHOLD voltage setting (000:4.0V, 001:4.1V, 010:4.2V, 011:4.3V, 100:4.4V, 101:4.5V, 110:4.6V, 111:4.7V).
        ///Raw value.
        pub fn set_vhold_voltage_setting_raw(&mut self, value: u8) {
            let raw = value;
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
        ///Write the `vbus_current_limit_is_100_ma` field of the register.
        ///
        ///VBUS current limit selection when enabled (true: 100mA, false: 500mA).
        pub fn set_vbus_current_limit_is_100_ma(&mut self, value: bool) {
            let raw = value as _;
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
                d.field(
                    "path_control_by_n_vbusen_pin",
                    &self.path_control_by_n_vbusen_pin(),
                );
            }
            {
                d.field(
                    "vhold_voltage_limit_enabled",
                    &self.vhold_voltage_limit_enabled(),
                );
            }
            {
                d.field("vhold_voltage_setting_raw", &self.vhold_voltage_setting_raw());
            }
            {
                d.field(
                    "vbus_current_limit_enabled",
                    &self.vbus_current_limit_enabled(),
                );
            }
            {
                d.field(
                    "vbus_current_limit_is_100_ma",
                    &self.vbus_current_limit_is_100_ma(),
                );
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
                "path_control_by_n_vbusen_pin: {=bool}, ",
                &self.path_control_by_n_vbusen_pin(),
            );
            defmt::write!(
                f,
                "vhold_voltage_limit_enabled: {=bool}, ",
                &self.vhold_voltage_limit_enabled(),
            );
            defmt::write!(
                f,
                "vhold_voltage_setting_raw: {=u8}, ",
                &self.vhold_voltage_setting_raw(),
            );
            defmt::write!(
                f,
                "vbus_current_limit_enabled: {=bool}, ",
                &self.vbus_current_limit_enabled(),
            );
            defmt::write!(
                f,
                "vbus_current_limit_is_100_ma: {=bool}, ",
                &self.vbus_current_limit_is_100_ma(),
            );
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
        ///Controls output enable for DC-DC1, DC-DC3, LDO2, LDO3, and provides alternative controls for EXTEN and DC-DC2.
        ///IMPORTANT: REG12H bit 6 (exten_output_enable) is linked to REG10H bit 2.
        ///             REG12H bit 4 (dcdc2_output_enable) is linked to REG10H bit 0.
        ///             These pairs control the same underlying hardware function.
        PowerOutputControl(PowerOutputControl),
        ///Sets the output voltage for the DC-DC2 converter.
        ///Formula: Output Voltage (V) = 0.7 + (voltage_setting_raw * 0.025).
        ///Effective range for voltage_setting_raw (bits 0-5): 0-63.
        ///Resulting voltage range: 0.7V to 2.275V, in 25mV steps.
        DcDc2VoltageSetting(DcDc2VoltageSetting),
        ///Configures parameters for DC-DC2 Dynamic Voltage Scaling / Voltage Ramp Control (VRC),
        ///including VRC enable state and voltage rise slope.
        DcDc2VrcParameter(DcDc2VrcParameter),
        ///Sets the output voltage for the DC-DC converter.
        ///Formula: Output Voltage (V) = 0.7 + (voltage_setting_raw * 0.025).
        ///Effective range for voltage_setting_raw (bits 0-6): 0-112 (decimal).
        ///Resulting voltage range: 0.7V to 3.5V, in 25mV steps.
        ///Values for voltage_setting_raw from 113 to 127 may be reserved or have undefined behavior.
        DcDc1VoltageSetting(DcDc1VoltageSetting),
        ///Sets the output voltage for both LDO2 and LDO3.
        ///LDO2 (bits 7-4): Formula: V_out = 1.8V + (setting * 100mV). Range: 1.8V-3.3V.
        ///LDO3 (bits 3-0): Formula: V_out = 1.8V + (setting * 100mV). Range: 1.8V-3.3V.
        Ldo2And3VoltageSetting(Ldo2And3VoltageSetting),
        ///Manages the VBUS to IPSOUT path, VHOLD voltage limiting, and VBUS current limiting.
        VbusIpsoutPathManagement(VbusIpsoutPathManagement),
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
}
