use std::sync::OnceLock;

use bevy::prelude::*;
use bevy_crossbeam_event::CrossbeamEventSender;

#[derive(Event, Clone, Debug, PartialEq, Eq)]
pub enum FfiEvents {
    Ad(AdFfi),
    App(AppFfi),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdFfi {
    AdmobInterstitial(InterstitailAdEvents),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InterstitailAdEvents {
    LoadSuccess,
    LoadFail(String),
    Showed,
    ShowFail(String),
    Dismissed,
    IsReady(bool),
}

pub(super) static SENDER: OnceLock<CrossbeamEventSender<FfiEvents>> = OnceLock::new();
pub fn set_sender(sender: CrossbeamEventSender<FfiEvents>) {
    let _ = SENDER.set(sender);
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AppFfi {
    InitEnd,
}
