use std::{
    fs::read_dir,
    io::Result,
    path::{Path, PathBuf},
};

#[derive(Default)]
pub struct Timezones {
    zones: Vec<Zone>,
}

impl Timezones {
    pub fn new() -> Result<Self> {
        let mut output = Self::default();

        for zone in read_dir("/usr/share/zoneinfo/")? {
            let zone = zone?;
            let zone_path = zone.path();
            if zone_path.is_dir() {
                let zone_name = zone.file_name().into_string().expect("Expected a Zone");
                let mut regions = Vec::new();
                for region in zone_path.read_dir()? {
                    let region = region?;
                    let region_path = region.path();
                    let region_name = region.file_name().into_string().expect("Expected a Region");
                    regions.push(Region {
                        name: region_name,
                        path: region_path,
                    });
                }

                regions.sort_unstable();
                output.zones.push(Zone {
                    name: zone_name,
                    regions,
                });
            }
        }

        output.zones.sort_unstable();
        Ok(output)
    }

    #[must_use]
    pub fn zones(&self) -> &Vec<Zone> {
        &self.zones
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Zone {
    name: String,
    regions: Vec<Region>,
}

impl Zone {
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn regions(&self) -> &Vec<Region> {
        &self.regions
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Region {
    name: String,
    path: PathBuf,
}

impl Region {
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}
