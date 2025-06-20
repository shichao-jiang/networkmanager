#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("DBus error: {0}")]
    ZBus(#[from] zbus::Error),
    #[error("Unsupported method")]
    UnsupportedMethod,
    #[error("Unsupported device")]
    UnsupportedDevice,
    #[error("Unsupported type")]
    UnsupportedType,
    #[error("Missing destination")]
    MissingDestination,
}
