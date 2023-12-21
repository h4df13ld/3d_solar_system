use std::time::Duration;
use bevy::prelude::*;

#[derive(Reflect, Component, Clone, Copy, Debug, Default)]
#[reflect(Component)]
pub enum CameraSetting {
    Manual,
    #[default]
    Automatic
}

#[derive(Reflect, Component, Default, Debug)]
#[reflect(Component)]
pub struct CameraSettingButton {
    button_type: CameraSetting
}

#[derive(Reflect, Component, Default, Debug)]
#[reflect(Component)]
pub enum TimeSpeedSetting {
    Slow,
    #[default]
    Normal,
    Fast
}

#[derive(Reflect, Component, Default, Debug)]
#[reflect(Component)]
pub struct TimeSpeedSettingButton {
    button_type: TimeSpeedSetting
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TimeSpeedSettingButton>()
            .register_type::<CameraSettingButton>()
            .add_startup_system(create_ui);
    }
}

fn create_ui(mut commands: Commands, asset_server: Res<AssetServer>){
    
}