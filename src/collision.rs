use bevy::prelude::*;
use crate::GameState;


pub struct CollisionPlugin;

#[derive(Component)]
pub struct Collider {
    pub(crate) rect: Rect,
}

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (check_collision).run_if(in_state(GameState::Game)))
        .add_event::<CollisionEvent>();
    }
}

#[derive(Event)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

fn check_collision(
    mut query: Query<(Entity, &GlobalTransform, &Collider)>,
    mut commands: Commands,
    mut collision_event_writer: EventWriter<CollisionEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (entity_a, transform_a, mut collider_a) in query.iter() {
        for (entity_b, transform_b, mut collider_b) in query.iter() {
            if (entity_a == entity_b) {
                continue;
            }

            let rect_a_pos = Rect::from_corners(transform_a.translation().xz(), transform_a.translation().xz() + collider_a.rect.max);
            let rect_b_pos = Rect::from_corners(transform_b.translation().xz(), transform_b.translation().xz() + collider_b.rect.max);
            if !rect_a_pos.intersect(rect_b_pos).is_empty() {
                // next_state.set(GameState::GameOver);
                collision_event_writer.send(CollisionEvent {entity_a, entity_b});
            }
        }
    }
}
