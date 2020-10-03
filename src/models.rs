use crate::decoder::*;
use crate::errors::Error;

#[derive(Default, Debug)]
pub struct Continent<'a> {
    pub geoname_id: Option<u32>,
    pub code: Option<&'a str>,
    pub name: Option<&'a str>,
}

impl<'a> Continent<'a> {
    pub(crate) fn from_bytes(&mut self, buffer: &'a [u8], offset: &mut usize) -> Result<(), Error> {
        let (data_type, size) = read_control(buffer, offset)?;
        match data_type {
            DATA_TYPE_MAP => self.from_bytes_map(buffer, offset, size),
            DATA_TYPE_POINTER => {
                let mut offset = read_pointer(buffer, offset, size)?;
                let (data_type, size) = read_control(buffer, &mut offset)?;
                match data_type {
                    DATA_TYPE_MAP => self.from_bytes_map(buffer, &mut offset, size),
                    _ => return dbg!(Err(Error::InvalidDataType(data_type))),
                }
            }
            _ => return dbg!(Err(Error::InvalidDataType(data_type))),
        }
    }

    fn from_bytes_map(
        &mut self,
        buffer: &'a [u8],
        offset: &mut usize,
        size: usize,
    ) -> Result<(), Error> {
        for _ in 0..size {
            match read_str(buffer, offset)? {
                "geoname_id" => self.geoname_id = Some(read_usize(buffer, offset)? as u32),
                "code" => self.code = Some(read_str(buffer, offset)?),
                "names" => self.name = read_translation_from_map(buffer, offset, "en")?,
                field => return Err(Error::UnknownField(field.into())),
            }
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct Country<'a> {
    pub geoname_id: Option<u32>,
    pub iso_code: Option<&'a str>,
    pub name: Option<&'a str>,
    pub is_in_european_union: Option<bool>,
    pub represented_country_type: Option<&'a str>, // [RepresentedCountry]
    pub confidence: Option<u16>,                   // Enterprise [Country, RegisteredCountry]
}

impl<'a> Country<'a> {
    pub(crate) fn from_bytes(&mut self, buffer: &'a [u8], offset: &mut usize) -> Result<(), Error> {
        let (data_type, size) = read_control(buffer, offset)?;
        match data_type {
            DATA_TYPE_MAP => self.from_bytes_map(buffer, offset, size),
            DATA_TYPE_POINTER => {
                let mut offset = read_pointer(buffer, offset, size)?;
                let (data_type, size) = read_control(buffer, &mut offset)?;
                match data_type {
                    DATA_TYPE_MAP => self.from_bytes_map(buffer, &mut offset, size),
                    _ => return dbg!(Err(Error::InvalidDataType(data_type))),
                }
            }
            _ => return dbg!(Err(Error::InvalidDataType(data_type))),
        }
    }

    fn from_bytes_map(
        &mut self,
        buffer: &'a [u8],
        offset: &mut usize,
        size: usize,
    ) -> Result<(), Error> {
        for _ in 0..size {
            match read_str(buffer, offset)? {
                "geoname_id" => self.geoname_id = Some(read_usize(buffer, offset)? as u32),
                "iso_code" => self.iso_code = Some(read_str(buffer, offset)?),
                "names" => self.name = read_translation_from_map(buffer, offset, "en")?,
                "is_in_european_union" => {
                    self.is_in_european_union = Some(read_bool(buffer, offset)?)
                }
                "type" => self.represented_country_type = Some(read_str(buffer, offset)?),
                "confidence" => self.confidence = Some(read_usize(buffer, offset)? as u16),
                field => return Err(Error::UnknownField(field.into())),
            }
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct Subdivision<'a> {
    pub geoname_id: Option<u32>,
    pub iso_code: Option<&'a str>,
    pub name: Option<&'a str>,
    pub confidence: Option<u16>, // Enterprise
}

impl<'a> Subdivision<'a> {
    pub(crate) fn from_bytes(&mut self, buffer: &'a [u8], offset: &mut usize) -> Result<(), Error> {
        let (data_type, size) = read_control(buffer, offset)?;
        match data_type {
            DATA_TYPE_MAP => self.from_bytes_map(buffer, offset, size),
            DATA_TYPE_POINTER => {
                let mut offset = read_pointer(buffer, offset, size)?;
                let (data_type, size) = read_control(buffer, &mut offset)?;
                match data_type {
                    DATA_TYPE_MAP => self.from_bytes_map(buffer, &mut offset, size),
                    _ => return dbg!(Err(Error::InvalidDataType(data_type))),
                }
            }
            _ => return dbg!(Err(Error::InvalidDataType(data_type))),
        }
    }

    fn from_bytes_map(
        &mut self,
        buffer: &'a [u8],
        offset: &mut usize,
        size: usize,
    ) -> Result<(), Error> {
        for _ in 0..size {
            match read_str(buffer, offset)? {
                "geoname_id" => self.geoname_id = Some(read_usize(buffer, offset)? as u32),
                "iso_code" => self.iso_code = Some(read_str(buffer, offset)?),
                "names" => self.name = read_translation_from_map(buffer, offset, "en")?,
                "confidence" => self.confidence = Some(read_usize(buffer, offset)? as u16),
                field => return Err(Error::UnknownField(field.into())),
            }
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct City<'a> {
    pub geoname_id: Option<u32>,
    pub name: Option<&'a str>,
    pub confidence: Option<u16>, // Enterprise
}

impl<'a> City<'a> {
    pub(crate) fn from_bytes(&mut self, buffer: &'a [u8], offset: &mut usize) -> Result<(), Error> {
        let (data_type, size) = read_control(buffer, offset)?;
        match data_type {
            DATA_TYPE_MAP => self.from_bytes_map(buffer, offset, size),
            DATA_TYPE_POINTER => {
                let mut offset = read_pointer(buffer, offset, size)?;
                let (data_type, size) = read_control(buffer, &mut offset)?;
                match data_type {
                    DATA_TYPE_MAP => self.from_bytes_map(buffer, &mut offset, size),
                    _ => return dbg!(Err(Error::InvalidDataType(data_type))),
                }
            }
            _ => return dbg!(Err(Error::InvalidDataType(data_type))),
        }
    }

    fn from_bytes_map(
        &mut self,
        buffer: &'a [u8],
        offset: &mut usize,
        size: usize,
    ) -> Result<(), Error> {
        for _ in 0..size {
            match read_str(buffer, offset)? {
                "geoname_id" => self.geoname_id = Some(read_usize(buffer, offset)? as u32),
                "names" => self.name = read_translation_from_map(buffer, offset, "en")?,
                "confidence" => self.confidence = Some(read_usize(buffer, offset)? as u16),
                field => return Err(Error::UnknownField(field.into())),
            }
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct Location<'a> {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy_radius: u16,
    pub time_zone: Option<&'a str>,
    pub metro_code: u16,
}

impl<'a> Location<'a> {
    pub(crate) fn from_bytes(&mut self, buffer: &'a [u8], offset: &mut usize) -> Result<(), Error> {
        let (data_type, size) = read_control(buffer, offset)?;
        match data_type {
            DATA_TYPE_MAP => self.from_bytes_map(buffer, offset, size),
            DATA_TYPE_POINTER => {
                let mut offset = read_pointer(buffer, offset, size)?;
                let (data_type, size) = read_control(buffer, &mut offset)?;
                match data_type {
                    DATA_TYPE_MAP => self.from_bytes_map(buffer, &mut offset, size),
                    _ => return dbg!(Err(Error::InvalidDataType(data_type))),
                }
            }
            _ => return dbg!(Err(Error::InvalidDataType(data_type))),
        }
    }

    fn from_bytes_map(
        &mut self,
        buffer: &'a [u8],
        offset: &mut usize,
        size: usize,
    ) -> Result<(), Error> {
        for _ in 0..size {
            match read_str(buffer, offset)? {
                "latitude" => self.latitude = read_f64(buffer, offset)?,
                "longitude" => self.longitude = read_f64(buffer, offset)?,
                "accuracy_radius" => self.accuracy_radius = read_usize(buffer, offset)? as u16,
                "time_zone" => self.time_zone = Some(read_str(buffer, offset)?),
                "metro_code" => self.metro_code = read_usize(buffer, offset)? as u16,
                field => return Err(Error::UnknownField(field.into())),
            }
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct Postal<'a> {
    pub code: Option<&'a str>,
    pub confidence: Option<u16>, // Enterprise
}

impl<'a> Postal<'a> {
    pub(crate) fn from_bytes(&mut self, buffer: &'a [u8], offset: &mut usize) -> Result<(), Error> {
        let (data_type, size) = read_control(buffer, offset)?;
        match data_type {
            DATA_TYPE_MAP => self.from_bytes_map(buffer, offset, size),
            DATA_TYPE_POINTER => {
                let mut offset = read_pointer(buffer, offset, size)?;
                let (data_type, size) = read_control(buffer, &mut offset)?;
                match data_type {
                    DATA_TYPE_MAP => self.from_bytes_map(buffer, &mut offset, size),
                    _ => return dbg!(Err(Error::InvalidDataType(data_type))),
                }
            }
            _ => return dbg!(Err(Error::InvalidDataType(data_type))),
        }
    }

    fn from_bytes_map(
        &mut self,
        buffer: &'a [u8],
        offset: &mut usize,
        size: usize,
    ) -> Result<(), Error> {
        for _ in 0..size {
            match read_str(buffer, offset)? {
                "code" => self.code = Some(read_str(buffer, offset)?),
                "confidence" => self.confidence = Some(read_usize(buffer, offset)? as u16),
                field => return Err(Error::UnknownField(field.into())),
            }
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
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

impl<'a> Traits<'a> {
    pub(crate) fn from_bytes(&mut self, buffer: &'a [u8], offset: &mut usize) -> Result<(), Error> {
        let (data_type, size) = read_control(buffer, offset)?;
        match data_type {
            DATA_TYPE_MAP => self.from_bytes_map(buffer, offset, size),
            DATA_TYPE_POINTER => {
                let mut offset = read_pointer(buffer, offset, size)?;
                let (data_type, size) = read_control(buffer, &mut offset)?;
                match data_type {
                    DATA_TYPE_MAP => self.from_bytes_map(buffer, &mut offset, size),
                    _ => return dbg!(Err(Error::InvalidDataType(data_type))),
                }
            }
            _ => return dbg!(Err(Error::InvalidDataType(data_type))),
        }
    }

    fn from_bytes_map(
        &mut self,
        buffer: &'a [u8],
        offset: &mut usize,
        size: usize,
    ) -> Result<(), Error> {
        for _ in 0..size {
            match read_str(buffer, offset)? {
                "is_anonymous_proxy" => self.is_anonymous_proxy = read_bool(buffer, offset)?,
                "is_satellite_provider" => self.is_satellite_provider = read_bool(buffer, offset)?,
                "is_legitimate_proxy" => {
                    self.is_legitimate_proxy = Some(read_bool(buffer, offset)?)
                }
                "static_ip_score" => self.static_ip_score = Some(read_f64(buffer, offset)?),
                "autonomous_system_number" => {
                    self.autonomous_system_number = Some(read_usize(buffer, offset)? as u32)
                }
                "autonomous_system_organization" => {
                    self.autonomous_system_organization = Some(read_str(buffer, offset)?)
                }
                "isp" => self.isp = Some(read_str(buffer, offset)?),
                "organization" => self.organization = Some(read_str(buffer, offset)?),
                "connection_type" => self.connection_type = Some(read_str(buffer, offset)?),
                "domain" => self.domain = Some(read_str(buffer, offset)?),
                "user_type" => self.user_type = Some(read_str(buffer, offset)?),
                field => return Err(Error::UnknownField(field.into())),
            }
        }
        Ok(())
    }
}
