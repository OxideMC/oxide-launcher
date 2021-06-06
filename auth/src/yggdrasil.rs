/*
 * Copyright (c) 2021 TheOddGarlic <umutinanerdogan62@gmail.com>
 * Licensed under the Open Software License version 3.0
 */

//! Adapted from [ozelot](http://github.com/C4K3/ozelot)
use anyhow::Result;
use curl::easy::{Easy, List};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Authenticate with Mojang
#[derive(Debug, Clone)]
pub struct Authenticate {
    username: String,
    password: String,
    client_token: Option<String>,
    request_user: bool,
}

impl Authenticate {
    fn get_endpoint() -> String {
        "https://authserver.mojang.com/authenticate".to_string()
    }

    pub fn perform(&self) -> Result<AuthenticationResponse> {
        let payload = json!({
            "agent": {
                "name": "Minecraft",
                "version": 1
            },
            "username": self.username,
            "password": self.password,
            "clientToken": self.client_token,
            "requestUser": self.request_user
        });

        let res = post_request(&Self::get_endpoint(), &payload.to_string())?;
        Ok(serde_json::from_str(&res)?)
    }

    pub fn new<S: Into<String>>(username: S, password: S) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            client_token: None,
            request_user: false,
        }
    }
}

/// Refresh a valid accessToken
#[derive(Debug, Serialize, Clone)]
pub struct Refresh {
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "clientToken")]
    client_token: String,
    #[serde(rename = "requestUser")]
    request_user: bool,
}

impl Refresh {
    fn get_endpoint() -> String {
        "https://authserver.mojang.com/refresh".to_string()
    }

    pub fn perform(&self) -> Result<AuthenticationResponse> {
        let payload = serde_json::to_string(self)?;
        let res = post_request(&Self::get_endpoint(), &payload)?;
        Ok(serde_json::from_str(&res)?)
    }

    pub fn new<S: Into<String>>(access_token: S, client_token: S, request_user: bool) -> Self {
        Self {
            access_token: access_token.into(),
            client_token: client_token.into(),
            request_user: request_user,
        }
    }
}

/// Validate an existing access token
#[derive(Debug, Serialize, Clone)]
pub struct Validate {
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "clientToken")]
    client_token: Option<String>,
}

impl Validate {
    fn get_endpoint() -> String {
        "https://authserver.mojang.com/validate".to_string()
    }

    pub fn perform(&self) -> Result<()> {
        let payload = serde_json::to_string(self)?;
        let _ = post_request(&Self::get_endpoint(), &payload)?;
        Ok(())
    }

    pub fn new<S: Into<String>>(access_token: S, client_token: Option<String>) -> Self {
        Self {
            access_token: access_token.into(),
            client_token: client_token.into(),
        }
    }
}

/// Invalidate an accessToken, using the client username/password
#[derive(Debug, Serialize, Clone)]
pub struct SignOut {
    username: String,
    password: String,
}

impl SignOut {
    fn get_endpoint() -> String {
        "https://authserver.mojang.com/signout".to_string()
    }

    pub fn perform(&self) -> Result<()> {
        let payload = serde_json::to_string(self)?;
        let _ = post_request(&Self::get_endpoint(), &payload)?;
        Ok(())
    }

    pub fn new<S: Into<String>>(username: S, password: S) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }
}

/// Invalidate an accessToken, using the accessToken and a clientToken
#[derive(Debug, Serialize, Clone)]
pub struct Invalidate {
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "clientToken")]
    client_token: String,
}

impl Invalidate {
    fn get_endpoint() -> String {
        "https://authserver.mojang.com/invalidate".to_string()
    }

    pub fn perform(&self) -> Result<()> {
        let payload = serde_json::to_string(self)?;
        let _ = post_request(&Self::get_endpoint(), &payload)?;
        Ok(())
    }

    pub fn new<S: Into<String>>(access_token: S, client_token: S) -> Self {
        Self {
            access_token: access_token.into(),
            client_token: client_token.into(),
        }
    }
}

/// Represents a response to a successful authentication
#[derive(Debug, Deserialize, Clone)]
pub struct AuthenticationResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "clientToken")]
    pub client_token: Option<String>,
    #[serde(rename = "availableProfiles")]
    pub available_profiles: Option<Vec<NameUUID>>,
    #[serde(rename = "selectedProfile")]
    pub selected_profile: NameUUID,
}

/// Represents a single username - UUID mapping.
#[derive(Debug, Deserialize, Clone)]
pub struct NameUUID {
    /// The uuid in hex without dashes
    pub id: String,
    /// Name of the player at the present point in time
    pub name: String,
    #[serde(default = "always_false")]
    pub legacy: bool,
    #[serde(default = "always_false")]
    pub demo: bool,
}

/// Helper function for performing a POST request to the given URL,
/// posting the given data to it, and returning the response content.
fn post_request(url: &str, post: &str) -> Result<String> {
    let mut handle = Easy::new();
    handle.url(url)?;
    handle.fail_on_error(true)?;

    let mut headers = List::new();
    headers.append("Content-Type: application/json")?;
    handle.http_headers(headers)?;
    handle.post_fields_copy(post.as_bytes())?;
    handle.post(true)?;

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

/// For use with Serde default values
fn always_false() -> bool {
    false
}
