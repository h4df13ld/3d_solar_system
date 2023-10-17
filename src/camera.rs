use bevy::prelude::*;
pub const CAMERA_MOVE_SPEED: f32 = 200.0;
pub const CAMERA_PAN_SPEED: f32 = 0.5;

pub struct UserCameraPlugin;

impl Plugin for UserCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_camera)
        .add_system(camera_controls)
        .add_system(camera_pan);
    }
}

// spawn a camera into the system
fn add_camera(
    mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        // transform: Transform::from_xyz(-44.0, 300.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        transform: Transform::from_xyz(0.0, 0.0, 400.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(Name::new("Camera"));
}

// create some camera controls
fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>) {


    let mut camera = camera_query.single_mut();
    let mut direction_to_move: Vec3 = Vec3::ZERO;

    if keyboard.pressed(KeyCode::W) {
        direction_to_move += camera.forward();
    }

    if keyboard.pressed(KeyCode::S) {
        direction_to_move += camera.back();
    }

    if keyboard.pressed(KeyCode::A) {
        direction_to_move += camera.left();
    }
    
    if keyboard.pressed(KeyCode::D) {
        direction_to_move += camera.right();
    }


    let movement: Vec3 = direction_to_move.normalize_or_zero() * CAMERA_MOVE_SPEED * time.delta_seconds();
    camera.translation += movement;

}

fn camera_pan(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>) {
    
    let mut camera = camera_query.single_mut();
    let mut rotate_around_z: Quat = Quat::from_rotation_z(0.0);


    if keyboard.pressed(KeyCode::Up) {
        camera.rotate_local_x(CAMERA_PAN_SPEED * time.delta_seconds());
    }

    if keyboard.pressed(KeyCode::Down) {
        camera.rotate_local_x(-CAMERA_PAN_SPEED * time.delta_seconds());
    }
    
    if keyboard.pressed(KeyCode::Right) {
        camera.rotate_local_y(-CAMERA_PAN_SPEED * time.delta_seconds());
    }

    if keyboard.pressed(KeyCode::Left) {
        camera.rotate_local_y(CAMERA_PAN_SPEED * time.delta_seconds());
    }

} 


// fn camera_controls(
//     keyboard: Res<Input<KeyCode>>,
//     mut camera_query: Query<&mut Transform, With<Camera3d>>,
//     time: Res<Time>) {
//     let mut camera = camera_query.single_mut();

// }