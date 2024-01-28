use crate::dbus_api::DBusAccessor;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerDHCP4Config;

#[derive(Clone, Debug)]
pub struct Dhcp4Config {
    dbus_accessor: DBusAccessor,
}

impl Dhcp4Config {
    pub(crate) fn new(dbus_accessor: DBusAccessor) -> Self {
        Dhcp4Config { dbus_accessor }
    }

    pub fn options(
        &self,
    ) -> Result<
        std::collections::HashMap<
            String,
            dbus::arg::Variant<std::boxed::Box<(dyn dbus::arg::RefArg + 'static)>>,
        >,
        Error,
    > {
        Ok(proxy!(self).options()?)
    }
}
