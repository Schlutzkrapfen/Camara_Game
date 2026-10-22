//! This minimal example showcases how to setup the lightyear plugins.
//!
//! Run with
//! - `cargo run -- server`
//! - `cargo run -- client`
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod client;
mod server;
mod shared;

use bevy::{post_process::bloom::Bloom, prelude::*};
use bevy::window::PrimaryWindow;
use crate::shared::{SharedPlugin, FIXED_TIMESTEP_HZ};
use bevy::prelude::*;
use clap::{ArgAction, Parser, Subcommand, ValueEnum};
use core::time::Duration;
use lightyear::connection::server::Start;
use lightyear::prelude::client::ClientPlugins;
use lightyear::prelude::server::ServerPlugins;
use lightyear::prelude::*;

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
    app.add_plugins(ClientPlugins {
        tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
    });
    app.add_plugins(ServerPlugins {
        tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
    });
    // ProtocolPlugin must be added after the Client/Server plugins.
    app.add_plugins(SharedPlugin);

    match cli.mode {
        Mode::Client => {
            app.add_plugins(client::ExampleClientPlugin);
        }
        Mode::Server => {
            app.add_plugins(server::ExampleServerPlugin);
        }
    }
    app.add_systems(Startup,  setup_camera);
    app.add_systems(Update,  spawn_tile);
    app.run();
}
fn spawn_tile(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
            return;
        }

        let (camera, camera_transform) = *camera;

        // cursor_position() is None if the cursor is outside the window
        let Some(cursor_pos) = window.cursor_position() else {
            return;
        };

        // Window coords -> world coords
        let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
            return;
        };

        println!("world pos: {world_pos:?}");



}
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Bloom::NATURAL));
}

// Update the camera position by tracking the player.
//fn update_camera(
//    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
//    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
//    time: Res<Time>,
//) {
//    let Vec3 { x, y, .. } = player.translation;
//    let direction = Vec3::new(x, y, camera.translation.z);
//
//    // Applies a smooth effect to camera movement using stable interpolation
//    // between the camera position and the player position on the x and y axes.
//    camera
//        .translation
//        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
//}
