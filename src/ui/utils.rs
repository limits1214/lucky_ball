use bevy::prelude::*;

use super::component::TextResize;

pub fn make_text(txt: &str) -> (TextBundle, TextResize) {
    (
        TextBundle::from_section(txt, TextStyle { ..default() }),
        TextResize,
    )
}

pub fn paginate_with_total<T>(items: &[T], page: usize, page_size: usize) -> (&[T], usize) {
    let total_pages = (items.len() + page_size - 1) / page_size; // 총 페이지 수 계산
    let start = page * page_size;
    let end = start + page_size;

    if start >= items.len() {
        (&[], total_pages)
    } else {
        (&items[start..std::cmp::min(end, items.len())], total_pages)
    }
}
