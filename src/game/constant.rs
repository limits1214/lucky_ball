//
pub const TWEEN_POOL_OUTLET_OPEN_END: u64 = 1;
pub const TWEEN_POOL_OUTLET_CLOSE_END: u64 = 2;
pub const TWEEN_BALL_MIXER_ROTATE_END: u64 = 3;
pub const TWEEN_DRAW_STICK_DOWN_END: u64 = 4;
pub const TWEEN_DRAW_STICK_UP_END: u64 = 5;
pub const TWEEN_INNER_DRAW_STICK_DOWN_END: u64 = 6;
pub const TWEEN_INNER_DRAW_STICK_UP_END: u64 = 7;
pub const TWEEN_BALL_CATCH_END: u64 = 8;

//
pub const STEP_POOL_BALL_ZERO: u64 = 2;
pub const STEP_POOL_OUTLET_OPEN_END: u64 = 3;
pub const STEP_POOL_OUTLET_CLOSE_END: u64 = 4;
pub const STEP_BALL_MIXER_ROTATE_END: u64 = 5;
pub const STEP_DRAW_STICK_DOWN_END: u64 = 6;
pub const STEP_BALL_CATCH_DONE: u64 = 7;
pub const STEP_DRAW_STICK_UP_END: u64 = 8;
pub const STEP_INNER_DRAW_STICK_DOWN_END: u64 = 9;
pub const STEP_INNER_DRAW_STICK_UP_END: u64 = 10;
pub const STEP_BALL_RELEASE_DONE: u64 = 11;

//
pub const STEP_BALL_RIGID_TO_STATIC: u64 = 100;
pub const STEP_BALL_RIGID_TO_DYNAMIC: u64 = 101;
pub const STEP_BALL_MIXER_ROTATE: u64 = 102;

pub const STEP_POOL_OUTLET_OPEN_START: u64 = 200;
pub const STEP_POOL_OUTLET_CLOSE_START: u64 = 201;

pub const STEP_BALL_CATCH: u64 = 202;

pub const STEP_DRAW_STICK_UP: u64 = 203;
pub const STEP_DRAW_STICK_DOWN: u64 = 204;
pub const STEP_INNER_DRAW_STICK_UP: u64 = 205;
pub const STEP_INNER_DRAW_STICK_DOWN: u64 = 206;
pub const STEP_BALL_RELEASE: u64 = 207;

pub const STEP_BALL_STICK_RIGID_TO_STATIC: u64 = 208;
pub const STEP_BALL_STICK_RIGID_TO_EMPTY: u64 = 209;

pub const STEP_GAME_RUN_COMMAND: u64 = 9999;

//
// ----
//

pub const BALL_NAMES: [(&str, u8); 45] = [
    ("BallTmp1_1", 1),
    ("BallTmp1_2", 2),
    ("BallTmp1_3", 3),
    ("BallTmp1_4", 4),
    ("BallTmp1_5", 5),
    ("BallTmp1_6", 6),
    ("BallTmp1_7", 7),
    ("BallTmp1_8", 8),
    ("BallTmp1_9", 9),
    ("BallTmp1_10", 10),
    ("BallTmp1_11", 11),
    ("BallTmp1_12", 12),
    ("BallTmp1_13", 13),
    ("BallTmp1_14", 14),
    //
    ("BallTmp2_1", 15),
    ("BallTmp2_2", 16),
    ("BallTmp2_3", 17),
    ("BallTmp2_4", 18),
    ("BallTmp2_5", 19),
    ("BallTmp2_6", 20),
    ("BallTmp2_7", 21),
    ("BallTmp2_8", 22),
    ("BallTmp2_9", 23),
    ("BallTmp2_10", 24),
    ("BallTmp2_11", 25),
    ("BallTmp2_12", 26),
    ("BallTmp2_13", 27),
    ("BallTmp2_14", 28),
    //
    ("BallTmp3_1", 29),
    ("BallTmp3_2", 30),
    ("BallTmp3_3", 31),
    ("BallTmp3_4", 32),
    ("BallTmp3_5", 33),
    ("BallTmp3_6", 34),
    ("BallTmp3_7", 35),
    ("BallTmp3_8", 36),
    ("BallTmp3_9", 37),
    ("BallTmp3_10", 38),
    ("BallTmp3_11", 39),
    ("BallTmp3_12", 40),
    ("BallTmp3_13", 41),
    ("BallTmp3_14", 42),
    //
    ("BallTmp4_1", 43),
    ("BallTmp4_2", 44),
    ("BallTmp4_3", 45),
    // ("BallTmp4_4", 46),
    // ("BallTmp4_5", 47),
    // ("BallTmp4_6", 48),
    // ("BallTmp4_7", 49),
    // ("BallTmp4_8", 50),
    // ("BallTmp4_9", 51),
    // ("BallTmp4_10", 52),
    // ("BallTmp4_11", 53),
    // ("BallTmp4_12", 54),
    // ("BallTmp4_13", 55),
    // ("BallTmp4_14", 56),
    // //
    // ("BallTmp5_1", 57),
    // ("BallTmp5_2", 58),
    // ("BallTmp5_3", 59),
    // ("BallTmp5_4", 60),
    // ("BallTmp5_5", 61),
    // ("BallTmp5_6", 62),
    // ("BallTmp5_7", 63),
    // ("BallTmp5_8", 64),
    // ("BallTmp5_9", 65),
    // ("BallTmp5_10", 66),
    // ("BallTmp5_11", 67),
    // ("BallTmp5_12", 68),
    // ("BallTmp5_13", 69),
    // ("BallTmp5_14", 70),
];
