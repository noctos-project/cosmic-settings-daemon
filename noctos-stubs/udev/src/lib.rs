//! Stand-in for the `udev` crate on systems without libudev (illumos).
//!
//! It offers the small part of the API that cosmic-settings-daemon uses and
//! reports that there are no devices: enumeration is empty and a monitor
//! cannot be created. The daemon then runs with no backlight device, which is
//! what it does on a Linux machine that has none.

use std::ffi::OsStr;
use std::io;
use std::ops::Deref;
use std::os::fd::{AsRawFd, RawFd};
use std::path::Path;

fn unsupported() -> io::Error {
    io::Error::new(io::ErrorKind::Unsupported, "udev is not available on this system")
}

#[derive(Clone, Debug)]
pub struct Device(());

impl Device {
    pub fn sysname(&self) -> &OsStr {
        OsStr::new("")
    }

    pub fn syspath(&self) -> &Path {
        Path::new("")
    }
}

pub struct Enumerator(());

impl Enumerator {
    pub fn new() -> io::Result<Self> {
        Ok(Self(()))
    }

    pub fn match_subsystem<T: AsRef<OsStr>>(&mut self, _subsystem: T) -> io::Result<()> {
        Ok(())
    }

    pub fn scan_devices(&mut self) -> io::Result<std::vec::IntoIter<Device>> {
        Ok(Vec::new().into_iter())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventType {
    Add,
    Change,
    Remove,
    Bind,
    Unbind,
    Unknown,
}

pub struct Event(Device);

impl Event {
    pub fn event_type(&self) -> EventType {
        EventType::Unknown
    }

    pub fn device(&self) -> Device {
        self.0.clone()
    }
}

impl Deref for Event {
    type Target = Device;

    fn deref(&self) -> &Device {
        &self.0
    }
}

pub struct MonitorBuilder(());

impl MonitorBuilder {
    pub fn new() -> io::Result<Self> {
        Ok(Self(()))
    }

    pub fn match_subsystem<T: AsRef<OsStr>>(self, _subsystem: T) -> io::Result<Self> {
        Ok(self)
    }

    pub fn listen(self) -> io::Result<MonitorSocket> {
        Err(unsupported())
    }
}

/// Never constructed: `MonitorBuilder::listen` always fails.
pub struct MonitorSocket(RawFd);

impl MonitorSocket {
    pub fn iter(&self) -> std::vec::IntoIter<Event> {
        Vec::new().into_iter()
    }
}

impl AsRawFd for MonitorSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}
