use bevy::app::App;
use bevy::math::{vec2, UVec2};
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_water::*;


pub const WATER_HEIGHT: f32 = 1.0;

pub const MAP_SIZE: u32 = 256;

#[derive(AssetCollection, Resource)]
pub struct MapAssets {
    #[asset(path = "3d/environment/rocks-a.glb")]
    rocks_a: Handle<Scene>,
    #[asset(path = "3d/environment/rocks-b.glb")]
    rocks_b: Handle<Scene>,
    #[asset(path = "3d/environment/rocks-c.glb")]
    rocks_c: Handle<Scene>
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {

        let mut water_settings = WaterSettings::default();
        water_settings.spawn_tiles = Some(UVec2::new(1, 1));
        water_settings.amplitude = 0.7;

        app
            .insert_resource(water_settings)
            .add_plugins(WaterPlugin)
        ;
    }
}

fn get_random_point_on_map() -> Vec<Vec2>{
    let default_points = vec![
        vec2(-1.0, -20.0),
        vec2(3.0, 2.0),
        vec2(5.0, 3.0),
        vec2(9.0, 8.0),
    ];

    default_points
}