//ai.rs
use crate::prelude::*;
use rand::Rng;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, process_passive_enemies);
}

fn process_passive_enemies(
    mut query: Query<
        (
            &mut KinematicCharacterController,
            &Speed,
            Option<&mut Destination>,
            &AiMode,
            &Transform,
            Entity,
        ),
        With<Enemy>,
    >,
    time: Res<Time>,
    mut commands: Commands,
) {
    let move_radius = 100.0;
    let min_proximity = 20.0;

    for (mut controller, speed, destination, ai_mode, transform, entity) in query.iter_mut() {
        if AiMode::Passive == *ai_mode {
            if let Some(destination) = destination {
                let current_position = controller
                    .translation
                    .unwrap_or_else(|| transform.translation.truncate());
                let direction = (destination.0
                    - Vec3::new(current_position.x, current_position.y, 0.0))
                .normalize();
                let distance =
                    Vec3::new(current_position.x, current_position.y, 0.0).distance(destination.0);

                if distance > min_proximity {
                    let movement =
                        Some(Vec2::new(direction.x, direction.y) * speed.0 * time.delta_seconds());
                    controller.translation = movement;
                } else {
                    if let Some(mut entity_ref) = commands.get_entity(entity) {
                        entity_ref.remove::<Destination>();
                    }
                }
            } else {
                if let Some(mut entity_ref) = commands.get_entity(entity) {
                    entity_ref.insert(Destination(random_point_within_radius(
                        transform.translation,
                        move_radius,
                    )));
                }
            }
        }
    }
}

fn random_point_within_radius(center: Vec3, radius: f32) -> Vec3 {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
    let distance = rng.gen_range(-radius..radius);
    let x_offset = distance * angle.cos();
    let y_offset = distance * angle.sin();
    Vec3::new(center.x + x_offset, center.y + y_offset, center.z)
}
