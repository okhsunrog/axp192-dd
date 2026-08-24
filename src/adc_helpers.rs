//! Recombination of the ADC readings the AXP192 splits across two registers.
//!
//! Per datasheet section 9.11.4, every ADC channel is stored as a "high 8 bit"
//! register followed by a "low 4 bit" (or "low 5 bit") one. The generated
//! fieldset getters already mask each half to its own field, so combining is
//! just a shift and an or.

/// Combine a 12-bit reading, where `high` is ADC[11:4] and `low` is ADC[3:0].
pub(crate) fn adc_12bit(high: u8, low: u8) -> u16 {
    ((high as u16) << 4) | low as u16
}

/// Combine a 13-bit reading, where `high` is ADC[12:5] and `low` is ADC[4:0].
pub(crate) fn adc_13bit(high: u8, low: u8) -> u16 {
    ((high as u16) << 5) | low as u16
}
