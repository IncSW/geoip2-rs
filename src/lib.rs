#![feature(test)]

mod decoder;
mod errors;
mod metadata;
pub mod models;
mod reader;

pub use reader::{AnonymousIP, City, ConnectionType, Country, Enterprise, Reader, ASN, ISP};

#[cfg(test)]
mod tests {
    extern crate maxminddb;
    extern crate test;
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};
    use test::Bencher;

    #[test]
    fn test_country() {
        let buffer = std::fs::read("./testdata/GeoIP2-Country.mmdb").unwrap();
        let reader = Reader::<Country>::from_bytes(&buffer).unwrap();
        let ip: IpAddr = Ipv4Addr::new(81, 2, 69, 142).into();
        let result = reader.lookup(ip).unwrap();
        println!("{:#?}", &result);
    }

    #[test]
    fn test_city() {
        let buffer = std::fs::read("./testdata/GeoIP2-City.mmdb").unwrap();
        let reader = Reader::<City>::from_bytes(&buffer).unwrap();
        let ip: IpAddr = Ipv4Addr::new(81, 2, 69, 142).into();
        let result = reader.lookup(ip).unwrap();
        println!("{:#?}", &result);
    }

    #[bench]
    fn bench_country(b: &mut Bencher) {
        let buffer = std::fs::read("./testdata/GeoIP2-Country.mmdb").unwrap();
        let reader = Reader::<Country>::from_bytes(&buffer).unwrap();
        let ip: IpAddr = Ipv4Addr::new(81, 2, 69, 142).into();
        b.iter(|| {
            reader.lookup(ip).unwrap();
        });
    }

    #[bench]
    fn bench_city(b: &mut Bencher) {
        let buffer = std::fs::read("./testdata/GeoIP2-City.mmdb").unwrap();
        let reader = Reader::<City>::from_bytes(&buffer).unwrap();
        let ip: IpAddr = Ipv4Addr::new(81, 2, 69, 142).into();
        b.iter(|| {
            reader.lookup(ip).unwrap();
        });
    }

    #[bench]
    fn bench_city_oschwald(b: &mut Bencher) {
        let reader = maxminddb::Reader::open_readfile("./testdata/GeoIP2-City.mmdb").unwrap();
        let ip: IpAddr = Ipv4Addr::new(81, 2, 69, 142).into();
        b.iter(|| {
            reader.lookup::<maxminddb::geoip2::City>(ip).unwrap();
        });
    }

    #[bench]
    fn bench_connection_type(b: &mut Bencher) {
        let buffer = std::fs::read("./testdata/GeoIP2-Connection-Type.mmdb").unwrap();
        let reader = Reader::<ConnectionType>::from_bytes(&buffer).unwrap();
        let ip: IpAddr = Ipv4Addr::new(81, 2, 69, 142).into();
        b.iter(|| {
            reader.lookup(ip).unwrap();
        });
    }
}
