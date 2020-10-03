use crate::decoder::*;
use crate::errors::Error;

const METADATA_START_MARKER: [u8; 14] = [
    0xAB, 0xCD, 0xEF, 0x4d, 0x61, 0x78, 0x4d, 0x69, 0x6e, 0x64, 0x2e, 0x63, 0x6f, 0x6d,
];

#[derive(Default, Debug)]
pub(crate) struct Metadata<'a> {
    binary_format_major_version: u16,
    binary_format_minor_version: u16,
    pub(crate) node_count: u32,
    pub(crate) record_size: u16,
    pub(crate) ip_version: u16,
    pub(crate) database_type: &'a str,
    languages: Vec<&'a str>,
    build_epoch: u64,
    description: Option<&'a str>,
}

impl<'a> Metadata<'a> {
    pub(crate) fn find_start(buffer: &[u8]) -> Option<usize> {
        if buffer.len() < 14 {
            return None;
        }
        let mut i = buffer.len() - 14;
        while i != 0 {
            i -= 1;
            if buffer[i] == METADATA_START_MARKER[0]
                && buffer[i + 13] == METADATA_START_MARKER[13]
                && buffer[i..i + 14] == METADATA_START_MARKER
            {
                return Some(i + 14);
            }
        }
        return None;
    }

    pub(crate) fn from_bytes(buffer: &'a [u8]) -> Result<Metadata, Error> {
        let mut offset = 0usize;
        let (data_type, size) = read_control(buffer, &mut offset)?;
        if data_type != DATA_TYPE_MAP {
            return dbg!(Err(Error::InvalidDataType(data_type)));
        }
        let mut metadata = Metadata::default();
        for _ in 0..size {
            match read_str(buffer, &mut offset)? {
                "binary_format_major_version" => {
                    metadata.binary_format_major_version = read_usize(buffer, &mut offset)? as u16
                }
                "binary_format_minor_version" => {
                    metadata.binary_format_minor_version = read_usize(buffer, &mut offset)? as u16
                }
                "build_epoch" => metadata.build_epoch = read_usize(buffer, &mut offset)? as u64,
                "database_type" => metadata.database_type = read_str(buffer, &mut offset)?,
                "description" => metadata.description = read_translation_from_map(buffer,&mut  offset, "en")?,
                "ip_version" => metadata.ip_version = read_usize(buffer, &mut offset)? as u16,
                "languages" => metadata.languages = read_array(buffer, &mut offset)?,
                "node_count" => metadata.node_count = read_usize(buffer, &mut offset)? as u32,
                "record_size" => metadata.record_size = read_usize(buffer, &mut offset)? as u16,
                field => return Err(Error::UnknownField(field.into())),
            }
        }
        Ok(metadata)
    }
}
