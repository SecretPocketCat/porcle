use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    ext::{QuatExt, Vec2Ext},
    AppSet,
};

use super::{
    input::CursorCoords,
    spawn::{
        ball::Ball,
        enemy::Enemy,
        paddle::{Paddle, PaddleRotation},
    },
};

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.add_systems(
        Update,
        (
            process_input.in_set(AppSet::ProcessInput),
            rotate_paddle,
            reflect_ball,
            accumulate_angle,
        ),
    );
}

pub const BALL_BASE_SPEED: f32 = 250.;

#[derive(Component, Debug)]
pub struct BallSpeed(f32);

impl Default for BallSpeed {
    fn default() -> Self {
        Self(BALL_BASE_SPEED)
    }
}

fn process_input(_input: Res<ButtonInput<KeyCode>>, mut _cmd: Commands) {}

fn rotate_paddle(
    mut rot_q: Query<(&mut Transform, &AccumulatedRotation), With<PaddleRotation>>,
    cursor: Res<CursorCoords>,
) {
    // todo: limit speed
    for (mut t, angle) in rot_q.iter_mut() {
        t.rotation = cursor.0.to_quat();

        info!(angle = angle.rotation.to_degrees());
    }
}

#[derive(Component, Debug, Default)]
pub struct AccumulatedRotation {
    prev: Option<Rot2>,
    rotation: f32,
}

fn accumulate_angle(mut acc_q: Query<(&mut AccumulatedRotation, &Transform), Changed<Transform>>) {
    for (mut acc, t) in &mut acc_q {
        let rot = t.rotation.to_rot2();
        if let Some(prev) = acc.prev {
            acc.rotation += prev.angle_between(rot);
        }
        acc.prev = Some(rot);
    }
}

fn reflect_ball(
    mut coll_q: Query<
        (
            Entity,
            &CollidingEntities,
            &mut LinearVelocity,
            &mut BallSpeed,
        ),
        With<Ball>,
    >,
    paddle_q: Query<(), With<Paddle>>,
    enemy_q: Query<(), With<Enemy>>,
    collisions: Res<Collisions>,
    mut cmd: Commands,
) {
    for (e, colliding, mut vel, mut speed) in &mut coll_q {
        if !colliding.is_empty() {
            let colliding_e = *colliding.0.iter().next().unwrap();
            if paddle_q.contains(colliding_e) {
                if let Some(coll) = collisions.get(e, colliding_e) {
                    if let Some(contact) = coll.manifolds.first() {
                        speed.0 += 5.;
                        vel.0 = contact.normal1 * -speed.0;
                    }
                }
            } else if enemy_q.contains(colliding_e) {
                cmd.entity(colliding_e).despawn_recursive();
            }
        }
    }
}
