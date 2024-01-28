use zbus::{zvariant::OwnedObjectPath, Connection};

/// A network device.
#[derive(Clone, Debug)]
pub struct ZDevice {
    pub(crate) zbus: Connection,
    pub(crate) path: OwnedObjectPath,
}
