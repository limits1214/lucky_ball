use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Event)]
pub struct ButtonClickEvent(pub Entity, pub f32);

impl From<ListenerInput<Pointer<Click>>> for ButtonClickEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        Self(event.target, event.hit.depth)
    }
}
#[derive(Event, Debug)]
pub struct GameButtonClickEvent(pub Entity, pub f32);

impl From<ListenerInput<Pointer<Click>>> for GameButtonClickEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        Self(event.target, event.hit.depth)
    }
}

#[derive(Event, Debug)]
pub struct GameBackButtonClickEvent(pub Entity, pub f32);

impl From<ListenerInput<Pointer<Click>>> for GameBackButtonClickEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        Self(event.target, event.hit.depth)
    }
}

#[derive(Event, Debug)]
pub struct Load70Fire5BtnClick(pub Entity, pub f32);

impl From<ListenerInput<Pointer<Click>>> for Load70Fire5BtnClick {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        Self(event.target, event.hit.depth)
    }
}

#[derive(Event, Debug)]
pub struct RuleSelectBackBtnClick(pub Entity, pub f32);

impl From<ListenerInput<Pointer<Click>>> for RuleSelectBackBtnClick {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        Self(event.target, event.hit.depth)
    }
}
