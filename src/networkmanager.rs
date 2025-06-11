use zbus::blocking::Connection;

use crate::device::Device;
use crate::errors::Error;
use crate::raw::networkmanager::NetworkManagerProxyBlocking;
use crate::settings::Settings;
use crate::types::ReloadFlags;

#[derive(Clone, Debug)]
pub struct NetworkManager {
    zbus: Connection,
}

crate::zproxy_unpathed!(NetworkManager, NetworkManagerProxyBlocking<'_>);

impl NetworkManager {
    /// Create a new NetworkManager instance.
    pub fn new() -> Result<Self, Error> {
        let zbus = Connection::system()?;
        Ok(Self::new_with_zbus(zbus))
    }

    /// Create a new NetworkManager instance with a custom D-Bus connection.
    pub fn new_with_zbus(zbus: Connection) -> Self {
        NetworkManager { zbus }
    }

    /// Reload NetworkManager.
    pub fn reload(&self, flags: ReloadFlags) -> Result<(), Error> {
        self.raw()?.reload(flags.bits()).map_err(Error::ZBus)
    }

    /// Get the list of realized network devices.
    ///
    /// Returns the network devices known to the system. Does not include device placeholders (see
    /// [`get_all_devices()`](#method.get_all_devices)).
    pub fn get_devices(&self) -> Result<impl Iterator<Item = Device> + '_, Error> {
        Ok(self.raw()?.get_devices()?.into_iter().map(|path| Device {
            zbus: self.zbus.clone(),
            path,
        }))
    }

    /// Get the list of all network devices.
    ///
    /// Includes device placeholders (eg, devices that do not yet exist but which could be
    /// automatically created by NetworkManager if one of their AvailableConnections was activated).
    pub fn get_all_devices(&self) -> Result<impl Iterator<Item = Device> + '_, Error> {
        Ok(self
            .raw()?
            .get_all_devices()?
            .into_iter()
            .map(|path| Device {
                zbus: self.zbus.clone(),
                path,
            }))
    }

    /// Get the network device referenced by its IP interface name.
    ///
    /// Note that some devices (usually modems) only have an IP interface name when they are
    /// connected.
    pub fn get_device_by_ip_interface_name(&self, iface: &str) -> Result<Device, Error> {
        Ok(Device {
            zbus: self.zbus.clone(),
            path: self.raw()?.get_device_by_ip_iface(iface)?,
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

    pub fn enable(&self, enabled: bool) -> Result<(), Error> {
        self.raw()?.enable(enabled)?;
        Ok(())
    }

    pub fn is_networking_enabled(&self) -> Result<bool, Error> {
        Ok(self.raw()?.networking_enabled()?)
    }

    pub fn is_wireless_enabled(&self) -> Result<bool, Error> {
        Ok(self.raw()?.wireless_enabled()?)
    }

    pub fn is_wireless_hardware_enabled(&self) -> Result<bool, Error> {
        Ok(self.raw()?.wireless_hardware_enabled()?)
    }

    pub fn is_wimax_enabled(&self) -> Result<bool, Error> {
        Ok(self.raw()?.wimax_enabled()?)
    }

    pub fn is_wimax_hardware_enabled(&self) -> Result<bool, Error> {
        Ok(self.raw()?.wimax_hardware_enabled()?)
    }

    pub fn is_wwan_enabled(&self) -> Result<bool, Error> {
        Ok(self.raw()?.wwan_enabled()?)
    }

    pub fn is_wwan_hardware_enabled(&self) -> Result<bool, Error> {
        Ok(self.raw()?.wwan_hardware_enabled()?)
    }

    pub fn is_connectivity_check_enabled(&self) -> Result<bool, Error> {
        Ok(self.raw()?.connectivity_check_enabled()?)
    }

    /// Indicates whether NetworkManager is still starting up.
    ///
    /// This becomes `false` when NetworkManager has finished attempting to activate every
    /// connection that it might be able to activate at startup.
    pub fn is_starting_up(&self) -> Result<bool, Error> {
        Ok(self.raw()?.startup()?)
    }

    /// Settings service object
    pub fn settings(&self) -> Settings {
        Settings::new_with_zbus(self.zbus.clone())
    }
}
