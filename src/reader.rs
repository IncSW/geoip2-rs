use std::marker::PhantomData;
use std::net::IpAddr;

use crate::decoder::{
    read_bool, read_control, read_pointer, read_str, read_usize, DATA_TYPE_MAP, DATA_TYPE_POINTER,
    DATA_TYPE_SLICE,
};
use crate::errors::Error;
use crate::metadata::Metadata;
use crate::models;
use geoip2_codegen::reader;

const DATA_SECTION_SEPARATOR_SIZE: usize = 16;

pub struct Reader<'a, T> {
    t: PhantomData<&'a T>,
    pub(crate) metadata: Metadata<'a>,
    pub(crate) decoder_buffer: &'a [u8],
    node_buffer: &'a [u8],
    node_offset_mult: usize,
    ip_v4_start: usize,
    ip_v4_start_bit_depth: usize,
}

impl<'a, T> Reader<'a, T> {
    fn from_bytes_raw(buffer: &'a [u8]) -> Result<Reader<'a, T>, Error> {
        let mut metadata_start = match Metadata::find_start(buffer) {
            Some(index) => index,
            None => return Err(Error::InvalidMetadata),
        };
        let mut metadata = Metadata::default();
        metadata.from_bytes(buffer, &mut metadata_start)?;
        if metadata.record_size != 24 && metadata.record_size != 28 && metadata.record_size != 32 {
            return Err(Error::InvalidRecordSize(metadata.record_size));
        }
        let node_offset_mult = (metadata.record_size as usize) / 4;
        let search_tree_size = (metadata.node_count as usize) * node_offset_mult;
        let data_section_start = search_tree_size + DATA_SECTION_SEPARATOR_SIZE;
        if data_section_start > metadata_start {
            return Err(Error::InvalidSearchTreeSize);
        }
        let mut reader = Reader {
            t: PhantomData,
            metadata,
            decoder_buffer: &buffer[data_section_start..metadata_start],
            node_buffer: &buffer[..search_tree_size],
            node_offset_mult,
            ip_v4_start: 0,
            ip_v4_start_bit_depth: 0,
        };
        if reader.metadata.ip_version == 6 {
            let mut node = 0usize;
            let mut i = 0usize;
            while i < 96 && node < reader.metadata.node_count as usize {
                i += 1;
                node = reader.read_left(node * node_offset_mult)
            }
            reader.ip_v4_start = node;
            reader.ip_v4_start_bit_depth = i;
        }
        Ok(reader)
    }

    fn find_address_in_tree(&self, ip: &[u8]) -> Result<usize, Error> {
        let bit_count = ip.len() * 8;
        let mut node: usize = if bit_count == 128 {
            0
        } else {
            self.ip_v4_start
        };
        let node_count = self.metadata.node_count as usize;
        for i in 0..bit_count {
            if node >= node_count {
                break;
            }
            let bit = 1 & (ip[i >> 3] >> (7 - (i % 8)));
            let offset = node * self.node_offset_mult;
            node = if bit == 0 {
                self.read_left(offset)
            } else {
                self.read_right(offset)
            }
        }
        match node_count {
            n if n == node => Ok(0),
            n if node > n => Ok(node),
            _ => Err(Error::InvalidNode),
        }
    }

    fn read_left(&self, node_number: usize) -> usize {
        match self.metadata.record_size {
            28 => {
                (((self.node_buffer[node_number + 3] as usize) & 0xF0) << 20)
                    | ((self.node_buffer[node_number] as usize) << 16)
                    | ((self.node_buffer[node_number + 1] as usize) << 8)
                    | (self.node_buffer[node_number + 2] as usize)
            }
            24 => {
                ((self.node_buffer[node_number] as usize) << 16)
                    | ((self.node_buffer[node_number + 1] as usize) << 8)
                    | (self.node_buffer[node_number + 2] as usize)
            }
            32 => {
                ((self.node_buffer[node_number] as usize) << 24)
                    | ((self.node_buffer[node_number + 1] as usize) << 16)
                    | ((self.node_buffer[node_number + 2] as usize) << 8)
                    | (self.node_buffer[node_number + 3] as usize)
            }
            _ => panic!(),
        }
    }

    fn read_right(&self, node_number: usize) -> usize {
        match self.metadata.record_size {
            28 => {
                (((self.node_buffer[node_number + 3] as usize) & 0x0F) << 24)
                    | ((self.node_buffer[node_number + 4] as usize) << 16)
                    | ((self.node_buffer[node_number + 5] as usize) << 8)
                    | (self.node_buffer[node_number + 6] as usize)
            }
            24 => {
                ((self.node_buffer[node_number + 3] as usize) << 16)
                    | ((self.node_buffer[node_number + 4] as usize) << 8)
                    | (self.node_buffer[node_number + 5] as usize)
            }
            32 => {
                ((self.node_buffer[node_number + 4] as usize) << 24)
                    | ((self.node_buffer[node_number + 5] as usize) << 16)
                    | ((self.node_buffer[node_number + 6] as usize) << 8)
                    | (self.node_buffer[node_number + 7] as usize)
            }
            _ => panic!(),
        }
    }

    fn lookup_pointer(&self, address: IpAddr) -> Result<usize, Error> {
        let ip_bytes = match address {
            IpAddr::V4(ip) => ip.octets().to_vec(),
            IpAddr::V6(ip) => {
                if self.metadata.ip_version == 4 {
                    return Err(Error::IPv4Only);
                }
                ip.octets().to_vec()
            }
        };
        let pointer = self.find_address_in_tree(&ip_bytes)?;
        if pointer == 0 {
            return Err(Error::NotFound);
        }
        Ok(pointer)
    }

    fn get_offset(&self, address: IpAddr) -> Result<usize, Error> {
        let pointer = self.lookup_pointer(address)?;
        let offset = pointer - self.metadata.node_count as usize - DATA_SECTION_SEPARATOR_SIZE;
        if offset >= self.decoder_buffer.len() {
            return Err(Error::CorruptSearchTree);
        }
        Ok(offset)
    }
}

#[reader("GeoIP2-Country", "GeoLite2-Country", "DBIP-Country-Lite")]
#[derive(Default, Debug)]
pub struct Country<'a> {
    pub continent: Option<models::Continent<'a>>,
    pub country: Option<models::Country<'a>>,
    pub registered_country: Option<models::Country<'a>>,
    pub represented_country: Option<models::RepresentedCountry<'a>>,
    pub traits: Option<models::Traits>,
}

