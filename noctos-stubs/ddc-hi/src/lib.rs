//! Stand-in for the `ddc-hi` crate on systems where it has no backend
//! (illumos: no i2c-dev, no udev).
//!
//! It offers the part of the API that cosmic-settings-daemon uses and
//! reports that no DDC/CI capable display is connected.

#[derive(Debug)]
pub struct Error(());

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DDC/CI is not available on this system")
    }
}

impl std::error::Error for Error {}

pub struct VcpValue(u16);

impl VcpValue {
    pub fn value(&self) -> u16 {
        self.0
    }
}

pub trait Ddc {
    fn get_vcp_feature(&mut self, code: u8) -> Result<VcpValue, Error>;
    fn set_vcp_feature(&mut self, code: u8, value: u16) -> Result<(), Error>;
}

pub struct Handle(());

impl Ddc for Handle {
    fn get_vcp_feature(&mut self, _code: u8) -> Result<VcpValue, Error> {
        Err(Error(()))
    }

    fn set_vcp_feature(&mut self, _code: u8, _value: u16) -> Result<(), Error> {
        Err(Error(()))
    }
}

pub struct Descriptor {
    pub code: u8,
}

pub struct Database(());

impl Database {
    pub fn get(&self, _code: u8) -> Option<&Descriptor> {
        None
    }
}

pub struct DisplayInfo {
    pub mccs_database: Database,
}

/// Never constructed: `Display::enumerate` is always empty.
pub struct Display {
    pub handle: Handle,
    pub info: DisplayInfo,
}

impl Display {
    pub fn enumerate() -> Vec<Display> {
        Vec::new()
    }

    pub fn update_capabilities(&mut self) -> Result<(), Error> {
        Err(Error(()))
    }
}
