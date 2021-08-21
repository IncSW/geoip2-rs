use crate::decoder::{
    read_bool, read_control, read_f64, read_map, read_pointer, read_str, read_usize, Map,
    DATA_TYPE_MAP, DATA_TYPE_POINTER,
};
use crate::errors::Error;
use geoip2_codegen::Decoder;

#[derive(Default, Debug, Decoder)]
pub struct Continent<'a> {
    pub geoname_id: u32,
    pub code: Option<&'a str>,
    pub names: Option<Map<'a>>,
}

#[derive(Default, Debug, Decoder)]
pub struct Country<'a> {
    pub geoname_id: u32,
    pub iso_code: Option<&'a str>,
    pub names: Option<Map<'a>>,
    pub is_in_european_union: bool,
    pub represented_country_type: Option<&'a str>, // [RepresentedCountry]
    pub confidence: Option<u16>,                   // Enterprise [Country, RegisteredCountry]
}

#[derive(Default, Debug, Decoder)]
pub struct Subdivision<'a> {
    pub geoname_id: u32,
    pub iso_code: Option<&'a str>,
    pub names: Option<Map<'a>>,
    pub confidence: Option<u16>, // Enterprise
}

#[derive(Default, Debug, Decoder)]
pub struct City<'a> {
    pub geoname_id: u32,
    pub names: Option<Map<'a>>,
    pub confidence: Option<u16>, // Enterprise
}

#[derive(Default, Debug, Decoder)]
pub struct Location<'a> {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy_radius: u16,
    pub time_zone: Option<&'a str>,
    pub metro_code: u16,
}

#[derive(Default, Debug, Decoder)]
pub struct Postal<'a> {
    pub code: Option<&'a str>,
    pub confidence: Option<u16>, // Enterprise
}

#[derive(Default, Debug, Decoder)]
pub struct Traits<'a> {
    pub is_anonymous_proxy: bool,
    pub is_satellite_provider: bool,
    pub is_legitimate_proxy: Option<bool>,     // Enterprise
    pub static_ip_score: Option<f64>,          // Enterprise
    pub autonomous_system_number: Option<u32>, // Enterprise
    pub autonomous_system_organization: Option<&'a str>, // Enterprise
    pub isp: Option<&'a str>,                  // Enterprise
    pub organization: Option<&'a str>,         // Enterprise
    pub connection_type: Option<&'a str>,      // Enterprise
    pub domain: Option<&'a str>,               // Enterprise
    pub user_type: Option<&'a str>,            // Enterprise
}

#[derive(Default, Debug, Decoder)]
pub struct AnonymousIP {
    pub is_anonymous: Option<bool>,
    pub is_anonymous_vpn: Option<bool>,
    pub is_hosting_provider: Option<bool>,
    pub is_public_proxy: Option<bool>,
    pub is_tor_exit_node: Option<bool>,
    pub is_residential_proxy: Option<bool>,
}

#[derive(Default, Debug, Decoder)]
pub struct ASN<'a> {
    pub autonomous_system_number: Option<u32>,
    pub autonomous_system_organization: Option<&'a str>,
}

#[derive(Default, Debug, Decoder)]
pub struct Domain<'a> {
    pub domain: Option<&'a str>,
}
