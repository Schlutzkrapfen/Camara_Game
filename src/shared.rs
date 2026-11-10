//! This module contains the shared code between the client and the server.

use bevy::prelude::*;
use bevy::prelude::*;
use bevy::sprite_render::{AlphaMode2d, TileData, TileOrientation, TilemapChunk, TilemapChunkTileData};
use crate::protocol::*;
// in cima a shared.rs
use lightyear::prelude::*;

#[cfg(all(feature = "webtransport", not(target_family = "wasm")))]
use bevy::tasks::IoTaskPool;
use core::net::{IpAddr, Ipv4Addr, SocketAddr};
use core::time::Duration;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

pub const FIXED_TIMESTEP_HZ: f64 = 64.0;
pub const TILEDISPLAYSIZE: UVec2 = UVec2::splat(100);


pub const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);
// shared.rs
pub const PROTOCOL_ID: u64 = 0;
pub const PRIVATE_KEY: [u8; 32] = [0; 32];

#[derive(Clone)]
pub struct SharedPlugin;

pub struct Channel1;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Message1(pub usize);



impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
            // messages
            app.register_message::<Message1>()
                .add_direction(NetworkDirection::ServerToClient);

            app.register_message::<BuildingMassage>()
                .add_direction(NetworkDirection::ClientToServer);

            // inputs
            //app.add_plugins(input::native::InputPlugin::<Inputs>::default());

            // components
            app.component::<PlayerId>().replicate();
            app.component::<PlayerPosition>();
                //.replicate()
                //.predict()
                //.add_linear_interpolation();
            //app.component::<PlayerColor>().replicate();

            // channels
            // CAMBIATO: da ServerToClient a Bidirectional
            app.add_channel::<Channel1>(ChannelSettings {
                mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
                ..default()
            })
            .add_direction(NetworkDirection::Bidirectional);
        }
}

/// System that draws the boxes of the player positions.
/// The components should be replicated from the server to the client
pub(crate) fn draw_boxes(mut gizmos: Gizmos, players: Query<&PlayerPosition>) {
    for position in &players {
        gizmos.rect_2d(
            Isometry2d::from_translation(position.0),
            Vec2::ONE * 50.0,
            Color::WHITE,
        );
    }
}
pub fn build(
    chunk: &TilemapChunk,
    data: &mut TilemapChunkTileData,
    grid_pos: UVec2,
    rotation: TileOrientation,
) {

    if grid_pos.x >= chunk.chunk_size.x || grid_pos.y >= chunk.chunk_size.y {
        return;
    }

    let index = (grid_pos.y * chunk.chunk_size.x + grid_pos.x) as usize;

    data.0[index] = Some(TileData {
        tileset_index: 0,
        color: Color::WHITE,
        visible: true,
        orientation: rotation,
    });

    info!("build at postion {:?} with rotation {:?}",grid_pos ,rotation);
}
pub fn place_tiles(
    chunk: &TilemapChunk,
    data: &mut TilemapChunkTileData,
    position:Vec2,
    rotation:TileOrientation
) {
    let tile = chunk.tile_display_size.as_vec2();
       let total = chunk.chunk_size.as_vec2() * tile;

       // origine in alto a sinistra del chunk (chunk centrato in 0,0)
       let local = Vec2::new(position.x + total.x / 2.0,  position.y + total.y /2.0);
       let cell = (local / tile).floor().as_ivec2();

    build(chunk, data, cell.as_uvec2(), rotation);

}
