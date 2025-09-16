use bevy::prelude::*;
mod move_item;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(move_item::move_item::MoveItem)
    .run();
}