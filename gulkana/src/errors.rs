use std::fmt;
use std::io::ErrorKind;
pub enum SerializeError {
    Unknown,
}
#[derive(Clone)]
pub enum DBOperationError {
    KeyAllreadyPresent,
    KeyNotFound,
    NodeNotLink,
    NodeNotData,
    SerializeError,
    FSError,
    ParseError,
    FileNotFound,
    FilePermissionDenied,
    NetworkConnectionRefused,
    NetworkConnectionReset,
    NetworkNotConnected,
    NetworkAddressInUse,
    NetworkAddrNotAvailable,
    BrokenPipe,
    FileAlreadyExists,
    WouldBlock,
    InvalidInput,
    InvalidData,
    TimedOut,
    Interrupted,
    Other,
    UnexpectedEof,
}
impl DBOperationError {
    fn to_string(self) -> String {
        match self {
            Self::SerializeError => "failed seriailzing database".into(),
            Self::FSError => "Failed to write".into(),
            Self::KeyAllreadyPresent => "Key Allready Present".into(),
            Self::KeyNotFound => "Key Not found".into(),
            Self::NodeNotLink => "Node Not Link".into(),
            Self::NodeNotData => "Node Not Data".into(),
            Self::ParseError => "Parse Error".into(),
            Self::FileNotFound => "File Not Found".into(),
            Self::FilePermissionDenied => "File Permission Denied".into(),
            Self::NetworkConnectionRefused => "Network COnnection Refused".into(),
            Self::NetworkConnectionReset => "Network Connection Reset".into(),
            Self::NetworkNotConnected => "Network Not Connected".into(),
            Self::NetworkAddressInUse => "Network Address In Use".into(),
            Self::NetworkAddrNotAvailable => "Network Address Not Availible".into(),
            Self::BrokenPipe => "Broken Pipe".into(),
            Self::FileAlreadyExists => "File ALready Exists".into(),
            Self::WouldBlock => "Would Block".into(),
            Self::InvalidInput => "Invalid Input".into(),
            Self::InvalidData => "Invalid Data".into(),
            Self::TimedOut => "Timed Out".into(),
            Self::Interrupted => "Interrupted".into(),
            Self::Other => "Other".into(),
            Self::UnexpectedEof => "Unexpected End of File".into(),
        }
    }
}
impl Into<String> for DBOperationError {
    fn into(self) -> String {
        self.to_string()
    }
}

impl fmt::Display for DBOperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.clone().to_string())
    }
}
impl From<SerializeError> for DBOperationError {
    fn from(error: SerializeError) -> Self {
        match error {
            _ => Self::SerializeError,
        }
    }
}
impl From<serde_json::error::Error> for DBOperationError {
    fn from(error: serde_json::error::Error) -> Self {
        match error {
            _ => Self::ParseError,
        }
    }
}
impl From<std::io::Error> for DBOperationError {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            ErrorKind::NotFound => Self::FileNotFound,
            ErrorKind::PermissionDenied => Self::FilePermissionDenied,
            ErrorKind::ConnectionRefused => Self::NetworkConnectionRefused,
            ErrorKind::ConnectionReset => Self::NetworkConnectionReset,
            ErrorKind::NotConnected => Self::NetworkNotConnected,
            ErrorKind::AddrInUse => Self::NetworkAddressInUse,
            ErrorKind::AddrNotAvailable => Self::NetworkAddrNotAvailable,
            ErrorKind::BrokenPipe => Self::BrokenPipe,
            ErrorKind::AlreadyExists => Self::FileAlreadyExists,
            ErrorKind::WouldBlock => Self::WouldBlock,
            ErrorKind::InvalidInput => Self::InvalidInput,
            ErrorKind::InvalidData => Self::InvalidData,
            ErrorKind::TimedOut => Self::TimedOut,
            ErrorKind::Interrupted => Self::Interrupted,
            ErrorKind::Other => Self::Other,
            ErrorKind::UnexpectedEof => Self::UnexpectedEof,
            _ => Self::Other,
        }
    }
}
