use bevy::app::App;
use bevy::prelude::Plugin;
use bevy_water::*;


pub const WATER_HEIGHT: f32 = 1.0;
#[cfg(feature = "atmosphere")]
pub const SPEED_MIN: f32 = 0.05;
#[cfg(feature = "atmosphere")]
pub const SPEED_DELTA: f32 = 0.01;
#[cfg(feature = "atmosphere")]
pub const SPEED_MAX: f32 = 1.0;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WaterSettings::default())
            .add_plugins(WaterPlugin)
        ;
    }
}