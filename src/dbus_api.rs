use dbus::{
    blocking::{Connection, Proxy},
    strings::BusName,
    Path,
};
use std::{fmt, rc::Rc, time::Duration};

const DBUS_TIMEOUT_MS: u64 = 5000;

macro_rules! proxy {
    ($input:ident) => {
        $input.dbus_accessor.create_proxy()
    };
}

#[derive(Clone)]
pub(crate) struct DBusAccessor {
    pub(crate) connection: Rc<Connection>,
    pub(crate) bus: BusName<'static>,
    pub(crate) path: Path<'static>,
}

impl DBusAccessor {
    pub(crate) fn new(connection: Rc<Connection>, bus: &str, path: &str) -> Self {
        DBusAccessor {
            connection: connection.clone(),
            bus: BusName::from(bus).into_static(),
            path: Path::from(path).into_static(),
        }
    }

    pub(crate) fn with_path(&self, path: Path<'_>) -> Self {
        DBusAccessor {
            connection: self.connection.clone(),
            bus: self.bus.clone(),
            path: path.into_static(),
        }
    }

    pub(crate) fn with_bus_and_path(&self, bus: BusName<'_>, path: Path<'_>) -> Self {
        DBusAccessor {
            connection: self.connection.clone(),
            bus: bus.into_static(),
            path: path.into_static(),
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
