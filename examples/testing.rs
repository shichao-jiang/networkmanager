// use passcod_networkmanager::devices::{Any, Device, Wired, Wireless};
use passcod_networkmanager::{Error, NetworkManager};

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
            // } else if let Some(eth) = dev.to_ethernet().await? {
            //     println!("Is autoconnected: {:?}", x.autoconnect()?);
            //     println!("Speed: {:?}", x.speed()?);
            //     println!("S390 Subchannels: {:?}", x.s390_subchannels()?);
            //     println!("Carrier: {:?}", x.carrier()?);
            //     let conf = x.ip4_config()?;
            //     println!("Gateway: {:?}", conf.gateway()?);
            //     let con = x.active_connection()?;
            //     println!("Connection id: {}", con.id()?);
        }
    }

    // let eth0 = nm.get_device_by_ip_interface_name("eth0").await?;
    // match eth0 {
    //     Device::Ethernet(x) => {
    //         // NetworkManager >= 1.24
    //         // println!("Hardware Address: {:?}", Any::hw_address(&x)?);

    //         // NetworkManager < 1.24
    //         // println!("Hardware Address: {:?}", Wired::hw_address(&x)?);

    //         println!("Speed: {:?}", x.speed()?);
    //     }
    //     _ => {}
    // }

    Ok(())
}
