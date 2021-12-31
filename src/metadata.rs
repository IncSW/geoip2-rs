use crate::decoder::{
    read_array, read_control, read_map, read_pointer, read_str, read_usize, Map, DATA_TYPE_MAP,
    DATA_TYPE_POINTER,
};
use crate::errors::Error;
use geoip2_codegen::Decoder;

const METADATA_START_MARKER: [u8; 14] = [
    0xAB, 0xCD, 0xEF, 0x4d, 0x61, 0x78, 0x4d, 0x69, 0x6e, 0x64, 0x2e, 0x63, 0x6f, 0x6d,
];

#[derive(Default, Debug, Decoder)]
pub struct Metadata<'a> {
    pub binary_format_major_version: u16,
    pub binary_format_minor_version: u16,
    pub node_count: u32,
    pub record_size: u16,
    pub ip_version: u16,
    pub database_type: &'a str,
    pub languages: Vec<&'a str>,
    pub build_epoch: u64,
    pub description: Map<'a>,
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
        None
    }
}
