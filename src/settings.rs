use std::collections::HashMap;

use zbus::zvariant::Value;

use crate::{connection::Connection, errors::Error};

/// Connection Settings Profile Manager.
pub struct Settings {
    zbus: zbus::blocking::Connection,
}

crate::zproxy_unpathed!(Settings, crate::raw::settings::SettingsProxyBlocking<'_>);

impl Settings {
    /// Create a new NetworkManager instance with a custom D-Bus connection.
    pub(crate) fn new_with_zbus(zbus: zbus::blocking::Connection) -> Self {
        Self { zbus }
    }

    /// List the saved network connections known to NetworkManager.
    pub fn list_connections(&self) -> Result<impl Iterator<Item = Connection> + '_, Error> {
        Ok(self
            .raw()?
            .list_connections()?
            .into_iter()
            .map(|path| Connection {
                zbus: self.zbus.clone(),
                path,
            }))
    }

    /// Retrieve the object path of a connection, given that connection's UUID.
    pub fn get_connection_by_uuid(&self, uuid: &str) -> Result<Connection, Error> {
        let path = self.raw()?.get_connection_by_uuid(uuid)?;
        Ok(Connection {
            zbus: self.zbus.clone(),
            path,
        })
    }

    /// Add a new connection and save it to disk.
    ///
    /// This operation does not start the network connection unless:
    /// - the device is idle and able to connect to the network described by the new connection, and
    /// - the connection is allowed to be started automatically.
    pub fn add_connection(
        &self,
        properties: HashMap<&str, HashMap<&str, Value<'_>>>,
    ) -> Result<Connection, Error> {
        let path = self.raw()?.add_connection(properties)?;
        Ok(Connection {
            zbus: self.zbus.clone(),
            path,
        })
    }

    // TODO: AddConnectionUnsaved()
    // TODO: AddConnection2()
    // TODO: LoadConnections()
    // TODO: ReloadConnections()
    // TODO: SaveHostname()

    /// The machine hostname stored in persistent configuration.
    pub fn hostname(&self) -> Result<String, Error> {
        self.raw()?.hostname().map_err(Error::ZBus)
    }

    /// Whether adding and modifying connections is supported.
    pub fn can_modify(&self) -> Result<bool, Error> {
        self.raw()?.can_modify().map_err(Error::ZBus)
    }

    // TODO: from 1.44
    // /// The version of the settings.
    // ///
    // /// This is incremented whenever the profile changes and can be used to detect concurrent modifications.
    // pub  fn version(&self) -> Result<u32, Error> {
    //     self.raw().await?.version().await.map_err(Error::ZBus)
    // }
}
