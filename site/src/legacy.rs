// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use self::console::{
    LegacyAgbMetadata, LegacyAgsMetadata, LegacyAgsPhotos, LegacyCgbMetadata, LegacyDmgMetadata,
    LegacyDmgPhotos, LegacyGbsMetadata, LegacyMgbMetadata, LegacyMglMetadata, LegacyOxyMetadata,
    LegacySgb2Metadata, LegacySgbMetadata,
};
use crate::process::DateCode;

pub mod cartridge;
pub mod console;

pub type LegacyCartridgeSubmission =
    LegacySubmission<cartridge::LegacyMetadata, LegacyCartridgePhotos>;
pub type LegacyDmgSubmission = LegacySubmission<LegacyDmgMetadata, LegacyDmgPhotos>;
pub type LegacySgbSubmission = LegacySubmission<LegacySgbMetadata, LegacyDefaultPhotos>;
pub type LegacyMgbSubmission = LegacySubmission<LegacyMgbMetadata, LegacyDefaultPhotos>;
pub type LegacyMglSubmission = LegacySubmission<LegacyMglMetadata, LegacyDefaultPhotos>;
pub type LegacySgb2Submission = LegacySubmission<LegacySgb2Metadata, LegacyDefaultPhotos>;
pub type LegacyCgbSubmission = LegacySubmission<LegacyCgbMetadata, LegacyDefaultPhotos>;
pub type LegacyAgbSubmission = LegacySubmission<LegacyAgbMetadata, LegacyDefaultPhotos>;
pub type LegacyAgsSubmission = LegacySubmission<LegacyAgsMetadata, LegacyAgsPhotos>;
pub type LegacyGbsSubmission = LegacySubmission<LegacyGbsMetadata, LegacyDefaultPhotos>;
pub type LegacyOxySubmission = LegacySubmission<LegacyOxyMetadata, LegacyDefaultPhotos>;

pub trait HasDateCode {
    fn date_code(&self) -> DateCode;
}

#[derive(Clone, Debug)]
pub struct LegacySubmission<M, P> {
    pub code: String,
    pub title: String,
    pub slug: String,
    pub sort_group: Option<String>,
    pub contributor: String,
    pub metadata: M,
    pub photos: P,
}

pub trait LegacyMetadata: 'static {
    const PLACEHOLDER_SVG: Option<&'static str> = None;
}

pub trait LegacyPhotos: 'static {
    fn front(&self) -> Option<&LegacyPhoto>;
    fn infos() -> Vec<PhotoInfo<Self>>;
    fn photos(&self) -> Vec<&LegacyPhoto>;
}

pub struct PhotoInfo<P: ?Sized> {
    pub kind: PhotoKind,
    pub label: &'static str,
    pub getter: Box<dyn Fn(&P) -> Option<&LegacyPhoto>>,
}

impl<P: ?Sized> PhotoInfo<P> {
    pub fn new(
        kind: PhotoKind,
        label: &'static str,
        getter: Box<dyn Fn(&P) -> Option<&LegacyPhoto>>,
    ) -> Self {
        PhotoInfo {
            kind,
            label,
            getter,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PhotoKind {
    MainUnit,
    MainBoard,
    Other,
}

#[derive(Clone, Debug, Default)]
pub struct LegacyDefaultPhotos {
    pub front: Option<LegacyPhoto>,
    pub back: Option<LegacyPhoto>,
    pub pcb_front: Option<LegacyPhoto>,
    pub pcb_back: Option<LegacyPhoto>,
}

impl LegacyPhotos for LegacyDefaultPhotos {
    fn infos() -> Vec<PhotoInfo<Self>> {
        vec![
            PhotoInfo::new(PhotoKind::MainUnit, "Front", Box::new(|p| p.front.as_ref())),
            PhotoInfo::new(PhotoKind::MainUnit, "Back", Box::new(|p| p.back.as_ref())),
            PhotoInfo::new(
                PhotoKind::MainBoard,
                "PCB front",
                Box::new(|p| p.pcb_front.as_ref()),
            ),
            PhotoInfo::new(
                PhotoKind::MainBoard,
                "PCB back",
                Box::new(|p| p.pcb_back.as_ref()),
            ),
        ]
    }

    fn front(&self) -> Option<&LegacyPhoto> {
        self.front.as_ref()
    }

    fn photos(&self) -> Vec<&LegacyPhoto> {
        [&self.front, &self.back, &self.pcb_front, &self.pcb_back]
            .iter()
            .filter_map(|photo| photo.as_ref())
            .collect()
    }
}

#[derive(Clone, Debug, Default)]
pub struct LegacyCartridgePhotos {
    pub front: Option<LegacyPhoto>,
    pub pcb_front: Option<LegacyPhoto>,
    pub pcb_back: Option<LegacyPhoto>,
    pub without_battery: Option<LegacyPhoto>,
    pub extra: Option<LegacyPhoto>,
}

impl LegacyPhotos for LegacyCartridgePhotos {
    fn infos() -> Vec<PhotoInfo<Self>> {
        vec![
            PhotoInfo::new(PhotoKind::MainUnit, "Front", Box::new(|p| p.front.as_ref())),
            PhotoInfo::new(
                PhotoKind::MainBoard,
                "PCB front",
                Box::new(|p| p.pcb_front.as_ref()),
            ),
            PhotoInfo::new(
                PhotoKind::MainBoard,
                "PCB back",
                Box::new(|p| p.pcb_back.as_ref()),
            ),
            PhotoInfo::new(
                PhotoKind::MainBoard,
                "Without battery",
                Box::new(|p| p.without_battery.as_ref()),
            ),
            PhotoInfo::new(
                PhotoKind::MainBoard,
                "Extra photo",
                Box::new(|p| p.extra.as_ref()),
            ),
        ]
    }

    fn front(&self) -> Option<&LegacyPhoto> {
        self.front.as_ref()
    }

    fn photos(&self) -> Vec<&LegacyPhoto> {
        [
            &self.front,
            &self.pcb_front,
            &self.pcb_back,
            &self.without_battery,
            &self.extra,
        ]
        .iter()
        .filter_map(|photo| photo.as_ref())
        .collect()
    }
}

#[derive(Clone, Debug)]
pub struct LegacyPhoto {
    pub path: String,
    pub name: String,
}
