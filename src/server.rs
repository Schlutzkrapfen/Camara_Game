use crate::shared::*;
use crate::protocol::*;
use crate::enums::*;
use bevy::gltf::gltf::json::image;
use lightyear::prelude::client::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::{
    image::{ImageArrayLayout, ImageLoaderSettings},
    prelude::*,
    sprite_render::{AlphaMode2d, TileData, TileOrientation, TilemapChunk, TilemapChunkTileData},
};


pub struct ExampleServerPlugin;

impl Plugin for ExampleServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (startup,setup_tilemap));
        app.add_observer(handle_new_client);
        app.add_systems(Update, (receive_position,spawn_tile));
    }
}


/// Whenever a new client connects to the server, a new entity will get spawned in the server's world
/// with a [`Link`] component to exchange bytes with the client.
///
/// It will also have some other components:
/// - [`Connected`] (or [`Disconnected`]) to track the connection state of the link
/// - [`LinkOf`] (relationship that tracks which server the link is associated with)
///
/// You can add more components to customize how this connection, for example by adding a
/// [`ReplicationSender`] (which means that the server will replicate the state of the world to this client)
fn handle_new_client(trigger: On<Add<Connected>>, mut commands: Commands) {
    commands.entity(trigger.entity).insert(ReplicationSender);

    // spawn an entity for this client, that we will replicate to all clients
    commands.spawn((
        PlayerPosition::default(),
        // this entity has a link to the client
        Replicate::to_clients(NetworkTarget::All),
    ));
}

fn startup(mut commands: Commands) -> Result {
    // start a server entity
    let server = commands
        .spawn((
            NetcodeServer::new(
                lightyear::netcode::server_plugin::NetcodeConfig::default(),
            ),
            // Links need a 'connection' component that provides them with a durable entity beyond a simple IP address.
            // Usually you would use something like NetcodeServer or SteamServer, but for this example
            // we will use [`RawServer`], which uses the IP as the durable network identifier for the entity
            Server,
            // you need to specify the address to bind the server to
            LocalAddr(SERVER_ADDR),
            // the transport that we will use is Udp
            ServerUdpIo,
        ))
        .id();
    // you can use triggers to start/stop the server
    commands.trigger(Start { entity: server });
    commands.spawn((
            Text::new(concat!(
                "Server"
            )),
            Node {
                position_type: PositionType::Absolute,
                bottom: px(12),
                left: px(12),
                ..default()
            },
        ));
    Ok(())
}
fn spawn_tile(
    mut query: Query<(&TilemapChunk, &mut TilemapChunkTileData), With<GridMap>>,
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
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
        let Ok((chunk, mut data)) = query.single_mut() else {
                       error!("GridMap non found!");
                       return;
                   };

        place_tiles(chunk, &mut data, world_pos, TileOrientation::Default);

}
#[derive(Component)]
struct GridMap;
fn setup_tilemap(mut commands: Commands, assets: Res<AssetServer>) {
    let chunk_size = UVec2::splat(100);
    let tile_display_size = UVec2::splat(100);

    // Tutte le tile partono vuote (None = nessuna tile)
    let tile_data: Vec<Option<TileData>> =
        vec![None; chunk_size.element_product() as usize];

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


fn receive_position(
    mut query: Query<(&TilemapChunk, &mut TilemapChunkTileData), With<GridMap>>,
       assets: Res<AssetServer>,
    mut receivers: Query<(Entity, &mut MessageReceiver<BuildingMassage>)>
)
{


    for (client, mut receiver) in &mut receivers {
        for msg in receiver.receive() {
            info!("Server: ricevuto {:?} with rotation {:?} client {client:?}", msg.0, msg.1);
            let Ok((chunk, mut data)) = query.single_mut() else {
                           warn!("GridMap non found!");
                           continue;
                       };

            place_tiles(chunk, &mut data, msg.0, orientation_from_u8(msg.1));
        }
    }
}
