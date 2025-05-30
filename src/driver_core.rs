// src/driver_core.rs
#![cfg_attr(not(test), no_std)]

// These `use super::*` pull in items from the respective `asynchronous` or `blocking` bisync scopes.
// Specifically, they bring in `bisync`, `only_sync`, `only_async`, `SYNC`, `ASYNC`.
use super::*;

// Bring in items from the crate root (lib.rs)
use crate::{
    AxpError, ChargeTargetVoltage, DcId, GpioMode, GpioPin, LdoId, PekBootTime, PekLongPressTime,
    PekShutdownDuration,
};

// Logging (optional, but good for drivers)
use log::{debug, info, warn}; // Ensure log crate is a dependency

// Required for trait implementations
use device_driver::ll::register::Register;

// --- Generate the Low-Level Driver ---
// This uses the YAML file to create the Axp192LowLevel struct and associated register accessors.
// The generated Axp192LowLevel struct will have both sync (.read()) and async (.read_async()) methods.
device_driver::create_device!(device_name: Axp192LowLevel, manifest: "src/axp192_registers.yaml");

// --- I2C Bus Interface (AxpInterface) ---
// This struct will implement the actual I2C communication.
// It needs to be generic over the I2C bus type to support both sync and async HALs.
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

// Synchronous implementation for RegisterInterface
// This will only be compiled and active when the `blocking` module path is used.
#[only_sync]
impl<I2CBus, E> device_driver::ll::register::RegisterInterface for AxpInterface<I2CBus>
where
    I2CBus: embedded_hal::i2c::I2c<Error = E>, // Sync I2C HAL trait
    E: core::fmt::Debug,
{
    type Address = u8; // Must match register_address_type in YAML
    type Error = AxpError<E>;

    fn read_register<R>(
        &mut self,
        _offset: Option<u8>,
        out: &mut R::Word,
    ) -> Result<(), Self::Error>
    where
        R: Register<Address = Self::Address>,
        R::Word: AsMut<[u8]>,
    {
        self.i2c_bus
            .write_read(self.device_address, &[R::ADDRESS], out.as_mut())
            .map_err(AxpError::I2c)
    }

    fn write_register<R>(&mut self, _offset: Option<u8>, value: &R::Word) -> Result<(), Self::Error>
    where
        R: Register<Address = Self::Address>,
        R::Word: AsRef<[u8]>,
    {
        let word_bytes = value.as_ref();
        let mut buffer = [0u8; 1 + R::WORD_SIZE_BYTES]; // Max register size for AXP192 is small
        buffer[0] = R::ADDRESS;
        buffer[1..1 + word_bytes.len()].copy_from_slice(word_bytes);

        self.i2c_bus
            .write(self.device_address, &buffer[..1 + word_bytes.len()])
            .map_err(AxpError::I2c)
    }
}

// Asynchronous implementation for AsyncRegisterInterface
// This will only be compiled and active when the `asynchronous` module path is used.
#[only_async]
impl<I2CBus, E> device_driver::AsyncRegisterInterface for AxpInterface<I2CBus>
where
    I2CBus: embedded_hal_async::i2c::I2c<Error = E>, // Async I2C HAL trait
    E: core::fmt::Debug,
{
    type Address = u8; // Must match register_address_type in YAML
    type Error = AxpError<E>;

    async fn read_register_async<R>(
        &mut self,
        _offset: Option<u8>,
        out: &mut R::Word,
    ) -> Result<(), Self::Error>
    where
        R: Register<Address = Self::Address>,
        R::Word: AsMut<[u8]>,
    {
        self.i2c_bus
            .write_read(self.device_address, &[R::ADDRESS], out.as_mut())
            .await
            .map_err(AxpError::I2c)
    }

    async fn write_register_async<R>(
        &mut self,
        _offset: Option<u8>,
        value: &R::Word,
    ) -> Result<(), Self::Error>
    where
        R: Register<Address = Self::Address>,
        R::Word: AsRef<[u8]>,
    {
        let word_bytes = value.as_ref();
        let mut buffer = [0u8; 1 + R::WORD_SIZE_BYTES];
        buffer[0] = R::ADDRESS;
        buffer[1..1 + word_bytes.len()].copy_from_slice(word_bytes);

        self.i2c_bus
            .write(self.device_address, &buffer[..1 + word_bytes.len()])
            .await
            .map_err(AxpError::I2c)
    }
}

// --- High-Level AXP192 Driver Struct ---
// The `I2CImpl` generic will be `AxpInterface<YourSyncI2cBus>` in the blocking context
// and `AxpInterface<YourAsyncI2cBus>` in the asynchronous context.
pub struct Axp192<I2CImpl> {
    ll: Axp192LowLevel<I2CImpl>,
    // We might need to store the I2C bus type E (error type) for AxpError
    // if AxpError itself is not generic enough or if I2CImpl doesn't expose it easily.
    // However, AxpError<E> where E is from the I2CBus should work.
}

