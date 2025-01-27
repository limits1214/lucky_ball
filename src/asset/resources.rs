use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
// use bevy_kira_audio::AudioSource;

#[derive(AssetCollection, Resource)]
pub struct MyAsstes {
    #[asset(path = "models/luckyball.glb")]
    pub luckyball: Handle<Gltf>,
    #[asset(path = "fonts/NanumGothicBold.ttf")]
    pub ttf_nanum_gothic_bold: Handle<Font>,
    #[asset(path = "images/back.png")]
    pub png_back: Handle<Image>,
    #[asset(path = "images/trash.png")]
    pub png_trash: Handle<Image>,
    #[asset(path = "images/shuffle.png")]
    pub png_shuffle: Handle<Image>,
    #[asset(path = "images/play.png")]
    pub png_play: Handle<Image>,
    #[asset(path = "images/save.png")]
    pub png_save: Handle<Image>,
    #[asset(path = "images/customize.png")]
    pub png_customize: Handle<Image>,
    // #[asset(path = "sounds/ball_sound.mp3")]
    // pub mp3_ballsound: Handle<AudioSource>,
}
