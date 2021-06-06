/*
 * Copyright (c) 2021 TheOddGarlic <umutinanerdogan62@gmail.com>
 * Licensed under the Open Software License version 3.0
 */

// todo: oh god why is this taking so long I have a lot of stuff to implement
//       whelp, am I ever going to finish this?

use serde::Deserialize;
use curl::easy::Easy;
use anyhow::Result;
use std::fs;

/// Get versions from https://launchermeta.mojang.com/mc/game/version_manifest_v2.json
pub struct VersionManifest;

impl VersionManifest {
    fn get_endpoint() -> String {
        "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json".to_string()
    }

    pub fn perform(&self) -> Result<Versions> {
        let res = get_request(&Self::get_endpoint())?;
        Ok(serde_json::from_str(&res)?)
    }

    pub fn new() -> VersionManifest {
        VersionManifest
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Versions {
    latest: LatestVersions,
    versions: Vec<Version>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LatestVersions {
    release: String,
    snapshot: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Version {
    id: String,
    r#type: String,
    url: String,
    time: String,
    #[serde(rename = "releaseTime")]
    release_time: String,
    sha1: String,
    #[serde(rename = "complianceLevel")]
    compliance_level: u8,
}

impl Version {
    #[cfg(target_os = "windows")]
    pub fn json<S: Into<String>>(&self, launcher_path: S) -> Result<()> {
        fs::write(
            format!(
                "{0}\\versions\\{1}\\{1}.json",
                launcher_path.into(),
                self.id
            ),
            get_request(&self.url)?,
        )?;
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    pub fn json<S: Into<String>>(&self, launcher_path: S) -> Result<()> {
        fs::write(
            format!("{0}/versions/{1}/{1}.json", launcher_path.into(), self.id),
            get_request(&self.url)?,
        )?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct VersionJson {
    // todo: implement this
    #[serde(skip)]
    arguments: (),
    #[serde(rename="assetIndex")]
    asset_index: AssetIndex,
    assets: String,
    #[serde(rename="complianceLevel")]
    compliance_level: Option<u8>,
    downloads: VersionDownloads,
    id: String,
    // todo: implement this
    #[serde(skip)]
    libraries: (),
    // todo: implement this
    #[serde(skip)]
    logging: (),
    #[serde(rename="mainClass")]
    main_class: String,
    #[serde(rename="minecraftArguments")]
    minecraft_arguments: Option<String>,
    #[serde(rename="minimumLauncherVersion")]
    minimum_launcher_version: u8,
    #[serde(rename="type")]
    version_type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AssetIndex {
    id: String,
    sha1: String,
    size: u64,
    #[serde(rename="totalSize")]
    total_size: u64,
    url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VersionDownloads {
    client: Download,
    // The server key was added in 1.2.5.
    server: Option<Download>,
    // Mappings were added in 1.14.4.
    client_mappings: Option<Download>,
    server_mappings: Option<Download>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Download {
    path: Option<String>,
    sha1: String,
    size: u64,
    url: String,
}

/// Helper function for performing a GET request to
/// the given URL, and returning the response content.
fn get_request(url: &str) -> Result<String> {
    let mut handle = Easy::new();
    handle.url(url)?;
    handle.fail_on_error(true)?;

    let mut response = Vec::new();

    {
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            response.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }

    Ok(String::from_utf8(response)?)
}
