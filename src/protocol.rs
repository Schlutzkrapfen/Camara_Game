// src/protocol.rs
use bevy::{prelude::*, sprite_render::TileOrientation};
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerId(pub PeerId);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Inputs {/* i tuoi campi */}

#[derive(Component)]
pub struct Player;
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BuildingData {
    pub id: u32, // so later messages can refer to it
    pub tile: Vec2,
    pub kind: u16, // index into your BuildingResource definitions
    pub rotation: u8,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AllBuildings(pub Vec<BuildingData>);
// register_message::<AllBuildings>() with NetworkDirection::ServerToClient

/// Server -> Client, afterwards.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BuildingPlaced(pub BuildingData);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BuildingRemoved(pub u32);
