use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const WIDTH: f32 = 1400.0;
pub const HEIGHT: f32 = 1000.0;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "3D Solar System".to_string(),
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin)
        .run();
}
