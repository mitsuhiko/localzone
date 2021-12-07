# localzone

[![Build Status](https://github.com/mitsuhiko/localzone/workflows/Tests/badge.svg?branch=main)](https://github.com/mitsuhiko/localzone/actions?query=workflow%3ATests)
[![Crates.io](https://img.shields.io/crates/d/localzone.svg)](https://crates.io/crates/localzone)
[![License](https://img.shields.io/github/license/mitsuhiko/localzone)](https://github.com/mitsuhiko/localzone/blob/main/LICENSE)
[![Documentation](https://docs.rs/localzone/badge.svg)](https://docs.rs/localzone)

Utility crate to figure out the IANA (Olson) timezone of the current machine.
The IANA timezones have been largely established as the standard for
identifying timezones but retrieving the current timezone in that format is a
surprisingly complex problem.  Additionally Windows machines use an
incompatible timezone system and this crate helps automaticaly mapping these to
the IANA timezones.  For this it uses the CLDR maintained timezone mapping.

## License and Links

- [Documentation](https://docs.rs/localzone/)
- [Issue Tracker](https://github.com/mitsuhiko/localzone/issues)
- License: [Apache-2.0](https://github.com/mitsuhiko/localzone/blob/main/LICENSE)

