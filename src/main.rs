use bevy::{ecs::system::Command, prelude::*, sprite::SpritePlugin};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugin(SpritePlugin::default());

    app.add_startup_system(load_camera.system());
    app.add_startup_system(load_players.system());

    app.add_system(generate_astroids.system());
    // app.add_system(printouts.system());

    app.run();
}

fn load_camera(mut commands: Commands) {
    println!("Loading the camera!");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // commands.spawn_bundle(UiCameraBundle::default());
}

fn load_players(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(50.0, 0.0, 0.1),
        texture: asset_server.load("sprites/player_ships/ship_0001.png"),
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(-50.0, 0.0, 0.1),
        texture: asset_server.load("sprites/player_ships/ship_0000.png"),
        ..Default::default()
    });
}

fn generate_astroids(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, -215.0, 0.0),
            scale: Vec3::new(120.0, 30.0, 0.0),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::rgb(0.5, 0.5, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn printouts(player: Query<&Transform, With<Sprite>>) {
    for player in player.iter() {
        println!("Something is here: {:?}", player);
    }
}
