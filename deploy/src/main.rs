// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use anyhow::{anyhow, Error};
use aws_config::BehaviorVersion;
use aws_sdk_s3::primitives::ByteStream;
use base64::Engine;
use log::{debug, info};
use md5::{Digest, Md5};
use rayon::prelude::*;
use simplelog::{ColorChoice, LevelFilter, TermLogger, TerminalMode};
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{self, BufReader},
    path::{Path, PathBuf},
    str,
};
use time::{Duration, OffsetDateTime};
use tokio::task::spawn_blocking;
use walkdir::{DirEntry, WalkDir};

static MIME_MAPPING: &[(&str, &str)] = &[
    ("html", "text/html"),
    ("png", "image/png"),
    ("jpg", "image/jpeg"),
    ("css", "text/css"),
    ("csv", "text/csv"),
    ("svg", "image/svg+xml"),
    ("txt", "text/plain"),
    ("ico", "image/vnd.microsoft.icon"),
    ("xml", "application/xml"),
    ("webmanifest", "application/manifest+json"),
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct LocalFile {
    absolute_path: PathBuf,
    relative_path: PathBuf,
    key: String,
    len: u64,
    md5: [u8; 16],
}

impl LocalFile {
    fn cache_control(&self) -> &'static str {
        static CC_S: &str = "max-age=3600,public";
        static CC_M: &str = "max-age=86400,public";
        static CC_L: &str = "max-age=1209600,public";
        if self.key.starts_with("static/") {
            if self.key.ends_with(".jpg") {
                CC_L
            } else {
                CC_S
            }
        } else if self.key.starts_with("consoles/") || self.key.starts_with("cartridges/") {
            CC_M
        } else {
            CC_S
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RemoteFile {
    key: String,
    len: u64,
    last_modified: Option<OffsetDateTime>,
    e_tag: Option<[u8; 16]>,
}

fn file_md5(path: &Path) -> Result<[u8; 16], Error> {
    let mut hasher = Md5::new();
    let mut file = BufReader::new(File::open(path)?);
    io::copy(&mut file, &mut hasher)?;
    Ok(hasher.finalize().into())
}

fn scan_local_file(root: &Path, entry: &DirEntry) -> Result<LocalFile, Error> {
    let metadata = entry.metadata()?;
    let relative_path = entry.path().strip_prefix(root)?.to_owned();
    let key = relative_path
        .to_str()
        .ok_or_else(|| anyhow!("Non-UTF8 filename encountered {:?}", relative_path))?
        .to_owned();
    Ok(LocalFile {
        absolute_path: entry.path().canonicalize()?,
        relative_path,
        key,
        len: metadata.len(),
        md5: file_md5(entry.path())?,
    })
}

fn scan_local_files(root: &Path) -> Result<Vec<LocalFile>, Error> {
    let mut entries = Vec::new();
    for entry in WalkDir::new(root) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        entries.push(entry);
    }

    Ok(entries
        .into_par_iter()
        .map(|entry| scan_local_file(root, &entry))
        .collect::<Result<Vec<_>, _>>()?)
}

fn parse_e_tag(e_tag: &str) -> Option<[u8; 16]> {
    let e_tag = e_tag.strip_prefix('"')?.strip_suffix('"')?;
    let mut result = [0; 16];
    for (idx, chunk) in e_tag.as_bytes().chunks(2).enumerate() {
        let byte_str = str::from_utf8(chunk).ok()?;
        let byte = u8::from_str_radix(byte_str, 16).ok()?;
        *(result.get_mut(idx)?) = byte;
    }
    Some(result)
}

async fn scan_remote_files(
    s3: &aws_sdk_s3::Client,
    bucket: &str,
) -> Result<Vec<RemoteFile>, Error> {
    let mut result = Vec::new();
    let mut stream = s3.list_objects_v2().bucket(bucket).into_paginator().send();
    while let Some(page) = stream.next().await {
        let page = page?;
        for obj in page.contents() {
            if let (Some(key), Some(size)) = (obj.key(), obj.size()) {
                result.push(RemoteFile {
                    key: key.to_owned(),
                    len: size as u64,
                    last_modified: obj.last_modified.and_then(|timestamp| {
                        OffsetDateTime::from_unix_timestamp_nanos(timestamp.as_nanos()).ok()
                    }),
                    e_tag: obj.e_tag.as_ref().and_then(|e_tag| parse_e_tag(&e_tag)),
                });
            }
        }
    }
    Ok(result)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = TermLogger::init(
        LevelFilter::Info,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );

    let mime_map = MIME_MAPPING.into_iter().copied().collect::<HashMap<_, _>>();

    let build_dir = Path::new("build");
    if !build_dir.exists() {
        return Err(anyhow!("Can't find build directory"));
    }
    info!("Scanning local files...");
    let local_files = spawn_blocking(move || scan_local_files(build_dir)).await??;
    info!("Scanned {} local files", local_files.len());

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let s3 = aws_sdk_s3::Client::new(&config);
    let bucket = "gbhwdb.gekkio.fi";

    info!("Scanning remote files...");
    let remote_files = scan_remote_files(&s3, bucket).await?;
    info!("Scanned {} remote files", remote_files.len());

    info!("Building deployment plan...");
    let local_index: BTreeMap<&str, &LocalFile> = local_files
        .iter()
        .map(|file| (file.key.as_str(), file))
        .collect();
    let remote_index: BTreeMap<&str, &RemoteFile> = remote_files
        .iter()
        .map(|file| (file.key.as_str(), file))
        .collect();

    let mut to_upload = Vec::new();
    for (&key, &local_file) in local_index.iter() {
        if let Some(remote_file) = remote_index.get(key) {
            if remote_file.e_tag == Some(local_file.md5) {
                debug!("Skipping local file {}: remote match found", key);
                continue;
            } else {
                debug!("Scheduling local file {}: remote ETag mismatch", key);
            }
        } else {
            debug!("Scheduling local file {}: missing from remote", key);
        }
        to_upload.push(local_file);
    }
    info!("{} files scheduled for upload", to_upload.len());

    let mut to_delete = Vec::new();
    for (&key, &remote_file) in remote_index.iter() {
        if local_index.contains_key(key) {
            continue;
        }
        if let Some(last_modified) = remote_file.last_modified {
            let elapsed = OffsetDateTime::now_utc() - last_modified;
            if elapsed > Duration::weeks(4) {
                to_delete.push(remote_file);
            }
        }
    }
    info!("{} files scheduled for deletion", to_delete.len());

    let base64 = base64::engine::general_purpose::STANDARD;

    for local_file in to_upload {
        let ext = local_file
            .absolute_path
            .extension()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Invalid file extension: {}", local_file.key))?;
        let mime = mime_map
            .get(ext)
            .ok_or_else(|| anyhow!("Failed to detect MIME type of {}", local_file.key))?;
        info!("Uploading {}", local_file.key);
        let body = ByteStream::from_path(&local_file.absolute_path).await?;
        s3.put_object()
            .bucket(bucket)
            .key(&local_file.key)
            .body(body)
            .content_type(*mime)
            .content_md5(base64.encode(local_file.md5))
            .cache_control(local_file.cache_control())
            .send()
            .await?;
    }

    for remote_file in to_delete {
        info!("Deleting {}", remote_file.key);
        s3.delete_object()
            .bucket(bucket)
            .key(&remote_file.key)
            .send()
            .await?;
    }

    info!("Site deployment complete");
    Ok(())
}