// Constructor for the blocking version
// This function itself is synchronous.
#[only_sync] // This means this `impl` block is only part of the `blocking` module
impl<I2CBus, E> Axp192<AxpInterface<I2CBus>>
// Specify the concrete type for I2CImpl
where
    I2CBus: embedded_hal::i2c::I2c<Error = E>, // Sync I2C HAL
    E: core::fmt::Debug,
    // Ensure that the specific AxpInterface instantiation implements the sync RegisterInterface
    AxpInterface<I2CBus>:
        device_driver::ll::register::RegisterInterface<Error = AxpError<E>, Address = u8>,
{
    pub fn new(i2c_bus: I2CBus) -> Self {
        // Returns Axp192<AxpInterface<SyncI2CBus>>
        let interface = AxpInterface::new(i2c_bus, 0x34); // AXP192 default I2C address
        Self {
            ll: Axp192LowLevel::new(interface),
        }
    }
}

// Constructor for the asynchronous version
// This function itself is synchronous (constructors usually are), but it sets up for async operations.
#[only_async] // This means this `impl` block is only part of the `asynchronous` module
impl<I2CBus, E> Axp192<AxpInterface<I2CBus>>
// Specify the concrete type for I2CImpl
where
    I2CBus: embedded_hal_async::i2c::I2c<Error = E>, // Async I2C HAL
    E: core::fmt::Debug,
    // Ensure that the specific AxpInterface instantiation implements the async AsyncRegisterInterface
    AxpInterface<I2CBus>: device_driver::AsyncRegisterInterface<Error = AxpError<E>, Address = u8>,
{
    pub fn new(i2c_bus: I2CBus) -> Self {
        // Returns Axp192<AxpInterface<AsyncI2CBus>>
        let interface = AxpInterface::new(i2c_bus, 0x34);
        Self {
            ll: Axp192LowLevel::new(interface),
        }
    }
}

