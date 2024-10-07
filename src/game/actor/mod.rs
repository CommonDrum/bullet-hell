use crate::game::prelude::*;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Size(pub f32);

#[derive(Component)]
pub struct Actor;

pub fn spawn_actor(
    collider: Collider,
    size: Size,
    health: Health,
    speed: Speed,
    position: Transform,
    commands: &mut Commands,
    ) -> Entity {

    commands
        .spawn(
            ( Actor,
            Game,
            collider,
            size,
            health,
            speed,
            position,
            KinematicCharacterController::default(),
            RigidBody::KinematicPositionBased)
        ).id()
}
