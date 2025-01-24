mod camera;

use crate::camera::PanCameraPlugin;
use bevy::prelude::*;
use bevy::utils::tracing::Instrument;

use std::f32::consts::*;
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        .insert_resource(DirectionalLightShadowMap { size: 4096 })


        .add_plugins(PanCameraPlugin)

        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_ship)


        .run();
}

fn spawn_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("3d/ships/boat-sail-a.glb"))),
        // SceneRoot(asset_server.load("3d/ships/boat-sail-b.glb")),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // // camera
    // commands.spawn((
    //     Camera3d::default(),
    //     Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    // ));
}


fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_secs() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}
