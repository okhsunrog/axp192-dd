#![no_std]
#![no_main]

use axp192_dd::{Axp192, AxpError, ChargeCurrentValue, Gpio0FunctionSelect, LdoId};
use defmt::info;
use esp_hal::{
    Blocking,
    delay::Delay,
    i2c::master::{Config as I2cConfig, Error as I2cError, I2c},
    time::Rate,
};
use panic_rtt_target as _;
use rtt_target::rtt_init_defmt;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal::main]
fn main() -> ! {
    rtt_init_defmt!();
    info!("Init!");

    let p = esp_hal::init(esp_hal::Config::default());
    let config: I2cConfig = I2cConfig::default().with_frequency(Rate::from_khz(400));
    let i2c = I2c::new(p.I2C0, config)
        .unwrap()
        .with_sda(p.GPIO6)
        .with_scl(p.GPIO7);

    init_pmic(i2c).unwrap();
    let delay = Delay::new();

    loop {
        info!("Hello world!");
        delay.delay_millis(250);
    }
}

#[rustfmt::skip]
fn init_pmic(i2c: I2c<'_, Blocking>) -> Result<(), AxpError<I2cError>> {
    let mut axp = Axp192::new(i2c);
    axp.set_ldo_voltage_mv(LdoId::Ldo2, 3300)?;
    axp.ll.adc_enable_1().write(|r| {
        r.set_battery_current_adc_enable(true);
        r.set_acin_voltage_adc_enable(true);
        r.set_acin_current_adc_enable(true);
        r.set_vbus_voltage_adc_enable(true);
        r.set_vbus_current_adc_enable(true);
        r.set_aps_voltage_adc_enable(true);
    })?;
    axp.ll.charge_control_1().write(|r| r.set_charge_current(ChargeCurrentValue::Ma100))?;
    axp.set_gpio0_ldo_voltage_mv(3300)?;
    axp.ll.gpio_0_control().write(|r| {
        r.set_function_select(Gpio0FunctionSelect::LowNoiseLdoOutput);
    })?;
    axp.ll.power_output_control().modify(|r| {
        r.set_dcdc_1_output_enable(true);
        r.set_dcdc_3_output_enable(false);
        r.set_ldo_2_output_enable(true);
        r.set_ldo_3_output_enable(true);
        r.set_dcdc_2_output_enable(false);
        r.set_exten_output_enable(true);
    })?;
    axp.set_battery_charge_high_temp_threshold_mv(3226)?;
    axp.ll.backup_battery_charge_control().write(|r| {
        r.set_backup_charge_enable(true);
    })?;

    info!("Battery voltage: {} mV", axp.get_battery_voltage_mv()?);
    info!("Charge current: {} mA", axp.get_battery_charge_current_ma()?);
    Ok(())
}
