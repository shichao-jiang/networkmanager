//! A [NetworkManager](https://wiki.gnome.org/Projects/NetworkManager) API library using the [D-Bus message bus system](https://www.freedesktop.org/wiki/Software/dbus/)
//!
//! ## Usage
//!
//! ```toml
//! [dependencies]
//! networkmanager = { package = "passcod-networkmanager", version = "0.5.0" }
//! ```
//!
//! ## Example
//!
//! ```rust,no_run
//! # use passcod_networkmanager as networkmanager;
//! use networkmanager::devices::{Any, Device, Wired, Wireless};
//! use networkmanager::{Error, NetworkManager};
//!
//! fn main() -> Result<(), Error> {
//!     let nm = NetworkManager::new()?;
//!
//!     for dev in nm.get_devices()? {
//!         match dev {
//!             Device::Ethernet(x) => {
//!                 println!("Is autoconnected: {:?}", x.autoconnect()?);
//!                 println!("Speed: {:?}", x.speed()?);
//!                 println!("S390 Subchannels: {:?}", x.s390_subchannels()?);
//!                 println!("Carrier: {:?}", x.carrier()?);
//!             }
//!             Device::WiFi(x) => {
//!                 println!("Bitrate: {:?}", x.bitrate()?);
//!                 x.request_scan(std::collections::HashMap::new())?;
//!                 for ap in x.get_all_access_points()? {
//!                     println!("SSID: {:?}", ap.ssid()?);
//!                 }
//!             }
//!             _ => {}
//!         }
//!     }
//!
//!     let eth0 = nm.get_device_by_ip_iface("eth0")?;
//!     match eth0 {
//!         Device::Ethernet(x) => {
//!             // NetworkManager >= 1.24
//!             // println!("Hardware Address: {:?}", Any::hw_address(&x)?);
//!
//!             // NetworkManager < 1.24
//!             // println!("Hardware Address: {:?}", Wired::hw_address(&x)?);
//!
//!             println!("Speed: {:?}", x.speed()?);
//!         }
//!         _ => {}
//!     }
//!
//!     Ok(())
//! }
//! ```

#![deny(rust_2018_idioms)]

mod gen;
#[macro_use]
mod dbus_api;
mod accesspoint;
mod errors;
mod networkmanager;
mod settings;

pub mod configs;
pub mod connection;
pub mod devices;
pub mod types;

pub use crate::accesspoint::AccessPoint;
pub use crate::errors::Error;
pub use crate::networkmanager::NetworkManager;
pub use crate::settings::Settings;
