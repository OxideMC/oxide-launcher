/*
 * Copyright (c) 2021 TheOddGarlic <umutinanerdogan62@gmail.com>
 * Licensed under the Open Software License version 3.0
 */

use anyhow::Result;
use auth::yggdrasil::Authenticate;
use clap::{App, Arg, ArgMatches, SubCommand};
use launcher_core::Launcher;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs, io};

fn main() -> Result<()> {
    let default_launcher_directory = launcher_directory()?;

    let matches = App::new("oxide-cli")
        .version("0.1")
        .about("Simple Minecraft CLI Launcher")
        .author("TheOddGarlic <umutinanerdogan62@gmail.com>")
        .arg(
            Arg::with_name("launcher_directory")
                .short("D")
                .long("launcher_directory")
                .value_name("LAUNCHER_DIRECTORY")
                .help("Sets a directory for the launcher and game to be run in.")
                .default_value(&default_launcher_directory)
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("download")
                .about("Downloads the desired Minecraft (vanilla) version."),
        )
        .get_matches();

    let launcher_directory = PathBuf::from(
        matches
            .value_of("LAUNCHER_DIRECTORY")
            .unwrap_or(&default_launcher_directory),
    );

    fs::create_dir_all(&launcher_directory)?;
    env::set_current_dir(&launcher_directory)?;

    match matches.subcommand() {
        ("download", Some(_)) => download(matches, launcher_directory)?,
        _ => launch(matches, launcher_directory)?,
    }

    Ok(())
}

fn download(_matches: ArgMatches, _launcher_directory: PathBuf) -> Result<()> {
    Ok(())
}

fn launch(_matches: ArgMatches, launcher_directory: PathBuf) -> Result<()> {
    // todo: save auth_response to file and load it if file exists,
    //       validate if the credentials are still valid and
    //       if they are not valid then authenticate again.
    let mut email_or_username = String::new();
    let mut password = String::new();

    print!("Email or username: ");
    io::stdout().flush()?;
    io::stdin()
        .read_line(&mut email_or_username)
        .expect("Reading email or username failed.");

    print!("Password: ");
    io::stdout().flush()?;
    io::stdin()
        .read_line(&mut password)
        .expect("Reading password failed.");

    let auth_response = Authenticate::new(email_or_username.trim(), password.trim()).perform()?;

    let _launcher = Launcher::builder(&auth_response)
        .game_directory(launcher_directory)
        .build()?;

    // todo: download minecraft
    // todo: launch minecraft

    Ok(())
}

fn launcher_directory() -> Result<String> {
    Ok(
        env::var("MC_LAUNCHER_DIRECTORY").unwrap_or(if cfg!(target_os = "windows") {
            format!("{}\\.oxide-cli", env::var("APPDATA").unwrap())
        } else if cfg!(target_os = "macos") {
            format!(
                "{}/Library/Application Support/.oxide-cli",
                env::var("HOME").unwrap()
            )
        } else {
            format!(
                "{}/.oxide-cli",
                env::var("XDG_CONFIG_HOME").unwrap_or(env::var("HOME").unwrap())
            )
        }),
    )
}
