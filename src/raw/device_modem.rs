//! # DBus interface proxy for: `org.freedesktop.NetworkManager.Device.Modem`
//!
//! This code was generated by `zbus-xmlgen` `3.1.1` from DBus introspection data.
//! Source: `org.freedesktop.NetworkManager.Device.Modem.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus_macros::proxy;

#[proxy(
    interface = "org.freedesktop.NetworkManager.Device.Modem",
    assume_defaults = true
)]
pub trait DeviceModem {
    /// Apn property
    #[zbus(property)]
    fn apn(&self) -> zbus::Result<String>;

    /// CurrentCapabilities property
    #[zbus(property)]
    fn current_capabilities(&self) -> zbus::Result<u32>;

    /// DeviceId property
    #[zbus(property)]
    fn device_id(&self) -> zbus::Result<String>;

    /// ModemCapabilities property
    #[zbus(property)]
    fn modem_capabilities(&self) -> zbus::Result<u32>;

    /// OperatorCode property
    #[zbus(property)]
    fn operator_code(&self) -> zbus::Result<String>;
}
