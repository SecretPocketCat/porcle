use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_trauma_shake::Shakes;

use crate::{ext::Vec2Ext, game::spawn::projectile::SpawnProjectile};

use super::{
    movement::{BaseSpeed, Velocity},
    spawn::{enemy::Enemy, paddle::PaddleAmmo, projectile::Projectile},
    time::{process_cooldown, Cooldown},
};

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.add_systems(
        Update,
        (
            fire_gun,
            handle_collisions,
            process_cooldown::<NoAmmoShake>,
            process_cooldown::<PaddleAmmo>,
        ),
    );
}

struct NoAmmoShake;

fn fire_gun(
    mut ammo_q: Query<
        (
            Entity,
            &mut PaddleAmmo,
            &GlobalTransform,
            Option<&Cooldown<NoAmmoShake>>,
        ),
        Without<Cooldown<PaddleAmmo>>,
    >,
    input: Res<ButtonInput<MouseButton>>,
    mut cmd: Commands,
    mut shake: Shakes,
) {
    if input.just_pressed(MouseButton::Left) {
        for (e, mut ammo, t, cooldown) in &mut ammo_q {
            // todo: cooldown
            if ammo.0 > 0 {
                let dir = Dir2::new(t.right().truncate()).unwrap();
                let rot = t.up().truncate().to_quat();
                cmd.trigger(SpawnProjectile {
                    dir,
                    transform: Transform::from_translation(
                        t.translation() + (rot * (-Vec3::Y * 70.0)),
                    )
                    .with_rotation(rot),
                });
                ammo.0 -= 1;
                shake.add_trauma(0.125);
                // todo: UI for shoot delay
                cmd.entity(e).insert(Cooldown::<PaddleAmmo>::new(0.14));
            } else if cooldown.is_none() {
                shake.add_trauma(0.4);
                cmd.entity(e).insert(Cooldown::<NoAmmoShake>::new(1.));
                // todo: add delay to avoid more shake

                // todo: some blinking UI or smt. to show there's no ammo
            }
        }
    }
}

fn handle_collisions(
    phys_spatial: SpatialQuery,
    ball_q: Query<(Entity, &GlobalTransform, &Projectile, &Velocity, &BaseSpeed)>,
    enemy_q: Query<(), With<Enemy>>,
    mut cmd: Commands,
    time: Res<Time>,
) {
    for (e, t, projectile, vel, speed) in &ball_q {
        if (vel.0 - Vec2::ZERO).length() < f32::EPSILON {
            // stationary
            continue;
        }

        for hit in phys_spatial.shape_hits(
            &Collider::rectangle(projectile.size.x, projectile.size.y),
            t.translation().truncate(),
            0.,
            Dir2::new(vel.0).expect("Non zero velocity"),
            (speed.0 * 1.05) * time.delta_seconds(),
            100,
            false,
            SpatialQueryFilter::default(),
        ) {
            let hit_e = hit.entity;
            if enemy_q.contains(hit.entity) {
                cmd.entity(hit_e).despawn_recursive();
                cmd.entity(e).despawn_recursive();
            }
        }
    }
}
