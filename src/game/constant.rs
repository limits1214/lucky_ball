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
    (1, "Ball__1_1"),
    (2, "Ball__1_2"),
    (3, "Ball__1_3"),
    (4, "Ball__1_4"),
    (5, "Ball__1_5"),
    (6, "Ball__1_6"),
    (7, "Ball__1_7"),
    (8, "Ball__1_8"),
    (9, "Ball__1_9"),
    (10, "Ball__1_10"),
    (11, "Ball__1_11"),
    (12, "Ball__1_12"),
    (13, "Ball__1_13"),
    (14, "Ball__1_14"),
    //
    (15, "Ball__2_1"),
    (16, "Ball__2_2"),
    (17, "Ball__2_3"),
    (18, "Ball__2_4"),
    (19, "Ball__2_5"),
    (20, "Ball__2_6"),
    (21, "Ball__2_7"),
    (22, "Ball__2_8"),
    (23, "Ball__2_9"),
    (24, "Ball__2_10"),
    (25, "Ball__2_11"),
    (26, "Ball__2_12"),
    (27, "Ball__2_13"),
    (28, "Ball__2_14"),
    //
    (29, "Ball__3_1"),
    (30, "Ball__3_2"),
    (31, "Ball__3_3"),
    (32, "Ball__3_4"),
    (33, "Ball__3_5"),
    (34, "Ball__3_6"),
    (35, "Ball__3_7"),
    (36, "Ball__3_8"),
    (37, "Ball__3_9"),
    (38, "Ball__3_10"),
    (39, "Ball__3_11"),
    (40, "Ball__3_12"),
    (41, "Ball__3_13"),
    (42, "Ball__3_14"),
    //
    (43, "Ball__4_1"),
    (44, "Ball__4_2"),
    (45, "Ball__4_3"),
    (46, "Ball__4_4"),
    (47, "Ball__4_5"),
    (48, "Ball__4_6"),
    (49, "Ball__4_7"),
    (50, "Ball__4_8"),
    (51, "Ball__4_9"),
    (52, "Ball__4_10"),
    (53, "Ball__4_11"),
    (54, "Ball__4_12"),
    (55, "Ball__4_13"),
    (56, "Ball__4_14"),
    //
    (57, "Ball__5_1"),
    (58, "Ball__5_2"),
    (59, "Ball__5_3"),
    (60, "Ball__5_4"),
    (61, "Ball__5_5"),
    (62, "Ball__5_6"),
    (63, "Ball__5_7"),
    (64, "Ball__5_8"),
    (65, "Ball__5_9"),
    (66, "Ball__5_10"),
    (67, "Ball__5_11"),
    (68, "Ball__5_12"),
    (69, "Ball__5_13"),
    (70, "Ball__5_14"),
];
