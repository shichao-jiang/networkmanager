use crate::dbus_api::DBusAccessor;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerDHCP6Config;

#[derive(Clone, Debug)]
pub struct Dhcp6Config {
    dbus_accessor: DBusAccessor,
}

impl Dhcp6Config {
    pub(crate) fn new(dbus_accessor: DBusAccessor) -> Self {
        Dhcp6Config { dbus_accessor }
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
