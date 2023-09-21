mod camera;
mod planet;

use std::env;

pub use camera::*;
pub use planet::*;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const WIDTH: f32 = 1400.0;
pub const HEIGHT: f32 = 1000.0;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
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
        .add_plugin(UserCameraPlugin)
        .add_plugin(SolarSystemObjectPlugin)
        .run();
}
