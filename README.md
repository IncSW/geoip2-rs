[![crates.io](https://img.shields.io/crates/v/geoip2?label=latest)](https://crates.io/crates/geoip2)
[![Documentation](https://docs.rs/geoip2/badge.svg?version=0.0.1)](https://docs.rs/geoip2/0.0.1)
[![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)
[![Dependency Status](https://deps.rs/crate/geoip2/0.0.1/status.svg)](https://deps.rs/crate/geoip2/0.0.1)
![downloads](https://img.shields.io/crates/d/geoip2.svg)

# GeoIP2 Reader for Rust

This library reads MaxMind GeoIP2 databases.

## Usage

```toml
[dependencies]
geoip2 = "0.0.1"
```

See [examples/lookup.rs](examples/lookup.rs) for a basic example.

## Performance

cargo 1.56.0-nightly, Intel i7-7700

### [IncSW/geoip2-rs](https://github.com/IncSW/geoip2-rs)
```
city      1,189 ns/iter (+/- 73)
country     553 ns/iter (+/- 43)
```

### [oschwald/maxminddb-rust](https://github.com/oschwald/maxminddb-rust)
```
city      4,224 ns/iter (+/- 153)
country   2,311 ns/iter (+/- 75)
```

## License

[MIT License](LICENSE).
