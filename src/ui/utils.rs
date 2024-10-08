use bevy::prelude::*;

use super::component::TextResize;

pub fn make_text(txt: &str) -> (TextBundle, TextResize) {
    (
        TextBundle::from_section(txt, TextStyle { ..default() }),
        TextResize,
    )
}
