/*
 * Copyright (c) 2021 TheOddGarlic <umutinanerdogan62@gmail.com>
 * Licensed under the Open Software License version 3.0
 */

use anyhow::Result;
use auth::yggdrasil::{AuthenticationResponse, NameUUID};
use serde_derive::Deserialize;
use std::path::PathBuf;

use meta::VersionManifest;

pub mod meta;

pub struct Launcher {
    pub access_token: String,
    pub client_token: Option<String>,
    pub profile: NameUUID,
    pub game_directory: PathBuf,
}

impl Launcher {
    /// Alias to [`LauncherBuilder::new()`]
    pub fn builder(auth_response: &AuthenticationResponse) -> LauncherBuilder {
        LauncherBuilder::new(auth_response)
    }

    pub fn version_manifest() -> Result<Versions> {
        VersionManifest.perform()
    }
}

pub struct LauncherBuilder {
    access_token: String,
    client_token: Option<String>,
    profile: NameUUID,
    game_directory: Option<PathBuf>,
}

impl LauncherBuilder {
    pub fn new(auth_response: &AuthenticationResponse) -> Self {
        Self {
            access_token: auth_response.accessToken.clone(),
            client_token: auth_response.clientToken.clone(),
            profile: auth_response.selectedProfile.clone(),
            game_directory: None,
        }
    }

    pub fn game_directory<P: Into<PathBuf>>(mut self, game_directory: P) -> Self {
        self.game_directory = Some(game_directory.into());
        self
    }

    pub fn build(self) -> Launcher {
        let game_directory = match self.game_directory {
            Some(dir) => dir,
            None => {
                if cfg!(target_os = "windows") {
                    PathBuf::from(format!("{}/.minecraft", std::env::var("APPDATA").unwrap()))
                } else if cfg!(target_os = "macos") {
                    PathBuf::from(format!(
                        "{}/Library/Application Support/.minecraft",
                        std::env::var("HOME").unwrap()
                    ))
                } else {
                    PathBuf::from(format!("{}/.minecraft", std::env::var("HOME").unwrap()))
                }
            }
        };

        Launcher {
            access_token: self.access_token,
            client_token: self.client_token,
            profile: self.profile,
            game_directory: game_directory,
        }
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

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
pub struct Version {
    id: String,
    r#type: String,
    url: String,
    time: String,
    releaseTime: String,
    sha1: String,
    complianceLevel: u8,
}
