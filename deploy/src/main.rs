use anyhow::{anyhow, Error};
use log::{debug, info};
use md5::{Digest, Md5};
use rayon::prelude::*;
use rusoto_core::Region;
use rusoto_s3::{ListObjectsV2Request, S3Client, S3};
use simplelog::{LevelFilter, TermLogger, TerminalMode};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use std::str;

use walkdir::{DirEntry, WalkDir};

#[derive(Clone, Debug, Eq, PartialEq)]
struct LocalFile {
    absolute_path: PathBuf,
    relative_path: PathBuf,
    len: u64,
    md5: [u8; 16],
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RemoteFile {
    key: String,
    len: u64,
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
    Ok(LocalFile {
        absolute_path: entry.path().canonicalize()?,
        relative_path: entry.path().strip_prefix(root)?.to_owned(),
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
    entries.truncate(10);

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

async fn scan_remote_files<S: S3>(s3: &S, bucket: &str) -> Result<Vec<RemoteFile>, Error> {
    let mut continuation_token = None;
    let mut result = Vec::new();
    loop {
        let output = s3
            .list_objects_v2(ListObjectsV2Request {
                bucket: bucket.to_owned(),
                continuation_token: continuation_token.clone(),
                ..ListObjectsV2Request::default()
            })
            .await?;
        if let Some(contents) = output.contents {
            for obj in contents {
                match (obj.key, obj.size) {
                    (Some(key), Some(size)) => {
                        result.push(RemoteFile {
                            key,
                            len: size as u64,
                            e_tag: obj.e_tag.and_then(|e_tag| parse_e_tag(&e_tag)),
                        });
                    }
                    _ => (),
                }
            }
        }
        continuation_token = output.next_continuation_token;
        if continuation_token.is_none() {
            break;
        }
    }
    Ok(result)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = TermLogger::init(
        LevelFilter::Debug,
        simplelog::Config::default(),
        TerminalMode::Mixed,
    );

    let build_dir = Path::new("build/site");
    if !build_dir.exists() {
        return Err(anyhow!("Can't find build directory"));
    }
    info!("Scanning local files...");
    let local_files = tokio::task::spawn_blocking(move || scan_local_files(build_dir)).await??;
    info!("Scanned {} local files", local_files.len());

    let s3 = S3Client::new(Region::EuWest1);
    let bucket = "gbhwdb.gekkio.fi";

    info!("Scanning remote files...");
    let remote_files = scan_remote_files(&s3, bucket).await?;
    info!("Scanned {} remote files", remote_files.len());

    info!("Building deployment plan...");
    let local_index: BTreeMap<&str, &LocalFile> = local_files
        .iter()
        .filter_map(|file| {
            let key = file.relative_path.to_str()?;
            Some((key, file))
        })
        .collect();
    let remote_index: BTreeMap<&str, &RemoteFile> = remote_files
        .iter()
        .map(|file| (file.key.as_str(), file))
        .collect();

    let mut to_upload = Vec::new();
    for (key, local_file) in local_index.iter() {
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

    Ok(())
}
