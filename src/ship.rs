use std::time::Duration;
use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_water::WaterParam;
use rand::prelude::*;

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
    fn sample(&self) -> Handle<Scene> {
        let mut rng = rand::thread_rng();
        let index: u8 = rng.gen_range(0..4);
        match index {
            0 => self.sail_a.clone(),
            1 => self.sail_b.clone(),
            2 => self.cargo_a.clone(),
            3 => self.cargo_b.clone(),
            4 => self.cargo_c.clone(),
            _ => unreachable!(),
        }
    }
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

/// generates a random point in bounds to the map
fn point_on_map() -> f32{
    let mut rng = rand::thread_rng();
    let rand_num = rng.gen_range(0..=255);
    // Shift the range to be between -125 and 125
    let rand_num = rand_num - 125;
    rand_num as f32
}

impl Command for Ship {
    fn apply(self, world: &mut World) {

        if let Some(ships)= world.get_resource::<ShipAssets>(){

            let mut transform_1 = Transform::from_xyz(0.0, 0.0, 0.0);
            transform_1.scale = Vec3::new(2., 2., 2.);

            let mut control_points : Vec<Vec3>  = Vec::new();
            for _ in 0..5 {
                control_points.push( Vec3::new(point_on_map(), 0., point_on_map()))
            }

            let curve = CubicCardinalSpline::new_catmull_rom(control_points).to_curve_cyclic().unwrap();

            let mut control_points : Vec<Vec3>  = Vec::new();
            for _ in 0..5 {
                control_points.push( Vec3::new(point_on_map(), 0., point_on_map()))
            }

            let curve = CubicCardinalSpline::new_catmull_rom(control_points).to_curve_cyclic().unwrap();

            world.spawn((
                SceneRoot(
                    // GltfAssetLabel::Scene(0).clone_from(ship_assets.sail_a.clone()),
                    ships.sample().into(),
                    // asset_server.load(GltfAssetLabel::Scene(0).from_asset("3d/ships/boat-sail-b.glb")),
                ),
                Ship::new(-0.100, -3., 3.0, -2.0, 2.0),
                transform_1,
                PathFollow{ curve, t: 0.0 }
            )).with_children(|parent| {
                parent.spawn((
                    PointLight {
                        shadows_enabled: false,
                        ..default()
                    },
                    Transform::from_xyz(0.0, 5.0, 0.0),
                ));
            });
        }

    }
}

#[derive(Resource)]
struct ShipSpawnManager {
    spawn_timer: Timer,
    current_ships: u32,
    max_ships: u32,
}

impl Default for ShipSpawnManager{
    fn default() -> Self {
        Self{
            spawn_timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
            current_ships: 1,
            max_ships: 5,
        }
    }
}


fn spawn_ships(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_manager: ResMut<ShipSpawnManager>,
) {
    spawn_manager.spawn_timer.tick(time.delta());

    // if it finished, despawn the bomb
    if spawn_manager.spawn_timer.finished() && spawn_manager.current_ships < spawn_manager.max_ships {
        commands.queue(Ship::new(-0.100, -3., 3.0, -2.0, 2.0));
        spawn_manager.current_ships += 1;
    }

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


pub fn move_ship(mut query: Query<(&mut PathFollow, &mut Transform), (With<Ship>)>, time: Res<Time>,){
    for (mut path_follow, mut ship_transform) in query.iter_mut() {
        path_follow.t += 0.1 * time.delta_secs();

        if path_follow.t > path_follow.curve.segments().len() as f32 {
            path_follow.t = 0.0;
        }

        let pos = path_follow.curve.position(path_follow.t);
        ship_transform.translation = pos;
        ship_transform.look_at(path_follow.curve.position(path_follow.t - 0.01), Dir3::Y);
    }
}

pub fn update_ships(
    water: WaterParam,
    mut ships: Query<(&Ship, &mut Transform, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    for (ship, mut transform, global) in ships.iter_mut() {
        let pos = global.translation();
        ship.update(&water, pos, &mut transform);
    }
}

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), setup)
            .add_systems(Update, spawn_ships.run_if(in_state(GameState::Game)))
            .add_systems(Update, update_ships.run_if(in_state(GameState::Game)))
            .add_systems(Update, draw_follow_path.run_if(in_state(GameState::Game)))
            .add_systems(Update, move_ship.run_if(in_state(GameState::Game)))
        ;
    }
}

fn setup( mut commands: Commands,){
    commands.queue(Ship::new(-0.100, -3., 3.0, -2.0, 2.0));
    commands.init_resource::<ShipSpawnManager>();
}
