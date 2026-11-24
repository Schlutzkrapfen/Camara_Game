use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Asset, TypePath, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct BuildingResource {
    pub size: IVec2,
    pub position_tilemap: IVec2,
    pub tilemap_id: i32,
    pub craft_time: f32,
    pub input: Vec<ItemKind>,
    pub placeable_on: Vec<ItemKind>,
    pub output: Vec<String>,
    pub output_tile: HashMap<IVec2, TileTransform>,
    pub input_size: i32,
    pub buildcost: i32,
    pub can_not_deleted: bool,
    pub factory_type: FactoryType,
}

impl Default for BuildingResource {
    fn default() -> Self {
        Self {
            size: IVec2::new(1, 1),
            position_tilemap: IVec2::ZERO,
            tilemap_id: 0,
            craft_time: 0.1,
            input: vec![],
            output: vec![],
            placable: vec![],
            output_tile: HashMap::new(),
            input_size: 1,
            buildcost: 1,
            can_not_deleted: false,
            factory_type: FactoryType::default(),
        }
    }
}
