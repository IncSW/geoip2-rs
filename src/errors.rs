#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidMetadata,
    InvalidRecordSize(u16),
    InvalidDatabaseType(String),
    InvalidSearchTreeSize,
    InvalidOffset,
    InvalidDataType(u8),
    InvalidNode,
    UnknownField(String),
    NotFound,
    IPv4Only,
    CorruptSearchTree,

    Utf8Error(std::str::Utf8Error),
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Error {
        Error::Utf8Error(err)
    }
}
