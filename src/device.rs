use num_traits::FromPrimitive;
use zbus::{blocking::Connection, zvariant::OwnedObjectPath};

use crate::{raw::device::DeviceProxyBlocking, types::DeviceType, Error};

#[doc(inline)]
pub use _methods::AppliedConnection;

mod _methods;
pub mod wireless;

/// A network device.
///
/// This provides the common interface for all network devices. The actual device types are
/// accessible via the [`Device::device_type()`] method and can be casted to the appropriate type
/// using the various `to_*` methods.
#[derive(Clone, Debug)]
pub struct Device {
    pub(crate) zbus: Connection,
    pub(crate) path: OwnedObjectPath,
}

crate::zproxy_pathed!(Device, DeviceProxyBlocking<'_>);

impl Device {
    /// The general type of the network device; ie Ethernet, Wi-Fi, etc.
    pub fn device_type(&self) -> Result<DeviceType, Error> {
        let dev_type = self.raw()?.device_type()?;
        FromPrimitive::from_u32(dev_type).ok_or(Error::UnsupportedType)
    }
}

macro_rules! cast {
    ($name:ident, $variant:ident, $cast:ty) => {
        impl Device {
            #[doc = concat!("Cast this device to a [`", stringify!($cast), "`].")]
            pub fn $name(&self) -> Result<Option<$cast>, Error> {
                if let DeviceType::$variant = self.device_type()? {
                    Ok(Some(<$cast>::new(self.clone())))
                } else {
                    Ok(None)
                }
            }
        }
    };
}

cast!(to_wireless, Wifi, wireless::WirelessDevice);
