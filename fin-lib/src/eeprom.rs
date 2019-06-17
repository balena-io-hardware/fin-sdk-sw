use crate::ethtool::get_eeprom_revision;

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
