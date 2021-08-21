#![feature(test)]

#[cfg(test)]
mod tests {
    extern crate test;
    use geoip2::{City, Country, Reader};
    use std::{net::IpAddr, str::FromStr};
    use test::Bencher;

    #[bench]
    fn bench_country(b: &mut Bencher) {
        let buffer = std::fs::read("./testdata/GeoIP2-Country.mmdb").unwrap();
        let reader = Reader::<Country>::from_bytes(&buffer).unwrap();
        let ip = IpAddr::from_str("81.2.69.142").unwrap();
        b.iter(|| {
            reader.lookup(ip).unwrap();
        });
    }

    #[bench]
    fn bench_city(b: &mut Bencher) {
        let buffer = std::fs::read("./testdata/GeoIP2-City.mmdb").unwrap();
        let reader = Reader::<City>::from_bytes(&buffer).unwrap();
        let ip = IpAddr::from_str("81.2.69.142").unwrap();
        b.iter(|| {
            reader.lookup(ip).unwrap();
        });
    }

    #[bench]
    fn bench_country_oschwald(b: &mut Bencher) {
        let reader = maxminddb::Reader::open_readfile("./testdata/GeoIP2-Country.mmdb").unwrap();
        let ip = IpAddr::from_str("81.2.69.142").unwrap();
        b.iter(|| {
            reader.lookup::<maxminddb::geoip2::Country>(ip).unwrap();
        });
    }

    #[bench]
    fn bench_city_oschwald(b: &mut Bencher) {
        let reader = maxminddb::Reader::open_readfile("./testdata/GeoIP2-City.mmdb").unwrap();
        let ip = IpAddr::from_str("81.2.69.142").unwrap();
        b.iter(|| {
            reader.lookup::<maxminddb::geoip2::City>(ip).unwrap();
        });
    }
}
