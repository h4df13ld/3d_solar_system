use bevy::prelude::*;

pub struct UserCameraPlugin;

impl Plugin for UserCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_camera);
           // .add_system(camera_controls);
    }
}

// spawn a camera into the system
fn add_camera(
    mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-44.0, 300.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(Name::new("Camera"));
}


// fn camera_controls(
//     keyboard: Res<Input<KeyCode>>,
//     mut camera_query: Query<&mut Transform, With<Camera3d>>,
//     time: Res<Time>) {
//     let mut camera = camera_query.single_mut();

// }