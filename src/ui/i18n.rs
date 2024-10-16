use std::sync::OnceLock;

const KO_KR: &str = "ko_KR";
const EN_KR: &str = "en_KR";
static LOCALE: OnceLock<String> = OnceLock::new();
pub fn set_locale(locale: String) {
    LOCALE.get_or_init(|| locale);
}

pub fn txt_start() -> String {
    match LOCALE.get().unwrap().as_str() {
        KO_KR | EN_KR => "시작",
        _ => "start",
    }
    .to_string()
}

pub fn txt_saved_numbers() -> String {
    match LOCALE.get().unwrap().as_str() {
        KO_KR | EN_KR => "저장된 번호",
        _ => "saved numbers",
    }
    .to_string()
}

pub fn txt_quit() -> String {
    match LOCALE.get().unwrap().as_str() {
        KO_KR | EN_KR => "종료",
        _ => "quit",
    }
    .to_string()
}

pub fn txt_ok() -> String {
    match LOCALE.get().unwrap().as_str() {
        KO_KR | EN_KR => "확인",
        _ => "ok",
    }
    .to_string()
}

pub fn txt_insert_balls() -> String {
    match LOCALE.get().unwrap().as_str() {
        KO_KR | EN_KR => "공 넣기",
        _ => "insert balls",
    }
    .to_string()
}

pub fn txt_draw_balls_count() -> String {
    match LOCALE.get().unwrap().as_str() {
        KO_KR | EN_KR => "뽑을 공 갯수",
        _ => "draw balls count",
    }
    .to_string()
}
