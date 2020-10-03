use std::marker::PhantomData;
use std::net::IpAddr;

use crate::decoder::*;
use crate::errors::Error;
use crate::metadata::Metadata;
use crate::models;

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
        let metadata_start = match Metadata::find_start(buffer) {
            Some(index) => index,
            None => return Err(Error::InvalidMetadata),
        };
        let metadata = Metadata::from_bytes(&buffer[metadata_start..])?;
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
            metadata: metadata,
            decoder_buffer: &buffer[data_section_start..metadata_start],
            node_buffer: &buffer[..search_tree_size],
            node_offset_mult: node_offset_mult,
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

    pub fn lookup_pointer(&self, address: IpAddr) -> Result<usize, Error> {
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

    pub fn get_offset(&self, address: IpAddr) -> Result<usize, Error> {
        let pointer = self.lookup_pointer(address)?;
        let offset = pointer - self.metadata.node_count as usize - DATA_SECTION_SEPARATOR_SIZE;
        if offset >= self.decoder_buffer.len() {
            return Err(Error::CorruptSearchTree);
        }
        Ok(offset)
    }
}

#[derive(Default, Debug)]
pub struct Country<'a> {
    pub continent: Option<models::Continent<'a>>,
    pub country: Option<models::Country<'a>>,
    pub registered_country: Option<models::Country<'a>>,
    pub represented_country: Option<models::Country<'a>>,
    pub traits: Option<models::Traits<'a>>,
}

impl<'a> Reader<'a, Country<'a>> {
    pub fn from_bytes(buffer: &[u8]) -> Result<Reader<Country>, Error> {
        let reader = Reader::from_bytes_raw(buffer)?;
        if reader.metadata.database_type != "GeoIP2-Country"
            && reader.metadata.database_type != "GeoLite2-Country"
        {
            return Err(Error::InvalidDatabaseType(
                reader.metadata.database_type.into(),
            ));
        }
        Ok(reader)
    }

    pub fn lookup(&self, address: IpAddr) -> Result<Country, Error> {
        let mut offset = self.get_offset(address)?;
        let (data_type, size) = read_control(self.decoder_buffer, &mut offset)?;
        if data_type != DATA_TYPE_MAP {
            return dbg!(Err(Error::InvalidDataType(data_type)));
        }
        let mut result = Country::default();
        for _ in 0..size {
            match read_str(self.decoder_buffer, &mut offset)? {
                "continent" => {
                    let mut model = models::Continent::default();
                    model.from_bytes(self.decoder_buffer, &mut offset)?;
                    result.continent = Some(model);
                }
                "country" => {
                    let mut model = models::Country::default();
                    model.from_bytes(self.decoder_buffer, &mut offset)?;
                    result.country = Some(model);
                }
                "registered_country" => {
                    let mut model = models::Country::default();
                    model.from_bytes(self.decoder_buffer, &mut offset)?;
                    result.registered_country = Some(model);
                }
                "represented_country" => {
                    let mut model = models::Country::default();
                    model.from_bytes(self.decoder_buffer, &mut offset)?;
                    result.represented_country = Some(model);
                }
                "traits" => {
                    let mut model = models::Traits::default();
                    model.from_bytes(self.decoder_buffer, &mut offset)?;
                    result.traits = Some(model);
                }
                field => return Err(Error::UnknownField(field.into())),
            }
        }
        Ok(result)
    }
}

#[derive(Default, Debug)]
pub struct City<'a> {
    pub continent: Option<models::Continent<'a>>,
    pub country: Option<models::Country<'a>>,
    pub subdivisions: Option<Vec<models::Subdivision<'a>>>,
    pub city: Option<models::City<'a>>,
    pub location: Option<models::Location<'a>>,
    pub postal: Option<models::Postal<'a>>,
    pub registered_country: Option<models::Country<'a>>,
    pub represented_country: Option<models::Country<'a>>,
    pub traits: Option<models::Traits<'a>>,
}

pub type Enterprise<'a> = City<'a>;

