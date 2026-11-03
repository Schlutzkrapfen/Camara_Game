//! This minimal example showcases how to setup the lightyear plugins.
//!
//! Run with
//! - `cargo run -- server`
//! - `cargo run -- client`
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
mod protocol;
mod client;
mod server;
mod shared;
mod enums;

use bevy::{post_process::bloom::Bloom, prelude::*};
use crate::shared::{SharedPlugin, FIXED_TIMESTEP_HZ};
use bevy::prelude::*;
use clap::{ArgAction, Parser, Subcommand, ValueEnum};
use core::time::Duration;
use lightyear::connection::server::Start;
use lightyear::prelude::client::ClientPlugins;
use lightyear::prelude::server::ServerPlugins;
use lightyear::prelude::*;
use crate::protocol::Inputs;

/// CLI options to create an [`App`]
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand, Debug)]
pub enum Mode {
    Client,
    Server,
}
/// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 2.;



fn main() {
    let cli = Cli::parse();
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    // ProtocolPlugin must be added after the Client/Server plugins.
    let tick = Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ);

    match cli.mode {
        Mode::Client => {

            app.add_plugins(ClientPlugins { tick_duration: tick });
            app.add_plugins(client::ExampleClientPlugin);

        }
        Mode::Server => {
            app.add_plugins(ServerPlugins { tick_duration: tick}) ;
            app.add_plugins(server::ExampleServerPlugin);
        }
    }
    app.add_plugins(SharedPlugin);
    app.add_systems(Startup,  setup_camera);
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Bloom::NATURAL));
}
