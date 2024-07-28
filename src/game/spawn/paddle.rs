use std::time::Duration;

use avian2d::prelude::*;
use bevy::{
    color::palettes::tailwind,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    game::{assets::SpriteAssets, movement::AccumulatedRotation},
    screen::Screen,
};

use super::level::RotateWithPaddle;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_paddle);
}

pub const PADDLE_RADIUS: f32 = 260.0;
pub const PADDLE_HEIGHT: f32 = 120.0;
pub const PADDLE_COLL_HEIGHT: f32 = PADDLE_HEIGHT + 10.;

#[derive(Event, Debug)]
pub struct SpawnPaddle;

#[derive(Component, Debug)]
pub struct Paddle {
    pub sprite_e: Entity,
    pub barrel_e: Entity,
}

#[derive(Component, Debug)]
pub enum PaddleMode {
    Reflect,
    Capture,
    Captured {
        shoot_rotation: Rot2,
        ball_e: Entity,
    },
}

#[derive(Component, Debug)]
pub struct PaddleRotation {
    pub cw_start: f32,
    pub ccw_start: f32,
    pub timer: Timer,
    pub prev_rot: f32,
    pub paddle_e: Entity,
}

impl PaddleRotation {
    fn new(paddle_e: Entity) -> Self {
        Self {
            cw_start: 0.,
            ccw_start: 0.,
            timer: Timer::new(Duration::from_millis(50), TimerMode::Once),
            prev_rot: 0.,
            paddle_e,
        }
    }
}

impl PaddleRotation {
    pub fn reset(&mut self, rotation: f32) {
        self.cw_start = rotation;
        self.ccw_start = rotation;
        self.prev_rot = rotation;
        self.timer.reset();
        self.timer.unpause();
    }
}

#[derive(Component, Debug)]
pub struct PaddleAmmo {
    ammo: usize,
    capacity: usize,
}

impl PaddleAmmo {
    pub fn ammo(&self) -> usize {
        self.ammo
    }

    pub fn offset(&mut self, delta: isize) {
        self.ammo = ((self.ammo as isize + delta) as usize).clamp(0, self.capacity);
    }

    pub fn factor(&self) -> f32 {
        self.ammo as f32 / self.capacity as f32
    }
}

fn spawn_paddle(
    _trigger: Trigger<SpawnPaddle>,
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    sprites: Res<SpriteAssets>,
) {
    let color: Color = tailwind::SKY_400.into();

    // rails/paddle radius
    for offset in [-10., 15.] {
        cmd.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Annulus::new(
                    PADDLE_RADIUS + offset,
                    PADDLE_RADIUS + offset + 10.,
                ))),
                material: materials.add(ColorMaterial::from_color(
                    bevy::color::palettes::tailwind::SKY_200,
                )),
                ..default()
            },
            StateScoped(Screen::Game),
        ));
    }

    let barrel_e = cmd
        .spawn(SpatialBundle::default())
        .with_children(|b| {
            b.spawn((
                Name::new("barrel"),
                SpriteBundle {
                    texture: sprites.paddle_barrel.clone(),
                    sprite: Sprite { color, ..default() },
                    transform: Transform::from_xyz(0., 55., 0.),
                    ..default()
                },
            ));
        })
        .id();

    let sprite_e = cmd
        .spawn((
            Name::new("base_sprite"),
            SpriteBundle {
                texture: sprites.paddle_base.clone(),
                sprite: Sprite { color, ..default() },
                transform: Transform::from_xyz(4.5, 0., 0.)
                    .with_rotation(Quat::from_rotation_z(-90f32.to_radians())),
                ..default()
            },
        ))
        .add_child(barrel_e)
        .with_children(|b| {
            for sign in [1., -1.] {
                b.spawn((
                    Name::new("wheel"),
                    SpriteBundle {
                        texture: sprites.paddle_wheel.clone(),
                        sprite: Sprite { color, ..default() },
                        transform: Transform::from_xyz(98. * sign, -16., 0.),
                        ..default()
                    },
                    RotateWithPaddle {
                        invert: true,
                        offset: Rot2::default(),
                        multiplier: 10.,
                    },
                ));
            }
        })
        .id();

    let paddle_e = cmd
        .spawn((
            Name::new("paddle"),
            SpatialBundle::from_transform(Transform::from_xyz(PADDLE_RADIUS, 0.0, 1.0)),
            Collider::capsule(23.0, PADDLE_COLL_HEIGHT),
            Paddle { sprite_e, barrel_e },
            PaddleMode::Reflect,
            PaddleAmmo {
                capacity: 50,
                ammo: 0,
            },
        ))
        .add_child(sprite_e)
        .id();

    cmd.spawn((
        Name::new("paddle_rotation"),
        SpatialBundle::default(),
        PaddleRotation::new(paddle_e),
        AccumulatedRotation::default(),
        StateScoped(Screen::Game),
    ))
    .add_child(paddle_e);
}
