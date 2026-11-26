//! This module contains the shared code between the client and the server.

use crate::protocol::*;
use bevy::prelude::*;
// in cima a shared.rs
use lightyear::prelude::*;

#[cfg(all(feature = "webtransport", not(target_family = "wasm")))]
use bevy::tasks::IoTaskPool;
use core::net::{IpAddr, Ipv4Addr, SocketAddr};
use core::time::Duration;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

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

        app.register_message::<BuildingPlaced>()
            .add_direction(NetworkDirection::Bidirectional);

        app.register_message::<AllBuildings>()
            .add_direction(NetworkDirection::ServerToClient);
        // inputs
        //app.add_plugins(input::native::InputPlugin::<Inputs>::default());

        // components
        app.component::<PlayerId>().replicate();
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
