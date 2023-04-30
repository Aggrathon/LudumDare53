use bevy::prelude::*;
use fastrand::Rng;

use crate::deck::TopTileRotated;
use crate::objective::Victory;
use crate::state::GameState;
use crate::world::PlaceTile;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioContainer>()
            .add_startup_system(load_audio)
            .add_system(on_rotate)
            .add_system(on_victory)
            .add_system(on_place)
            .add_system(on_menu.in_schedule(OnEnter(GameState::MainMenu)));
    }
}

#[derive(Default, Resource)]
struct AudioContainer {
    pub music: Vec<Handle<AudioSource>>,
    pub fanfare: Vec<Handle<AudioSource>>,
    pub rotate: Vec<Handle<AudioSource>>,
    pub place: Vec<Handle<AudioSource>>,
}

fn load_audio(mut ac: ResMut<AudioContainer>, asset_server: Res<AssetServer>) {
    ac.music.push(asset_server.load("audio/music.ogg"));
    ac.fanfare.push(asset_server.load("audio/victory.ogg"));
    ac.rotate.push(asset_server.load("audio/shuffle1.ogg"));
    ac.rotate.push(asset_server.load("audio/shuffle2.ogg"));
    ac.rotate.push(asset_server.load("audio/shuffle3.ogg"));
    ac.rotate.push(asset_server.load("audio/shuffle4.ogg"));
    ac.place.push(asset_server.load("audio/thud1.ogg"));
    ac.place.push(asset_server.load("audio/thud2.ogg"));
    ac.place.push(asset_server.load("audio/thud3.ogg"));
}

fn on_rotate(event: EventReader<TopTileRotated>, ac: Res<AudioContainer>, audio: Res<Audio>) {
    if !event.is_empty() {
        let rnd = Rng::new();
        audio.play_with_settings(
            ac.rotate[rnd.usize(0..ac.rotate.len())].clone(),
            PlaybackSettings::default().with_speed(0.9 + rnd.f32() * 0.2),
        );
    }
}

fn on_place(event: EventReader<PlaceTile>, ac: Res<AudioContainer>, audio: Res<Audio>) {
    if !event.is_empty() {
        let rnd = Rng::new();
        audio.play_with_settings(
            ac.place[rnd.usize(0..ac.place.len())].clone(),
            PlaybackSettings::default().with_speed(0.9 + rnd.f32() * 0.2),
        );
    }
}

fn on_victory(event: EventReader<Victory>, ac: Res<AudioContainer>, audio: Res<Audio>) {
    if !event.is_empty() {
        let rnd = Rng::new();
        audio.play(ac.fanfare[rnd.usize(0..ac.fanfare.len())].clone());
    }
}

fn on_menu(ac: Res<AudioContainer>, audio: Res<Audio>) {
    let rnd = Rng::new();
    audio.play(ac.music[rnd.usize(0..ac.music.len())].clone());
}
