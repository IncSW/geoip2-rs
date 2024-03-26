#[cfg(test)]
mod tests {
    use geoip2::{City, Country, Reader, ASN};
    use std::{net::IpAddr, str::FromStr};

    #[test]
    fn test_city() {
        let buffer = std::fs::read("./testdata/dbip-city-lite.mmdb").unwrap();
        let reader = Reader::<City>::from_bytes(&buffer).unwrap();

        let result = reader
            .lookup(IpAddr::from_str("66.30.184.198").unwrap())
            .unwrap();

        let city = result.city.unwrap();
        assert_eq!(city.geoname_id, None);
        let names = city.names.unwrap();
        assert_eq!(names.get("en"), Some("Medfield"));

        let location = result.location.unwrap();
        assert_eq!(location.latitude, Some(42.1876));
        assert_eq!(location.longitude, Some(-71.3065));

        let subdivisions = result.subdivisions.unwrap();
        assert_eq!(subdivisions.len(), 1);
        let subdivision = &subdivisions[0];
        let names = subdivision.names.as_ref().unwrap();
        assert_eq!(names.get("en"), Some("Massachusetts"));
    }

    #[test]
    fn test_country() {
        let buffer = std::fs::read("./testdata/dbip-country-lite.mmdb").unwrap();
        let reader = Reader::<Country>::from_bytes(&buffer).unwrap();

        let result = reader
            .lookup(IpAddr::from_str("66.30.184.198").unwrap())
            .unwrap();
        let continent = result.continent.unwrap();
        assert_eq!(continent.geoname_id, Some(6255149));
        assert_eq!(continent.code, Some("NA"));
        let names = continent.names.unwrap();
        assert_eq!(names.get("en"), Some("North America"));
        assert_eq!(names.get("ru"), Some("Северная Америка"));

        let country = result.country.unwrap();
        assert_eq!(country.geoname_id, Some(6252001));
        assert_eq!(country.iso_code, Some("US"));
        let names = country.names.unwrap();
        assert_eq!(names.get("fr"), Some("États-Unis"));
        assert_eq!(names.get("pt-BR"), Some("Estados Unidos"));
        assert_eq!(country.is_in_european_union, Some(false));
    }

    #[test]
    fn test_asn() {
        let buffer = std::fs::read("./testdata/dbip-asn-lite.mmdb").unwrap();
        let reader = Reader::<ASN>::from_bytes(&buffer).unwrap();

        let result = reader
            .lookup(IpAddr::from_str("66.30.184.198").unwrap())
            .unwrap();
        assert_eq!(result.autonomous_system_number, Some(7015));
        assert_eq!(
            result.autonomous_system_organization,
            Some("Comcast Cable Communications, LLC")
        );
    }
}
