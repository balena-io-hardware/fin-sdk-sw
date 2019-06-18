use crate::ethtool::{get_eeprom_revision, get_raw_eeprom, set_raw_eeprom};

use crate::i2c::probe_i2c_pca9633;

pub fn get_revision() -> String {
    let mut revision = "09".to_string();

    if let Some(eeprom_revision) = get_eeprom_revision() {
        revision = eeprom_revision;
    } else if probe_i2c_pca9633().is_some() {
        revision = "10".to_string();
    }

    revision
}

pub fn set_eeprom(eeprom: &str) {
    set_raw_eeprom(eeprom);
}

pub fn get_eeprom() -> String {
    if let Some(eeprom) = get_raw_eeprom() {
        eeprom
    } else {
        "".to_string()
    }
}
