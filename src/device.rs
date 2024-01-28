use num_traits::FromPrimitive;
use zbus::{zvariant::OwnedObjectPath, Connection};

use crate::{raw::device::DeviceProxy, types::DeviceType, Error};

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

crate::zproxy_pathed!(Device, DeviceProxy<'_>);

impl Device {
    /// The general type of the network device; ie Ethernet, Wi-Fi, etc.
    pub async fn device_type(&self) -> Result<DeviceType, Error> {
        let dev_type = self.raw().await?.device_type().await?;
        FromPrimitive::from_u32(dev_type).ok_or(Error::UnsupportedType)
    }
}

macro_rules! cast {
    ($name:ident, $variant:ident, $cast:ty) => {
        impl Device {
            #[doc = concat!("Cast this device to a [`", stringify!($cast), "`].")]
            pub async fn $name(&self) -> Result<Option<$cast>, Error> {
                if let DeviceType::$variant = self.device_type().await? {
                    Ok(Some(<$cast>::new(self.clone())))
                } else {
                    Ok(None)
                }
            }
        }
    };
}

cast!(to_wireless, Wifi, wireless::WirelessDevice);
