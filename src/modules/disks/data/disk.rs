use std::path::PathBuf;

use libparted::{Device, DeviceType};

use crate::modules::disks::serial::get_serial;

pub struct Disk {
    pub model_name: String,
    pub serial: String,
    pub device_path: PathBuf,
    pub size: u64,
    pub device_type: String,
}

impl Disk {
    pub fn new(device: &mut Device) -> Disk {
        log::info!("Obtaining disk information for {}", device.path().display());
        let model_name = device.model().into();
        let device_path = device.path().to_owned();
        let serial = match device.type_() {
            DeviceType::PED_DEVICE_DM | DeviceType::PED_DEVICE_LOOP => "".into(),
            _ => get_serial(&device_path).unwrap_or_else(|_| "".into()),
        };
        let size = device.length();
        let device_type = format!("{:?}", device.type_());

        Disk {
            model_name,
            serial,
            device_path,
            size,
            device_type,
        }
    }
}
