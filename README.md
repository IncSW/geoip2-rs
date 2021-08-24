<div align="center">
	<h1>GeoIP2 Reader for Rust</h1>
	<p>
		<strong>This library reads MaxMind GeoIP2 databases</strong>
	</p>

[![Build Status](https://github.com/cristalhq/base64/workflows/build/badge.svg)](https://github.com/IncSW/geoip2/actions)
[![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)
![Downloads](https://img.shields.io/crates/d/geoip2.svg)

[![crates.io](https://img.shields.io/crates/v/geoip2?label=latest)](https://crates.io/crates/geoip2)
[![Documentation](https://docs.rs/geoip2/badge.svg?version=0.1.3)](https://docs.rs/geoip2/0.1.3)
[![Dependency Status](https://deps.rs/crate/geoip2/0.1.3/status.svg)](https://deps.rs/crate/geoip2/0.1.3)


</div>

## Usage

```toml
[dependencies]
geoip2 = "0.1.3"
```

See [examples/lookup.rs](examples/lookup.rs) for a basic example.

## Benchmarks

Benchmarks required `nightly` Rust.

Place `GeoIP2-Country.mmdb` and `GeoIP2-City.mmdb` in the `testdata` folder? then run:
```
cargo bench
```

Tested on paid DB on cargo 1.56.0-nightly, Intel i7-7700, Debian 9.1.

### [IncSW/geoip2-rs](https://github.com/IncSW/geoip2-rs)
`default`
```
city      2,175 ns/iter (+/- 124)
country   1,123 ns/iter (+/- 111)
```
`unsafe-str`
```
city      1,113 ns/iter (+/- 76)
country     524 ns/iter (+/- 31)
```

### [oschwald/maxminddb-rust](https://github.com/oschwald/maxminddb-rust).
`default`
```
city      4,224 ns/iter (+/- 153)
country   2,270 ns/iter (+/- 158)
```
`unsafe-str-decode`
```
city      3,266 ns/iter (+/- 191)
country   1,802 ns/iter (+/- 75)
```

## License

[MIT License](LICENSE).
