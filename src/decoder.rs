use crate::errors::Error;

pub(crate) const DATA_TYPE_EXTENDED: u8 = 0;
pub(crate) const DATA_TYPE_POINTER: u8 = 1;
pub(crate) const DATA_TYPE_STRING: u8 = 2;
pub(crate) const DATA_TYPE_FLOAT64: u8 = 3;
// pub(crate) const DATA_TYPE_BYTES: u8 = 4;
pub(crate) const DATA_TYPE_UINT16: u8 = 5;
pub(crate) const DATA_TYPE_UINT32: u8 = 6;
pub(crate) const DATA_TYPE_MAP: u8 = 7;
pub(crate) const DATA_TYPE_INT32: u8 = 8;
pub(crate) const DATA_TYPE_UINT64: u8 = 9;
pub(crate) const DATA_TYPE_UINT128: u8 = 10;
pub(crate) const DATA_TYPE_SLICE: u8 = 11;
// pub(crate) const DATA_TYPE_DATA_CACHE_CONTAINER: u8 = 12;
// pub(crate) const DATA_TYPE_END_MARKER: u8 = 13;
pub(crate) const DATA_TYPE_BOOL: u8 = 14;
// pub(crate) const DATA_TYPE_FLOAT32: u8 = 15;

pub(crate) fn read_bytes<'a>(
    buffer: &'a [u8],
    offset: &mut usize,
    size: usize,
) -> Result<&'a [u8], Error> {
    let new_offset = *offset + size;
    if new_offset > buffer.len() {
        return Err(Error::InvalidOffset);
    }
    let bytes = &buffer[*offset..new_offset];
    *offset = new_offset;
    Ok(bytes)
}

#[inline]
pub(crate) fn read_control(buffer: &[u8], offset: &mut usize) -> Result<(u8, usize), Error> {
    let control_byte = buffer[*offset];
    *offset += 1;
    let mut data_type = control_byte >> 5;
    if data_type == DATA_TYPE_EXTENDED {
        data_type = buffer[*offset] + 7;
        *offset += 1;
    }
    let mut size = (control_byte as usize) & 0x1f;
    if data_type == DATA_TYPE_EXTENDED || size < 29 {
        return Ok((data_type, size));
    }
    let bytes_to_read = size - 28;
    size = bytes_to_usize(read_bytes(buffer, offset, bytes_to_read)?);
    size += match bytes_to_read {
        1 => 29,
        2 => 285,
        _ => 65_821,
    };
    Ok((data_type, size))
}

pub(crate) fn read_pointer(buffer: &[u8], offset: &mut usize, size: usize) -> Result<usize, Error> {
    let pointer_size = ((size >> 3) & 0x3) + 1;
    let mut prefix = 0usize;
    if pointer_size != 4 {
        prefix = size & 0x7
    }
    let unpacked = bytes_to_usize_with_prefix(prefix, read_bytes(buffer, offset, pointer_size)?);
    let pointer_value_offset = match pointer_size {
        2 => 2048,
        3 => 526_336,
        _ => 0,
    };
    Ok(unpacked + pointer_value_offset)
}

pub(crate) fn read_usize(buffer: &[u8], offset: &mut usize) -> Result<usize, Error> {
    let (data_type, size) = read_control(buffer, offset)?;
    match data_type {
        DATA_TYPE_UINT16 | DATA_TYPE_UINT32 | DATA_TYPE_INT32 | DATA_TYPE_UINT64
        | DATA_TYPE_UINT128 => Ok(bytes_to_usize(read_bytes(buffer, offset, size)?)),
        DATA_TYPE_POINTER => {
            let offset = &mut read_pointer(buffer, offset, size)?;
            let (data_type, size) = read_control(buffer, offset)?;
            match data_type {
                DATA_TYPE_UINT16 | DATA_TYPE_UINT32 | DATA_TYPE_INT32 | DATA_TYPE_UINT64
                | DATA_TYPE_UINT128 => Ok(bytes_to_usize(read_bytes(buffer, offset, size)?)),
                _ => Err(Error::InvalidDataType(data_type)),
            }
        }
        _ => Err(Error::InvalidDataType(data_type)),
    }
}

pub(crate) fn read_bool(buffer: &[u8], offset: &mut usize) -> Result<bool, Error> {
    let (data_type, size) = read_control(buffer, offset)?;
    match data_type {
        DATA_TYPE_BOOL => Ok(size != 0),
        DATA_TYPE_POINTER => {
            let offset = &mut read_pointer(buffer, offset, size)?;
            let (data_type, size) = read_control(buffer, offset)?;
            match data_type {
                DATA_TYPE_BOOL => Ok(size != 0),
                _ => Err(Error::InvalidDataType(data_type)),
            }
        }
        _ => Err(Error::InvalidDataType(data_type)),
    }
}

