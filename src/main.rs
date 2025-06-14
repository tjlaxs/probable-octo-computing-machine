use bevy::prelude::*;

use status::GitStatusPlugin;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GitStatusPlugin)
        .add_systems(Startup, setup)
        .run();
}
