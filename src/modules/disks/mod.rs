use std::ffi::OsStr;

use libparted::{Device, DeviceType};

use self::data::disk::Disk;

mod data;
mod serial;

#[derive(Default)]
pub struct Disks {
    pub physical: Vec<Disk>,
}

impl Disks {
    pub fn add(&mut self, disk: Disk) {
        self.physical.push(disk);
    }

    #[must_use]
    pub fn probe() -> Disks {
        let mut disks = Self::default();
        for mut device in Device::devices(true) {
            if let Some(_name) = device.path().file_name().and_then(OsStr::to_str) {
                log::info!("Probed device {}", device.path().display());

                match device.type_() {
                    DeviceType::PED_DEVICE_UNKNOWN
                    | DeviceType::PED_DEVICE_LOOP
                    | DeviceType::PED_DEVICE_FILE
                    | DeviceType::PED_DEVICE_DM => continue,
                    _ => disks.add(Disk::new(&mut device)),
                }
            }
        }

        disks
    }
}