pub(crate) fn read_f64(buffer: &[u8], offset: &mut usize) -> Result<f64, Error> {
    let (data_type, size) = read_control(buffer, offset)?;
    match data_type {
        DATA_TYPE_FLOAT64 => Ok(f64::from_bits(
            bytes_to_usize(read_bytes(buffer, offset, size)?) as u64,
        )),
        DATA_TYPE_POINTER => {
            let offset = &mut read_pointer(buffer, offset, size)?;
            let (data_type, size) = read_control(buffer, offset)?;
            match data_type {
                DATA_TYPE_FLOAT64 => Ok(f64::from_bits(bytes_to_usize(read_bytes(
                    buffer, offset, size,
                )?) as u64)),
                _ => Err(Error::InvalidDataType(data_type)),
            }
        }
        _ => Err(Error::InvalidDataType(data_type)),
    }
}

#[cfg(feature = "unsafe-str")]
pub(crate) fn read_str<'a>(buffer: &'a [u8], offset: &mut usize) -> Result<&'a str, Error> {
    let (data_type, size) = read_control(buffer, offset)?;
    match data_type {
        DATA_TYPE_STRING => {
            Ok(unsafe { std::str::from_utf8_unchecked(read_bytes(buffer, offset, size)?) })
        }
        DATA_TYPE_POINTER => {
            let offset = &mut read_pointer(buffer, offset, size)?;
            let (data_type, size) = read_control(buffer, offset)?;
            match data_type {
                DATA_TYPE_STRING => {
                    Ok(unsafe { std::str::from_utf8_unchecked(read_bytes(buffer, offset, size)?) })
                }
                _ => Err(Error::InvalidDataType(data_type)),
            }
        }
        _ => Err(Error::InvalidDataType(data_type)),
    }
}

#[cfg(not(feature = "unsafe-str"))]
pub(crate) fn read_str<'a>(buffer: &'a [u8], offset: &mut usize) -> Result<&'a str, Error> {
    let (data_type, size) = read_control(buffer, offset)?;
    match data_type {
        DATA_TYPE_STRING => Ok(std::str::from_utf8(read_bytes(buffer, offset, size)?)?),
        DATA_TYPE_POINTER => {
            let offset = &mut read_pointer(buffer, offset, size)?;
            let (data_type, size) = read_control(buffer, offset)?;
            match data_type {
                DATA_TYPE_STRING => Ok(std::str::from_utf8(read_bytes(buffer, offset, size)?)?),
                _ => Err(Error::InvalidDataType(data_type)),
            }
        }
        _ => Err(Error::InvalidDataType(data_type)),
    }
}

#[derive(Default, Debug)]
pub struct Map<'a>(Vec<(&'a str, &'a str)>);

impl<'a> Map<'a> {
    pub fn get(&self, key: &'a str) -> Option<&'a str> {
        for tp in self.0.iter() {
            if tp.0 == key {
                return Some(tp.1);
            }
        }
        None
    }

    pub fn iter(&'a self) -> std::slice::Iter<'a, (&'a str, &'a str)> {
        self.0.iter()
    }
}

pub(crate) fn read_map<'a>(buffer: &'a [u8], offset: &mut usize) -> Result<Map<'a>, Error> {
    let (data_type, size) = read_control(buffer, offset)?;
    match data_type {
        DATA_TYPE_MAP => {
            let mut map = Vec::with_capacity(size);
            for _ in 0..size {
                map.push((read_str(buffer, offset)?, read_str(buffer, offset)?));
            }
            Ok(Map(map))
        }
        DATA_TYPE_POINTER => {
            let offset = &mut read_pointer(buffer, offset, size)?;
            let (data_type, size) = read_control(buffer, offset)?;
            match data_type {
                DATA_TYPE_MAP => {
                    let mut map = Vec::with_capacity(size);
                    for _ in 0..size {
                        map.push((read_str(buffer, offset)?, read_str(buffer, offset)?));
                    }
                    Ok(Map(map))
                }
                _ => Err(Error::InvalidDataType(data_type)),
            }
        }
        _ => Err(Error::InvalidDataType(data_type)),
    }
}

