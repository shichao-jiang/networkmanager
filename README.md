# NetworkManager bindings for Rust (fork)

> Fork by @passcod to rewrite a few APIs to be more ergonomic.

[NetworkManager](https://wiki.gnome.org/Projects/NetworkManager) bindings for Rust using the [D-Bus message bus system](https://www.freedesktop.org/wiki/Software/dbus/)

## Status

This project is still under development. Currently implemented parts can be found in the docs.

- NetworkManager D-Bus API >= v1.42.2

## Usage

Add networkmanager to your `Cargo.toml` with:

```toml
[dependencies]
networkmanager = { package = "passcod-networkmanager", version = "=0.8.0" }
tokio = { version = "1", features = ["full"] }
```

## Example

```rust,no_run
use networkmanager::{Error, NetworkManager};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let nm = NetworkManager::new().await?;

    for dev in nm.get_devices().await? {
        if let Some(wifi) = dev.to_wireless().await? {
            println!("Bitrate: {:?}", wifi.bitrate().await?);
            wifi.request_scan().await?;
            for ap in wifi.get_all_access_points().await? {
                let raw = ap.ssid().await?;
                println!("SSID: {} {raw:02x?}", String::from_utf8_lossy(&raw));
            }
        }
    }

    Ok(())
}
```

## Build prerequisites

- ### Debian and its derivatives (e.g. Ubuntu)

  - network-manager
  - libdbus-1-dev
  - pkg-config

- ### Fedora

  - NetworkManager
  - dbus-devel
  - pkg-config

## Todo

- Implementations
  - Devices
    - [x] Top level
    - [ ] Generic
    - [x] Wireless
    - [ ] Wired
    - [ ] ADSL
    - [ ] Bluetooth
    - [ ] Bond
    - [ ] Bridge
    - [ ] Dummy
    - [ ] Infiniband
    - [ ] IpTunnel
    - [ ] Lowpan
    - [ ] Macsec
    - [ ] MacVLAN
    - [ ] Modem
    - [ ] OLPCMesh
    - [ ] OVSBridge
    - [ ] OVSInterface
    - [ ] OVSPort
    - [ ] PPP
    - [ ] Statistics
    - [ ] Team
    - [ ] TUN/TAP
    - [ ] VETH
    - [ ] VLAN
    - [ ] VRF
    - [ ] VXLAN
    - [ ] WifiP2P
    - [ ] WiMax
    - [ ] Wireguard
    - [ ] Wpan
  - Configs
    - [x] IP4
    - [ ] IP6
    - [ ] DHCP4
    - [ ] DHCP6
  - [x] Accesspoint
  - [ ] ConnectionActive
  - [x] NetworkManager (partially implemented)
  - [ ] AgentManager
  - [ ] Checkpoint
  - [ ] DNSManager
  - [ ] PPP
  - [ ] SecretAgent
  - [x] Settings
  - [x] Settings Connection
  - [ ] VPN Connection
  - [ ] VPN Plugin
  - [ ] WifiP2P
  - [ ] Wimax NSP

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
