// src/protocol.rs
use bevy::prelude::*;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerId(pub PeerId);

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct PlayerPosition(pub Vec2);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PositionMessage(pub Vec2);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Inputs { /* i tuoi campi */ }

#[derive(Component)]
pub struct Player;
