use crate::shared::*;
use crate::protocol::*;
use bevy::gltf::gltf::json::image;
use lightyear::prelude::client::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use bevy::{
    image::{ImageArrayLayout, ImageLoaderSettings},
    prelude::*,
    sprite_render::{AlphaMode2d, TileData, TileOrientation, TilemapChunk, TilemapChunkTileData},
};


pub struct ExampleServerPlugin;

impl Plugin for ExampleServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
        app.add_observer(handle_new_client);
        app.add_systems(Update, receive_position);
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
    Ok(())
}


fn build(
    commands: &mut Commands,
        assets: &AssetServer,
    position: Vec2,

) {
    let chunk_size = UVec2::splat(8);
    let tile_display_size = UVec2::splat(64);
    let tile_data = (0..chunk_size.element_product())
            .map(|i| {
                let row = i / 8;
                let col = i % 8;
                Some(TileData {
                    // Alternate tiles per row
                    tileset_index: (row % 2) as u16,
                    color: Color::WHITE,
                    // Last (top) row is invisible
                    visible: row != 7,
                    orientation:  TileOrientation::Default,
                })
            })
            .collect();
    commands.spawn((

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
    ));
}
fn receive_position(
    mut commands: Commands,
       assets: Res<AssetServer>,
    mut receivers: Query<(Entity, &mut MessageReceiver<PositionMessage>)>,
) {
    for (client, mut receiver) in &mut receivers {
        for msg in receiver.receive() {
            info!("Server: ricevuto {:?} dal client {client:?}", msg.0);
            build(&mut commands, &assets, msg.0);
        }
    }
}
