pub const GRAV_CONST: f64 = 6.6743e-11;  // m3 kg-1 s-2
pub const SUN_MASS: f64 = 1.989e30; //kg
pub const SOLAR_SYSTEM_TIME_FACTOR: f64 = 100000.0; // used to speed up the solar system
pub const SOLAR_SYSTEM_SPIN_FACTOR: f32 = 20.0;

use bevy::prelude::*;

// struct to contain data for a solar system object
#[derive(Reflect, Component, Default, Debug)]
#[reflect(Component)]
pub struct SolarSystemObjectData {
    pub name: String,
    pub mass_kg: f64,
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: f64,
    pub speed_x: f64,
    pub speed_y: f64,
    pub speed_z: f64,
    pub acceleration_x: f64,
    pub acceleration_y: f64,
    pub acceleration_z: f64,
    pub spin: f64,
    pub tilt: f32
}

// #[derive(Resource)]
// pub struct GameAssets {
//     sun_scene: Handle<Scene>,
// }

pub struct SolarSystemObjectPlugin;

impl Plugin for SolarSystemObjectPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SolarSystemObjectData>()
            .add_startup_system(add_solar_system_objects)
            .add_startup_system(set_planetary_objects_tilt)
            // .add_system(set_planetary_objects_scale)
            .add_system(move_solar_system_objects)
            .add_system(spin_planetary_objects);
            // .add_system(check_solar_object_collision);
    }
}

