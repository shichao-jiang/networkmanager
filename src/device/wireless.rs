use crate::{
    raw::device_wireless::DeviceWirelessProxy,
    types::{WirelessClientCapabilityFlags, WirelessClientMode},
    Error,
};

use super::Device;

#[doc(inline)]
pub use access_point::AccessPoint;
use num_traits::FromPrimitive;

mod access_point;

/// A Wi-Fi device.
#[derive(Clone, Debug)]
pub struct WirelessDevice {
    pub(crate) parent: Device,
}

crate::zproxy_sub!(Device, WirelessDevice, DeviceWirelessProxy<'_>);

impl WirelessDevice {
    /// Get the underlying [`Device`].
    pub fn device(&self) -> &Device {
        &self.parent
    }

    /// Get the list of all access points visible to this device.
    ///
    /// This includes "hidden" access points, for which the SSID is not yet known.
    pub async fn get_all_access_points(
        &self,
    ) -> Result<impl Iterator<Item = AccessPoint> + '_, Error> {
        Ok(self
            .raw()
            .await?
            .get_all_access_points()
            .await?
            .into_iter()
            .map(|path| AccessPoint {
                zbus: self.parent.zbus.clone(),
                path,
            }))
    }

    /// Request a new scan for access points.
    pub async fn request_scan(&self) -> Result<(), Error> {
        self.raw()
            .await?
            .request_scan(std::collections::HashMap::new())
            .await?;
        Ok(())
    }

    /// Request a new scan for access points with the given SSIDs.
    pub async fn request_scan_with_ssids(&self, ssids: Vec<Vec<u8>>) -> Result<(), Error> {
        self.raw()
            .await?
            .request_scan(std::collections::HashMap::from_iter([(
                "ssids",
                ssids.into(),
            )]))
            .await?;
        Ok(())
    }
}

impl WirelessDevice {
    /// The permanent hardware address of the device.
    pub async fn permanent_hardware_address(&self) -> Result<String, Error> {
        self.raw()
            .await?
            .perm_hw_address()
            .await
            .map_err(Error::ZBus)
    }

    /// The operating mode of the wireless device.
    pub async fn mode(&self) -> Result<WirelessClientMode, Error> {
        let value = self.raw().await?.mode().await?;
        FromPrimitive::from_u32(value).ok_or(Error::UnsupportedType)
    }

    /// The current bit rate used by the device, in kilobits/second.
    pub async fn bitrate(&self) -> Result<u32, Error> {
        self.raw().await?.bitrate().await.map_err(Error::ZBus)
    }

    /// The list of access points visible to this device.
    pub async fn access_points(&self) -> Result<impl Iterator<Item = AccessPoint> + '_, Error> {
        Ok(self
            .raw()
            .await?
            .access_points()
            .await?
            .into_iter()
            .map(|path| AccessPoint {
                zbus: self.parent.zbus.clone(),
                path,
            }))
    }

    /// The access point currently used by the wireless device.
    pub async fn active_access_point(&self) -> Result<Option<AccessPoint>, Error> {
        let path = self.raw().await?.active_access_point().await?;
        // TODO: check path for null?
        Ok(Some(AccessPoint {
            zbus: self.parent.zbus.clone(),
            path,
        }))
    }

    /// The capabilities of the wireless device.
    pub async fn capabilities(&self) -> Result<WirelessClientCapabilityFlags, Error> {
        let value = self.raw().await?.wireless_capabilities().await?;
        Ok(WirelessClientCapabilityFlags::from_bits_retain(value))
    }

    /// The timestamp for the last finished network scan.
    ///
    /// This is in `CLOCK_BOOTTIME` seconds.
    ///
    /// A value of None means the device never scanned for access points.
    pub async fn last_scan(&self) -> Result<Option<u64>, Error> {
        let ts = self.raw().await?.last_scan().await?;
        if let Ok(ts) = u64::try_from(ts) {
            Ok(Some(ts))
        } else {
            Ok(None)
        }
    }
}
