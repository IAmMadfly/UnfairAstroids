use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_startup_system(load_camera.system());
    app.add_startup_system(load_players.system());

    app.add_system(generate_astroids.system());

    app.run();
}

fn load_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn load_players(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 0.0, 0.0),
            flip_x: false,
            flip_y: false,
            custom_size: Some(Vec2::new(200.0, 200.0)),
        },
        transform: Transform::from_xyz(100.0, 100.0, 0.1),
        global_transform: Default::default(),
        texture: Default::default(),
        visibility: Visibility { is_visible: true },
    });
}

fn generate_astroids() {}
