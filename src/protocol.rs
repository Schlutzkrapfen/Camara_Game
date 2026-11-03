// src/protocol.rs
use bevy::{prelude::*, sprite_render::TileOrientation};
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerId(pub PeerId);

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct PlayerPosition(pub Vec2);
/// Position,Rotation,BuildIndex
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BuildingMassage(pub Vec2,pub u8);


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Inputs { /* i tuoi campi */ }

#[derive(Component)]
pub struct Player;
