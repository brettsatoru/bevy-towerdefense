use bevy::prelude::*;
use bevy::math::*;

pub struct MoveItem;
impl Plugin for MoveItem {
    fn build(&self, app : &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, sprite_movement);
    }
}

#[derive(Component)]
struct Velocity(f32);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, Transform::from_xyz(0., 0. , 10.)));

    commands.spawn((Sprite::from_image(asset_server.load("circle.png")),
                    Transform::from_xyz(0., 0., 0.),
                    Velocity(150.)));
}

fn sprite_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut sprite_position : Query<(&Velocity, &mut Transform)>) {
    for (vel, mut transform) in &mut sprite_position {
        let mut move2 = Vec2::new(0., 0.);
        
        if keyboard_input.pressed(KeyCode::KeyA) {
            move2.x -= 1.;
        }
        else if keyboard_input.pressed(KeyCode::KeyD) {
            move2.x += 1.;
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            move2.y += 1.;
        } else if keyboard_input.pressed(KeyCode::KeyS) {
            move2.y -= 1.;
        }

        move2 = move2.normalize_or_zero() * vel.0 * time.delta_secs();

        transform.translation.x += move2.x;
        transform.translation.y += move2.y;
    }
}