// This code was autogenerated with `dbus-codegen-rust -s -g -m None -d org.bluez -p /org/bluez/hci0 -i org.freedesktop.DBus`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait Introspectable {
    fn introspect(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> Introspectable for blocking::Proxy<'a, C> {

    fn introspect(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgBluezAdapter1 {
    fn start_discovery(&self) -> Result<(), dbus::Error>;
    fn set_discovery_filter(&self, properties: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(), dbus::Error>;
    fn stop_discovery(&self) -> Result<(), dbus::Error>;
    fn remove_device(&self, device: dbus::Path) -> Result<(), dbus::Error>;
    fn get_discovery_filters(&self) -> Result<Vec<String>, dbus::Error>;
    fn address(&self) -> Result<String, dbus::Error>;
    fn address_type(&self) -> Result<String, dbus::Error>;
    fn name(&self) -> Result<String, dbus::Error>;
    fn alias(&self) -> Result<String, dbus::Error>;
    fn set_alias(&self, value: String) -> Result<(), dbus::Error>;
    fn class(&self) -> Result<u32, dbus::Error>;
    fn powered(&self) -> Result<bool, dbus::Error>;
    fn set_powered(&self, value: bool) -> Result<(), dbus::Error>;
    fn discoverable(&self) -> Result<bool, dbus::Error>;
    fn set_discoverable(&self, value: bool) -> Result<(), dbus::Error>;
    fn discoverable_timeout(&self) -> Result<u32, dbus::Error>;
    fn set_discoverable_timeout(&self, value: u32) -> Result<(), dbus::Error>;
    fn pairable(&self) -> Result<bool, dbus::Error>;
    fn set_pairable(&self, value: bool) -> Result<(), dbus::Error>;
    fn pairable_timeout(&self) -> Result<u32, dbus::Error>;
    fn set_pairable_timeout(&self, value: u32) -> Result<(), dbus::Error>;
    fn discovering(&self) -> Result<bool, dbus::Error>;
    fn uuids(&self) -> Result<Vec<String>, dbus::Error>;
    fn modalias(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgBluezAdapter1 for blocking::Proxy<'a, C> {

    fn start_discovery(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Adapter1", "StartDiscovery", ())
    }

    fn set_discovery_filter(&self, properties: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Adapter1", "SetDiscoveryFilter", (properties, ))
    }

    fn stop_discovery(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Adapter1", "StopDiscovery", ())
    }

    fn remove_device(&self, device: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Adapter1", "RemoveDevice", (device, ))
    }

    fn get_discovery_filters(&self) -> Result<Vec<String>, dbus::Error> {
        self.method_call("org.bluez.Adapter1", "GetDiscoveryFilters", ())
            .and_then(|r: (Vec<String>, )| Ok(r.0, ))
    }

    fn address(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Address")
    }

    fn address_type(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "AddressType")
    }

    fn name(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Name")
    }

    fn alias(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Alias")
    }

    fn class(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Class")
    }

    fn powered(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Powered")
    }

    fn discoverable(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Discoverable")
    }

    fn discoverable_timeout(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "DiscoverableTimeout")
    }

    fn pairable(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Pairable")
    }

    fn pairable_timeout(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "PairableTimeout")
    }

    fn discovering(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Discovering")
    }

    fn uuids(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "UUIDs")
    }

    fn modalias(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Modalias")
    }

    fn set_alias(&self, value: String) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "Alias", value)
    }

    fn set_powered(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "Powered", value)
    }

    fn set_discoverable(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "Discoverable", value)
    }

    fn set_discoverable_timeout(&self, value: u32) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "DiscoverableTimeout", value)
    }

    fn set_pairable(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "Pairable", value)
    }

    fn set_pairable_timeout(&self, value: u32) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "PairableTimeout", value)
    }
}

pub trait Properties {
    fn get<R0: for<'b> arg::Get<'b> + 'static>(&self, interface: &str, name: &str) -> Result<R0, dbus::Error>;
    fn set<I2: arg::Arg + arg::Append>(&self, interface: &str, name: &str, value: I2) -> Result<(), dbus::Error>;
    fn get_all(&self, interface: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> Properties for blocking::Proxy<'a, C> {

    fn get<R0: for<'b> arg::Get<'b> + 'static>(&self, interface: &str, name: &str) -> Result<R0, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Get", (interface, name, ))
            .and_then(|r: (arg::Variant<R0>, )| Ok((r.0).0, ))
    }

