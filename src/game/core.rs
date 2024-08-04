use avian2d::prelude::*;
use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_trauma_shake::Shakes;
use tiny_bail::prelude::*;

use crate::{
    ext::{EventReaderExt, QuatExt},
    screen::{in_game_state, NextTransitionedState, Screen},
    ui::palette::COL_GEARS_DISABLED,
};

use super::{
    gun::ProjectileDespawn,
    movement::MovementPaused,
    spawn::{
        enemy::{DespawnEnemy, Enemy},
        level::{AmmoFill, Core, Health, RotateWithPaddle, AMMO_FILL_RADIUS},
        paddle::{PaddleAmmo, PaddleRotation, PADDLE_RADIUS},
        projectile::{Projectile, ProjectileTarget},
    },
    tween::{get_relative_scale_anim, get_relative_sprite_color_anim},
};

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.add_event::<TakeDamage>().add_systems(
        Update,
        (
            handle_collisions,
            rotate_gears,
            take_damage,
            update_ammo_fill,
            clear_paddle_radius_on_dmg,
        )
            .run_if(in_game_state),
    );
}

#[derive(Event, Default)]
pub struct TakeDamage;

fn handle_collisions(
    core_q: Query<&CollidingEntities, With<Core>>,
    enemy_q: Query<(&Enemy, &GlobalTransform)>,
    mut taken_dmg_w: EventWriter<TakeDamage>,
    mut despawn_enemy_w: EventWriter<DespawnEnemy>,
) {
    for coll in &core_q {
        for coll_e in coll.iter().filter(|e| enemy_q.contains(**e)) {
            taken_dmg_w.send_default();
            despawn_enemy_w.send(DespawnEnemy(*coll_e));
        }
    }
}

fn rotate_gears(
    paddle_rot_q: Query<&Transform, With<PaddleRotation>>,
    mut gear_q: Query<
        (&mut Transform, &RotateWithPaddle),
        (Without<PaddleRotation>, Without<MovementPaused>),
    >,
) {
    if let Some(paddle_t) = paddle_rot_q.iter().next() {
        for (mut gear_t, gear) in &mut gear_q {
            gear_t.rotation = Quat::from_rotation_z(
                (gear.offset.as_radians() + paddle_t.rotation.z_angle_rad())
                    * (if gear.invert { 1. } else { -1. })
                    * gear.multiplier,
            );
        }
    }
}

fn update_ammo_fill(
    ammo_q: Query<&PaddleAmmo, Changed<PaddleAmmo>>,
    ammo_fill_q: Query<Entity, With<AmmoFill>>,
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if let Some(ammo) = ammo_q.iter().next() {
        for e in &ammo_fill_q {
            cmd.entity(e)
                .try_insert(Mesh2dHandle(meshes.add(CircularSegment::from_turns(
                    AMMO_FILL_RADIUS,
                    // not sure why, but the segments fills at 95% already
                    ammo.factor() * 0.95,
                ))));
        }
    }
}

fn take_damage(
    mut ev_r: EventReader<TakeDamage>,
    mut core_q: Query<(&mut Core, &mut Health)>,
    mut cmd: Commands,
    mut next: ResMut<NextTransitionedState>,
    mut shake: Shakes,
) {
    let (mut core, mut hp) = or_return_quiet!(core_q.get_single_mut());
    if !ev_r.is_empty() {
        ev_r.clear();
        shake.add_trauma(0.7);

        let (e, active) = or_return!(core.gear_entity_ids.iter_mut().find(|(_, active)| *active));
        *active = false;
        cmd.entity(*e).try_insert((
            get_relative_scale_anim(
                Vec2::splat(0.7).extend(1.),
                350,
                Some(bevy_tweening::EaseFunction::BackIn),
            ),
            get_relative_sprite_color_anim(COL_GEARS_DISABLED, 350, None),
            MovementPaused,
        ));

        // todo: spawn take dmg particles
        // and destroy all enemies and projectiles inside the core

        hp.0 -= 1;
        if hp.0 == 0 {
            next.set(Screen::GameOver);
        }
    }
}

fn clear_paddle_radius_on_dmg(
    mut ev_r: EventReader<TakeDamage>,
    projectile_q: Query<(Entity, &Projectile, &GlobalTransform)>,
    enemy_q: Query<(Entity, &GlobalTransform), With<Enemy>>,
    mut projectile_despawn_w: EventWriter<ProjectileDespawn>,
    mut despawn_enemy_w: EventWriter<DespawnEnemy>,
) {
    if ev_r.clear_any() {
        // todo: spawn take dmg particles
        // and destroy all enemies and projectiles inside the core

        for (projectile_e, ..) in projectile_q.iter().filter(|(_, p, t, ..)| {
            p.target == ProjectileTarget::Core && t.translation().length() < PADDLE_RADIUS
        }) {
            projectile_despawn_w.send(ProjectileDespawn(projectile_e));
        }

        for (enemy_e, ..) in enemy_q
            .iter()
            .filter(|(_, t, ..)| t.translation().length() < PADDLE_RADIUS)
        {
            despawn_enemy_w.send(DespawnEnemy(enemy_e));
        }
    }
}
