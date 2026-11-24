//! The client plugin.
use crate::enums::*;
use crate::protocol::*;
use crate::shared::*;
use bevy::window::PrimaryWindow;
use bevy::{
    image::{ImageArrayLayout, ImageLoaderSettings},
    prelude::*,
    sprite_render::{AlphaMode2d, TileData, TileOrientation, TilemapChunk, TilemapChunkTileData},
};
use core::net::Ipv4Addr;
use core::net::{IpAddr, SocketAddr};
use lightyear::netcode::{Key, NetcodeClient};
use lightyear::prelude::client::*;
use lightyear::prelude::*;
use rand::Rng;
use std::sync::Mutex;

pub struct ExampleClientPlugin;

static ROTATION: Mutex<TileOrientation> = Mutex::new(TileOrientation::Default);
const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4000);

impl Plugin for ExampleClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (startup, setup_tilemap, receive_all_buildings));
        app.add_systems(
            Update,
            (
                spawn_tile,
                receive_position,
                check_connection,
                rotatie_tile_client,
            ),
        );
    }
}
fn rotatie_tile_client(keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        let mut orientation = ROTATION.lock().unwrap();
        *orientation = next_orientation(*orientation);
    }
}

fn startup(mut commands: Commands) -> Result {
    let mut rng = rand::rng();
    let auth = Authentication::Manual {
        server_addr: SERVER_ADDR,
        client_id: rng.random(),
        private_key: PRIVATE_KEY,
        protocol_id: PROTOCOL_ID,
    };

    let client = commands
        .spawn((
            Client::default(),
            LocalAddr(CLIENT_ADDR),
            PeerAddr(SERVER_ADDR),
            Link::default(),
            ReplicationReceiver::default(),
            UdpIo::default(),
            NetcodeClient::new(
                auth,
                lightyear::netcode::client_plugin::NetcodeConfig::default(),
            )
            .unwrap(),
        ))
        .id();

    commands.trigger(Connect { entity: client });
    commands.spawn((
        Text::new(concat!("Client")),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        },
    ));
    Ok(())
}

#[derive(Component)]
struct GridMap;
fn setup_tilemap(mut commands: Commands, assets: Res<AssetServer>) {
    let chunk_size = UVec2::splat(100);
    let tile_display_size = UVec2::splat(100);

    let tile_data: Vec<Option<TileData>> = vec![None; chunk_size.element_product() as usize];

    commands.spawn((
        GridMap,
        TilemapChunk {
            chunk_size,
            tile_display_size,
            tileset: assets
                .load_builder()
                .with_settings(|settings: &mut ImageLoaderSettings| {
                    settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 2 });
                })
                .load::<Image>("tiles.png"),
            alpha_mode: AlphaMode2d::Blend,
        },
        TilemapChunkTileData(tile_data),
        Transform::default(),
    ));
}
fn rotatie_tile(keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        let mut orientation = ROTATION.lock().unwrap();
        *orientation = next_orientation(*orientation);
    }
}
fn spawn_tile(
    mut query: Query<(&TilemapChunk, &mut TilemapChunkTileData), With<GridMap>>,
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut sender: Single<&mut MessageSender<BuildingPlaced>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let (camera, camera_transform) = camera.into_inner();

    // cursor_position() is None if the cursor is outside the window
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    // Window coords -> world coords
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };
    let building = BuildingData {
        id: 1,
        tile: world_pos,
        kind: 0,
        rotation: orientation_to_u8(*ROTATION.lock().unwrap()),
    };

    sender.send::<Channel1>(BuildingPlaced(building));
    let Ok((chunk, mut data)) = query.single_mut() else {
        error!("GridMap non found!");
        return;
    };
    place_tiles(chunk, &mut data, world_pos, *ROTATION.lock().unwrap());
}
fn log_connected(query: Query<Entity, Added<Connected>>) {
    for entity in &query {
        println!("client: connected! entity {:?}", entity);
    }
}
fn check_connection(q: Query<Entity, (With<Client>, Added<Connected>)>) {
    for e in &q {
        info!("✅ connection stable {e:?}");
    }
}

fn receive_position(
    mut query: Query<(&TilemapChunk, &mut TilemapChunkTileData), With<GridMap>>,
    assets: Res<AssetServer>,
    mut receivers: Query<(Entity, &mut MessageReceiver<BuildingPlaced>)>,
) {
    for (server, mut receiver) in &mut receivers {
        for msg in receiver.receive() {
            info!(
                "Client: ricevuto {:?} with rotation {:?} Server {server:?}",
                msg.0.tile, msg.0.rotation
            );
            let Ok((chunk, mut data)) = query.single_mut() else {
                warn!("GridMap non found!");
                continue;
            };

            place_tiles(
                chunk,
                &mut data,
                msg.0.tile,
                orientation_from_u8(msg.0.rotation),
            );
        }
    }
}

fn receive_all_buildings(
    mut receivers: Query<&mut MessageReceiver<AllBuildings>>,
    mut map: Query<(&TilemapChunk, &mut TilemapChunkTileData), With<GridMap>>,
) {
    let Ok((chunk, mut data)) = map.single_mut() else {
        return;
    };
    for mut rx in &mut receivers {
        for AllBuildings(list) in rx.receive() {
            for building in list {
                print!("HELLIO");
                place_tiles(
                    chunk,
                    &mut data,
                    building.tile,
                    orientation_from_u8(building.rotation),
                );
                // write the tile into `data` from b.tile, b.kind, b.rotation
            }
        }
    }
}
