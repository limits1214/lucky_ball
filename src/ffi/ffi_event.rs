use std::sync::OnceLock;

use bevy::prelude::*;
use bevy_crossbeam_event::CrossbeamEventSender;

#[derive(Event, Clone, Debug)]
pub enum FfiEvents {
    Ad(AdFfi),
}

#[derive(Clone, Debug)]
pub enum AdFfi {
    AdmobInterstitial(InterstitailAdEvents),
}

#[derive(Clone, Debug)]
pub enum InterstitailAdEvents {
    LoadSuccess,
    LoadFail(String),
    Showed,
    ShowFail(String),
    Dismissed,
}

pub(super) static SENDER: OnceLock<CrossbeamEventSender<FfiEvents>> = OnceLock::new();
pub fn set_sender(sender: CrossbeamEventSender<FfiEvents>) {
    let _ = SENDER.set(sender);
}
