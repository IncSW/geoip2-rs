#[cfg(test)]
mod tests {
    use geoip2::{ASN, AnonymousIP, City, ConnectionType, Country, Domain, Enterprise, Error, ISP, Reader};
    use std::{net::IpAddr, str::FromStr};

    #[test]
    fn test_invalid_database_type() {
        let buffer = std::fs::read("./testdata/GeoIP2-Anonymous-IP-Test.mmdb").unwrap();
        let reader = Reader::<Country>::from_bytes(&buffer);
        if let Err(Error::InvalidDatabaseType(msg)) = reader {
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
            assert_eq!(result.is_anonymous, Some(true));
            assert_eq!(result.is_anonymous_vpn, Some(true));
            assert_eq!(result.is_hosting_provider, Some(true));
            assert_eq!(result.is_public_proxy, Some(true));
            assert_eq!(result.is_tor_exit_node, Some(true));
            assert_eq!(result.is_residential_proxy, Some(true));
        }
        {
            let result = reader
                .lookup(IpAddr::from_str("186.30.236.0").unwrap())
                .unwrap();
            assert_eq!(result.is_anonymous, Some(true));
            assert_eq!(result.is_anonymous_vpn, None);
            assert_eq!(result.is_hosting_provider, None);
            assert_eq!(result.is_public_proxy, Some(true));
            assert_eq!(result.is_tor_exit_node, None);
            assert_eq!(result.is_residential_proxy, None);
        }
    }

    #[test]
    fn test_enterprise() {
        let buffer = std::fs::read("./testdata/GeoIP2-Enterprise-Test.mmdb").unwrap();
        let reader = Reader::<Enterprise>::from_bytes(&buffer).unwrap();
        {
            let result = reader
                .lookup(IpAddr::from_str("74.209.24.0").unwrap())
                .unwrap();

            let city = result.city.unwrap();
            assert_eq!(city.confidence, Some(11));

            let country = result.country.unwrap();
            assert_eq!(country.confidence, Some(99));

            let postal = result.postal.unwrap();
            assert_eq!(postal.code, Some("12037"));
            assert_eq!(postal.confidence, Some(11));

            let subdivisions = result.subdivisions.unwrap();
            assert_eq!(subdivisions.len(), 1);
            let subdivision = &subdivisions[0];
            assert_eq!(subdivision.confidence, Some(93));

            let traits = result.traits.unwrap();
            assert_eq!(traits.autonomous_system_number, Some(14671));
            assert_eq!(
                traits.autonomous_system_organization,
                Some("FairPoint Communications")
            );
            assert_eq!(traits.isp, Some("Fairpoint Communications"));
            assert_eq!(traits.organization, Some("Fairpoint Communications"));
            assert_eq!(traits.connection_type, Some("Cable/DSL"));
            assert_eq!(traits.domain, Some("frpt.net"));
            assert_eq!(traits.static_ip_score, Some(0.34));
            assert_eq!(traits.user_type, Some("residential"));
        }
        {
            let result = reader
                .lookup(IpAddr::from_str("81.2.69.160").unwrap())
                .unwrap();

            let traits = result.traits.unwrap();
            assert_eq!(traits.isp, Some("Andrews & Arnold Ltd"));
            assert_eq!(traits.organization, Some("STONEHOUSE office network"));
            assert_eq!(traits.connection_type, Some("Corporate"));
            assert_eq!(traits.domain, Some("in-addr.arpa"));
            assert_eq!(traits.static_ip_score, Some(0.34));
            assert_eq!(traits.user_type, Some("government"));
        }
    }