    fn set<I2: arg::Arg + arg::Append>(&self, interface: &str, name: &str, value: I2) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Set", (interface, name, arg::Variant(value), ))
    }

    fn get_all(&self, interface: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "GetAll", (interface, ))
            .and_then(|r: (::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, )| Ok(r.0, ))
    }
}

#[derive(Debug)]
pub struct PropertiesPropertiesChanged {
    pub interface: String,
    pub changed_properties: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for PropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for PropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(PropertiesPropertiesChanged {
            interface: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for PropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

pub trait OrgBluezGattManager1 {
    fn register_application(&self, application: dbus::Path, options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(), dbus::Error>;
    fn unregister_application(&self, application: dbus::Path) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgBluezGattManager1 for blocking::Proxy<'a, C> {

    fn register_application(&self, application: dbus::Path, options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.GattManager1", "RegisterApplication", (application, options, ))
    }

    fn unregister_application(&self, application: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.GattManager1", "UnregisterApplication", (application, ))
    }
}

pub trait OrgBluezLEAdvertisingManager1 {
    fn register_advertisement(&self, advertisement: dbus::Path, options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(), dbus::Error>;
    fn unregister_advertisement(&self, service: dbus::Path) -> Result<(), dbus::Error>;
    fn active_instances(&self) -> Result<u8, dbus::Error>;
    fn supported_instances(&self) -> Result<u8, dbus::Error>;
    fn supported_includes(&self) -> Result<Vec<String>, dbus::Error>;
    fn supported_secondary_channels(&self) -> Result<Vec<String>, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgBluezLEAdvertisingManager1 for blocking::Proxy<'a, C> {

    fn register_advertisement(&self, advertisement: dbus::Path, options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.LEAdvertisingManager1", "RegisterAdvertisement", (advertisement, options, ))
    }

    fn unregister_advertisement(&self, service: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.LEAdvertisingManager1", "UnregisterAdvertisement", (service, ))
    }

    fn active_instances(&self) -> Result<u8, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.LEAdvertisingManager1", "ActiveInstances")
    }

    fn supported_instances(&self) -> Result<u8, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.LEAdvertisingManager1", "SupportedInstances")
    }

    fn supported_includes(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.LEAdvertisingManager1", "SupportedIncludes")
    }

    fn supported_secondary_channels(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.LEAdvertisingManager1", "SupportedSecondaryChannels")
    }
}

pub trait OrgBluezMedia1 {
    fn register_endpoint(&self, endpoint: dbus::Path, properties: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(), dbus::Error>;
    fn unregister_endpoint(&self, endpoint: dbus::Path) -> Result<(), dbus::Error>;
    fn register_player(&self, player: dbus::Path, properties: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(), dbus::Error>;
    fn unregister_player(&self, player: dbus::Path) -> Result<(), dbus::Error>;
    fn register_application(&self, application: dbus::Path, options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(), dbus::Error>;
    fn unregister_application(&self, application: dbus::Path) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgBluezMedia1 for blocking::Proxy<'a, C> {

    fn register_endpoint(&self, endpoint: dbus::Path, properties: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Media1", "RegisterEndpoint", (endpoint, properties, ))
    }

    fn unregister_endpoint(&self, endpoint: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Media1", "UnregisterEndpoint", (endpoint, ))
    }

    fn register_player(&self, player: dbus::Path, properties: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Media1", "RegisterPlayer", (player, properties, ))
    }

    fn unregister_player(&self, player: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Media1", "UnregisterPlayer", (player, ))
    }

    fn register_application(&self, application: dbus::Path, options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Media1", "RegisterApplication", (application, options, ))
    }

    fn unregister_application(&self, application: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Media1", "UnregisterApplication", (application, ))
    }
}

pub trait OrgBluezNetworkServer1 {
    fn register(&self, uuid: &str, bridge: &str) -> Result<(), dbus::Error>;
    fn unregister(&self, uuid: &str) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgBluezNetworkServer1 for blocking::Proxy<'a, C> {

    fn register(&self, uuid: &str, bridge: &str) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.NetworkServer1", "Register", (uuid, bridge, ))
    }

    fn unregister(&self, uuid: &str) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.NetworkServer1", "Unregister", (uuid, ))
    }
}
