use bevy::{ecs::query::WithFetch, prelude::*};

#[derive(Component)]
struct Player(u8);

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_startup_system(load_camera.system());
    app.add_startup_system(load_players.system());

    app.add_system(generate_astroids.system());
    app.add_system(handle_input.system());
    // app.add_system(printouts.system());

    app.run();
}

fn load_camera(mut commands: Commands) {
    println!("Loading the camera!");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // commands.spawn_bundle(UiCameraBundle::default());
}

fn load_players(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(50.0, 0.0, 0.1),
            texture: asset_server.load("sprites/player_ships/ship_0001.png"),
            ..Default::default()
        })
        .insert(Player(0));

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(-50.0, 0.0, 0.1),
            texture: asset_server.load("sprites/player_ships/ship_0000.png"),
            ..Default::default()
        })
        .insert(Player(1));
}

fn handle_input(input: Res<Input<KeyCode>>, mut players: Query<(&mut Transform, &Player)>) {
    let mut first_forward = 0.0;
    let mut first_right = 0.0;

    let mut second_forward = 0.0;
    let mut second_right = 0.0;

    for key in input.get_pressed() {
        match key {
            KeyCode::A => first_right -= 1.0,
            KeyCode::D => first_right += 1.0,
            KeyCode::S => first_forward -= 1.0,
            KeyCode::W => first_forward += 1.0,
            KeyCode::Left => second_right -= 1.0,
            KeyCode::Right => second_right += 1.0,
            KeyCode::Down => second_forward -= 1.0,
            KeyCode::Up => second_forward += 1.0,
            _ => {}
        }
    }

    for (mut trans, player) in players.iter_mut() {
        match player.0 {
            0 => {
                let curr_rotation = trans.rotation.to_euler(EulerRot::XYZ);
                trans.translation.y += curr_rotation.2.cos() * first_forward;
                trans.translation.x += curr_rotation.2.sin() * -first_forward;
                trans.rotate(Quat::from_rotation_z(first_right * 0.2));
            }
            1 => {
                let curr_rotation = trans.rotation.to_euler(EulerRot::XYZ);
                trans.translation.y += curr_rotation.2.cos() * second_forward;
                trans.translation.x += curr_rotation.2.sin() * -second_forward;
                trans.rotate(Quat::from_rotation_z(second_right * 0.2));
            }
            _ => {}
        }
    }
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
