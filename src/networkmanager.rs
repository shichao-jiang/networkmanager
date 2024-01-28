use zbus::Connection;

use crate::errors::Error;
use crate::genz::networkmanager::NetworkManagerProxy;
use crate::settings::Settings;
use crate::types::ReloadFlags;
use crate::zdevice::ZDevice;

#[derive(Clone, Debug)]
pub struct NetworkManager {
    zbus: Connection,
}

crate::zproxy!(NetworkManager, NetworkManagerProxy<'_>);

impl NetworkManager {
    /// Create a new NetworkManager instance.
    pub async fn new() -> Result<Self, Error> {
        let zbus = Connection::system().await?;
        Ok(Self::new_with_zbus(zbus))
    }

    /// Create a new NetworkManager instance with a custom D-Bus connection.
    pub fn new_with_zbus(zbus: Connection) -> Self {
        NetworkManager { zbus }
    }

    /// Reload NetworkManager.
    pub async fn reload(&self, flags: ReloadFlags) -> Result<(), Error> {
        self.raw()
            .await?
            .reload(flags.bits())
            .await
            .map_err(Error::ZBus)
    }

    /// Get the list of realized network devices.
    ///
    /// Returns the network devices known to the system. Does not include device placeholders (see
    /// [`get_all_devices()`](#method.get_all_devices)).
    pub async fn get_devices(&self) -> Result<impl Iterator<Item = ZDevice> + '_, Error> {
        Ok(self
            .raw()
            .await?
            .get_devices()
            .await?
            .into_iter()
            .map(|path| ZDevice {
                zbus: self.zbus.clone(),
                path,
            }))
    }

    /// Get the list of all network devices.
    ///
    /// Includes device placeholders (eg, devices that do not yet exist but which could be
    /// automatically created by NetworkManager if one of their AvailableConnections was activated).
    pub async fn get_all_devices(&self) -> Result<impl Iterator<Item = ZDevice> + '_, Error> {
        Ok(self
            .raw()
            .await?
            .get_all_devices()
            .await?
            .into_iter()
            .map(|path| ZDevice {
                zbus: self.zbus.clone(),
                path,
            }))
    }

    /// Get the network device referenced by its IP interface name.
    ///
    /// Note that some devices (usually modems) only have an IP interface name when they are
    /// connected.
    pub async fn get_device_by_ip_interface_name(&self, iface: &str) -> Result<ZDevice, Error> {
        Ok(ZDevice {
            zbus: self.zbus.clone(),
            path: self.raw().await?.get_device_by_ip_iface(iface).await?,
        })
    }

    // TODO: ActivateConnection()
    // TODO: AddAndActivateConnection()
    // TODO: AddAndActivateConnection2()
    // TODO: DeactivateConnection()
    // TODO: Sleep()
    // TODO: GetPermissions()
    // TODO: SetLogging()
    // TODO: GetLogging()
    // TODO: CheckConnectivity()
    // TODO: State()
    // TODO: CheckpointCreate()
    // TODO: CheckpointDestroy()
    // TODO: CheckpointRollback()
    // TODO: CheckpointAdjustRollbackTimeout()

    pub async fn enable(&self, enabled: bool) -> Result<(), Error> {
        self.raw().await?.enable(enabled).await?;
        Ok(())
    }

    pub async fn is_networking_enabled(&self) -> Result<bool, Error> {
        Ok(self.raw().await?.networking_enabled().await?)
    }

    pub async fn is_wireless_enabled(&self) -> Result<bool, Error> {
        Ok(self.raw().await?.wireless_enabled().await?)
    }

    pub async fn is_wireless_hardware_enabled(&self) -> Result<bool, Error> {
        Ok(self.raw().await?.wireless_hardware_enabled().await?)
    }

    pub async fn is_wimax_enabled(&self) -> Result<bool, Error> {
        Ok(self.raw().await?.wimax_enabled().await?)
    }

    pub async fn is_wimax_hardware_enabled(&self) -> Result<bool, Error> {
        Ok(self.raw().await?.wimax_hardware_enabled().await?)
    }

    pub async fn is_wwan_enabled(&self) -> Result<bool, Error> {
        Ok(self.raw().await?.wwan_enabled().await?)
    }

    pub async fn is_wwan_hardware_enabled(&self) -> Result<bool, Error> {
        Ok(self.raw().await?.wwan_hardware_enabled().await?)
    }

    pub async fn is_connectivity_check_enabled(&self) -> Result<bool, Error> {
        Ok(self.raw().await?.connectivity_check_enabled().await?)
    }

    /// Indicates whether NetworkManager is still starting up.
    ///
    /// This becomes `false` when NetworkManager has finished attempting to activate every
    /// connection that it might be able to activate at startup.
    pub async fn is_starting_up(&self) -> Result<bool, Error> {
        Ok(self.raw().await?.startup().await?)
    }

    /// Settings service object
    pub fn settings(&self) -> Result<Settings, Error> {
        todo!()
    }
}
