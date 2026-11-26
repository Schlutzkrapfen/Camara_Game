use bevy::prelude::*;
pub struct BuildingBuildsPlugin;

pub const FIXED_TIMESTEP_HZ: f64 = 64.0;
pub const TILEDISPLAYSIZE: UVec2 = UVec2::splat(100);

use bevy::sprite_render::{
    AlphaMode2d, TileData, TileOrientation, TilemapChunk, TilemapChunkTileData,
};
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

    info!(
        "build at postion {:?} with rotation {:?}",
        grid_pos, rotation
    );
}
pub fn place_tiles(
    chunk: &TilemapChunk,
    data: &mut TilemapChunkTileData,
    position: Vec2,
    rotation: TileOrientation,
) {
    let tile = chunk.tile_display_size.as_vec2();
    let total = chunk.chunk_size.as_vec2() * tile;

    // origine in alto a sinistra del chunk (chunk centrato in 0,0)
    let local = Vec2::new(position.x + total.x / 2.0, position.y + total.y / 2.0);
    let cell = (local / tile).floor().as_ivec2();

    build(chunk, data, cell.as_uvec2(), rotation);
}
