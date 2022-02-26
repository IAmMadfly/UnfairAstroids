use bevy::{prelude::*, sprite::SpritePlugin};

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
        texture: asset_server.load("sprites/player_ships/ship_0000.png"),
        ..Default::default()
    });
}

fn generate_astroids() {}

fn printouts(player: Query<&Transform, With<Sprite>>) {
    for player in player.iter() {
        println!("Something is here: {:?}", player);
    }
}
