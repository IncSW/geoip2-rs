use crate::decoder::{
    read_bool, read_control, read_f64, read_map, read_pointer, read_str, read_usize, Map,
    DATA_TYPE_MAP, DATA_TYPE_POINTER,
};
use crate::errors::Error;
use geoip2_codegen::Decoder;

#[derive(Default, Debug, Decoder)]
pub struct Continent<'a> {
    pub geoname_id: Option<u32>,
    pub code: Option<&'a str>,
    pub names: Option<Map<'a>>,
}

#[derive(Default, Debug, Decoder)]
pub struct Country<'a> {
    pub geoname_id: Option<u32>,
    pub iso_code: Option<&'a str>,
    pub names: Option<Map<'a>>,
    pub is_in_european_union: Option<bool>,
}

#[derive(Default, Debug, Decoder)]
pub struct EnterpriseCountry<'a> {
    pub geoname_id: Option<u32>,
    pub iso_code: Option<&'a str>,
    pub names: Option<Map<'a>>,
    pub is_in_european_union: Option<bool>,
    pub confidence: Option<u16>,
}

#[derive(Default, Debug, Decoder)]
pub struct RepresentedCountry<'a> {
    pub geoname_id: Option<u32>,
    pub iso_code: Option<&'a str>,
    pub names: Option<Map<'a>>,
    pub is_in_european_union: Option<bool>,
    pub country_type: Option<&'a str>,
}

#[derive(Default, Debug, Decoder)]
pub struct EnterpriseRepresentedCountry<'a> {
    pub geoname_id: Option<u32>,
    pub iso_code: Option<&'a str>,
    pub names: Option<Map<'a>>,
    pub is_in_european_union: Option<bool>,
    pub country_type: Option<&'a str>,
    pub confidence: Option<u16>,
}

#[derive(Default, Debug, Decoder)]
pub struct Subdivision<'a> {
    pub geoname_id: Option<u32>,
    pub iso_code: Option<&'a str>,
    pub names: Option<Map<'a>>,
}

#[derive(Default, Debug, Decoder)]
pub struct EnterpriseSubdivision<'a> {
    pub geoname_id: Option<u32>,
    pub iso_code: Option<&'a str>,
    pub names: Option<Map<'a>>,
    pub confidence: Option<u16>,
}

#[derive(Default, Debug, Decoder)]
pub struct City<'a> {
    pub geoname_id: Option<u32>,
    pub names: Option<Map<'a>>,
}

#[derive(Default, Debug, Decoder)]
pub struct EnterpriseCity<'a> {
    pub geoname_id: Option<u32>,
    pub names: Option<Map<'a>>,
    pub confidence: Option<u16>,
}

#[derive(Default, Debug, Decoder)]
pub struct Location<'a> {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub accuracy_radius: Option<u16>,
    pub time_zone: Option<&'a str>,
    pub metro_code: Option<u16>,
}

#[derive(Default, Debug, Decoder)]
pub struct Postal<'a> {
    pub code: Option<&'a str>,
}

#[derive(Default, Debug, Decoder)]
pub struct EnterprisePostal<'a> {
    pub code: Option<&'a str>,
    pub confidence: Option<u16>,
}

#[derive(Default, Debug, Decoder)]
pub struct Traits {
    pub is_anonymous_proxy: Option<bool>,
    pub is_satellite_provider: Option<bool>,
}

#[derive(Default, Debug, Decoder)]
pub struct EnterpriseTraits<'a> {
    pub is_anonymous_proxy: Option<bool>,
    pub is_satellite_provider: Option<bool>,
    pub is_legitimate_proxy: Option<bool>,
    pub static_ip_score: Option<f64>,
    pub autonomous_system_number: Option<u32>,
    pub autonomous_system_organization: Option<&'a str>,
    pub isp: Option<&'a str>,
    pub organization: Option<&'a str>,
    pub connection_type: Option<&'a str>,
    pub domain: Option<&'a str>,
    pub user_type: Option<&'a str>,
}
