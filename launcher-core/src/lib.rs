use auth::yggdrasil::{AuthenticationResponse, NameUUID};
use std::path::PathBuf;

pub struct Launcher {
    pub access_token: String,
    pub client_token: Option<String>,
    pub profile: NameUUID,
    pub game_directory: PathBuf,
}

impl Launcher {
    pub fn builder(auth_response: &AuthenticationResponse) -> LauncherBuilder {
        LauncherBuilder::new(auth_response)
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

    pub fn game_directory<S: Into<PathBuf>>(mut self, game_directory: S) -> Self {
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