impl<'a> Reader<'a, City<'a>> {
    pub fn from_bytes(buffer: &[u8]) -> Result<Reader<City>, Error> {
        let reader = Reader::from_bytes_raw(buffer)?;
        if reader.metadata.database_type != "GeoIP2-City"
            && reader.metadata.database_type != "GeoLite2-City"
            && reader.metadata.database_type != "GeoIP2-Enterprise"
        {
            return Err(Error::InvalidDatabaseType(
                reader.metadata.database_type.into(),
            ));
        }
        Ok(reader)
    }

    pub fn lookup(&self, address: IpAddr) -> Result<City, Error> {
        let mut offset = self.get_offset(address)?;
        let (data_type, size) = read_control(self.decoder_buffer, &mut offset)?;
        if data_type != DATA_TYPE_MAP {
            return dbg!(Err(Error::InvalidDataType(data_type)));
        }
        let mut result = City::default();
        for _ in 0..size {
            match read_str(self.decoder_buffer, &mut offset)? {
                "continent" => {
                    let mut model = models::Continent::default();
                    model.from_bytes(self.decoder_buffer, &mut offset)?;
                    result.continent = Some(model);
                }
                "country" => {
                    let mut model = models::Country::default();
                    model.from_bytes(self.decoder_buffer, &mut offset)?;
                    result.country = Some(model);
                }
                "registered_country" => {
                    let mut model = models::Country::default();
                    model.from_bytes(self.decoder_buffer, &mut offset)?;
                    result.registered_country = Some(model);
                }
                "represented_country" => {
                    let mut model = models::Country::default();
                    model.from_bytes(self.decoder_buffer, &mut offset)?;
                    result.represented_country = Some(model);
                }
                "city" => {
                    let mut model = models::City::default();
                    model.from_bytes(self.decoder_buffer, &mut offset)?;
                    result.city = Some(model);
                }
                "location" => {
                    let mut model = models::Location::default();
                    model.from_bytes(self.decoder_buffer, &mut offset)?;
                    result.location = Some(model);
                }
                "postal" => {
                    let mut model = models::Postal::default();
                    model.from_bytes(self.decoder_buffer, &mut offset)?;
                    result.postal = Some(model);
                }
                "subdivisions" => {
                    let (data_type, size) = read_control(self.decoder_buffer, &mut offset)?;
                    result.subdivisions = Some(match data_type {
                        DATA_TYPE_SLICE => {
                            let mut array: Vec<models::Subdivision<'a>> = Vec::with_capacity(size);
                            for _ in 0..size {
                                let mut model = models::Subdivision::default();
                                model.from_bytes(self.decoder_buffer, &mut offset)?;
                                array.push(model);
                            }
                            array
                        }
                        DATA_TYPE_POINTER => {
                            let mut offset = read_pointer(self.decoder_buffer, &mut offset, size)?;
                            let (data_type, size) = read_control(self.decoder_buffer, &mut offset)?;
                            match data_type {
                                DATA_TYPE_SLICE => {
                                    let mut array: Vec<models::Subdivision<'a>> =
                                        Vec::with_capacity(size);
                                    for _ in 0..size {
                                        let mut model = models::Subdivision::default();
                                        model.from_bytes(self.decoder_buffer, &mut offset)?;
                                        array.push(model);
                                    }
                                    array
                                }
                                _ => return dbg!(Err(Error::InvalidDataType(data_type))),
                            }
                        }
                        _ => return dbg!(Err(Error::InvalidDataType(data_type))),
                    })
                }
                "traits" => {
                    let mut model = models::Traits::default();
                    model.from_bytes(self.decoder_buffer, &mut offset)?;
                    result.traits = Some(model);
                }
                field => return Err(Error::UnknownField(field.into())),
            }
        }
        Ok(result)
    }
}

#[derive(Default, Debug)]
pub struct ISP<'a> {
    pub autonomous_system_number: u32,
    pub autonomous_system_organization: Option<&'a str>,
    pub isp: Option<&'a str>,
    pub organization: Option<&'a str>,
}

impl<'a> Reader<'a, ISP<'a>> {
    pub fn from_bytes(buffer: &[u8]) -> Result<Reader<ISP>, Error> {
        let reader = Reader::from_bytes_raw(buffer)?;
        if reader.metadata.database_type != "GeoIP2-ISP" {
            return Err(Error::InvalidDatabaseType(
                reader.metadata.database_type.into(),
            ));
        }
        Ok(reader)
    }

    pub fn lookup(&self, address: IpAddr) -> Result<ISP, Error> {
        let mut offset = self.get_offset(address)?;
        let (data_type, size) = read_control(self.decoder_buffer, &mut offset)?;
        if data_type != DATA_TYPE_MAP {
            return dbg!(Err(Error::InvalidDataType(data_type)));
        }
        let mut result = ISP::default();
        for _ in 0..size {
            match read_str(self.decoder_buffer, &mut offset)? {
                "autonomous_system_number" => {
                    result.autonomous_system_number =
                        read_usize(self.decoder_buffer, &mut offset)? as u32
                }
                "autonomous_system_organization" => {
                    result.autonomous_system_organization =
                        Some(read_str(self.decoder_buffer, &mut offset)?)
                }
                "isp" => result.isp = Some(read_str(self.decoder_buffer, &mut offset)?),
                "organization" => {
                    result.organization = Some(read_str(self.decoder_buffer, &mut offset)?)
                }
                field => return Err(Error::UnknownField(field.into())),
            }
        }
        Ok(result)
    }
}