// add all objects to the solar system
fn add_solar_system_objects(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    // ADD BACKGROUND GALAXY
    commands.spawn((
            SceneBundle {
                // scene: planetary_assets.sun_scene.clone(),
                // scene: assets.load("sun.glb#Scene0"),
                scene: assets.load("galaxy.glb#Scene0"),
                transform: Transform::from_xyz(10.0, 0.0, 10.0)
                    .with_scale(Vec3::splat(3000.0)),
                ..default()
                },
        )).insert(Name::new("Background Galaxy"));
        
    // ADD SOLAR SYSTEM OBJECTS 
    commands.spawn((
        SceneBundle {
            scene: assets.load("sun.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
            },
        SolarSystemObjectData {
            name: "Sun".to_string(),
            mass_kg: SUN_MASS,
            position_x: 0.0,
            position_y: 0.0,
            position_z: 0.0,
            speed_x: 0.0,
            speed_y: 0.0,
            speed_z: 0.0,
            acceleration_x: 0.0,
            acceleration_y: 0.0,
            acceleration_z: 0.0,
            spin: 0.0,
            tilt: 0.0
            }
    )).insert(Name::new("Test Sun"));
    

    commands.spawn((
        SceneBundle {
            scene: assets.load("earth2.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 100.0)
                .with_scale(Vec3::splat(1.0)),
            ..default()
        },
        SolarSystemObjectData {
            name: "Earth".to_string(),
            mass_kg: 5.972e24,
            position_x: 149.6e9,
            position_y: 0.0,
            position_z: 0.0,
            speed_x: 0.0,
            speed_y: 0.0,
            speed_z: 29780.0,
            acceleration_x: 0.02,
            acceleration_y: 0.0,
            acceleration_z: 0.0,
            spin: 1.0,
            tilt: 23.0
        }
    )).insert(Name::new("Earth"));


    commands.spawn((
        SceneBundle {
            scene: assets.load("moon.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 100.0)
                .with_scale(Vec3::splat(0.05)),
            ..default()
        },
        SolarSystemObjectData {
            name: "Moon".to_string(),
            mass_kg: 7.34767e22,
            position_x: 149.9844e9,
            position_y: 0.0,
            position_z: 0.0,
            speed_x: 0.0,
            speed_y: 0.0,
            speed_z: 30780.0,
            acceleration_x: 0.0002,
            acceleration_y: 0.0,
            acceleration_z: 0.0,
            spin: -0.65,
            tilt: 10.0
        }
    )).insert(Name::new("Moon"));


    commands.spawn((
        SceneBundle {
            scene: assets.load("mars_2.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 100.0)
                .with_scale(Vec3::splat(1.0)),
            ..default()
        },
        SolarSystemObjectData {
            name: "Mars".to_string(),
            mass_kg: 6.39e23,
            position_x: 228.6e9,
            position_y: 0.0,
            position_z: 0.0,
            speed_x: 0.0,
            speed_y: 0.0,
            speed_z: 24077.0,
            acceleration_x: 0.02,
            acceleration_y: 0.0,
            acceleration_z: 0.0,
            spin: 0.9671,
            tilt: 25.0
        }
    )).insert(Name::new("Mars"));


    // LIGHT TO ILLUMINATE SOLAR SYSTEM
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 7e6,
            range: 1e12,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-200.0, 0.0, 0.0),
        ..default()
        }).insert(Name::new("Point Light -x"));


    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 7e6,
            range: 1e12,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(200.0, 0.0, 0.0),
        ..default()
        }).insert(Name::new("Point Light +x"));


    commands.spawn(SpotLightBundle {
        spot_light: SpotLight {
            intensity: 7e6,
            range: 1e12,
            shadows_enabled: true,
            outer_angle: 0.07,
            inner_angle: 0.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 200.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(Name::new("Spot Light +z"));
    

    commands.spawn(SpotLightBundle {
        spot_light: SpotLight {
            intensity: 7e6,
            range: 1e12,
            shadows_enabled: true,
            outer_angle: 0.07,
            inner_angle: 0.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, -200.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(Name::new("Spot Light -z"));
}




/// Iterate through the solar system objects and calculate total forces on each solar system object.
/// Use total force to determine a instant acceleration, and use that to determine
/// new object speed and position. Acceleration, speed and position are all
/// split into x, y & z components
fn move_solar_system_objects(
    mut object_query: Query<(&mut SolarSystemObjectData, &mut Transform)>,
    time: Res<Time>
) {  
    const RADIUS_TO_TRANSLATION_RATIO: f64 = 644444444.0;

    // obtain current coordinates and mass of each solar system object
    // and add into an array
    // need (Name, mass, position_x, position_y, position_z)
    let mut all_planatery_objects_data: Vec<(String, f64, f64, f64, f64)> = Vec::new();
    for (object_data, _transform) in &object_query {
        let name: String = object_data.name.clone();
        let mass: f64 = object_data.mass_kg.clone();
        let position_x: f64 = object_data.position_x.clone();
        let position_y: f64 = object_data.position_y.clone();
        let position_z: f64 = object_data.position_z.clone();
        all_planatery_objects_data.push((name, mass, position_x, position_y, position_z));
    }

    // setup a new array to manage total forces on each solar system object
    // set as (name, total_x_force, total_y_force, total_z_force)
    let mut total_forces_on_objects: Vec<(String, f64, f64, f64)> = Vec::new();

    // itterate through each solar system object and sum x, y and z forces excerted
    // on it by every other solar system object
    for (object_data, _transform) in &object_query {
        let object_name: String = object_data.name.clone();
        total_forces_on_objects.push((object_data.name.clone(), 0.0, 0.0, 0.0));
        for (other_object_name,
            other_object_mass,
            other_object_pos_x,
            other_object_pos_y,
            other_object_pos_z) in &all_planatery_objects_data {

                if object_name.to_string() == other_object_name.to_string() {
                    // DONT COMPARE OBJECT TO ITSELF
                    continue;
                }
                // determine x, y and z differences in distance between objects
                let diff_x_distance: f64 = 
                    other_object_pos_x - object_data.position_x.clone();
                let diff_y_distance: f64 = 
                    other_object_pos_y - object_data.position_y.clone();
                let diff_z_distance: f64 = 
                    other_object_pos_z - object_data.position_z.clone();

                // square the differences
                let diff_x_squared: f64 = diff_x_distance.powi(2);
                let diff_y_squared: f64 = diff_y_distance.powi(2);
                let diff_z_squared: f64 = diff_z_distance.powi(2);

                // obtain total distance between objects
                let distance_between_objects: f64 = (diff_x_squared +
                    diff_y_squared + diff_z_squared).sqrt();

                // determine the force between objects
                // https://physics.stackexchange.com/questions/17285/split-gravitational-force-into-x-y-and-z-componenets
                let force_x: f64 = ((GRAV_CONST * other_object_mass * 
                    object_data.mass_kg.clone()) / 
                    (distance_between_objects.powi(3)))
                    * diff_x_distance;
                let force_y: f64 = ((GRAV_CONST * other_object_mass * 
                    object_data.mass_kg.clone()) / 
                    (distance_between_objects.powi(3)))
                    * diff_y_distance;
                let force_z: f64 = ((GRAV_CONST * other_object_mass * 
                    object_data.mass_kg.clone()) / 
                    (distance_between_objects.powi(3)))
                    * diff_z_distance;

                if let Some(object) = 
                    total_forces_on_objects
                    .iter_mut()
                    .find(|(name, _, _, _)| name == &object_name
                ) {
                    object.1 += force_x;
                    object.2 += force_y;
                    object.3 += force_z;
                } else {
                    println!("No object found with name {}", &object_name);
                }        
            }        
        }   

    // MATCH OBJECT FORCES IN LIST TO OBJECT, THEN USE FORCES TO UPDATE TRANSLATION
    for (mut object_data, mut transform) 
        in &mut object_query {
        if let Some(object_total_force)
            = total_forces_on_objects
            .iter_mut()
            .find(|(name, _, _, _)| name == &object_data.name.clone()
        ){
            
            // determine acceleration components (F=ma -> a=F/m)
            object_data.acceleration_x = object_total_force.1 / object_data.mass_kg;            
            object_data.acceleration_y = object_total_force.2 / object_data.mass_kg;            
            object_data.acceleration_z = object_total_force.3 / object_data.mass_kg;     

            // update object x, y and z speed components
            object_data.speed_x += 1.0 * object_data.acceleration_x * 
                time.delta_seconds_f64() * SOLAR_SYSTEM_TIME_FACTOR;
            object_data.speed_y += 1.0 * object_data.acceleration_y * 
                time.delta_seconds_f64() * SOLAR_SYSTEM_TIME_FACTOR;
            object_data.speed_z += 1.0 * object_data.acceleration_z * 
                time.delta_seconds_f64() * SOLAR_SYSTEM_TIME_FACTOR;

            // update object x. y and z position components
            object_data.position_x += object_data.speed_x * 
                time.delta_seconds_f64() * SOLAR_SYSTEM_TIME_FACTOR;
            object_data.position_y += object_data.speed_y * 
                time.delta_seconds_f64() * SOLAR_SYSTEM_TIME_FACTOR;
            object_data.position_z += object_data.speed_z * 
                time.delta_seconds_f64() * SOLAR_SYSTEM_TIME_FACTOR;

            // update the translation position of object
            transform.translation.x = (object_data.position_x / 
                RADIUS_TO_TRANSLATION_RATIO) as f32;
            transform.translation.y = (object_data.position_y /
                RADIUS_TO_TRANSLATION_RATIO) as f32;
            transform.translation.z = (object_data.position_z / 
                RADIUS_TO_TRANSLATION_RATIO) as f32;
        } else {
            println!("Failed to update component positions")
        }
    }
}


fn set_planetary_objects_tilt(
    mut object_query: Query<(&mut Transform, &SolarSystemObjectData)>
){
    for (mut transform, solar_system_object)
    in &mut object_query {
        let tilt: f32 = solar_system_object.tilt;
        transform.rotate_local_z(tilt.to_radians());
        println!("Tilted")
    }
}

fn spin_planetary_objects(
    mut object_query: Query<(&mut Transform, &SolarSystemObjectData)>,
    time: Res<Time>
){    
    for (mut transform, solar_system_object) 
    in &mut object_query {
        let tilt: f32 = solar_system_object.tilt;
        let tilt_radians: f32 = tilt.to_radians();        
        let spin_rate: f32 = solar_system_object.spin as f32;
        let angle_to_rotate: f32 = (spin_rate * time.delta_seconds() * 
            SOLAR_SYSTEM_SPIN_FACTOR).to_radians();
        transform.rotate_axis(Vec3 { x: (0.0), y: (1.0), z: (tilt_radians) }, angle_to_rotate);
    }
}