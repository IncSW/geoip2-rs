#[cfg(test)]
mod tests {
    use geoip2::{AnonymousIP, City, ConnectionType, Country, Domain, Error, Reader, ASN, ISP};
    use std::{net::IpAddr, str::FromStr};

    #[test]
    fn test_invalid_database_type() {
        let buffer = std::fs::read("./testdata/GeoIP2-Anonymous-IP-Test.mmdb").unwrap();
        let result = Reader::<Country>::from_bytes(&buffer);
        if let Err(Error::InvalidDatabaseType(msg)) = result {
            assert_eq!(msg, "GeoIP2-Anonymous-IP");
            return;
        }
        assert!(false);
    }

    #[test]
    fn test_anonymous_ip() {
        let buffer = std::fs::read("./testdata/GeoIP2-Anonymous-IP-Test.mmdb").unwrap();
        let reader = Reader::<AnonymousIP>::from_bytes(&buffer).unwrap();
        {
            let result = reader
                .lookup(IpAddr::from_str("81.2.69.0").unwrap())
                .unwrap();
            assert_eq!(result.is_anonymous, true);
            assert_eq!(result.is_anonymous_vpn, true);
            assert_eq!(result.is_hosting_provider, true);
            assert_eq!(result.is_public_proxy, true);
            assert_eq!(result.is_tor_exit_node, true);
            assert_eq!(result.is_residential_proxy, true);
        }
        {
            let result = reader
                .lookup(IpAddr::from_str("186.30.236.0").unwrap())
                .unwrap();
            assert_eq!(result.is_anonymous, true);
            assert_eq!(result.is_anonymous_vpn, false);
            assert_eq!(result.is_hosting_provider, false);
            assert_eq!(result.is_public_proxy, true);
            assert_eq!(result.is_tor_exit_node, false);
            assert_eq!(result.is_residential_proxy, false);
        }
    }

    // TestReaderZeroLength

    #[test]
    fn test_city() {
        let buffer = std::fs::read("./testdata/GeoIP2-City-Test.mmdb").unwrap();
        let reader = Reader::<City>::from_bytes(&buffer).unwrap();
        {
            let result = reader
                .lookup(IpAddr::from_str("81.2.69.142").unwrap())
                .unwrap();

            let city = result.city.unwrap();
            assert_eq!(city.geoname_id, 2643743);
            let names = city.names.unwrap();
            assert_eq!(names.get("de").unwrap(), "London");
            assert_eq!(names.get("es").unwrap(), "Londres");

            let location = result.location.unwrap();
            assert_eq!(location.accuracy_radius, 10);
            assert_eq!(location.latitude, 51.5142);
            assert_eq!(location.longitude, -0.0931);
            assert_eq!(location.time_zone.unwrap(), "Europe/London");

            let subdivisions = result.subdivisions.unwrap();
            assert_eq!(subdivisions.len(), 1);
            let subdivision = &subdivisions[0];
            assert_eq!(subdivision.geoname_id, 6269131);
            assert_eq!(subdivision.iso_code.unwrap(), "ENG");
            let names = subdivision.names.as_ref().unwrap();
            assert_eq!(names.get("en").unwrap(), "England");
            assert_eq!(names.get("pt-BR").unwrap(), "Inglaterra");
        }
        {
            let result = reader
                .lookup(IpAddr::from_str("2a02:ff80::").unwrap())
                .unwrap();

            assert!(result.city.is_none());

            let country = result.country.unwrap();
            assert_eq!(country.is_in_european_union, true);

            let location = result.location.unwrap();
            assert_eq!(location.accuracy_radius, 100);
            assert_eq!(location.latitude, 51.5);
            assert_eq!(location.longitude, 10.5);
            assert_eq!(location.time_zone.unwrap(), "Europe/Berlin");

            assert!(result.subdivisions.is_none());
        }
    }

    #[test]
    fn test_connection_type() {
        let buffer = std::fs::read("./testdata/GeoIP2-Connection-Type-Test.mmdb").unwrap();
        let reader = Reader::<ConnectionType>::from_bytes(&buffer).unwrap();
        {
            let result = reader
                .lookup(IpAddr::from_str("1.0.0.0").unwrap())
                .unwrap()
                .connection_type
                .unwrap();
            assert_eq!(result, "Dialup");
        }
        {
            let result = reader
                .lookup(IpAddr::from_str("1.0.1.0").unwrap())
                .unwrap()
                .connection_type
                .unwrap();
            assert_eq!(result, "Cable/DSL");
        }
    }

