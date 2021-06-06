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
    pub launcher_directory: PathBuf,
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
    launcher_directory: Option<PathBuf>,
}

impl LauncherBuilder {
    pub fn new(auth_response: &AuthenticationResponse) -> Self {
        Self {
            access_token: auth_response.accessToken.clone(),
            client_token: auth_response.clientToken.clone(),
            profile: auth_response.selectedProfile.clone(),
            launcher_directory: None,
        }
    }

    pub fn launcher_directory<P: Into<PathBuf>>(mut self, launcher_directory: P) -> Self {
        self.launcher_directory = Some(launcher_directory.into());
        self
    }

    pub fn build(self) -> Result<Launcher> {
        let launcher_directory = match self.launcher_directory {
            Some(dir) => dir,
            None => PathBuf::from(default_launcher_dir()?),
        };

        Ok(Launcher {
            access_token: self.access_token,
            client_token: self.client_token,
            profile: self.profile,
            launcher_directory: launcher_directory,
        })
    }
}

#[cfg(target_os = "windows")]
fn default_launcher_dir() -> Result<String> {
    Ok(format!("{}/.minecraft", std::env::var("APPDATA")?))
}

#[cfg(target_os = "macos")]
fn default_launcher_dir() -> Result<String> {
    Ok(format!(
        "{}/Library/Application Support/.minecraft",
        std::env::var("HOME")?
    ))
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn default_launcher_dir() -> Result<String> {
    Ok(format!("{}/.minecraft", std::env::var("HOME")?))
}
