use bevy::prelude::*;
use bevy::math::*;

#[derive(Component)]
struct Velocity(f32);
impl Default for Velocity {
    fn default() -> Self {
        Velocity(50.)
    }
}

#[derive(Component)]
#[require(Transform, Velocity, Name)]
struct PathEntity
{
    current: i32,
    path : Vec<Vec2>
}

pub struct PathAgent;
impl Plugin for PathAgent {
    fn build(&self, app : &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, run_pathing);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        PathEntity{
            current: 0,      
            path: vec![Vec2::splat(200.), Vec2::ZERO, Vec2::splat(-200.)]},
        Transform::from_translation(Vec3::new(-50., 50., 0.)),
        Sprite::from_image(asset_server.load("circle.png"))
        ));
}

fn run_pathing(time: Res<Time>,
    mut pather : Query<(&Velocity, &mut Transform, &mut PathEntity)>) {
        for (vel, mut transform, mut path_agent) in &mut pather {
            if path_agent.current < path_agent.path.len() as i32 {
                let target = path_agent.path[path_agent.current as usize];
                let new_pos = transform.translation.move_towards(Vec3::new(target.x, target.y, 0.), vel.0 * time.delta_secs());
                if new_pos == Vec3::new(target.x, target.y, 0.)
                {
                    path_agent.current += 1;
                    if path_agent.current > path_agent.path.len() as i32 {
                        path_agent.current = 0;
                    }
                }
                transform.translation = new_pos;
            }
        }
}

