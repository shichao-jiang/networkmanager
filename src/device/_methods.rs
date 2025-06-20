use std::collections::HashMap;

use num_traits::FromPrimitive;
use zbus::zvariant::{OwnedValue, Value};

use crate::{
    ip4config::Ip4Config,
    types::{
        CapabilityFlags, ConnectivityState, DeviceInterfaceFlags, DeviceState, DeviceStateReason,
        MeteredStatus,
    },
    Error,
};

use super::Device;

impl Device {
    /// Attempts to update the configuration of a device without deactivating it.
    ///
    /// NetworkManager has the concept of _connections_, which are _profiles_ that contain the
    /// configuration for a networking device. Those _connections_ are exposed via D-Bus as
    /// individual objects that can be created, modified and deleted. When activating such a
    /// _connection_ on a device, the profile is duplicated, becomes an _applied connection_, and is
    /// used to configure the device (see [`Device::get_applied_connection()`]).
    ///
    /// Subsequent modification of the original _connection_ don't propagate automatically to the
    /// device's _applied connection_ profile (with exception of the `firewall-zone` and `metered`
    /// properties). For the changes to take effect, you can either re-activate the _connection_, or
    /// call [`Device::reapply()`].
    ///
    /// The reapply call allows you to directly update the _connection_ and reconfigure the device.
    /// Reapply can also be useful if the currently _applied connection_ is equal to the
    /// _connection_ that is about to be reapplied. This allows to reconfigure the device and revert
    /// external changes like removing or adding an IP address (which NetworkManager doesn't revert
    /// automatically because it is assumed that the user made these changes intentionally outside
    /// of NetworkManager).
    ///
    /// Reapply can make the _applied connection_ different from the _connection_, just like
    /// updating the _connection_ can make them different.
    pub async fn reapply(
        &self,
        connection: HashMap<&str, HashMap<&str, Value<'_>>>,
        version_id: u64,
        flags: u32,
    ) -> Result<(), Error> {
        self.raw()
            .await?
            .reapply(connection, version_id, flags)
            .await
            .map_err(Error::ZBus)
    }

    /// Get the currently applied connection on the device.
    ///
    /// This is a snapshot of the last activated connection on the device, ie the configuration that
    /// is currently applied on the device.
    ///
    /// Usually this is the same as [`Connection::settings()`](crate::connection::Connection::settings())
    /// of the referenced _connection_. However, it can differ if the _connection_ was subsequently
    /// modified or the _applied connection_ was modified by [`Device::reapply()`]. The
    /// _applied connection_ is set when activating a device or when calling reapply.
    pub async fn get_applied_connection(&self) -> Result<AppliedConnection, Error> {
        let (settings, version) = self.raw().await?.get_applied_connection(0).await?;
        Ok(AppliedConnection { settings, version })
    }

    /// Disconnects a device and prevents the device from automatically activating further connections without user intervention.
    pub async fn disconnect(&self) -> Result<(), Error> {
        self.raw().await?.disconnect().await.map_err(Error::ZBus)
    }

    /// Deletes a software device from NetworkManager and removes the interface from the system.
    ///
    /// The method returns an error when called for a hardware device.
    pub async fn delete(&self) -> Result<(), Error> {
        self.raw().await?.delete().await.map_err(Error::ZBus)
    }

    /// OS-specific transient device hardware identifier.
    ///
    /// This is an opaque string representing the underlying hardware for the device, and shouldn't
    /// be used to keep track of individual devices.
    ///
    /// For some device types (Bluetooth, Modems) it is an identifier used by the hardware service
    /// (ie bluez or ModemManager) to refer to that device, and client programs use it get
    /// additional information from those services which NetworkManager does not provide.
    ///
    /// The Udi is not guaranteed to be consistent across reboots or hotplugs of the hardware. If
    /// you're looking for a way to uniquely track each device in your application, use the object
    /// path. If you're looking for a way to track a specific piece of hardware across reboot or
    /// hotplug, use a MAC address or USB serial number.
    pub async fn udi(&self) -> Result<String, Error> {
        self.raw().await?.udi().await.map_err(Error::ZBus)
    }

    /// The path of the device as exposed by the udev property `ID_PATH`.
    pub async fn path(&self) -> Result<String, Error> {
        self.raw().await?.path().await.map_err(Error::ZBus)
    }

    /// The name of the device's control (and often data) interface.
    pub async fn interface(&self) -> Result<String, Error> {
        self.raw().await?.interface().await.map_err(Error::ZBus)
    }

