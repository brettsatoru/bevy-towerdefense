use bevy::prelude::*;
mod move_item;
mod pathing_entity;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(move_item::move_item::MoveItem)
    .add_plugins(pathing_entity::pathing_entity::PathAgent)
    .run();
}