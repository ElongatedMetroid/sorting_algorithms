use bevy::prelude::*;
use visualization::sort_visualize::SortVisualizePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SortVisualizePlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}