    #[test]
    fn test_city() {
        let buffer = std::fs::read("./testdata/GeoIP2-City-Test.mmdb").unwrap();
        let reader = Reader::<City>::from_bytes(&buffer).unwrap();
        {
            let result = reader
                .lookup(IpAddr::from_str("81.2.69.142").unwrap())
                .unwrap();

            let city = result.city.unwrap();
            assert_eq!(city.geoname_id, Some(2643743));
            let names = city.names.unwrap();
            assert_eq!(names.get("de"), Some("London"));
            assert_eq!(names.get("es"), Some("Londres"));

            let location = result.location.unwrap();
            assert_eq!(location.accuracy_radius, Some(10));
            assert_eq!(location.latitude, Some(51.5142));
            assert_eq!(location.longitude, Some(-0.0931));
            assert_eq!(location.time_zone, Some("Europe/London"));

            let subdivisions = result.subdivisions.unwrap();
            assert_eq!(subdivisions.len(), 1);
            let subdivision = &subdivisions[0];
            assert_eq!(subdivision.geoname_id, Some(6269131));
            assert_eq!(subdivision.iso_code, Some("ENG"));
            let names = subdivision.names.as_ref().unwrap();
            assert_eq!(names.get("en"), Some("England"));
            assert_eq!(names.get("pt-BR"), Some("Inglaterra"));
        }
        {
            let result = reader
                .lookup(IpAddr::from_str("2a02:ff80::").unwrap())
                .unwrap();

            assert!(result.city.is_none());

            let country = result.country.unwrap();
            assert_eq!(country.is_in_european_union, Some(true));

            let location = result.location.unwrap();
            assert_eq!(location.accuracy_radius, Some(100));
            assert_eq!(location.latitude, Some(51.5));
            assert_eq!(location.longitude, Some(10.5));
            assert_eq!(location.time_zone, Some("Europe/Berlin"));

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
            assert_eq!(continent.geoname_id, Some(6255149));
            assert_eq!(continent.code, Some("NA"));
            let names = continent.names.unwrap();
            assert_eq!(names.get("es"), Some("Norteamérica"));
            assert_eq!(names.get("ru"), Some("Северная Америка"));

            let country = result.country.unwrap();
            assert_eq!(country.geoname_id, Some(6252001));
            assert_eq!(country.iso_code, Some("US"));
            let names = country.names.unwrap();
            assert_eq!(names.get("fr"), Some("États-Unis"));
            assert_eq!(names.get("pt-BR"), Some("Estados Unidos"));
            assert_eq!(country.is_in_european_union, None);

            let registered_country = result.registered_country.unwrap();
            assert_eq!(registered_country.geoname_id, Some(6252001));

            assert!(result.represented_country.is_none());

            let traits = result.traits.unwrap();
            assert_eq!(traits.is_anonymous_proxy, Some(true));
            assert_eq!(traits.is_satellite_provider, Some(true));
        }
        {
            let result = reader
                .lookup(IpAddr::from_str("2a02:ffc0::").unwrap())
                .unwrap();
            let continent = result.continent.unwrap();
            assert_eq!(continent.geoname_id, Some(6255148));
            assert_eq!(continent.code, Some("EU"));
            let names = continent.names.unwrap();
            assert_eq!(names.get("en"), Some("Europe"));
            assert_eq!(names.get("zh-CN"), Some("欧洲"));

            let country = result.country.unwrap();
            assert_eq!(country.geoname_id, Some(2411586));
            assert_eq!(country.iso_code, Some("GI"));
            let names = country.names.unwrap();
            assert_eq!(names.get("en"), Some("Gibraltar"));
            assert_eq!(names.get("ja"), Some("ジブラルタル"));
            assert_eq!(country.is_in_european_union, None);

            let registered_country = result.registered_country.unwrap();
            assert_eq!(registered_country.geoname_id, Some(2411586));

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

    #[test]
    fn test_isp() {
        let buffer = std::fs::read("./testdata/GeoIP2-ISP-Test.mmdb").unwrap();
        let reader = Reader::<ISP>::from_bytes(&buffer).unwrap();
        {
            let result = reader
                .lookup(IpAddr::from_str("1.128.0.0").unwrap())
                .unwrap();
            assert_eq!(result.autonomous_system_number, Some(1221));
            assert_eq!(
                result.autonomous_system_organization,
                Some("Telstra Pty Ltd")
            );
            assert_eq!(result.isp, Some("Telstra Internet"));
            assert_eq!(result.organization, Some("Telstra Internet"));
            assert_eq!(
                result.autonomous_system_organization,
                Some("Telstra Pty Ltd")
            );
        }
        {
            let result = reader.lookup(IpAddr::from_str("4.0.0.0").unwrap()).unwrap();
            assert_eq!(result.autonomous_system_number, None);
            assert_eq!(result.autonomous_system_organization, None);
            assert_eq!(result.isp, Some("Level 3 Communications"));
            assert_eq!(result.organization, Some("Level 3 Communications"));
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
            assert_eq!(result.autonomous_system_number, Some(1221));
            assert_eq!(
                result.autonomous_system_organization,
                Some("Telstra Pty Ltd")
            );
        }
        {
            let result = reader
                .lookup(IpAddr::from_str("2600:6000::").unwrap())
                .unwrap();
            assert_eq!(result.autonomous_system_number, Some(237));
            assert_eq!(
                result.autonomous_system_organization,
                Some("Merit Network Inc.")
            );
        }
    }
}
