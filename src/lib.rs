//! A [NetworkManager](https://wiki.gnome.org/Projects/NetworkManager) API library using the [D-Bus message bus system](https://www.freedesktop.org/wiki/Software/dbus/)
//!
//! ## Usage
//!
//! ```toml
//! [dependencies]
//! networkmanager = { package = "passcod-networkmanager", version = "=0.7.0-pre.1" }
//! tokio = { version = "1", features = ["full"] }
//! ```
//!
//! ## Example
//!
//! ```rust,no_run
//! # use passcod_networkmanager as networkmanager;
//! use networkmanager::{Error, NetworkManager};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let nm = NetworkManager::new().await?;
//!
//!     for dev in nm.get_devices().await? {
//!         if let Some(wifi) = dev.to_wireless().await? {
//!             println!("Bitrate: {:?}", wifi.bitrate().await?);
//!             wifi.request_scan().await?;
//!             for ap in wifi.get_all_access_points().await? {
//!                 let raw = ap.ssid().await?;
//!                 println!("SSID: {} {raw:02x?}", String::from_utf8_lossy(&raw));
//!             }
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! - `raw`: Enable access to the raw D-Bus proxies. This is useful if you need to access methods
//!   that are not wrapped by this library, or if you need to access the D-Bus signals.

#![deny(rust_2018_idioms)]

mod errors;
mod ip4config;
mod networkmanager;
mod raw;
mod settings;

// pub mod configs;
// pub mod devices;
pub mod connection;
pub mod device;
pub mod types;

pub use crate::errors::Error;
pub use crate::ip4config::Ip4Config;
pub use crate::networkmanager::NetworkManager;
pub use crate::settings::Settings;

#[cfg(feature = "raw")]
pub use raw;

macro_rules! zproxy_unpathed {
    ($facade:ty, $proxy:ty) => {
        impl $facade {
            #[cfg(not(feature = "raw"))]
            pub(crate) async fn raw(&self) -> Result<$proxy, crate::Error> {
                use zbus::proxy::Defaults;

                let destination = crate::raw::networkmanager::NetworkManagerProxy::DESTINATION
                    .as_ref()
                    .ok_or(crate::Error::MissingDestination)?;
                <$proxy>::builder(&self.zbus)
                    .destination(destination)?
                    .build()
                    .await
                    .map_err(crate::Error::ZBus)
            }

            /// Get the raw D-Bus proxy.
            #[cfg(feature = "raw")]
            pub async fn raw(&self) -> Result<$proxy, crate::Error> {
                use zbus::proxy::Defaults;

                let destination = crate::raw::networkmanager::NetworkManagerProxy::DESTINATION
                    .as_ref()
                    .ok_or(crate::Error::MissingDestination)?;
                <$proxy>::builder(&self.zbus)
                    .destination(destination)?
                    .build()
                    .await
                    .map_err(crate::Error::ZBus)
            }
        }
    };
}
macro_rules! zproxy_pathed {
    ($facade:ty, $proxy:ty) => {
        impl $facade {
            #[cfg(not(feature = "raw"))]
            pub(crate) async fn raw(&self) -> Result<$proxy, crate::Error> {
                use zbus::proxy::Defaults;

                let destination = crate::raw::networkmanager::NetworkManagerProxy::DESTINATION
                    .as_ref()
                    .ok_or(crate::Error::MissingDestination)?;
                <$proxy>::builder(&self.zbus)
                    .path(&self.path)?
                    .destination(destination)?
                    .build()
                    .await
                    .map_err(crate::Error::ZBus)
            }

            /// Get the raw D-Bus proxy.
            #[cfg(feature = "raw")]
            pub async fn raw(&self) -> Result<$proxy, crate::Error> {
                use zbus::proxy::Defaults;

                let destination = crate::raw::networkmanager::NetworkManagerProxy::DESTINATION
                    .as_ref()
                    .ok_or(crate::Error::MissingDestination)?;
                <$proxy>::builder(&self.zbus)
                    .path(&self.path)?
                    .destination(destination)?
                    .build()
                    .await
                    .map_err(crate::Error::ZBus)
            }
        }
    };
}
macro_rules! zproxy_sub {
    ($parent:ty, $facade:ty, $proxy:ty) => {
        impl $facade {
            #[cfg(not(feature = "raw"))]
            pub(crate) async fn raw(&self) -> Result<$proxy, crate::Error> {
                use zbus::proxy::Defaults;

                let destination = crate::raw::networkmanager::NetworkManagerProxy::DESTINATION
                    .as_ref()
                    .ok_or(crate::Error::MissingDestination)?;
                <$proxy>::builder(&self.parent.zbus)
                    .path(&self.parent.path)?
                    .destination(destination)?
                    .build()
                    .await
                    .map_err(crate::Error::ZBus)
            }

            /// Get the raw D-Bus proxy.
            #[cfg(feature = "raw")]
            pub async fn raw(&self) -> Result<$proxy, crate::Error> {
                use zbus::proxy::Defaults;

                let destination = crate::raw::networkmanager::NetworkManagerProxy::DESTINATION
                    .as_ref()
                    .ok_or(crate::Error::MissingDestination)?;
                <$proxy>::builder(&self.parent.zbus)
                    .path(&self.parent.path)?
                    .destination(destination)?
                    .build()
                    .await
                    .map_err(crate::Error::ZBus)
            }

            pub(crate) fn new(parent: $parent) -> Self {
                Self { parent }
            }
        }
    };
}
pub(crate) use zproxy_pathed;
pub(crate) use zproxy_sub;
pub(crate) use zproxy_unpathed;
