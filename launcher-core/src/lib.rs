/*
 * Copyright (c) 2021 TheOddGarlic <umutinanerdogan62@gmail.com>
 * Licensed under the Open Software License version 3.0
 */

pub mod meta;

use anyhow::Result;
use auth::yggdrasil::{AuthenticationResponse, NameUUID};
use std::path::PathBuf;

use meta::{VersionManifest, Versions};

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

    pub fn build(self) -> Result<Launcher> {
        let game_directory = match self.game_directory {
            Some(dir) => dir,
            None => PathBuf::from(if cfg!(target_os = "windows") {
                format!("{}/.minecraft", std::env::var("APPDATA")?)
            } else if cfg!(target_os = "macos") {
                format!(
                    "{}/Library/Application Support/.minecraft",
                    std::env::var("HOME")?
                )
            } else {
                format!("{}/.minecraft", std::env::var("HOME")?)
            }),
        };

        Ok(Launcher {
            access_token: self.access_token,
            client_token: self.client_token,
            profile: self.profile,
            game_directory: game_directory,
        })
    }
}
