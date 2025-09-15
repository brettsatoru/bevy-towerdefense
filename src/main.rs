use bevy::prelude::*;
mod greet_system;
use greet_system::greetsystem::HelloPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(HelloPlugin)
    .run();
}