#[reader("GeoIP2-City", "GeoLite2-City", "DBIP-City-Lite")]
#[derive(Default, Debug)]
pub struct City<'a> {
    pub continent: Option<models::Continent<'a>>,
    pub country: Option<models::Country<'a>>,
    pub subdivisions: Option<Vec<models::Subdivision<'a>>>,
    pub city: Option<models::City<'a>>,
    pub location: Option<models::Location<'a>>,
    pub postal: Option<models::Postal<'a>>,
    pub registered_country: Option<models::Country<'a>>,
    pub represented_country: Option<models::RepresentedCountry<'a>>,
    pub traits: Option<models::Traits>,
}

#[reader("GeoIP2-Enterprise")]
#[derive(Default, Debug)]
pub struct Enterprise<'a> {
    pub continent: Option<models::Continent<'a>>,
    pub country: Option<models::EnterpriseCountry<'a>>,
    pub subdivisions: Option<Vec<models::EnterpriseSubdivision<'a>>>,
    pub city: Option<models::EnterpriseCity<'a>>,
    pub location: Option<models::Location<'a>>,
    pub postal: Option<models::EnterprisePostal<'a>>,
    pub registered_country: Option<models::EnterpriseCountry<'a>>,
    pub represented_country: Option<models::EnterpriseRepresentedCountry<'a>>,
    pub traits: Option<models::EnterpriseTraits<'a>>,
}

#[reader("GeoIP2-ISP")]
#[derive(Default, Debug)]
pub struct ISP<'a> {
    pub autonomous_system_number: Option<u32>,
    pub autonomous_system_organization: Option<&'a str>,
    pub isp: Option<&'a str>,
    pub organization: Option<&'a str>,
}

#[reader("GeoIP2-Connection-Type")]
#[derive(Default, Debug)]
pub struct ConnectionType<'a> {
    pub connection_type: Option<&'a str>,
}

#[reader("GeoIP2-Anonymous-IP")]
#[derive(Default, Debug)]
pub struct AnonymousIP {
    pub is_anonymous: Option<bool>,
    pub is_anonymous_vpn: Option<bool>,
    pub is_hosting_provider: Option<bool>,
    pub is_public_proxy: Option<bool>,
    pub is_tor_exit_node: Option<bool>,
    pub is_residential_proxy: Option<bool>,
}

#[reader("GeoLite2-ASN", "DBIP-ASN-Lite", "DBIP-ASN-Lite (compat=GeoLite2-ASN)")]
#[derive(Default, Debug)]
pub struct ASN<'a> {
    pub autonomous_system_number: Option<u32>,
    pub autonomous_system_organization: Option<&'a str>,
}

#[reader("GeoIP2-Domain")]
#[derive(Default, Debug)]
pub struct Domain<'a> {
    pub domain: Option<&'a str>,
}
