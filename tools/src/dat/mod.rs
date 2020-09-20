use anyhow::Error;
use retro_dat::DatReader;
use std::collections::HashSet;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct DatFile {
    pub header: String,
    pub version: String,
    pub names: HashSet<String>,
}

pub fn from_path<P: AsRef<Path>>(path: P) -> Result<DatFile, Error> {
    let mut dat_reader = DatReader::from_file(path)?;
    dat_reader.set_strict(false);
    let data_file = dat_reader.read_all()?;
    let names = data_file
        .games
        .into_iter()
        .map(|game| game.description)
        .collect();
    let (header, version) = data_file
        .header
        .map(|header| (header.description, header.version))
        .unwrap_or_else(|| (String::new(), String::new()));
    Ok(DatFile {
        header,
        version,
        names,
    })
}
