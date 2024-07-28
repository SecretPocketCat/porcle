use bevy::{audio::PlaybackMode, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Music>();
    app.observe(play_soundtrack);
}

fn play_soundtrack(
    trigger: Trigger<PlayMusic>,
    mut commands: Commands,
    soundtrack_query: Query<Entity, With<Music>>,
) {
    for entity in &soundtrack_query {
        commands.entity(entity).despawn_recursive();
    }

    let handle = match trigger.event() {
        PlayMusic::Track(key) => key.clone(),
        PlayMusic::Disable => return,
    };
    commands.spawn((
        AudioSourceBundle {
            source: handle,
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
        Music,
    ));
}

#[derive(Event)]
pub enum PlayMusic {
    Track(Handle<AudioSource>),
    Disable,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Music;
