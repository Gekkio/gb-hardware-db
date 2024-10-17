// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use anyhow::Error;
use gbhwdb_backend::hash::{Crc32, Md5, Sha1, Sha256};
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
    pub id: String,
    pub name: String,
    pub rom_verified: bool,
    pub crc32: Option<Crc32>,
    pub md5: Option<Md5>,
    pub sha1: Option<Sha1>,
    pub sha256: Option<Sha256>,
}

pub fn from_path<P: AsRef<Path>>(path: P) -> Result<DatFile, Error> {
    let mut dat_reader = DatReader::from_file(path)?;
    dat_reader.set_strict(false);
    let data_file = dat_reader.read_all()?;
    let games = data_file
        .games
        .into_iter()
        .map(|game| {
            let (rom_verified, crc32, md5, sha1, sha256) = {
                match game.roms.first() {
                    Some(rom) => (
                        rom.status == Status::Verified,
                        Crc32::parse(&rom.crc).ok(),
                        Md5::parse(&rom.md5).ok(),
                        Sha1::parse(&rom.sha1).ok(),
                        Sha256::parse(&rom.sha256).ok(),
                    ),
                    None => (false, None, None, None, None),
                }
            };
            (
                game.name.clone(),
                DatGame {
                    id: game.id,
                    name: game.name,
                    rom_verified,
                    crc32,
                    md5,
                    sha1,
                    sha256,
                },
            )
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
