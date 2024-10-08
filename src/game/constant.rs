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

pub const BALL_REFS: [(u8, &str); 70] = [
    (1, "BallTmp1_1"),
    (2, "BallTmp1_2"),
    (3, "BallTmp1_3"),
    (4, "BallTmp1_4"),
    (5, "BallTmp1_5"),
    (6, "BallTmp1_6"),
    (7, "BallTmp1_7"),
    (8, "BallTmp1_8"),
    (9, "BallTmp1_9"),
    (10, "BallTmp1_10"),
    (11, "BallTmp1_11"),
    (12, "BallTmp1_12"),
    (13, "BallTmp1_13"),
    (14, "BallTmp1_14"),
    //
    (15, "BallTmp2_1"),
    (16, "BallTmp2_2"),
    (17, "BallTmp2_3"),
    (18, "BallTmp2_4"),
    (19, "BallTmp2_5"),
    (20, "BallTmp2_6"),
    (21, "BallTmp2_7"),
    (22, "BallTmp2_8"),
    (23, "BallTmp2_9"),
    (24, "BallTmp2_10"),
    (25, "BallTmp2_11"),
    (26, "BallTmp2_12"),
    (27, "BallTmp2_13"),
    (28, "BallTmp2_14"),
    //
    (29, "BallTmp3_1"),
    (30, "BallTmp3_2"),
    (31, "BallTmp3_3"),
    (32, "BallTmp3_4"),
    (33, "BallTmp3_5"),
    (34, "BallTmp3_6"),
    (35, "BallTmp3_7"),
    (36, "BallTmp3_8"),
    (37, "BallTmp3_9"),
    (38, "BallTmp3_10"),
    (39, "BallTmp3_11"),
    (40, "BallTmp3_12"),
    (41, "BallTmp3_13"),
    (42, "BallTmp3_14"),
    //
    (43, "BallTmp4_1"),
    (44, "BallTmp4_2"),
    (45, "BallTmp4_3"),
    (46, "BallTmp4_4"),
    (47, "BallTmp4_5"),
    (48, "BallTmp4_6"),
    (49, "BallTmp4_7"),
    (50, "BallTmp4_8"),
    (51, "BallTmp4_9"),
    (52, "BallTmp4_10"),
    (53, "BallTmp4_11"),
    (54, "BallTmp4_12"),
    (55, "BallTmp4_13"),
    (56, "BallTmp4_14"),
    //
    (57, "BallTmp5_1"),
    (58, "BallTmp5_2"),
    (59, "BallTmp5_3"),
    (60, "BallTmp5_4"),
    (61, "BallTmp5_5"),
    (62, "BallTmp5_6"),
    (63, "BallTmp5_7"),
    (64, "BallTmp5_8"),
    (65, "BallTmp5_9"),
    (66, "BallTmp5_10"),
    (67, "BallTmp5_11"),
    (68, "BallTmp5_12"),
    (69, "BallTmp5_13"),
    (70, "BallTmp5_14"),
];
