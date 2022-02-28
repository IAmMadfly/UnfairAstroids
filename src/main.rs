use bevy::prelude::*;

const ROTATION_FACTOR: f32 = 0.05;
const TRANSLATION_FACTOR: f32 = 0.1;

#[derive(Component)]
struct Player(u8);

#[derive(Component, Default)]
struct Inertia {
    rotational: f32,
    x: f32,
    y: f32,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_startup_system(load_camera.system());
    app.add_startup_system(load_players.system());

    app.add_system(generate_astroids.system());
    app.add_system(handle_input.system());
    app.add_system(run_inertia.system());
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
        .insert(Player(0))
        .insert(Inertia::default());

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(-50.0, 0.0, 0.1),
            texture: asset_server.load("sprites/player_ships/ship_0000.png"),
            ..Default::default()
        })
        .insert(Player(1))
        .insert(Inertia::default());
}

fn run_inertia(mut elements: Query<(&mut Transform, &Inertia)>) {
    for (mut trans, inertia) in elements.iter_mut() {
        trans.rotate(Quat::from_rotation_z(inertia.rotational));
        trans.translation.x = trans.translation.x + inertia.x;
        trans.translation.y = trans.translation.y + inertia.y;
    }
}

fn handle_input(
    input: Res<Input<KeyCode>>,
    mut players: Query<(&Transform, &Player, &mut Inertia)>,
) {
    let mut first_forward = 0.0;
    let mut first_right = 0.0;

    let mut second_forward = 0.0;
    let mut second_right = 0.0;

    for key in input.get_pressed() {
        match key {
            KeyCode::A => first_right -= 0.01,
            KeyCode::D => first_right += 0.01,
            KeyCode::S => first_forward -= 0.1,
            KeyCode::W => first_forward += 0.1,
            KeyCode::Left => second_right -= 0.01,
            KeyCode::Right => second_right += 0.01,
            KeyCode::Down => second_forward -= 0.1,
            KeyCode::Up => second_forward += 0.1,
            _ => {}
        }
    }

    for (trans, player, mut inertia) in players.iter_mut() {
        let curr_rotation = trans.rotation.to_euler(EulerRot::XYZ);
        match player.0 {
            0 => {
                inertia.y = (inertia.y + curr_rotation.2.cos() * first_forward).min(0.5);
                inertia.x = (inertia.x + curr_rotation.2.sin() * -first_forward).min(0.5);
                inertia.rotational = (inertia.rotational + first_right * 0.2).min(0.1);
                // trans.rotate(Quat::from_rotation_z(first_right * 0.2));
            }
            1 => {
                inertia.y = (inertia.y + curr_rotation.2.cos() * second_forward).min(0.5);
                inertia.x = (inertia.x + curr_rotation.2.sin() * -second_forward).min(0.5);
                inertia.rotational = (inertia.rotational + second_right * 0.2).min(0.5);
                // trans.rotate(Quat::from_rotation_z(first_right * 0.2));
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
