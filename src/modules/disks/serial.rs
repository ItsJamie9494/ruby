use std::{
    io::{Error, ErrorKind, Result},
    path::Path,
    process::Command,
};

const PATTERN: &str = "E: ID_SERIAL=";

pub fn get_serial(path: &Path) -> Result<String> {
    log::info!("Obtaining serial model from {}", path.display());
    Command::new("udevadm")
        .args(&["info", "--query=all", &format!("--name={}", path.display())])
        .output()
        .and_then(|output| parse_serial(&output.stdout))
}

fn parse_serial(data: &[u8]) -> Result<String> {
    String::from_utf8_lossy(data)
        .lines()
        .find(|line| line.starts_with(PATTERN))
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "No serial field"))
        .map(|serial| serial.split_at(PATTERN.len()).1.into())
}

// TODO: Instead of using a command, use this parser :)
// pub fn get_serial(path: &Path) {
//     let resolved = path.read_link();

//     let name = match resolved.as_ref() {
//         Ok(resolved) => resolved.file_name(),
//         _ => path.file_name(),
//     };
//     let device = [
//         "/sys/class/block/",
//         name.expect("Missing file_name")
//             .to_str()
//             .unwrap_or_default(),
//         "/uevent",
//     ]
//     .concat();

//     match Path::new(&device).is_file() {
//         true => todo!(),
//         false => todo!(),
//     }
// }
