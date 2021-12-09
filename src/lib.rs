//! Utility crate to figure out the IANA (Olson) timezone of the current
//! machine.  The IANA timezones have been largely established as the standard
//! for identifying timezones but retrieving the current timezone in that format
//! is a surprisingly complex problem.  Additionally Windows machines use an
//! incompatible timezone system and this crate helps automaticaly mapping these
//! to the IANA timezones.  For this it uses the CLDR maintained timezone mapping.
//!
//! # Getting Timezones
//!
//! To retrieve the current timezone call the [`get_local_zone`] function.  It will
//! try different sources to get the current timezone.  When the `auto_validation`
//! feature is enabled it will attempt to validate each source against the current
//! [`chrono_tz`](https://crates.io/crates/chrono-tz) database.  Alternatively
//! [`get_local_zone_with_validation`] can be used an be supplied an alternative
//! validator function.
//!
//! If the timezone cannot be found a user of this crate shall assume the system is
//! running in UTC.
//!
//! # Features
//!
//! The following optional features exist:
//!
//! * `auto_validation`: when enabled timezones encountered are validated with
//!   `chrono_tz` by default.
//! * `win_zones`: when enabled functions are exposed to map between windows and
//!   IANA timezones.
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(unix)]
mod unix;
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
mod wasm32;
#[cfg(any(feature = "win_zones", windows))]
mod windows;

#[cfg(feature = "win_zones")]
pub use crate::windows::{iana_to_win_zone, win_zone_to_iana};

/// Returns the local timezone if it can be found.
///
/// If the `auto_validation` feature is enabled then encountered timezones
/// are validated with `chrono_tz`.
pub fn get_local_zone() -> Option<String> {
    #[cfg(feature = "auto_validation")]
    {
        get_local_zone_with_validation(|tz| tz.parse::<chrono_tz::Tz>().is_ok())
    }
    #[cfg(not(feature = "auto_validation"))]
    {
        get_local_zone_with_validation(|_| true)
    }
}

/// Returns the local timezone validating it with a custom function.
///
/// As this function probes different sources for the current timezone a validation
/// function is invoked to check if one of the choices is valid.  If it is, then
/// that match is returned.
pub fn get_local_zone_with_validation<F: FnMut(&str) -> bool>(is_valid: F) -> Option<String> {
    #[cfg(unix)]
    {
        unix::get_local_zone(is_valid)
    }
    #[cfg(windows)]
    {
        let mut is_valid = is_valid;
        windows::get_local_zone().filter(|x| is_valid(&x))
    }
    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    {
        let _ = is_valid;
        wasm32::get_local_zone()
    }
}

#[test]
fn test_get_zone() {
    assert!(get_local_zone().is_some());
}
