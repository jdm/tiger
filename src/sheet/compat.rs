use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use crate::sheet::{self, Sheet};

pub mod version1;
pub mod version2;
pub mod version3;

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum Version {
    Tiger1,
    Tiger2,
    Tiger3,
}
const CURRENT_VERSION: Version = Version::Tiger3;

#[derive(Deserialize)]
struct Versioned {
    version: Version,
}

#[derive(Serialize)]
struct VersionedSheet<'a> {
    version: Version,
    sheet: &'a Sheet,
}

pub fn read_sheet<T: AsRef<Path>>(path: T) -> anyhow::Result<Sheet> {
    let versioned: Versioned = serde_json::from_reader(BufReader::new(File::open(path.as_ref())?))?;
    sheet::read_file(versioned.version, path)
}

pub fn write_sheet<T: AsRef<Path>>(path: T, sheet: &Sheet) -> anyhow::Result<()> {
    let file = BufWriter::new(File::create(path.as_ref())?);
    let versioned_sheet = VersionedSheet {
        version: CURRENT_VERSION,
        sheet: &sheet,
    };
    serde_json::to_writer_pretty(file, &versioned_sheet)?;
    Ok(())
}
