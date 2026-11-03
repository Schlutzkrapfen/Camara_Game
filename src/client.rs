//! The client plugin.
use crate::shared::*;
use bevy::prelude::*;
use core::net::Ipv4Addr;
use core::net::{IpAddr, SocketAddr};
use lightyear::prelude::client::*;
use lightyear::prelude::*;
use crate::protocol::*;
use bevy::window::PrimaryWindow;
use lightyear::netcode::{Key, NetcodeClient};


fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Transform::from_xyz(0.0, 0.0, 0.0),
        // qui il tuo mesh/sprite, es. Sprite::default() oppure Mesh2d(...)
    ));
}
pub struct ExampleClientPlugin;
const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4000);

impl Plugin for ExampleClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update,  spawn_tile);
        app.add_systems(Update, check_connection);
    }
}

fn startup(mut commands: Commands) -> Result {
    let auth = Authentication::Manual {
        server_addr: SERVER_ADDR,
        client_id: 0,
        private_key: Key::default(),
        protocol_id: 0,
    };

    let auth = Authentication::Manual {
        server_addr: SERVER_ADDR,
        client_id: 0,
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
                ).unwrap(),
        ))
        .id();

    commands.trigger(Connect { entity: client });
    Ok(())
}
fn send_position(
    player: Single<&Transform, With<Player>>,
    mut sender: Single<&mut MessageSender<BuildingMassage>>,
) {
    let pos = player.translation.truncate(); // Vec3 -> Vec2
    sender.send::<Channel1>(BuildingMassage(pos,1));
}
fn spawn_tile(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
     mut sender: Single<&mut MessageSender<BuildingMassage>>
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
        sender.send::<Channel1>(BuildingMassage(world_pos,1));

        println!("Position Clicked: {world_pos:?}");
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
