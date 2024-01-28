use num_traits::FromPrimitive;
use zbus::{zvariant::OwnedObjectPath, Connection};

use crate::{
    types::{AccessPointCapabilityFlags, AccessPointMode, AccessPointSecurityFlags},
    Error,
};

/// A wireless access point.
///
/// This is obtained from [`WirelessDevice`](super::WirelessDevice)'s methods.
#[derive(Clone, Debug)]
pub struct AccessPoint {
    pub(crate) zbus: Connection,
    pub(crate) path: OwnedObjectPath,
}

crate::zproxy_pathed!(AccessPoint, crate::raw::accesspoint::AccessPointProxy<'_>);

impl AccessPoint {
    /// Flags describing the capabilities of the access point.
    pub async fn capability_flags(&self) -> Result<AccessPointCapabilityFlags, Error> {
        let value = self.raw().await?.flags().await?;
        Ok(AccessPointCapabilityFlags::from_bits_retain(value))
    }

    /// Flags describing the access point's capabilities according to WPA (Wifi Protected Access).
    pub async fn wpa_security_flags(&self) -> Result<AccessPointSecurityFlags, Error> {
        let value = self.raw().await?.wpa_flags().await?;
        Ok(AccessPointSecurityFlags::from_bits_retain(value))
    }

    /// Flags describing the access point's capabilities according to RSN (Robust Secure Network).
    pub async fn rsn_security_flags(&self) -> Result<AccessPointSecurityFlags, Error> {
        let value = self.raw().await?.rsn_flags().await?;
        Ok(AccessPointSecurityFlags::from_bits_retain(value))
    }

    /// The access point's SSID (Service Set IDentifier).
    ///
    /// This is the raw octets, not a human-readable string. Generally, you'll be able to read this
    /// as a UTF-8 string, but it is not guaranteed.
    ///
    /// May be empty if the access point has a hidden SSID, and can be up to 32 bytes long.
    pub async fn ssid(&self) -> Result<Vec<u8>, Error> {
        self.raw().await?.ssid().await.map_err(Error::ZBus)
    }

    /// The radio channel frequency in use by the access point, in MHz.
    pub async fn frequency(&self) -> Result<u32, Error> {
        self.raw().await?.frequency().await.map_err(Error::ZBus)
    }

    /// The hardware address (BSSID) of the access point.
    pub async fn bssid(&self) -> Result<String, Error> {
        self.raw().await?.hw_address().await.map_err(Error::ZBus)
    }

    /// The operating mode of the access point.
    pub async fn mode(&self) -> Result<AccessPointMode, Error> {
        let state = self.raw().await?.mode().await?;
        FromPrimitive::from_u32(state).ok_or(Error::UnsupportedType)
    }

    /// The maximum bitrate this access point is capable of, in kilobits/second (Kb/s).
    pub async fn max_bitrate(&self) -> Result<u32, Error> {
        self.raw().await?.max_bitrate().await.map_err(Error::ZBus)
    }

    /// The current signal quality of the access point, in percent.
    pub async fn strength(&self) -> Result<u8, Error> {
        self.raw().await?.strength().await.map_err(Error::ZBus)
    }

    /// The timestamp for the last time the access point was found in scan results.
    ///
    /// This is in `CLOCK_BOOTTIME` seconds.
    ///
    /// A value of None means the access point has never been found in scan results.
    pub async fn last_seen(&self) -> Result<Option<u32>, Error> {
        let ts = self.raw().await?.last_seen().await?;
        if let Ok(ts) = u32::try_from(ts) {
            Ok(Some(ts))
        } else {
            Ok(None)
        }
    }
}
