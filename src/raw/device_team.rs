//! # DBus interface proxy for: `org.freedesktop.NetworkManager.Device.Team`
//!
//! This code was generated by `zbus-xmlgen` `3.1.1` from DBus introspection data.
//! Source: `org.freedesktop.NetworkManager.Device.Team.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus_macros::proxy;

#[proxy(
    interface = "org.freedesktop.NetworkManager.Device.Team",
    assume_defaults = true
)]
pub trait DeviceTeam {
    /// Carrier property
    #[zbus(property)]
    fn carrier(&self) -> zbus::Result<bool>;

    /// Config property
    #[zbus(property)]
    fn config(&self) -> zbus::Result<String>;

    /// HwAddress property
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Slaves property
    #[zbus(property)]
    fn slaves(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
}