pub(crate) fn read_array<'a>(buffer: &'a [u8], offset: &mut usize) -> Result<Vec<&'a str>, Error> {
    let (data_type, size) = read_control(buffer, offset)?;
    match data_type {
        DATA_TYPE_SLICE => {
            let mut array = Vec::with_capacity(size);
            for _ in 0..size {
                array.push(read_str(buffer, offset)?);
            }
            Ok(array)
        }
        DATA_TYPE_POINTER => {
            let offset = &mut read_pointer(buffer, offset, size)?;
            let (data_type, size) = read_control(buffer, offset)?;
            match data_type {
                DATA_TYPE_SLICE => {
                    let mut array = Vec::with_capacity(size);
                    for _ in 0..size {
                        array.push(read_str(buffer, offset)?);
                    }
                    Ok(array)
                }
                _ => Err(Error::InvalidDataType(data_type)),
            }
        }
        _ => Err(Error::InvalidDataType(data_type)),
    }
}

pub(crate) fn bytes_to_usize(buffer: &[u8]) -> usize {
    match buffer.len() {
        1 => buffer[0] as usize,
        2 => (buffer[0] as usize) << 8 | (buffer[1] as usize),
        3 => ((buffer[0] as usize) << 8 | (buffer[1] as usize)) << 8 | (buffer[2] as usize),
        4 => {
            (((buffer[0] as usize) << 8 | (buffer[1] as usize)) << 8 | (buffer[2] as usize)) << 8
                | (buffer[3] as usize)
        }
        5 => {
            ((((buffer[0] as usize) << 8 | (buffer[1] as usize)) << 8 | (buffer[2] as usize)) << 8
                | (buffer[3] as usize))
                << 8
                | (buffer[4] as usize)
        }
        6 => {
            (((((buffer[0] as usize) << 8 | (buffer[1] as usize)) << 8 | (buffer[2] as usize))
                << 8
                | (buffer[3] as usize))
                << 8
                | (buffer[4] as usize))
                << 8
                | (buffer[5] as usize)
        }
        7 => {
            ((((((buffer[0] as usize) << 8 | (buffer[1] as usize)) << 8 | (buffer[2] as usize))
                << 8
                | (buffer[3] as usize))
                << 8
                | (buffer[4] as usize))
                << 8
                | (buffer[5] as usize))
                << 8
                | (buffer[6] as usize)
        }
        8 => {
            (((((((buffer[0] as usize) << 8 | (buffer[1] as usize)) << 8 | (buffer[2] as usize))
                << 8
                | (buffer[3] as usize))
                << 8
                | (buffer[4] as usize))
                << 8
                | (buffer[5] as usize))
                << 8
                | (buffer[6] as usize))
                << 8
                | (buffer[7] as usize)
        }
        _ => 0,
    }
}

fn bytes_to_usize_with_prefix(prefix: usize, buffer: &[u8]) -> usize {
    match buffer.len() {
        0 => prefix,
        1 => prefix << 8 | (buffer[0] as usize),
        2 => (prefix << 8 | (buffer[0] as usize)) << 8 | (buffer[1] as usize),
        3 => {
            ((prefix << 8 | (buffer[0] as usize)) << 8 | (buffer[1] as usize)) << 8
                | (buffer[2] as usize)
        }
        4 => {
            (((prefix << 8 | (buffer[0] as usize)) << 8 | (buffer[1] as usize)) << 8
                | (buffer[2] as usize))
                << 8
                | (buffer[3] as usize)
        }
        5 => {
            ((((prefix << 8 | (buffer[0] as usize)) << 8 | (buffer[1] as usize)) << 8
                | (buffer[2] as usize))
                << 8
                | (buffer[3] as usize))
                << 8
                | (buffer[4] as usize)
        }
        6 => {
            (((((prefix << 8 | (buffer[0] as usize)) << 8 | (buffer[1] as usize)) << 8
                | (buffer[2] as usize))
                << 8
                | (buffer[3] as usize))
                << 8
                | (buffer[4] as usize))
                << 8
                | (buffer[5] as usize)
        }
        7 => {
            ((((((prefix << 8 | (buffer[0] as usize)) << 8 | (buffer[1] as usize)) << 8
                | (buffer[2] as usize))
                << 8
                | (buffer[3] as usize))
                << 8
                | (buffer[4] as usize))
                << 8
                | (buffer[5] as usize))
                << 8
                | (buffer[6] as usize)
        }
        8 => {
            (((((((prefix << 8 | (buffer[0] as usize)) << 8 | (buffer[1] as usize)) << 8
                | (buffer[2] as usize))
                << 8
                | (buffer[3] as usize))
                << 8
                | (buffer[4] as usize))
                << 8
                | (buffer[5] as usize))
                << 8
                | (buffer[6] as usize))
                << 8
                | (buffer[7] as usize)
        }
        _ => 0,
    }
}
