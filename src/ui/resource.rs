use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

use crate::ffi::ffi_fn::{kv_get, kv_set};

use super::component::CustomRuleBall;

// custom rule setting save
// numbers save
#[derive(Resource, Debug)]
pub struct UiConfig {
    pub saved_custom_rule: SavedCustomRule,
    pub saved_ball_numbers: Vec<BallNumber>,
}

impl UiConfig {
    pub fn new() -> Self {
        Self {
            saved_ball_numbers: Vec::<BallNumber>::init_items(),
            saved_custom_rule: SavedCustomRule::init(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedCustomRule {
    pub load: Vec<CustomRuleBall>,
    pub fire: u8,
}

impl SavedCustomRule {
    pub fn init() -> Self {
        let get = kv_get("saved_custom_rule");
        match serde_json::from_str::<SavedCustomRule>(&get) {
            Ok(a) => a,
            Err(err) => {
                error!("SavedCustomRule init err: {err:?}");
                let mut v: Vec<CustomRuleBall> = vec![];
                for i in 1..=70 {
                    v.push(CustomRuleBall(i, true));
                }
                let a = SavedCustomRule { load: v, fire: 5 };
                let json_str = match serde_json::to_string(&a) {
                    Ok(s) => s,
                    Err(err) => {
                        info!("saved custom rull init err: {err:?}");
                        String::new()
                    }
                };
                kv_set("saved_custom_rule", &json_str);
                a
            }
        }
    }

    pub fn save_custom_rule(&self) {
        let json_str = match serde_json::to_string(self) {
            Ok(s) => s,
            Err(err) => {
                info!("saved custom rull init err: {err:?}");
                String::new()
            }
        };
        kv_set("saved_custom_rule", &json_str);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BallNumber {
    pub id: String,
    pub numbers: Vec<u8>,
    pub game_type: String,
    pub time: u64,
}

// impl BallNumber {
//     pub fn init_vec() -> Vec<Self> {
//         let mut v: Vec<Self> = vec![];

//         for i in 1..100 {
//             v.push(Self {
//                 game_type: "".to_string(),
//                 id: Uuid::new_v4().to_string(),
//                 numbers: vec![1, 2, 3, 4],
//                 time: i,
//             });
//         }
//         let get = kv_get("saved_ball_numbers");
//         let vec_ballnum = match serde_json::from_str::<Vec<BallNumber>>(&get) {
//             Ok(a) => a,
//             Err(err) => {
//                 error!("BallNumber vec init err: {err:?}");
//                 let v: Vec<BallNumber> = vec![];
//                 let json_str = match serde_json::to_string(&v) {
//                     Ok(s) => s,
//                     Err(err) => {
//                         info!("saved custom rull init err: {err:?}");
//                         String::new()
//                     }
//                 };
//                 kv_set("saved_ball_numbers", &json_str);
//                 v
//             }
//         };
//         // vec_ballnum
//         v
//     }
// }

pub trait VecBallNumberExt {
    fn init_items() -> Vec<BallNumber>;
    fn save_item(&mut self, ball_number: BallNumber);
    fn delete_item(&mut self, id: String);
}

impl VecBallNumberExt for Vec<BallNumber> {
    fn init_items() -> Vec<BallNumber> {
        // let mut v: Vec<BallNumber> = vec![];

        // for i in 1..10 {
        //     v.push(BallNumber {
        //         game_type: "".to_string(),
        //         id: Uuid::new_v4().to_string(),
        //         numbers: vec![1, 2, 3, 4],
        //         time: i,
        //     });
        // }
        let get = kv_get("saved_ball_numbers");
        let vec_ballnum = match serde_json::from_str::<Vec<BallNumber>>(&get) {
            Ok(a) => a,
            Err(err) => {
                error!("BallNumber vec init err: {err:?}");
                let v: Vec<BallNumber> = vec![];
                let json_str = match serde_json::to_string(&v) {
                    Ok(s) => s,
                    Err(err) => {
                        info!("saved custom rull init err: {err:?}");
                        String::new()
                    }
                };
                kv_set("saved_ball_numbers", &json_str);
                v
            }
        };
        vec_ballnum
        // v
    }

    fn save_item(&mut self, ball_number: BallNumber) {
        self.push(ball_number);
        let json_str = match serde_json::to_string(&self) {
            Ok(s) => s,
            Err(err) => {
                info!("saved custom rull init err: {err:?}");
                String::new()
            }
        };
        kv_set("saved_ball_numbers", &json_str);
    }

    fn delete_item(&mut self, id: String) {
        self.retain(|bn| bn.id != id);
        let json_str = match serde_json::to_string(&self) {
            Ok(s) => s,
            Err(err) => {
                info!("saved custom rull init err: {err:?}");
                String::new()
            }
        };
        kv_set("saved_ball_numbers", &json_str);
    }
}
