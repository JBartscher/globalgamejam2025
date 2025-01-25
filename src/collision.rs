use bevy::color::palettes::basic::GREEN;
use bevy::prelude::*;
use crate::GameState;

pub struct CollisionPlugin;

#[derive(Component)]
pub struct Collider {
    pub(crate) radius: f32,
}

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_collision.run_if(in_state(GameState::Game)))
        .add_systems(Update, draw_collider_gizmos.run_if(in_state(GameState::Game)));
    }
}

fn check_collision(
    mut query: Query<(Entity, &GlobalTransform, &Collider)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (entity_a, transform_a, mut collider_a) in query.iter() {
        for (entity_b, transform_b, mut collider_b) in query.iter() {
            let distance = transform_a
                .translation()
                .distance(transform_b.translation());
            if (entity_a == entity_b) {
                continue;
            }
            if distance < (collider_a.radius + collider_b.radius) {
                print!("Collision!");
                next_state.set(GameState::GameOver);
            }
        }
    }
}

fn draw_collider_gizmos(
    mut query: Query<(&Transform, &Collider)>,
    mut gizmos: Gizmos,
) {
    for (transform, collider) in query.iter() {
        gizmos.cuboid(
            transform.with_scale(Vec3::splat(collider.radius)),
            GREEN,
        );
    }
}
