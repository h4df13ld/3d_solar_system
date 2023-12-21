use std::f32::consts::PI;

use bevy::prelude::*;
use crate::SolarSystemObjectData;

pub const CAMERA_MOVE_SPEED: f32 = 200.0;
pub const CAMERA_PAN_SPEED: f32 = 0.5;
pub const CAMERA_CHANGE_TIME: f32 = 5.0;
pub const ORBITAL_SPEED_CONSTANT: f32 = 100.0; 


#[derive(Reflect, Component, Default, Debug)]
#[reflect(Component)]
pub struct AutomaticCameraParameters {
    pub orbit_angle: f32,
    pub orbit_speed: f32,
    pub orbit_distance: f32,
    pub viewing_height: f32,
    pub automatic_on: bool
}

#[derive(Reflect)]
#[derive(Resource)]
pub struct CameraViewTimer {
    pub timer: Timer,
}

impl Default for CameraViewTimer {
    fn default() -> CameraViewTimer {
        CameraViewTimer {timer: Timer::from_seconds(
            CAMERA_CHANGE_TIME, TimerMode::Repeating)
        }
    }
}

pub struct UserCameraPlugin;
impl Plugin for UserCameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AutomaticCameraParameters>()
        .register_type::<CameraViewTimer>()
        .init_resource::<CameraViewTimer>()
        .add_startup_system(add_camera)
        .add_system(camera_controls)
        .add_system(camera_pan)
        .add_system(automatic_camera)
        .add_system(change_camera_view_type);
    }
}



// add a timer to change camera view from auto to manual after a set period of time
fn change_camera_view_type(
    mut commands: Commands,
    mut camera_timer: ResMut<CameraViewTimer>,
    mut camera_query: Query<&mut AutomaticCameraParameters, With<Camera3d>>,
    time: Res<Time>
) {    
    camera_timer.timer.tick(time.delta());
    if camera_timer.timer.just_finished() {
        println!("Ding Dong");
        if let Ok(mut automatic_camera) = 
        camera_query.get_single_mut() {
            match automatic_camera.automatic_on {
                false => automatic_camera.automatic_on = true,
                true => automatic_camera.automatic_on = false
            }
        }
    }
}

// spawn a camera into the system
fn add_camera(
    mut commands: Commands
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 400.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        AutomaticCameraParameters{
            orbit_angle: 0.0,
            orbit_speed: 1.0,
            orbit_distance: 5.0,
            viewing_height: 5.0,
            automatic_on: true
        }
    )).insert(Name::new("Camera"));
}

// create some camera controls
fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>
) {
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

    let movement: Vec3 = direction_to_move.normalize_or_zero() * 
        CAMERA_MOVE_SPEED * time.delta_seconds();
    camera.translation += movement;

}

fn camera_pan(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>
) {
    let mut camera = camera_query.single_mut();

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

fn automatic_camera(
    mut camera_query: Query<(&mut Transform, &mut AutomaticCameraParameters), With<Camera3d>>,
    solar_system_object_query: Query<(&Transform, &SolarSystemObjectData), Without<Camera3d>>,
    time: Res<Time>
) {

    if let Ok((mut camera_transform, mut camera_parameters)) 
    = camera_query.get_single_mut() {
        if camera_parameters.automatic_on == false {
            return
        }
        for (solar_system_object_transform, solar_system_object_data)
        in &solar_system_object_query {
            if solar_system_object_data.name != "Earth".to_string() {
                continue;
            }
            let planet_coordinates: Vec3 = solar_system_object_transform.translation;
            let orbit_angle: f32 = camera_parameters.orbit_angle;
            let orbit_distance: f32 = camera_parameters.orbit_distance;
            let orbit_speed: f32 = camera_parameters.orbit_speed;
            let viewing_height: f32 = camera_parameters.viewing_height;

            let mut new_orbit_angle: f32 = orbit_angle + 
                (time.delta_seconds() * (orbit_speed / ORBITAL_SPEED_CONSTANT));
            if new_orbit_angle > (2.0 * PI) {
                new_orbit_angle = 0.0;
            }
            camera_parameters.orbit_angle = new_orbit_angle;

            let orbit_x_distance: f32 = orbit_distance * new_orbit_angle.cos();
            let orbit_z_distance: f32 = orbit_distance * new_orbit_angle.sin();

            camera_transform.translation.x = planet_coordinates.x + orbit_x_distance; 
            camera_transform.translation.z = planet_coordinates.z + orbit_z_distance;
            camera_transform.translation.y = planet_coordinates.y + viewing_height;
            camera_transform.look_at(planet_coordinates, Vec3::Y);
        }
    }
}
   