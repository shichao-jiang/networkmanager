//! # DBus interface proxy for: `org.freedesktop.NetworkManager.WiMax.Nsp`
//!
//! This code was generated by `zbus-xmlgen` `3.1.1` from DBus introspection data.
//! Source: `org.freedesktop.NetworkManager.WiMax.Nsp.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus_macros::proxy;

#[proxy(
    interface = "org.freedesktop.NetworkManager.WiMax.Nsp",
    assume_defaults = true
)]
pub trait WiMaxNsp {
    /// Name property
    #[zbus(property)]
    fn name(&self) -> zbus::Result<String>;

    /// NetworkType property
    #[zbus(property)]
    fn network_type(&self) -> zbus::Result<u32>;

    /// SignalQuality property
    #[zbus(property)]
    fn signal_quality(&self) -> zbus::Result<u32>;
}
