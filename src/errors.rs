#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DBus(dbus::Error),
    #[error("Unsupported method")]
    UnsupportedMethod,
    #[error("Unsupported device")]
    UnsupportedDevice,
    #[error("Unsupported type")]
    UnsupportedType,
}

impl From<dbus::Error> for Error {
    fn from(error: dbus::Error) -> Self {
        Error::DBus(error)
    }
}
