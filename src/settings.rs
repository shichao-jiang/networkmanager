use crate::connection::Connection;
use crate::dbus_api::DBusAccessor;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerSettings;

const SETTINGS_BUS: &str = "org.freedesktop.NetworkManager";
const SETTINGS_PATH: &str = "/org/freedesktop/NetworkManager/Settings";

pub struct Settings {
    dbus_accessor: DBusAccessor,
}

impl Settings {
    pub(crate) fn new(dbus_accessor: DBusAccessor) -> Self {
        Self {
            dbus_accessor: dbus_accessor
                .with_bus_and_path(SETTINGS_BUS.into(), SETTINGS_PATH.into()),
        }
    }

    pub fn list_connections(&self) -> Result<impl Iterator<Item = Connection> + '_, Error> {
        Ok(proxy!(self)
            .list_connections()?
            .into_iter()
            .map(move |path| Connection::new(self.dbus_accessor.with_path(path))))
    }

    pub fn get_connection_by_uuid(&self, uuid: &str) -> Result<Connection, Error> {
        Ok(proxy!(self)
            .get_connection_by_uuid(uuid)
            .map(move |path| Connection::new(self.dbus_accessor.with_path(path)))?)
    }
}
