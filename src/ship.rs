use crate::GameState;
use bevy::color::palettes::basic::YELLOW;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_water::WaterParam;

pub struct ShipPlugin;

#[derive(AssetCollection, Resource)]
pub struct ShipAssets {
    #[asset(path = "3d/ships/boat-sail-a.glb#Scene0")]
    sail_a: Handle<Scene>,
    #[asset(path = "3d/ships/boat-sail-b.glb#Scene0")]
    sail_b: Handle<Scene>,
    #[asset(path = "3d/ships/ship-cargo-a.glb#Scene0")]
    cargo_a: Handle<Scene>,
    #[asset(path = "3d/ships/ship-cargo-b.glb#Scene0")]
    cargo_b: Handle<Scene>,
    #[asset(path = "3d/ships/ship-cargo-c.glb#Scene0")]
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

#[derive(Component)]
struct PathFollow {
    pub curve: CubicCurve<Vec3>,
    t: f32
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
    let mut transform_1 = Transform::from_xyz(0.0, 0.0, 0.0);
    transform_1.scale = Vec3::new(2., 2., 2.);

    let control_points = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(125.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -125.0),
        Vec3::new(-125.0, 0.0, 0.0),
    ];
    // Create the CubicBSpline -> CubicCurve<Vec3>
    let curve = CubicCardinalSpline::new_catmull_rom(control_points).to_curve_cyclic().unwrap();

    // sail a
    commands.spawn((
        SceneRoot(
            // GltfAssetLabel::Scene(0).clone_from(ship_assets.sail_a.clone()),
            ship_assets.sail_a.clone().into(),
            // asset_server.load(GltfAssetLabel::Scene(0).from_asset("3d/ships/boat-sail-b.glb")),
        ),
        Ship::new(-0.100, -3., 3.0, -2.0, 2.0),
        transform_1,
        PathFollow{ curve, t: 0.0 }
    ));
    
}

/// This system uses gizmos to draw the current [`Curve`] by breaking it up into a large number
/// of line segments.
fn draw_follow_path(path_follow_query: Query<&PathFollow>, mut gizmos: Gizmos) {

    for (p) in &path_follow_query {
        // Scale resolution with curve length so it doesn't degrade as the length increases.
       //  let resolution = 100 * p.curve. .len();
        gizmos.linestrip(
            p.curve.iter_positions(100),
            Color::srgb(1.0, 0.5, 0.2),
        );
    }

}


pub fn move_ship(mut ships: Query<(&Ship, &mut Transform)>){

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
        gizmos.arrow(ship.front + transform.translation, ship.front + transform.translation + Vec3::new(0., 10., 0.), YELLOW);
        // back left
        gizmos.arrow(
            ship.back_left + transform.translation,
            ship.back_left + transform.translation + Vec3::new(0., 10., 0.),
            YELLOW,
        );
        // back right
        gizmos.arrow(
            ship.back_right + transform.translation,
            ship.back_right + transform.translation + Vec3::new(0., 10., 0.),
            YELLOW,
        );
    }
}

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), spawn_ship)
            .add_systems(Update, update_ships.run_if(in_state(GameState::Game)))
            .add_systems(Update, draw_follow_path.run_if(in_state(GameState::Game)))
        ;
    }
}
