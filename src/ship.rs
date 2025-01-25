use crate::GameState;
use bevy::color::palettes::basic::YELLOW;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_water::WaterParam;

pub struct ShipPlugin;

#[derive(AssetCollection, Resource)]
pub struct ShipAssets {
    #[asset(path = "3d/ships/boat-sail-a.glb")]
    sail_a: Handle<Scene>,
    #[asset(path = "3d/ships/boat-sail-a.glb")]
    sail_b: Handle<Scene>,
    #[asset(path = "3d/ships/ship-cargo-a.glb")]
    cargo_a: Handle<Scene>,
    #[asset(path = "3d/ships/ship-cargo-b.glb")]
    cargo_b: Handle<Scene>,
    #[asset(path = "3d/ships/ship-cargo-c.glb")]
    cargo_c: Handle<Scene>,
}

impl ShipAssets {
    pub fn random() {}
}

#[derive(Component)]
#[require(Transform)]
struct Ship {
    water_line: f32,
    front: Vec3,
    back_left: Vec3,
    back_right: Vec3,
}

impl Ship {
    pub fn new(water_line: f32, front: f32, back: f32, left: f32, right: f32) -> Self {
        Self {
            water_line,
            front: Vec3::new(0.0, 0.0, front),
            back_left: Vec3::new(left, 0.0, back),
            back_right: Vec3::new(right, 0.0, back),
        }
    }

    fn update(&self, water: &WaterParam, pos: Vec3, transform: &mut Transform) {
        let (yaw, _pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let global = Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(yaw));

        // Get the wave position at the front, back_left and back_right.
        let mut front = water.wave_point(global.transform_point(self.front));
        let left = water.wave_point(global.transform_point(self.back_left));
        let right = water.wave_point(global.transform_point(self.back_right));
        let normal = (left - front).cross(right - front).normalize();

        front.y += self.water_line - 0.1;
        transform.look_at(front, normal);

        transform.translation.y = ((front.y + left.y + right.y) / 3.0) + self.water_line;
    }
}

fn spawn_ship(
    mut commands: Commands,
    ship_assets: Res<ShipAssets>,
    asset_server: Res<AssetServer>,
) {
    let mut transform = Transform::from_xyz(0.0, 0.5, 0.0);
    transform.scale = Vec3::new(2., 2., -2.);

    commands.spawn((
        SceneRoot(
            // GltfAssetLabel::Scene(0).clone_from(ship_assets.sail_a.clone()),
            // ship_assets.sail_b.clone().into(),
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("3d/ships/boat-sail-b.glb")),
        ),
        Ship::new(-0.100, -3., 3.0, -2.0, 2.0),
        transform,
    ));
}

pub fn update_ships(
    water: WaterParam,
    mut ships: Query<(&Ship, &mut Transform, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    for (ship, mut transform, global) in ships.iter_mut() {
        let pos = global.translation();
        ship.update(&water, pos, &mut transform);

        // front
        gizmos.arrow(ship.front, ship.front + Vec3::new(0., 10., 0.), YELLOW);
        // back left
        gizmos.arrow(
            ship.back_left,
            ship.back_left + Vec3::new(0., 10., 0.),
            YELLOW,
        );
        // back right
        gizmos.arrow(
            ship.back_right,
            ship.back_right + Vec3::new(0., 10., 0.),
            YELLOW,
        );
    }
}

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), spawn_ship)
            .add_systems(Update, update_ships.run_if(in_state(GameState::Game)));
    }
}
