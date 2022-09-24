#![deny(clippy::pedantic)]

use clap::{App, Arg};
use ruby::{
    log::Logger,
    modules::{disks::Disks, timezones::Timezones},
};

fn main() {
    let matches = App::new("ruby")
        .arg(
            Arg::with_name("timezone")
                .long("tz")
                .help("The timezone for the install")
                .value_delimiter('/')
                .min_values(2)
                .max_values(2),
        )
        .get_matches();

    if let Err(err) = Logger::new(None, None).log() {
        eprintln!("Failed to initalise logger: {}", err);
    }

    let timezones;
    let timezone = match matches.values_of("timezone") {
        Some(mut tz) => {
            let (zone, region) = (
                tz.next().expect("Expected a Zone"),
                tz.next().expect("Expected a Region"),
            );

            timezones = Timezones::new().expect("Failed to get Timezone list");
            let zone = timezones
                .zones()
                .iter()
                .find(|z| z.name() == zone)
                .unwrap_or_else(|| panic!("failed to find zone: {}", zone));
            let region = zone
                .regions()
                .iter()
                .find(|r| r.name() == region)
                .unwrap_or_else(|| panic!("failed to find region: {}", region));
            Some(region)
        }
        None => None,
    };

    let disks = Disks::probe();
    for disk in disks.physical {
        log::info!("{}, {}", disk.device_path.display(), disk.model_name);
    }

    log::info!("{:?}", timezone.expect("Expected a Timezone").path());
}
