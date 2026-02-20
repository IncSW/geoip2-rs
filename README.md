<div align="center">
	<h1>GeoIP2 Reader for Rust</h1>
	<p>
		<strong>This library reads MaxMind GeoIP2 databases</strong>
	</p>

[![Build Status](https://github.com/IncSW/geoip2-rs/workflows/build/badge.svg)](https://github.com/IncSW/geoip2-rs/actions)
[![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/geoip2.svg)](https://crates.io/crates/geoip2)

[![crates.io](https://img.shields.io/crates/v/geoip2?label=latest)](https://crates.io/crates/geoip2)
[![Documentation](https://docs.rs/geoip2/badge.svg?version=0.1.8)](https://docs.rs/geoip2/0.1.8)
[![Dependency Status](https://deps.rs/crate/geoip2/0.1.8/status.svg)](https://deps.rs/crate/geoip2/0.1.8)


</div>

## Usage

```toml
[dependencies]
geoip2 = "0.1.8"
```

See [examples/lookup.rs](examples/lookup.rs) for a basic example.

## Benchmarks

Benchmarks required `nightly` Rust.

Place `GeoIP2-Country.mmdb` and `GeoIP2-City.mmdb` in the `testdata` folder, then run:
```
cargo bench
```

Tested on paid DB on cargo 1.95.0-nightly, Ryzen 5 3600, Ubuntu 20.04.3 LTS.

### [IncSW/geoip2-rs](https://github.com/IncSW/geoip2-rs)
`default`
```
city      1,782.38 ns/iter (+/- 28.49)
country     908.45 ns/iter (+/- 7.62)
```
`unsafe-str`
```
city        999.06 ns/iter (+/- 8.87)
country     467.06 ns/iter (+/- 5.79)
```

### [oschwald/maxminddb-rust](https://github.com/oschwald/maxminddb-rust) 0.27.2.
`default`
```
city      1,915.71 ns/iter (+/- 29.17)
country     993.89 ns/iter (+/- 47.24)
```
`simdutf8`
```
city      1,870.60 ns/iter (+/- 32.75)
country     960.99 ns/iter (+/- 22.91)
```
`unsafe-str-decode`
```
city      1,708.32 ns/iter (+/- 27.23)
country     875.96 ns/iter (+/- 44.86)
```

## License

[MIT License](LICENSE).
