use anyhow::Error;
use retro_dat::{DatReader, Status};
use std::{collections::HashMap, path::Path};

#[derive(Clone, Debug)]
pub struct DatFile {
    pub header: String,
    pub version: String,
    pub games: HashMap<String, DatGame>,
}

#[derive(Clone, Debug)]
pub struct DatGame {
    pub rom_verified: bool,
}

pub fn from_path<P: AsRef<Path>>(path: P) -> Result<DatFile, Error> {
    let mut dat_reader = DatReader::from_file(path)?;
    dat_reader.set_strict(false);
    let data_file = dat_reader.read_all()?;
    let games = data_file
        .games
        .into_iter()
        .map(|game| {
            let rom_verified =
                game.roms.len() > 0 && game.roms.iter().all(|rom| rom.status == Status::Verified);
            (game.name, DatGame { rom_verified })
        })
        .collect();
    let (header, version) = data_file
        .header
        .map(|header| (header.description, header.version))
        .unwrap_or_else(|| (String::new(), String::new()));
    Ok(DatFile {
        header,
        version,
        games,
    })
}
