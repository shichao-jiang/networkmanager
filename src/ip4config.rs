use crate::errors::Error;
use zbus::{zvariant::OwnedObjectPath, Connection};

pub struct Ip4Config {
    pub(crate) zbus: Connection,
    pub(crate) path: OwnedObjectPath,
}

crate::zproxy_pathed!(Ip4Config, crate::raw::ip4config::IP4ConfigProxy<'_>);

impl Ip4Config {
    pub async fn address_data(
        &self,
    ) -> Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>, Error> {
        self.raw().await?.address_data().await.map_err(Error::ZBus)
    }

    pub async fn addresses(&self) -> Result<Vec<Vec<u32>>, Error> {
        self.raw().await?.addresses().await.map_err(Error::ZBus)
    }

    pub async fn dns_options(&self) -> Result<Vec<String>, Error> {
        self.raw().await?.dns_options().await.map_err(Error::ZBus)
    }

    pub async fn dns_priority(&self) -> Result<i32, Error> {
        self.raw().await?.dns_priority().await.map_err(Error::ZBus)
    }

    pub async fn domains(&self) -> Result<Vec<String>, Error> {
        self.raw().await?.domains().await.map_err(Error::ZBus)
    }

    pub async fn gateway(&self) -> Result<String, Error> {
        self.raw().await?.gateway().await.map_err(Error::ZBus)
    }

    pub async fn nameserver_data(
        &self,
    ) -> Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>, Error> {
        self.raw()
            .await?
            .nameserver_data()
            .await
            .map_err(Error::ZBus)
    }

    pub async fn nameservers(&self) -> Result<Vec<u32>, Error> {
        self.raw().await?.nameservers().await.map_err(Error::ZBus)
    }

    pub async fn route_data(
        &self,
    ) -> Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>, Error> {
        self.raw().await?.route_data().await.map_err(Error::ZBus)
    }

    pub async fn routes(&self) -> Result<Vec<Vec<u32>>, Error> {
        self.raw().await?.routes().await.map_err(Error::ZBus)
    }

    pub async fn searches(&self) -> Result<Vec<String>, Error> {
        self.raw().await?.searches().await.map_err(Error::ZBus)
    }

    pub async fn wins_server_data(&self) -> Result<Vec<String>, Error> {
        self.raw()
            .await?
            .wins_server_data()
            .await
            .map_err(Error::ZBus)
    }

    pub async fn wins_servers(&self) -> Result<Vec<u32>, Error> {
        self.raw().await?.wins_servers().await.map_err(Error::ZBus)
    }
}
