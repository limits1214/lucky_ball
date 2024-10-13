use bevy::prelude::*;
use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};

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

pub fn time_formatting(timestamp: i64, time_offset: i32) -> String {
    let datetime = DateTime::from_timestamp(timestamp, 0).unwrap_or(Utc::now());

    let offset = if time_offset > 0 {
        FixedOffset::east_opt(time_offset).unwrap()
    } else {
        FixedOffset::east_opt(time_offset).unwrap()
        // FixedOffset::west_opt(-time_offset).unwrap()
    };
    let datetime = datetime.with_timezone(&offset);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
