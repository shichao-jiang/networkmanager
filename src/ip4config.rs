use crate::errors::Error;
use zbus::{blocking::Connection, zvariant::OwnedObjectPath};

pub struct Ip4Config {
    pub(crate) zbus: Connection,
    pub(crate) path: OwnedObjectPath,
}

crate::zproxy_pathed!(Ip4Config, crate::raw::ip4config::IP4ConfigProxyBlocking<'_>);

impl Ip4Config {
    pub fn address_data(
        &self,
    ) -> Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>, Error> {
        self.raw()?.address_data().map_err(Error::ZBus)
    }

    pub fn addresses(&self) -> Result<Vec<Vec<u32>>, Error> {
        self.raw()?.addresses().map_err(Error::ZBus)
    }

    pub fn dns_options(&self) -> Result<Vec<String>, Error> {
        self.raw()?.dns_options().map_err(Error::ZBus)
    }

    pub fn dns_priority(&self) -> Result<i32, Error> {
        self.raw()?.dns_priority().map_err(Error::ZBus)
    }

    pub fn domains(&self) -> Result<Vec<String>, Error> {
        self.raw()?.domains().map_err(Error::ZBus)
    }

    pub fn gateway(&self) -> Result<String, Error> {
        self.raw()?.gateway().map_err(Error::ZBus)
    }

    pub fn nameserver_data(
        &self,
    ) -> Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>, Error> {
        self.raw()?.nameserver_data().map_err(Error::ZBus)
    }

    pub fn nameservers(&self) -> Result<Vec<u32>, Error> {
        self.raw()?.nameservers().map_err(Error::ZBus)
    }

    pub fn route_data(
        &self,
    ) -> Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>, Error> {
        self.raw()?.route_data().map_err(Error::ZBus)
    }

    pub fn routes(&self) -> Result<Vec<Vec<u32>>, Error> {
        self.raw()?.routes().map_err(Error::ZBus)
    }

    pub fn searches(&self) -> Result<Vec<String>, Error> {
        self.raw()?.searches().map_err(Error::ZBus)
    }

    pub fn wins_server_data(&self) -> Result<Vec<String>, Error> {
        self.raw()?.wins_server_data().map_err(Error::ZBus)
    }

    pub fn wins_servers(&self) -> Result<Vec<u32>, Error> {
        self.raw()?.wins_servers().map_err(Error::ZBus)
    }
}