    #[test]
    fn test_country() {
        let buffer = std::fs::read("./testdata/GeoIP2-Country-Test.mmdb").unwrap();
        let reader = Reader::<Country>::from_bytes(&buffer).unwrap();
        {
            let result = reader
                .lookup(IpAddr::from_str("74.209.24.0").unwrap())
                .unwrap();
            let continent = result.continent.unwrap();
            assert_eq!(continent.geoname_id, 6255149);
            assert_eq!(continent.code.unwrap(), "NA");
            let names = continent.names.unwrap();
            assert_eq!(names.get("es").unwrap(), "Norteamérica");
            assert_eq!(names.get("ru").unwrap(), "Северная Америка");

            let country = result.country.unwrap();
            assert_eq!(country.geoname_id, 6252001);
            assert_eq!(country.iso_code.unwrap(), "US");
            let names = country.names.unwrap();
            assert_eq!(names.get("fr").unwrap(), "États-Unis");
            assert_eq!(names.get("pt-BR").unwrap(), "Estados Unidos");
            assert_eq!(country.is_in_european_union, false);

            let registered_country = result.registered_country.unwrap();
            assert_eq!(registered_country.geoname_id, 6252001);

            assert!(result.represented_country.is_none());

            let traits = result.traits.unwrap();
            assert_eq!(traits.is_anonymous_proxy, true);
            assert_eq!(traits.is_satellite_provider, true);
        }
        {
            let result = reader
                .lookup(IpAddr::from_str("2a02:ffc0::").unwrap())
                .unwrap();
            let continent = result.continent.unwrap();
            assert_eq!(continent.geoname_id, 6255148);
            assert_eq!(continent.code.unwrap(), "EU");
            let names = continent.names.unwrap();
            assert_eq!(names.get("en").unwrap(), "Europe");
            assert_eq!(names.get("zh-CN").unwrap(), "欧洲");

            let country = result.country.unwrap();
            assert_eq!(country.geoname_id, 2411586);
            assert_eq!(country.iso_code.unwrap(), "GI");
            let names = country.names.unwrap();
            assert_eq!(names.get("en").unwrap(), "Gibraltar");
            assert_eq!(names.get("ja").unwrap(), "ジブラルタル");
            assert_eq!(country.is_in_european_union, false);

            let registered_country = result.registered_country.unwrap();
            assert_eq!(registered_country.geoname_id, 2411586);

            assert!(result.represented_country.is_none());

            assert!(result.traits.is_none());
        }
    }

    #[test]
    fn test_domain() {
        let buffer = std::fs::read("./testdata/GeoIP2-Domain-Test.mmdb").unwrap();
        let reader = Reader::<Domain>::from_bytes(&buffer).unwrap();
        {
            let result = reader
                .lookup(IpAddr::from_str("1.2.0.0").unwrap())
                .unwrap()
                .domain
                .unwrap();
            assert_eq!(result, "maxmind.com");
        }
        {
            let result = reader
                .lookup(IpAddr::from_str("186.30.236.0").unwrap())
                .unwrap()
                .domain
                .unwrap();
            assert_eq!(result, "replaced.com");
        }
    }

    // TestEnterprise

    #[test]
    fn test_isp() {
        let buffer = std::fs::read("./testdata/GeoIP2-ISP-Test.mmdb").unwrap();
        let reader = Reader::<ISP>::from_bytes(&buffer).unwrap();
        {
            let result = reader
                .lookup(IpAddr::from_str("1.128.0.0").unwrap())
                .unwrap();
            assert_eq!(result.autonomous_system_number, 1221);
            assert_eq!(
                result.autonomous_system_organization.unwrap(),
                "Telstra Pty Ltd"
            );
            assert_eq!(result.isp.unwrap(), "Telstra Internet");
            assert_eq!(result.organization.unwrap(), "Telstra Internet");
            assert_eq!(
                result.autonomous_system_organization.unwrap(),
                "Telstra Pty Ltd"
            );
        }
        {
            let result = reader.lookup(IpAddr::from_str("4.0.0.0").unwrap()).unwrap();
            assert_eq!(result.autonomous_system_number, 0);
            assert!(result.autonomous_system_organization.is_none());
            assert_eq!(result.isp.unwrap(), "Level 3 Communications");
            assert_eq!(result.organization.unwrap(), "Level 3 Communications");
        }
    }

    #[test]
    fn test_asn() {
        let buffer = std::fs::read("./testdata/GeoLite2-ASN-Test.mmdb").unwrap();
        let reader = Reader::<ASN>::from_bytes(&buffer).unwrap();
        {
            let result = reader
                .lookup(IpAddr::from_str("1.128.0.0").unwrap())
                .unwrap();
            assert_eq!(result.autonomous_system_number, 1221);
            assert_eq!(
                result.autonomous_system_organization.unwrap(),
                "Telstra Pty Ltd"
            );
        }
        {
            let result = reader
                .lookup(IpAddr::from_str("2600:6000::").unwrap())
                .unwrap();
            assert_eq!(result.autonomous_system_number, 237);
            assert_eq!(
                result.autonomous_system_organization.unwrap(),
                "Merit Network Inc."
            );
        }
    }
}
