use bevy::prelude::*;
use bevy_tweening::{Animator, EaseFunction};

use crate::{
    game::{
        assets::{ParticleAssets, SpriteAssets},
        ball::{BallSpeed, BALL_BASE_SPEED},
        movement::{MovementBundle, MovementPaused},
        tween::{delay_tween, get_relative_scale_tween},
    },
    screen::Screen,
    ui::palette::COL_BALL,
};

use super::paddle::PaddleMode;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_ball);
}

pub const BALL_BASE_RADIUS: f32 = 40.;

#[derive(Event, Debug)]
pub struct SpawnBall {
    pub paddle_e: Entity,
    pub tween_delay_ms: u64,
}

#[derive(Component, Debug)]
pub struct Ball {
    pub radius: f32,
    pub last_reflection_time: f32,
    pub sprite_e: Entity,
    pub particles_e: Entity,
}

#[derive(Component, Debug)]
#[component(storage = "SparseSet")]
pub struct InsidePaddleRadius;

impl Ball {
    fn new(sprite_e: Entity, particles_e: Entity) -> Self {
        Self {
            radius: BALL_BASE_RADIUS,
            last_reflection_time: 0.,
            sprite_e,
            particles_e,
        }
    }
}

fn spawn_ball(
    trigger: Trigger<SpawnBall>,
    mut cmd: Commands,
    ball_q: Query<Entity, With<Ball>>,
    mut paddle_q: Query<&mut PaddleMode>,
    sprites: Res<SpriteAssets>,
    particles: Res<ParticleAssets>,
) {
    for e in &ball_q {
        cmd.entity(e).despawn_recursive();
    }

    let ev = trigger.event();
    if let Ok(mut paddle_mode) = paddle_q.get_mut(ev.paddle_e) {
        let sprite_e = cmd
            .spawn((
                Name::new("sprite"),
                SpriteBundle {
                    texture: sprites.ball.clone(),
                    sprite: Sprite {
                        color: COL_BALL,
                        ..default()
                    },
                    transform: Transform::from_scale(Vec3::Z),
                    ..default()
                },
                Animator::new(delay_tween(
                    get_relative_scale_tween(Vec3::ONE, 500, Some(EaseFunction::BackOut)),
                    ev.tween_delay_ms,
                )),
            ))
            .id();

        //particles

        let particles_e = cmd
            .spawn((
                particles.square_particle_spawner(particles.ball.clone(), Transform::default()),
            ))
            .id();

        let ball_e = cmd
            .spawn((
                Name::new("Ball"),
                SpatialBundle::from_transform(Transform::from_xyz(
                    BALL_BASE_RADIUS * -1.1,
                    0.,
                    0.9,
                )),
                BallSpeed::default(),
                MovementBundle::new(Vec2::X, BALL_BASE_SPEED),
                MovementPaused,
                Ball::new(sprite_e, particles_e),
                InsidePaddleRadius,
                StateScoped(Screen::Game),
            ))
            .add_child(sprite_e)
            .add_child(particles_e)
            .set_parent(ev.paddle_e)
            .id();
        *paddle_mode = PaddleMode::Captured {
            ball_e,
            shoot_rotation: Rot2::degrees(0.),
        };
    }
}
