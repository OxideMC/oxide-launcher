/*
 * Copyright (c) 2021 TheOddGarlic <umutinanerdogan62@gmail.com>
 * Licensed under the Open Software License version 3.0
 */

//! Adapted from [ozelot](http://github.com/C4K3/ozelot)
#![allow(non_snake_case)]

use curl::easy::{Easy, List};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use anyhow::Result;

/// Authenticate with Mojang
#[derive(Debug, Clone)]
pub struct Authenticate {
    username: String,
    password: String,
    clientToken: Option<String>,
    requestUser: bool,
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
            "clientToken": self.clientToken,
            "requestUser": self.requestUser
        });

        let res = post_request(&Self::get_endpoint(), &payload.to_string())?;
        Ok(serde_json::from_str(&res)?)
    }

    pub fn new(username: String, password: String) -> Self {
        Self {
            username: username,
            password: password,
            clientToken: None,
            requestUser: false,
        }
    }
}

/// Refresh a valid accessToken
#[derive(Debug, Serialize, Clone)]
pub struct Refresh {
    accessToken: String,
    clientToken: String,
    requestUser: bool,
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

    pub fn new(accessToken: String, clientToken: String, requestUser: bool) -> Self {
        Self {
            accessToken: accessToken,
            clientToken: clientToken,
            requestUser: requestUser,
        }
    }
}

/// Validate an existing access token
#[derive(Debug, Serialize, Clone)]
pub struct Validate {
    accessToken: String,
    clientToken: Option<String>,
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

    pub fn new(accessToken: String, clientToken: Option<String>) -> Self {
        Self {
            accessToken: accessToken,
            clientToken: clientToken,
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

    pub fn new(username: String, password: String) -> Self {
        Self {
            username: username,
            password: password,
        }
    }
}

/// Invalidate an accessToken, using the accessToken and a clientToken
#[derive(Debug, Serialize, Clone)]
pub struct Invalidate {
    accessToken: String,
    clientToken: String,
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

    pub fn new(accessToken: String, clientToken: String) -> Self {
        Self {
            accessToken: accessToken,
            clientToken: clientToken,
        }
    }
}

/// Represents a response to a successful authentication
#[derive(Debug, Deserialize, Clone)]
pub struct AuthenticationResponse {
    pub accessToken: String,
    pub clientToken: Option<String>,
    pub availableProfiles: Option<Vec<NameUUID>>,
    pub selectedProfile: NameUUID,
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
