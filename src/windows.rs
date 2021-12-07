#[derive(Debug)]
pub struct ZoneMapping {
    windows: &'static str,
    territory: &'static str,
    iana: &'static [&'static str],
}

include!("windows_zones.gen.rs");

/// Given a windows zone name and an optional territory code returns the IANA zone.
pub fn win_zone_to_iana(zone: &str, territory: Option<&str>) -> Option<&'static str> {
    ZONE_MAPPINGS
        .iter()
        .find(|x| x.windows == zone && (territory.is_none() || Some(x.territory) == territory))
        .map(|x| x.iana[0])
}

/// Look up a windows timezone for an IANA zone.
///
/// The return value is in the format `(windows_zone_name, territory_code)` as they
/// are maintained in the CLDR mapping.
pub fn iana_to_win_zone(zone: &str) -> Option<(&'static str, &'static str)> {
    ZONE_MAPPINGS
        .iter()
        .find(|x| x.iana.contains(&zone))
        .map(|x| (x.windows, x.territory))
}

#[cfg(windows)]
mod win_impl {
    use windows::Win32::System::Time::{GetTimeZoneInformation, TIME_ZONE_INFORMATION};

    fn string_from_utf16(wide: &[u16]) -> String {
        String::from_utf16_lossy(wide.split(|x| *x == 0).next().unwrap())
    }

    /// Find the timezone in the windows registry
    pub fn get_local_zone() -> Option<String> {
        let mut tz = TIME_ZONE_INFORMATION::default();
        if let 0 | 1 | 2 = unsafe { GetTimeZoneInformation(&mut tz) } {
            let zone = string_from_utf16(&tz.StandardName);
            win_zone_to_iana(&zone, None)
        } else {
            None
        }
    }
}

#[cfg(windows)]
pub use self::win_impl::*;

#[test]
fn test_win_zone_to_iana() {
    assert_eq!(
        win_zone_to_iana("US Mountain Standard Time", Some("CA")),
        Some("America/Creston")
    );
    assert_eq!(
        win_zone_to_iana("US Mountain Standard Time", None),
        Some("America/Phoenix")
    );
}

#[test]
fn test_iana_to_win_zone() {
    assert_eq!(
        iana_to_win_zone("Europe/Vienna"),
        Some(("W. Europe Standard Time", "AT"))
    );
}
