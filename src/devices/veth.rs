// Virtual Ethernet Device

use super::Device;
use super::VethDevice;
use crate::gen::OrgFreedesktopNetworkManagerDeviceVeth;
use crate::Error;

pub trait Veth {
    fn peer(&self) -> Result<Device, Error>;
}

impl Veth for VethDevice {
    fn peer(&self) -> Result<Device, Error> {
        let path = proxy!(self).peer()?;
        Device::new(self.dbus_accessor.with_path(path))
    }
}
