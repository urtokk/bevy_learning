use bevy::prelude::*;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

mod debug;
mod movement;
mod spaceship;

fn main() {
    App::new()
        .add_plugins(SpaceshipPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