    /// The name of the device's data interface when available.
    ///
    /// This property may not refer to the actual data interface until the device has successfully
    /// established a data connection, indicated by [`Device::state()`] becoming
    /// [`DeviceState::Activated`].
    pub async fn ip_interface(&self) -> Result<String, Error> {
        self.raw().await?.ip_interface().await.map_err(Error::ZBus)
    }

    /// The driver handling the device.
    pub async fn driver(&self) -> Result<String, Error> {
        self.raw().await?.driver().await.map_err(Error::ZBus)
    }

    /// The version of the driver handling the device.
    pub async fn driver_version(&self) -> Result<String, Error> {
        self.raw()
            .await?
            .driver_version()
            .await
            .map_err(Error::ZBus)
    }

    /// The firmware version for the device.
    pub async fn firmware_version(&self) -> Result<String, Error> {
        self.raw()
            .await?
            .firmware_version()
            .await
            .map_err(Error::ZBus)
    }

    /// Flags describing the capabilities of the device.
    pub async fn capabilities(&self) -> Result<CapabilityFlags, Error> {
        let cap = self.raw().await?.capabilities().await?;
        Ok(CapabilityFlags::from_bits_retain(cap))
    }

    /// The current state of the device.
    pub async fn state(&self) -> Result<DeviceState, Error> {
        let state = self.raw().await?.state_property().await?;
        FromPrimitive::from_u32(state).ok_or(Error::UnsupportedType)
    }

    /// The current state of the device and the reason for that state.
    pub async fn state_with_reason(&self) -> Result<(DeviceState, DeviceStateReason), Error> {
        let (state, reason) = self.raw().await?.state_reason().await?;
        Ok((
            FromPrimitive::from_u32(state).ok_or(Error::UnsupportedType)?,
            FromPrimitive::from_u32(reason).ok_or(Error::UnsupportedType)?,
        ))
    }

    // pub async fn active_connection(&self) -> Result<Connection, Error> {
    //     todo!()
    // }

    pub async fn ip4_config(&self) -> Result<Ip4Config, Error> {
        let path = self.raw().await?.ip4_config().await?;
        Ok(Ip4Config {
            zbus: self.zbus.clone(),
            path,
        })
    }

    // fn dhcp4_config(&self) -> Result<Dhcp4Config, Error> {
    //     todo!()
    // }

    // fn ipv6_config(&self) -> Result<Ip6Config, Error> {
    //     todo!()
    // }

    // fn dhcp6_config(&self) -> Result<Dhcp6Config, Error> {
    //     todo!()
    // }

    /// Whether or not this device is managed by NetworkManager.
    pub async fn is_managed(&self) -> Result<bool, Error> {
        self.raw().await?.managed().await.map_err(Error::ZBus)
    }

    /// Set whether or not this device is managed by NetworkManager.
    ///
    /// This has a similar effect to configuring the device as unmanaged via the
    /// `keyfile.unmanaged-devices` setting in `NetworkManager.conf`.
    ///
    /// Changes to this value are not persistent and lost after NetworkManager restarts.
    pub async fn set_managed(&self, managed: bool) -> Result<(), Error> {
        self.raw()
            .await?
            .set_managed(managed)
            .await
            .map_err(Error::ZBus)
    }

    /// Indicates the device is allowed to autoconnect.
    pub async fn can_autoconnect(&self) -> Result<bool, Error> {
        self.raw().await?.autoconnect().await.map_err(Error::ZBus)
    }

    /// Set whether or not this device is allowed to autoconnect.
    ///
    /// If false, manual intervention is required before the device will automatically connect to a
    /// known network, such as activating a connection using the device, or setting this property to
    /// true.
    ///
    /// This property cannot be set to true for default-unmanaged devices, since they never
    /// autoconnect.
    pub async fn set_autoconnect(&self, autoconnect: bool) -> Result<(), Error> {
        self.raw()
            .await?
            .set_autoconnect(autoconnect)
            .await
            .map_err(Error::ZBus)
    }

    /// Indicates the device is likely missing firmware necessary for its operation.
    pub async fn is_firmware_missing(&self) -> Result<bool, Error> {
        self.raw()
            .await?
            .firmware_missing()
            .await
            .map_err(Error::ZBus)
    }

