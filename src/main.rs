mod camera;
mod planet;

use std::env;

pub use camera::*;
pub use planet::*;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const WIDTH: f32 = 1400.0;
pub const HEIGHT: f32 = 1000.0;

fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

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
        .add_system(exit_game)
        .run();
}
