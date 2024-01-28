use dbus::blocking::{Connection, Proxy};
use std::{fmt, sync::Arc, time::Duration};

const DBUS_TIMEOUT_MS: u64 = 5000;

macro_rules! proxy {
    ($input:ident) => {
        $input.dbus_accessor.create_proxy()
    };
}

#[derive(Clone)]
pub(crate) struct DBusAccessor {
    pub(crate) connection: Arc<Connection>,
    pub(crate) bus: String,
    pub(crate) path: String,
}

impl DBusAccessor {
    pub(crate) fn new(connection: Arc<Connection>, bus: &str, path: &str) -> Self {
        DBusAccessor {
            connection: connection.clone(),
            bus: bus.to_owned(),
            path: path.to_owned(),
        }
    }

    pub(crate) fn create_proxy(&self) -> Proxy<'_, &Connection> {
        self.connection.with_proxy(
            &self.bus,
            &self.path,
            Duration::from_millis(DBUS_TIMEOUT_MS),
        )
    }
}

impl fmt::Debug for DBusAccessor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DBusAccessor")
            .field("bus", &self.bus)
            .field("path", &self.path)
            .finish_non_exhaustive()
    }
}