mod any;
mod bridge;
mod generic;
mod veth;
mod wired;
mod wireless;

pub use self::any::Any;
pub use self::bridge::Bridge;
pub use self::generic::Generic;
pub use self::veth::Veth;
pub use self::wired::Wired;
pub use self::wireless::Wireless;
use crate::dbus_api::DBusAccessor;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerDevice;
use crate::types::DeviceType;
use num_traits::FromPrimitive;

#[derive(Clone, Debug)]
pub enum Device {
    WiFi(WiFiDevice),
    Ethernet(EthernetDevice),
    Generic(GenericDevice),
    Bridge(BridgeDevice),
    Veth(VethDevice),
    UnsupportedDevice,
}

#[derive(Clone, Debug)]
pub struct GenericDevice {
    dbus_accessor: DBusAccessor,
}

#[derive(Clone, Debug)]
pub struct WiFiDevice {
    dbus_accessor: DBusAccessor,
}

#[derive(Clone, Debug)]
pub struct EthernetDevice {
    dbus_accessor: DBusAccessor,
}

#[derive(Clone, Debug)]
pub struct BridgeDevice {
    dbus_accessor: DBusAccessor,
}

#[derive(Clone, Debug)]
pub struct VethDevice {
    dbus_accessor: DBusAccessor,
}

impl Device {
    pub(crate) fn new(dbus_accessor: DBusAccessor) -> Result<Self, Error> {
        let dev_type = dbus_accessor.create_proxy().device_type()?;
        match FromPrimitive::from_u32(dev_type) {
            Some(x) => match x {
                DeviceType::Wifi => Ok(Device::WiFi(WiFiDevice { dbus_accessor })),
                DeviceType::Ethernet => Ok(Device::Ethernet(EthernetDevice { dbus_accessor })),
                DeviceType::Generic => Ok(Device::Generic(GenericDevice { dbus_accessor })),
                DeviceType::Bridge => Ok(Device::Bridge(BridgeDevice { dbus_accessor })),
                DeviceType::Veth => Ok(Device::Veth(VethDevice { dbus_accessor })),
                _ => Ok(Device::UnsupportedDevice),
            },
            None => Err(Error::UnsupportedType),
        }
    }
}