    /// Indicates the NetworkManager plugin for the device is likely missing or misconfigured.
    pub async fn is_plugin_missing(&self) -> Result<bool, Error> {
        self.raw()
            .await?
            .nm_plugin_missing()
            .await
            .map_err(Error::ZBus)
    }

    // fn available_connections(&self) -> Result<Vec<Connection>, Error> {
    //     let paths = proxy!(self).available_connections()?;
    //     let mut connections = Vec::with_capacity(paths.len());
    //     for path in paths {
    //         connections.push(Connection::new(self.dbus_accessor.with_path(path)));
    //     }
    //     Ok(connections)
    // }

    // pub async fn ports(&self) -> Result<Vec<Port>, Error> {
    // }

    /// An opaque indicator of the physical network port associated with the device.
    ///
    /// This can be used to recognize when two seemingly-separate hardware devices are actually just
    /// different virtual interfaces to the same physical port.
    pub async fn physical_port_id(&self) -> Result<Option<String>, Error> {
        let id = self.raw().await?.physical_port_id().await?;
        if id.is_empty() {
            Ok(None)
        } else {
            Ok(Some(id))
        }
    }

    /// The MTU (Maximum Transmission Unit) of the device.
    pub async fn mtu(&self) -> Result<u32, Error> {
        self.raw().await?.mtu().await.map_err(Error::ZBus)
    }

    /// The device's metered state.
    ///
    /// This is determined by the profile which is currently active. If the profile explicitly
    /// specifies [`MeteredSetting::No`](crate::types::MeteredSetting::No) or
    /// [`MeteredSetting::Yes`](crate::types::MeteredSetting::Yes), then the device's metered state
    /// is as such. If the connection profile leaves it
    /// [`MeteredSetting::Undecided`](crate::types::MeteredSetting::Undecided) (the default), then
    /// NetworkManager tries to guess the metered state, for example based on the device type or on
    /// DHCP options (like Android devices exposing a `ANDROID_METERED` DHCP vendor option). This
    /// then leads to either [`MeteredStatus::GuessNo`] or [`MeteredStatus::GuessYes`].
    pub async fn is_metered(&self) -> Result<MeteredStatus, Error> {
        let value = self.raw().await?.device_type().await?;
        FromPrimitive::from_u32(value).ok_or(Error::UnsupportedType)
    }

    /// Array of LLDP neighbors.
    pub async fn lldp_neighbors(&self) -> Result<Vec<HashMap<String, OwnedValue>>, Error> {
        self.raw()
            .await?
            .lldp_neighbors()
            .await
            .map_err(Error::ZBus)
    }

    /// Whether the device is real or a placeholder.
    ///
    /// Placeholder devices do not yet exist but could be automatically created by NetworkManager if
    /// one of their AvailableConnections was activated.
    pub async fn is_real(&self) -> Result<bool, Error> {
        self.raw().await?.real().await.map_err(Error::ZBus)
    }

    /// The result of the last IPv4 connectivity check.
    pub async fn ipv4_connectivity(&self) -> Result<ConnectivityState, Error> {
        let value = self.raw().await?.ip4_connectivity().await?;
        FromPrimitive::from_u32(value).ok_or(Error::UnsupportedType)
    }

    /// The result of the last IPv6 connectivity check.
    pub async fn ipv6_connectivity(&self) -> Result<ConnectivityState, Error> {
        let value = self.raw().await?.ip6_connectivity().await?;
        FromPrimitive::from_u32(value).ok_or(Error::UnsupportedType)
    }

    /// The flags of the network interface.
    pub async fn interface_flags(&self) -> Result<DeviceInterfaceFlags, Error> {
        let value = self.raw().await?.interface_flags().await?;
        Ok(DeviceInterfaceFlags::from_bits_retain(value))
    }

    /// The hardware address of the device.
    pub async fn hardware_address(&self) -> Result<String, Error> {
        self.raw().await?.hw_address().await.map_err(Error::ZBus)
    }
}

/// The currently applied connection on the device.
///
/// Obtained from [`Device::get_applied_connection()`].
pub struct AppliedConnection {
    /// The effective connection settings that the connection has currently applied.
    pub settings: HashMap<String, HashMap<String, OwnedValue>>,

    /// The version ID of the currently applied connection.
    ///
    /// This can be specified during [`Device::reapply()`] to avoid races where you first fetch the
    /// _applied connection_, modify it and try to reapply it. If the _applied connection_ is
    /// modified in the meantime, the `version` gets incremented and the reapply will fail.
    pub version: u64,
}
