mod camera;
mod map;
mod ship;

use crate::ship::ShipAssets;
use crate::camera::PanCameraPlugin;
use bevy::prelude::*;
use bevy::utils::tracing::Instrument;
use bevy::pbr::{DirectionalLightShadowMap};
use bevy_asset_loader::prelude::*;
use crate::map::{MapAssets, MapPlugin};
use crate::ship::ShipPlugin;


#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Game,
    GameOver
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // load
        .init_state::<GameState>()
        .add_loading_state(
        LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::Game)
            .load_collection::<ShipAssets>()
            .load_collection::<MapAssets>()
        )

        .add_plugins(PanCameraPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(ShipPlugin)

        .insert_resource(DirectionalLightShadowMap { size: 4048 })

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
