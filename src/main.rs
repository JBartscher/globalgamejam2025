mod camera;
mod map;
mod ship;

use crate::camera::PanCameraPlugin;
use bevy::prelude::*;
use bevy::utils::tracing::Instrument;

use std::f32::consts::*;
use bevy::pbr::{DirectionalLightShadowMap};
use crate::map::MapPlugin;
use crate::ship::ShipPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        .insert_resource(DirectionalLightShadowMap { size: 8024 })

        .add_plugins(PanCameraPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(ShipPlugin)

        .add_systems(Startup, setup)

        .run();
}



fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
