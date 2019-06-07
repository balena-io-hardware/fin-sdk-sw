use crate::ethtool::get_eeprom_version;

use crate::i2c::probe_i2c_pca9633;

pub fn get_fin_version() -> String {
    let mut version = "1.0".to_string();

    if let Some(eeprom_version) = get_eeprom_version() {
        version = eeprom_version;
    } else if probe_i2c_pca9633().is_some() {
        version = "1.1".to_string();
    }

    version
}