#[derive(Default, Debug)]
pub struct ConnectionType<'a> {
    pub connection_type: Option<&'a str>,
}

impl<'a> Reader<'a, ConnectionType<'a>> {
    pub fn from_bytes(buffer: &[u8]) -> Result<Reader<ConnectionType>, Error> {
        let reader = Reader::from_bytes_raw(buffer)?;
        if reader.metadata.database_type != "GeoIP2-Connection-Type" {
            return Err(Error::InvalidDatabaseType(
                reader.metadata.database_type.into(),
            ));
        }
        Ok(reader)
    }

    pub fn lookup(&self, address: IpAddr) -> Result<ConnectionType, Error> {
        let mut offset = self.get_offset(address)?;
        let (data_type, size) = read_control(self.decoder_buffer, &mut offset)?;
        if data_type != DATA_TYPE_MAP {
            return dbg!(Err(Error::InvalidDataType(data_type)));
        }
        let mut result = ConnectionType::default();
        for _ in 0..size {
            match read_str(self.decoder_buffer, &mut offset)? {
                "connection_type" => {
                    result.connection_type = Some(read_str(self.decoder_buffer, &mut offset)?)
                }
                field => return Err(Error::UnknownField(field.into())),
            }
        }
        Ok(result)
    }
}

#[derive(Default, Debug)]
pub struct AnonymousIP {
    pub is_anonymous: bool,
    pub is_anonymous_vpn: bool,
    pub is_hosting_provider: bool,
    pub is_public_proxy: bool,
    pub is_tor_exit_node: bool,
}

impl<'a> Reader<'a, AnonymousIP> {
    pub fn from_bytes(buffer: &[u8]) -> Result<Reader<AnonymousIP>, Error> {
        let reader = Reader::from_bytes_raw(buffer)?;
        if reader.metadata.database_type != "GeoIP2-Anonymous-IP" {
            return Err(Error::InvalidDatabaseType(
                reader.metadata.database_type.into(),
            ));
        }
        Ok(reader)
    }

    pub fn lookup(&self, address: IpAddr) -> Result<AnonymousIP, Error> {
        let mut offset = self.get_offset(address)?;
        let (data_type, size) = read_control(self.decoder_buffer, &mut offset)?;
        if data_type != DATA_TYPE_MAP {
            return dbg!(Err(Error::InvalidDataType(data_type)));
        }
        let mut result = AnonymousIP::default();
        for _ in 0..size {
            match read_str(self.decoder_buffer, &mut offset)? {
                "is_anonymous" => {
                    result.is_anonymous = read_bool(self.decoder_buffer, &mut offset)?
                }
                "is_anonymous_vpn" => {
                    result.is_anonymous_vpn = read_bool(self.decoder_buffer, &mut offset)?
                }
                "is_hosting_provider" => {
                    result.is_hosting_provider = read_bool(self.decoder_buffer, &mut offset)?
                }
                "is_public_proxy" => {
                    result.is_public_proxy = read_bool(self.decoder_buffer, &mut offset)?
                }
                "is_tor_exit_node" => {
                    result.is_tor_exit_node = read_bool(self.decoder_buffer, &mut offset)?
                }
                field => return Err(Error::UnknownField(field.into())),
            }
        }
        Ok(result)
    }
}

#[derive(Default, Debug)]
pub struct ASN<'a> {
    pub autonomous_system_number: u32,
    pub autonomous_system_organization: Option<&'a str>,
}

impl<'a> Reader<'a, ASN<'a>> {
    pub fn from_bytes(buffer: &[u8]) -> Result<Reader<ASN>, Error> {
        let reader = Reader::from_bytes_raw(buffer)?;
        if reader.metadata.database_type != "GeoLite2-ASN" {
            return Err(Error::InvalidDatabaseType(
                reader.metadata.database_type.into(),
            ));
        }
        Ok(reader)
    }

    pub fn lookup(&self, address: IpAddr) -> Result<ASN, Error> {
        let mut offset = self.get_offset(address)?;
        let (data_type, size) = read_control(self.decoder_buffer, &mut offset)?;
        if data_type != DATA_TYPE_MAP {
            return dbg!(Err(Error::InvalidDataType(data_type)));
        }
        let mut result = ASN::default();
        for _ in 0..size {
            match read_str(self.decoder_buffer, &mut offset)? {
                "autonomous_system_number" => {
                    result.autonomous_system_number =
                        read_usize(self.decoder_buffer, &mut offset)? as u32
                }
                "autonomous_system_organization" => {
                    result.autonomous_system_organization =
                        Some(read_str(self.decoder_buffer, &mut offset)?)
                }
                field => return Err(Error::UnknownField(field.into())),
            }
        }
        Ok(result)
    }
}
