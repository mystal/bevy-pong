use bevy::prelude::*;
use bevy_kira_audio::{AudioPlugin, AudioSource};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(AudioPlugin)
            .init_resource::<Assets>()
            .add_startup_system(load_assets);
    }
}

#[derive(Default)]
pub struct Assets {
    pub font: Handle<Font>,
    pub bounce: Handle<AudioSource>,
}

pub fn load_assets(
    server: Res<AssetServer>,
    mut assets: ResMut<Assets>,
) {
    assets.font = server.load("fonts/SourceSansPro-Regular.ttf");
    assets.bounce = server.load("sounds/bounce.wav");
}
