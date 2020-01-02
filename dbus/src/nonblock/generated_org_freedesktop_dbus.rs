// This code was autogenerated with `dbus-codegen-rust -d org.freedesktop.DBus -p /org/freedesktop/DBus -m none -c nonblock -g -i org.freedesktop. --dbuscrate crate --filter DBus -o dbus/src/nonblock/generated_org_freedesktop_dbus.rs`, see https://github.com/diwic/dbus-rs
#![allow(missing_docs)]
use crate as dbus;
use crate::arg;
use crate::nonblock;

pub trait DBus {
    fn hello(&self) -> nonblock::MethodReply<String>;
    fn request_name(&self, arg0: &str, arg1: u32) -> nonblock::MethodReply<u32>;
    fn release_name(&self, arg0: &str) -> nonblock::MethodReply<u32>;
    fn start_service_by_name(&self, arg0: &str, arg1: u32) -> nonblock::MethodReply<u32>;
    fn update_activation_environment(&self, arg0: ::std::collections::HashMap<&str, &str>) -> nonblock::MethodReply<()>;
    fn name_has_owner(&self, arg0: &str) -> nonblock::MethodReply<bool>;
    fn list_names(&self) -> nonblock::MethodReply<Vec<String>>;
    fn list_activatable_names(&self) -> nonblock::MethodReply<Vec<String>>;
    fn add_match(&self, arg0: &str) -> nonblock::MethodReply<()>;
    fn remove_match(&self, arg0: &str) -> nonblock::MethodReply<()>;
    fn get_name_owner(&self, arg0: &str) -> nonblock::MethodReply<String>;
    fn list_queued_owners(&self, arg0: &str) -> nonblock::MethodReply<Vec<String>>;
    fn get_connection_unix_user(&self, arg0: &str) -> nonblock::MethodReply<u32>;
    fn get_connection_unix_process_id(&self, arg0: &str) -> nonblock::MethodReply<u32>;
    fn get_adt_audit_session_data(&self, arg0: &str) -> nonblock::MethodReply<Vec<u8>>;
    fn get_connection_selinux_security_context(&self, arg0: &str) -> nonblock::MethodReply<Vec<u8>>;
    fn reload_config(&self) -> nonblock::MethodReply<()>;
    fn get_id(&self) -> nonblock::MethodReply<String>;
    fn get_connection_credentials(&self, arg0: &str) -> nonblock::MethodReply<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>>;
    fn features(&self) -> nonblock::MethodReply<Vec<String>>;
    fn interfaces(&self) -> nonblock::MethodReply<Vec<String>>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target=T>> DBus for nonblock::Proxy<'a, C> {

