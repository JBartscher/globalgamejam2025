mod camera;
mod map;
mod ship;
mod collision;
mod ui;

use bevy::color::palettes::css::WHITE_SMOKE;
use crate::ship::ShipAssets;
use crate::camera::PanCameraPlugin;
use bevy::prelude::*;
use bevy::utils::tracing::Instrument;
use bevy::pbr::{DirectionalLightShadowMap};
use bevy_asset_loader::prelude::*;
use bevy_rand::prelude::*;
use crate::collision::CollisionPlugin;
use crate::map::{MapAssets, MapPlugin};
use crate::ship::ShipPlugin;
use crate::ui::UiPlugin;

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
        .add_plugins(CollisionPlugin)
        .add_plugins(UiPlugin)

        .add_plugins(EntropyPlugin::<WyRand>::default())

        .insert_resource(DirectionalLightShadowMap { size: 4048 })

        .add_systems(OnEnter(GameState::Game), setup)

        .run();
}

fn setup(mut commands: Commands) {
    // ambient light
    commands.insert_resource(AmbientLight {
        color: WHITE_SMOKE.into(),
        brightness: 80.,
    });

}