// --- High-level methods common to both sync and async ---
// These methods will be transformed by `bisync`.
// The `I2CImpl` needs to satisfy bounds for *both* sync and async low-level calls
// that `Axp192LowLevel` might make. `device-driver` generates both `.read()` and `.read_async()`.
// `bisync` will pick the correct one.
impl<I2CImpl, I2CBusErr> Axp192<I2CImpl>
where
    // This is the crucial part: I2CImpl must provide *both* interfaces
    // for the Axp192LowLevel to use, so bisync can correctly
    // strip/keep .await and call the underlying sync/async methods.
    I2CImpl: device_driver::ll::register::RegisterInterface<Address = u8, Error = AxpError<I2CBusErr>>
        + device_driver::AsyncRegisterInterface<Address = u8, Error = AxpError<I2CBusErr>>,
    I2CBusErr: core::fmt::Debug,
{
    // Example: Getting battery voltage
    #[bisync]
    pub async fn get_battery_voltage_mv(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        // In async context: self.ll.battery_voltage_adc().read_async().await?.raw();
        // In sync context:  self.ll.battery_voltage_adc().read()?.raw();
        let raw_adc = self.ll.battery_voltage_adc().read().await?.raw();
        let val_12bit = raw_adc >> 4; // AXP192 ADC values are high-aligned
        Ok(val_12bit as f32 * 1.1) // 1.1mV per LSB
    }

    // Add the `init_m5stack_core2` method here, marked with `#[bisync]`
    // and using `.await` for all register operations.
    // It will look very similar to the async version you had before.
    #[bisync]
    pub async fn init_m5stack_core2(&mut self) -> Result<(), AxpError<I2CBusErr>> {
        info!(
            "Initializing AXP192 (ASYNC compile: {}) for M5Stack Core2 profile",
            ASYNC
        );

        // Set DCDC1 (ESP32 VDD) to 3.35V
        self.set_dcdc_voltage(DcId::Dcdc1, 3350).await?;
        self.set_output_enable_dcdc(DcId::Dcdc1, true).await?;

        // Set DCDC3 (LCD Power) to 2.8V
        self.set_dcdc_voltage(DcId::Dcdc3, 2800).await?;
        self.set_output_enable_dcdc(DcId::Dcdc3, true).await?;

        // Set LDO2 (Peripherals) to 3.3V
        self.set_ldo_voltage(LdoId::Ldo2, 3300).await?;
        self.set_output_enable_ldo(LdoId::Ldo2, true).await?;

        // Set LDO3 (Vibrator Motor) to 2.0V, initially off
        self.set_ldo_voltage(LdoId::Ldo3, 2000).await?;
        self.set_output_enable_ldo(LdoId::Ldo3, false).await?;

        self.ll
            .adc_enable1()
            .modify(|r| {
                // .await will be handled by bisync
                r.set_battery_voltage_adc_enable(true)
                    .set_battery_current_adc_enable(true)
                    .set_acin_voltage_adc_enable(true)
                    .set_acin_current_adc_enable(true)
                    .set_vbus_voltage_adc_enable(true)
                    .set_vbus_current_adc_enable(true)
                    .set_ts_pin_adc_enable(false);
            })
            .await?;

        self.set_pek_settings(
            PekBootTime::S1,
            PekLongPressTime::Ms1500,
            true,
            true,
            PekShutdownDuration::S4,
        )
        .await?;

        self.ll
            .battery_charge_control()
            .modify(|r| {
                r.set_backup_battery_charge_enable(true);
            })
            .await?;

        self.set_charge_current_ma(100).await?;

        self.set_gpio_mode(GpioPin::Gpio1, GpioMode::NmosOpenDrainOutput)
            .await?;
        self.set_gpio_output_level(GpioPin::Gpio1, true).await?;

        self.set_gpio_mode(GpioPin::Gpio2, GpioMode::NmosOpenDrainOutput)
            .await?;
        self.set_gpio_output_level(GpioPin::Gpio2, true).await?;

        info!("AXP192 M5Stack Core2 profile applied.");
        Ok(())
    }

    // Implement other high-level methods using #[bisync] and .await
    // (set_dcdc_voltage, set_output_enable_dcdc, etc. from previous full example)
    // Example:
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
                self.ll
                    .dcdc1_voltage()
                    .modify(|r| r.set_setting(setting))
                    .await?
            }
            DcId::Dcdc3 => {
                self.ll
                    .dcdc3_voltage()
                    .modify(|r| r.set_setting(setting))
                    .await?
            }
        }
        Ok(())
    }

    #[bisync]
    pub async fn set_output_enable_dcdc(
        &mut self,
        dc: DcId,
        enable: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        self.ll
            .dcdc13ldo23_control()
            .modify(|r| {
                match dc {
                    DcId::Dcdc1 => r.set_dcdc1_enable(enable),
                    DcId::Dcdc3 => r.set_dcdc3_enable(enable),
                };
            })
            .await
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
        self.ll
            .ldo23_voltage()
            .modify(|r| {
                match ldo {
                    LdoId::Ldo2 => r.set_ldo2_setting(setting),
                    LdoId::Ldo3 => r.set_ldo3_setting(setting),
                };
            })
            .await
    }

    #[bisync]
    pub async fn set_output_enable_ldo(
        &mut self,
        ldo: LdoId,
        enable: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        self.ll
            .dcdc13ldo23_control()
            .modify(|r| {
                match ldo {
                    LdoId::Ldo2 => r.set_ldo2_enable(enable),
                    LdoId::Ldo3 => r.set_ldo3_enable(enable),
                };
            })
            .await
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
            641..=730 => 0b0111,
            731..=1000 => 0b1000, // Note: max current depends on settings
            _ => return Err(AxpError::InvalidCurrent(current_ma)),
        };
        self.ll
            .charge_control1()
            .modify(|r| r.set_charge_current_setting(current_bits))
            .await
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
                self.ll
                    .gpio0_control()
                    .modify(|r| r.set_mode(mode_val))
                    .await?
            }
            GpioPin::Gpio1 => {
                self.ll
                    .gpio1_control()
                    .modify(|r| r.set_mode(mode_val))
                    .await?
            }
            GpioPin::Gpio2 => {
                self.ll
                    .gpio2_control()
                    .modify(|r| r.set_mode(mode_val))
                    .await?
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
        self.ll
            .gpio20_signal_status()
            .modify(|r| {
                match pin {
                    GpioPin::Gpio0 => r.set_gpio0_out_level(level_high), // YAML uses gpioX_out_level
                    GpioPin::Gpio1 => r.set_gpio1_out_level(level_high),
                    GpioPin::Gpio2 => r.set_gpio2_out_level(level_high),
                };
            })
            .await
    }

    #[bisync]
    pub async fn set_pek_settings(
        &mut self,
        boot_time: PekBootTime,
        long_press: PekLongPressTime,
        auto_shutdown_en: bool,
        pwrok_delay_64ms: bool,
        shutdown_duration: PekShutdownDuration,
    ) -> Result<(), AxpError<I2CBusErr>> {
        self.ll
            .pek_settings()
            .write(|w| {
                // `write_async` or `write` will be chosen by device-driver
                w.set_boot_time_setting(boot_time as u8)
                    .set_long_press_time_setting(long_press as u8)
                    .set_auto_shutdown_by_pwrok_en(auto_shutdown_en) // Check YAML field name
                    .set_pwrok_signal_delay(if pwrok_delay_64ms { 1 } else { 0 }) // Adjust if field is bool or u8
                    .set_shutdown_duration_setting(shutdown_duration as u8);
            })
            .await
    }

    #[bisync]
    pub async fn set_led_enable(&mut self, enable: bool) -> Result<(), AxpError<I2CBusErr>> {
        self.set_gpio_output_level(GpioPin::Gpio1, !enable).await // Assuming LED on Gpio1, active low
    }
}
