use bevy::prelude::*;
use bevy_water::WaterParam;

pub struct ShipPlugin;

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

        front.y += self.water_line - 0.2;
        transform.look_at(front, normal);

        transform.translation.y = ((front.y + left.y + right.y) / 3.0) + self.water_line;
    }
}

fn spawn_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    commands.spawn((
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("3d/ships/boat-sail-a.glb")),
        ),
        Ship::new(-0.100, -4.0, 8.0, -2.0, 2.0),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}

pub fn update_ships(
    water: WaterParam,
    mut ships: Query<(&Ship, &mut Transform, &GlobalTransform)>,
) {
    for (ship, mut transform, global) in ships.iter_mut() {
        let pos = global.translation();
        ship.update(&water, pos, &mut transform);
    }
}

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ship)
            .add_systems(Update, update_ships);
    }
}