    fn hello(&self) -> nonblock::MethodReply<String> {
        self.method_call("org.freedesktop.DBus", "Hello", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }

    fn request_name(&self, arg0: &str, arg1: u32) -> nonblock::MethodReply<u32> {
        self.method_call("org.freedesktop.DBus", "RequestName", (arg0, arg1, ))
            .and_then(|r: (u32, )| Ok(r.0, ))
    }

    fn release_name(&self, arg0: &str) -> nonblock::MethodReply<u32> {
        self.method_call("org.freedesktop.DBus", "ReleaseName", (arg0, ))
            .and_then(|r: (u32, )| Ok(r.0, ))
    }

    fn start_service_by_name(&self, arg0: &str, arg1: u32) -> nonblock::MethodReply<u32> {
        self.method_call("org.freedesktop.DBus", "StartServiceByName", (arg0, arg1, ))
            .and_then(|r: (u32, )| Ok(r.0, ))
    }

    fn update_activation_environment(&self, arg0: ::std::collections::HashMap<&str, &str>) -> nonblock::MethodReply<()> {
        self.method_call("org.freedesktop.DBus", "UpdateActivationEnvironment", (arg0, ))
    }

    fn name_has_owner(&self, arg0: &str) -> nonblock::MethodReply<bool> {
        self.method_call("org.freedesktop.DBus", "NameHasOwner", (arg0, ))
            .and_then(|r: (bool, )| Ok(r.0, ))
    }

    fn list_names(&self) -> nonblock::MethodReply<Vec<String>> {
        self.method_call("org.freedesktop.DBus", "ListNames", ())
            .and_then(|r: (Vec<String>, )| Ok(r.0, ))
    }

    fn list_activatable_names(&self) -> nonblock::MethodReply<Vec<String>> {
        self.method_call("org.freedesktop.DBus", "ListActivatableNames", ())
            .and_then(|r: (Vec<String>, )| Ok(r.0, ))
    }

    fn add_match(&self, arg0: &str) -> nonblock::MethodReply<()> {
        self.method_call("org.freedesktop.DBus", "AddMatch", (arg0, ))
    }

    fn remove_match(&self, arg0: &str) -> nonblock::MethodReply<()> {
        self.method_call("org.freedesktop.DBus", "RemoveMatch", (arg0, ))
    }

    fn get_name_owner(&self, arg0: &str) -> nonblock::MethodReply<String> {
        self.method_call("org.freedesktop.DBus", "GetNameOwner", (arg0, ))
            .and_then(|r: (String, )| Ok(r.0, ))
    }

    fn list_queued_owners(&self, arg0: &str) -> nonblock::MethodReply<Vec<String>> {
        self.method_call("org.freedesktop.DBus", "ListQueuedOwners", (arg0, ))
            .and_then(|r: (Vec<String>, )| Ok(r.0, ))
    }

    fn get_connection_unix_user(&self, arg0: &str) -> nonblock::MethodReply<u32> {
        self.method_call("org.freedesktop.DBus", "GetConnectionUnixUser", (arg0, ))
            .and_then(|r: (u32, )| Ok(r.0, ))
    }

    fn get_connection_unix_process_id(&self, arg0: &str) -> nonblock::MethodReply<u32> {
        self.method_call("org.freedesktop.DBus", "GetConnectionUnixProcessID", (arg0, ))
            .and_then(|r: (u32, )| Ok(r.0, ))
    }

    fn get_adt_audit_session_data(&self, arg0: &str) -> nonblock::MethodReply<Vec<u8>> {
        self.method_call("org.freedesktop.DBus", "GetAdtAuditSessionData", (arg0, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn get_connection_selinux_security_context(&self, arg0: &str) -> nonblock::MethodReply<Vec<u8>> {
        self.method_call("org.freedesktop.DBus", "GetConnectionSELinuxSecurityContext", (arg0, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn reload_config(&self) -> nonblock::MethodReply<()> {
        self.method_call("org.freedesktop.DBus", "ReloadConfig", ())
    }

    fn get_id(&self) -> nonblock::MethodReply<String> {
        self.method_call("org.freedesktop.DBus", "GetId", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }

    fn get_connection_credentials(&self, arg0: &str) -> nonblock::MethodReply<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>> {
        self.method_call("org.freedesktop.DBus", "GetConnectionCredentials", (arg0, ))
            .and_then(|r: (::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, )| Ok(r.0, ))
    }

    fn features(&self) -> nonblock::MethodReply<Vec<String>> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.DBus", "Features")
    }

    fn interfaces(&self) -> nonblock::MethodReply<Vec<String>> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.DBus", "Interfaces")
    }
}

#[derive(Debug)]
pub struct DBusNameOwnerChanged {
    pub arg0: String,
    pub arg1: String,
    pub arg2: String,
}

impl arg::AppendAll for DBusNameOwnerChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.arg0, i);
        arg::RefArg::append(&self.arg1, i);
        arg::RefArg::append(&self.arg2, i);
    }
}

impl arg::ReadAll for DBusNameOwnerChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(DBusNameOwnerChanged {
            arg0: i.read()?,
            arg1: i.read()?,
            arg2: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for DBusNameOwnerChanged {
    const NAME: &'static str = "NameOwnerChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus";
}

#[derive(Debug)]
pub struct DBusNameLost {
    pub arg0: String,
}

impl arg::AppendAll for DBusNameLost {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.arg0, i);
    }
}

impl arg::ReadAll for DBusNameLost {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(DBusNameLost {
            arg0: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for DBusNameLost {
    const NAME: &'static str = "NameLost";
    const INTERFACE: &'static str = "org.freedesktop.DBus";
}

#[derive(Debug)]
pub struct DBusNameAcquired {
    pub arg0: String,
}

impl arg::AppendAll for DBusNameAcquired {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.arg0, i);
    }
}

impl arg::ReadAll for DBusNameAcquired {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(DBusNameAcquired {
            arg0: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for DBusNameAcquired {
    const NAME: &'static str = "NameAcquired";
    const INTERFACE: &'static str = "org.freedesktop.DBus";
}