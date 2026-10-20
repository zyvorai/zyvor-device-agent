// SPDX-License-Identifier: Apache-2.0

use crate::model::ThermalZone;
use std::fs;

/// Sysfs `temp` is always reported in millidegrees Celsius, including
/// negative and sub-1 degree values.
fn millidegrees_to_celsius(v: f64) -> f64 {
    v / 1000.0
}

pub fn collect() -> Vec<ThermalZone> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir("/sys/class/thermal") else {
        return out;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.starts_with("thermal_zone") {
            continue;
        }
        let base = entry.path();
        let kind = fs::read_to_string(base.join("type"))
            .unwrap_or_default()
            .trim()
            .to_string();
        let celsius = fs::read_to_string(base.join("temp"))
            .ok()
            .and_then(|v| v.trim().parse::<f64>().ok())
            .map(millidegrees_to_celsius);
        out.push(ThermalZone {
            name,
            kind,
            celsius,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::millidegrees_to_celsius;

    #[test]
    fn converts_negative_and_small_readings() {
        assert_eq!(millidegrees_to_celsius(-5000.0), -5.0);
        assert_eq!(millidegrees_to_celsius(500.0), 0.5);
        assert_eq!(millidegrees_to_celsius(45000.0), 45.0);
    }
}
