use bevy::prelude::*;

#[derive(Event)]
pub struct ButtonClickEvent(pub Entity, pub f32);

// impl From<ListenerInput<Pointer<Click>>> for ButtonClickEvent {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }
#[derive(Event, Debug)]
pub struct GameRuleSelectButtonClickEvent(pub Entity, pub f32);

// impl From<ListenerInput<Pointer<Click>>> for GameRuleSelectButtonClickEvent {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }

#[derive(Event, Debug)]
pub struct BackToMainMenuBtnClickEvent(pub Entity, pub f32);

// impl From<ListenerInput<Pointer<Click>>> for BackToMainMenuBtnClickEvent {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }

#[derive(Event, Debug)]
pub struct Load69Fire5BtnClick(pub Entity, pub f32);

// impl From<ListenerInput<Pointer<Click>>> for Load69Fire5BtnClick {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }

#[derive(Event, Debug)]
pub struct Load26Fire1BtnClick(pub Entity, pub f32);

// impl From<ListenerInput<Pointer<Click>>> for Load26Fire1BtnClick {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }

#[derive(Event, Debug)]
pub struct Load45Fire6BtnClick(pub Entity, pub f32);

// impl From<ListenerInput<Pointer<Click>>> for Load45Fire6BtnClick {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }

#[derive(Event, Debug)]
pub struct CustomGameRuleBtnClick(pub Entity, pub f32);

// impl From<ListenerInput<Pointer<Click>>> for CustomGameRuleBtnClick {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }

#[derive(Event, Debug)]
pub struct BackToGameRuleSelectBtnClick(pub Entity, pub f32);

// impl From<ListenerInput<Pointer<Click>>> for BackToGameRuleSelectBtnClick {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }

#[derive(Event, Debug)]
pub struct GameRunBtnClick(pub Entity, pub f32);

// impl From<ListenerInput<Pointer<Click>>> for GameRunBtnClick {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }

#[derive(Event, Debug)]
pub struct CustomRuleBallClick(pub Entity, pub f32);

// impl From<ListenerInput<Pointer<Down>>> for CustomRuleBallClick {
//     fn from(event: ListenerInput<Pointer<Down>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }

#[derive(Event, Debug)]
pub struct CustomRuleFireCntDownClick(pub Entity, pub f32);

// impl From<ListenerInput<Pointer<Click>>> for CustomRuleFireCntDownClick {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }
#[derive(Event, Debug)]
pub struct CustomRuleFireCntUpClick(pub Entity, pub f32);

// impl From<ListenerInput<Pointer<Click>>> for CustomRuleFireCntUpClick {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }

#[derive(Event, Debug)]
pub struct CustomRuleRunBtnClick(pub Entity, pub f32);

// impl From<ListenerInput<Pointer<Click>>> for CustomRuleRunBtnClick {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }

#[derive(Event, Debug)]
pub struct NumbersBtnClick(pub Entity, pub f32);

// impl From<ListenerInput<Pointer<Click>>> for NumbersBtnClick {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }

#[derive(Event, Debug)]
pub struct QuitBtnClick(pub Entity, pub f32);

// impl From<ListenerInput<Pointer<Click>>> for QuitBtnClick {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }

#[derive(Event, Debug)]
pub struct NumbersPagingPrevBtnClick(pub Entity, pub f32);

// impl From<ListenerInput<Pointer<Click>>> for NumbersPagingPrevBtnClick {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }

#[derive(Event, Debug)]
pub struct NumbersPagingNextBtnClick(pub Entity, pub f32);

// impl From<ListenerInput<Pointer<Click>>> for NumbersPagingNextBtnClick {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }

#[derive(Event, Debug)]
pub struct NumbersItemDeleteBtnClick(pub Entity, pub f32);

// impl From<ListenerInput<Pointer<Click>>> for NumbersItemDeleteBtnClick {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }

#[derive(Event, Debug)]
pub struct GameMenuShuffleBtnClick(pub Entity, pub f32);

// impl From<ListenerInput<Pointer<Click>>> for GameMenuShuffleBtnClick {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }

#[derive(Event, Debug)]
pub struct GameResultRetryBtnClick(pub Entity, pub f32);
// impl From<ListenerInput<Pointer<Click>>> for GameResultRetryBtnClick {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }

#[derive(Event, Debug)]
pub struct GameResultSaveBtnClick(pub Entity, pub f32);
// impl From<ListenerInput<Pointer<Click>>> for GameResultSaveBtnClick {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         Self(event.target, event.hit.depth)
//     }
// }
