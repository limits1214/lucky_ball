use avian3d::prelude::*;
use bevy::{
    gltf::{GltfMesh, GltfNode},
    math::vec3,
    prelude::*,
};
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween, TweenCompleted};
use rand::Rng;
use std::time::Duration;

use crate::{
    asset::resources::MyAsstes,
    ffi::{
        ffi_event::{AdFfi, FfiEvents, InterstitailAdEvents},
        ffi_fn::{admob_interstitial_load, admob_interstitial_show},
    },
    game::{
        component::{
            Ball, BallDrawStick, BallDrawStickIn, BallMixer, BallOutletGuideHolderLast,
            PoolOutletCover,
        },
        constant::{STEP_BALL_CATCH_DONE, STEP_INNER_DRAW_STICK_DOWN_END, TWEEN_BALL_CATCH_END},
        resource::GivenBall,
    },
};

use super::{
    component::{
        BallCatchSensor, BallReleaseSensor, Catched, Picked, PickedStatic, PoolBallCntSensor,
    },
    constant::{
        STEP_BALL_CATCH, STEP_BALL_MIXER_ROTATE, STEP_BALL_MIXER_ROTATE_END, STEP_BALL_RELEASE,
        STEP_BALL_RELEASE_DONE, STEP_BALL_RIGID_TO_DYNAMIC, STEP_BALL_RIGID_TO_STATIC,
        STEP_BALL_STICK_RIGID_TO_EMPTY, STEP_BALL_STICK_RIGID_TO_STATIC, STEP_DRAW_STICK_DOWN,
        STEP_DRAW_STICK_DOWN_END, STEP_DRAW_STICK_UP, STEP_DRAW_STICK_UP_END,
        STEP_GAME_RUN_COMMAND, STEP_INNER_DRAW_STICK_DOWN, STEP_INNER_DRAW_STICK_UP,
        STEP_INNER_DRAW_STICK_UP_END, STEP_POOL_BALL_ZERO, STEP_POOL_OUTLET_CLOSE_END,
        STEP_POOL_OUTLET_CLOSE_START, STEP_POOL_OUTLET_OPEN_END, STEP_POOL_OUTLET_OPEN_START,
        TWEEN_BALL_MIXER_ROTATE_END, TWEEN_DRAW_STICK_DOWN_END, TWEEN_DRAW_STICK_UP_END,
        TWEEN_INNER_DRAW_STICK_DOWN_END, TWEEN_INNER_DRAW_STICK_UP_END,
        TWEEN_POOL_OUTLET_CLOSE_END, TWEEN_POOL_OUTLET_OPEN_END,
    },
    event::{
        BallClearEvent, BallSpawnEvent, DrawStickResetEvent, GameEndEvent, GameRunEvent,
        GameStepData, GameStepFinishEvent, GameStepStartEvent,
    },
    resource::GameConfig,
    MyAngularVelocityYLens,
};

// pub fn spawn_balls(mut ew: EventWriter<BallSpawnEvent>) {
//     ew.send(BallSpawnEvent(false));
// }

pub fn spawn_setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //
    my_assets: Res<MyAsstes>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_node: Res<Assets<GltfNode>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
) {
    // commands.spawn((
    //     PointLightBundle {
    //         point_light: PointLight {
    //             // shadows_enabled: true,
    //             intensity: 10_000_000.,
    //             range: 100.0,
    //             // shadow_depth_bias: 0.2,
    //             ..default()
    //         },
    //         transform: Transform::from_xyz(0.0, 16.0, 0.0),
    //         ..default()
    //     },
    //     Name::new("PointLight"),
    // ));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                // shadows_enabled: true, // 필요한 경우 이 줄을 활성화할 수 있습니다.
                illuminance: light_consts::lux::AMBIENT_DAYLIGHT / 2., // intensity와 유사한 개념 (단위: 루멘)
                ..default()
            },
            // DirectionalLight는 위치보다는 방향이 중요하므로, 아래 transform을 통해 방향 설정
            transform: Transform::from_xyz(16.0, 16.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y), // 빛의 방향 설정 (0, 0, 0)을 향하도록
            ..default()
        },
        Name::new("DirectionalLight"),
    ));

    //////
    if let Some(gltf) = assets_gltf.get(my_assets.luckyball.id()) {
        for (node_name, _node_handle) in gltf.named_nodes.iter() {
            info!("node name: {:?}", node_name);
            let GltfNode {
                mesh, transform, ..
            } = assets_node.get(&gltf.named_nodes[node_name]).unwrap();

            if let Some(a) = mesh {
                let b = assets_gltfmesh.get(a.id()).unwrap();
                let mat = match &b.primitives[0].material {
                    Some(a) => a.clone(),
                    None => materials.add(StandardMaterial::default()),
                };

                let mat_handle = mat;
                let mesh_handle = b.primitives[0].mesh.clone();
                let mesh = meshes.get(mesh_handle.id()).unwrap();
                let transform = *transform;
                let node_name = node_name.as_ref();
                if node_name == "BallCase" {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: mat_handle,
                            transform,
                            ..default()
                        })
                        .insert(RigidBody::Static)
                        .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        .insert(Name::new("BallCase"));
                } else if node_name == "BallInletCover" {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: mat_handle,
                            transform,
                            ..default()
                        })
                        // .insert(RigidBody::Static)
                        // .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        .insert(Name::new("BallInletCover"));
                } else if node_name == "BallMixer2" {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: mat_handle,
                            transform,
                            ..default()
                        })
                        .insert(RigidBody::Kinematic)
                        .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        .insert(BallMixer)
                        .insert(AngularVelocity(vec3(0., 1., 0.)))
                        .insert(Name::new("BallMixer"));
                } else if node_name == "BallMixerColliderd" {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: materials.add(StandardMaterial {
                                base_color: Color::srgba(1.0, 1.0, 1.0, 0.0), // 투명한 색상
                                alpha_mode: AlphaMode::Blend, // 알파 블렌딩 모드 설정
                                ..default()
                            }),
                            transform,
                            ..default()
                        })
                        .insert(RigidBody::Kinematic)
                        .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        .insert(BallMixer)
                        .insert(AngularVelocity(vec3(0., 0., 0.)))
                        .insert(Name::new("BallMixerCollider"));
                } else if node_name == "BallDrawStick" {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: mat_handle,
                            transform,
                            ..default()
                        })
                        .insert(RigidBody::Static)
                        .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        .insert(BallDrawStick)
                        .insert(Name::new("BallDrawStick"));
                } else if node_name == "BallDrawStickIn" {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: mat_handle,
                            transform,
                            ..default()
                        })
                        .insert(RigidBody::Static)
                        .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        .insert(BallDrawStickIn)
                        .insert(Name::new("BallDrawStickIn"));
                } else if node_name == "pool" {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: mat_handle,
                            transform,
                            ..default()
                        })
                        .insert(RigidBody::Static)
                        .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        .insert(Name::new("pool"));
                } else if node_name == "BottomSupport" {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: mat_handle,
                            transform,
                            ..default()
                        })
                        // .insert(RigidBody::Static)
                        // .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        .insert(Name::new("BottomSupport"));
                } else if node_name == "BallOutletGuide" {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: mat_handle,
                            transform,
                            ..default()
                        })
                        .insert(RigidBody::Static)
                        .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        .insert(Name::new("test3"));
                } else if node_name == "BallOutletGuideHolder1"
                    || node_name == "BallOutletGuideHolder2"
                    || node_name == "BallOutletGuideHolder3"
                    || node_name == "BallOutletGuideHolder4"
                {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: mat_handle,
                            transform,
                            ..default()
                        })
                        // .insert(RigidBody::Static)
                        // .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        .insert(Name::new("BallOutletGuideHolder"));
                } else if node_name == "BallOutletGuard" {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: mat_handle,
                            transform,
                            ..default()
                        })
                        .insert(RigidBody::Static)
                        .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        .insert(Name::new("BallOutletGuard"));
                } else if node_name == "BallOutletGuideHolderLast" {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: mat_handle,
                            transform,
                            ..default()
                        })
                        .insert(RigidBody::Static)
                        .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        .insert(BallOutletGuideHolderLast)
                        .insert(Name::new("BallOutletGuideHolderLast"));
                } else if node_name == "Base" {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: mat_handle,
                            transform,
                            ..default()
                        })
                        // .insert(RigidBody::Static)
                        // .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        // .insert(BallOutletGuideHolderLast)
                        .insert(Name::new("Base"));
                } else if node_name == "poolOutletCover" {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: mat_handle,
                            transform,
                            ..default()
                        })
                        // .insert(RigidBody::Static)
                        // .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        // .insert(BallOutletGuideHolderLast)
                        .insert(PoolOutletCover)
                        .insert(Name::new("poolOutletCover"));
                } else if node_name == "BallOutletGuideResultCollider" {
                    commands
                        .spawn(Name::new("BallOutletGuideResultCollider"))
                        .insert(RigidBody::Static)
                        .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        .insert(TransformBundle::from_transform(transform));
                    // .insert(BallOutletGuideHolderLast);
                } else if node_name == "poolSupport" {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: mat_handle,
                            transform,
                            ..default()
                        })
                        // .insert(RigidBody::Static)
                        // .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        // .insert(BallOutletGuideHolderLast)
                        .insert(Name::new("poolSupport"));
                }
            }
        }
    }

    commands.spawn((
        Name::new("BallCatchSensor"),
        Sensor,
        Collider::sphere(0.01),
        BallCatchSensor,
        TransformBundle::from_transform(Transform::from_xyz(0., -0.9, 0.)),
    ));

    commands
        .spawn_empty()
        .insert(Name::new("pool ball cnt sensor"))
        .insert(PoolBallCntSensor)
        .insert(Sensor)
        .insert(TransformBundle::from_transform(
            Transform::from_xyz(-1., 1., 0.).with_scale(vec3(4., 1., 1.)),
        ))
        .insert(Collider::cuboid(0.1, 2., 1.0));

    commands
        .spawn_empty()
        .insert(Name::new("ball release sensor"))
        .insert(TransformBundle::from_transform(Transform::from_xyz(
            0., 1.1, 0.,
        )))
        .insert(BallReleaseSensor)
        .insert(Collider::cuboid(0.3, 0.3, 0.3));
}

pub fn er_ball_spawn(
    mut er: EventReader<BallSpawnEvent>,
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //
    my_assets: Res<MyAsstes>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_node: Res<Assets<GltfNode>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    config: Res<GameConfig>,
    q_ball: Query<Entity, With<Ball>>,
) {
    for BallSpawnEvent(is_shuffle) in er.read() {
        for entity in &q_ball {
            commands.entity(entity).despawn_recursive();
        }

        if let Some(gltf) = assets_gltf.get(my_assets.luckyball.id()) {
            struct Balls {
                node_name: String,
                transform: Transform,
                number: u8,
                mesh_handle: Handle<Mesh>,
                mat_handle: Handle<StandardMaterial>,
            }

            let mut balls: Vec<Balls> = Vec::new();

            let mut transforms: Vec<Transform> = Vec::new();

            // collect balls
            for (node_name, _node_handle) in gltf.named_nodes.iter() {
                let GltfNode {
                    mesh, transform, ..
                } = assets_node.get(&gltf.named_nodes[node_name]).unwrap();

                if let Some(a) = mesh {
                    let b = assets_gltfmesh.get(a.id()).unwrap();
                    let mat = match &b.primitives[0].material {
                        Some(a) => a.clone(),
                        None => materials.add(StandardMaterial::default()),
                    };

                    let mat_handle = mat;
                    let mesh_handle = b.primitives[0].mesh.clone();
                    let transform = *transform;
                    let node_name = node_name.as_ref();
                    if node_name.contains("Ball__") {
                        transforms.push(transform);
                    }
                    for GivenBall(num, name) in &config.rule_given_ball {
                        if name == node_name {
                            balls.push(Balls {
                                node_name: name.clone(),
                                transform,
                                number: num.clone(),
                                mat_handle: mat_handle.clone(),
                                mesh_handle: mesh_handle.clone(),
                            });
                        }
                    }
                }
            }

            // shuffle
            if *is_shuffle {
                for Balls { transform, .. } in balls.iter_mut() {
                    // info!("transfroms: {transforms:?} {}", transforms.len());
                    let mut rng = rand::thread_rng();
                    let idx = rng.gen_range(0..transforms.len());
                    let tr = transforms.remove(idx);
                    transform.translation = tr.translation;
                    transform.scale = tr.scale;
                    transform.rotation = tr.rotation;
                }
            }

            // spawn balls
            for Balls {
                node_name,
                transform,
                number,
                mat_handle,
                mesh_handle,
            } in balls
            {
                commands
                    .spawn(PbrBundle {
                        mesh: mesh_handle,
                        material: mat_handle,
                        transform,
                        ..default()
                    })
                    .insert(RigidBody::Static)
                    .insert(Friction::new(0.4))
                    .insert(Restitution::new(0.9))
                    .insert(Collider::sphere(1.))
                    .insert(Ball(number))
                    .insert(SpeculativeMargin(4.0))
                    .insert(Name::new(node_name));
            }
        }
    }
}

pub fn er_ball_clear(
    mut commands: Commands,
    mut er: EventReader<BallClearEvent>,
    q_ball: Query<Entity, With<Ball>>,
) {
    for _ in er.read() {
        for entity in &q_ball {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn draw_stick_up_event(
    mut er: EventReader<GameStepStartEvent>,
    mut commands: Commands,
    q_stick: Query<Entity, With<BallDrawStick>>,
    q_stick_in: Query<Entity, With<BallDrawStickIn>>,
    q_catched_ball: Query<(Entity, &Transform), (With<Ball>, With<Catched>)>,
) {
    for GameStepStartEvent { event_id, .. } in er.read() {
        if *event_id == STEP_DRAW_STICK_UP {
            let tween = Tween::new(
                EaseFunction::QuarticInOut,
                Duration::from_secs(2),
                TransformPositionLens {
                    start: vec3(0., -1.85, 0.),
                    end: vec3(0., 0.001, 0.),
                },
            )
            .with_completed_event(TWEEN_DRAW_STICK_UP_END);
            if let Ok(entity) = q_stick.get_single() {
                commands.entity(entity).insert(Animator::new(tween));
            }
            let tween = Tween::new(
                EaseFunction::QuarticInOut,
                Duration::from_secs(2),
                TransformPositionLens {
                    start: vec3(0., -1.85 + 0.65, 0.),
                    end: vec3(0., 0.65, 0.),
                },
            );
            if let Ok(entity) = q_stick_in.get_single() {
                commands.entity(entity).insert(Animator::new(tween));
            }
            if let Ok((entity, transform)) = q_catched_ball.get_single() {
                let tween = Tween::new(
                    EaseFunction::QuarticInOut,
                    Duration::from_secs(2),
                    TransformPositionLens {
                        start: transform.translation,
                        end: vec3(0., 1.0, 0.),
                    },
                );
                commands.entity(entity).insert(Animator::new(tween));
            }
        }
    }
}

pub fn draw_stick_down_event(
    mut er: EventReader<GameStepStartEvent>,
    mut commands: Commands,
    q_stick: Query<Entity, With<BallDrawStick>>,
    q_stick_in: Query<Entity, With<BallDrawStickIn>>,
) {
    for GameStepStartEvent { event_id, .. } in er.read() {
        if *event_id == STEP_DRAW_STICK_DOWN {
            let tween = Tween::new(
                EaseFunction::QuarticInOut,
                Duration::from_secs(2),
                TransformPositionLens {
                    start: vec3(0., 0.001, 0.),
                    end: vec3(0., -1.85, 0.),
                },
            )
            .with_completed_event(TWEEN_DRAW_STICK_DOWN_END);
            if let Ok(entity) = q_stick.get_single() {
                commands.entity(entity).insert(Animator::new(tween));
            }
            let tween = Tween::new(
                EaseFunction::QuarticInOut,
                Duration::from_secs(2),
                TransformPositionLens {
                    start: vec3(0., 0.65, 0.),
                    end: vec3(0., -1.85 + 0.65, 0.),
                },
            );
            if let Ok(entity) = q_stick_in.get_single() {
                commands.entity(entity).insert(Animator::new(tween));
            }
        }
    }
}

pub fn draw_inner_stick_up_event(
    mut er: EventReader<GameStepStartEvent>,
    mut commands: Commands,
    q_stick_in: Query<Entity, With<BallDrawStickIn>>,
    q_catched_ball: Query<(Entity, &Transform), (With<Ball>, With<Catched>)>,
) {
    for GameStepStartEvent { event_id, .. } in er.read() {
        if *event_id == STEP_INNER_DRAW_STICK_UP {
            let tween = Tween::new(
                EaseFunction::QuarticInOut,
                Duration::from_millis(500),
                TransformPositionLens {
                    start: vec3(0., 0.65, 0.),
                    end: vec3(0., 0.73, 0.),
                },
            )
            .with_completed_event(TWEEN_INNER_DRAW_STICK_UP_END);
            if let Ok(entity) = q_stick_in.get_single() {
                commands.entity(entity).insert(Animator::new(tween));
            }

            if let Ok((entity, transform)) = q_catched_ball.get_single() {
                let tween = Tween::new(
                    EaseFunction::QuarticInOut,
                    Duration::from_millis(500),
                    TransformPositionLens {
                        start: transform.translation,
                        end: vec3(0., 1.09, 0.),
                    },
                );
                commands.entity(entity).insert(Animator::new(tween));
            }
        }
    }
}

pub fn draw_inner_stick_down_event(
    mut er: EventReader<GameStepStartEvent>,
    mut commands: Commands,
    q_stick_in: Query<Entity, With<BallDrawStickIn>>,
) {
    for GameStepStartEvent { event_id, .. } in er.read() {
        if *event_id == STEP_INNER_DRAW_STICK_DOWN {
            let tween = Tween::new(
                EaseFunction::QuarticInOut,
                Duration::from_millis(500),
                TransformPositionLens {
                    start: vec3(0., 0.73, 0.),
                    end: vec3(0., 0.65, 0.),
                },
            )
            .with_completed_event(TWEEN_INNER_DRAW_STICK_DOWN_END);
            if let Ok(entity) = q_stick_in.get_single() {
                commands.entity(entity).insert(Animator::new(tween));
            }
        }
    }
}

pub fn draw_stick_reset_event(
    mut er: EventReader<DrawStickResetEvent>,
    mut commands: Commands,
    q_stick: Query<Entity, With<BallDrawStick>>,
    q_stick_in: Query<Entity, With<BallDrawStickIn>>,
) {
    for _ in er.read() {
        let tween = Tween::new(
            EaseFunction::QuarticInOut,
            Duration::from_millis(1),
            TransformPositionLens {
                start: vec3(0., 0.001, 0.),
                end: vec3(0., 0.001, 0.),
            },
        )
        .with_completed_event(TWEEN_DRAW_STICK_DOWN_END);
        if let Ok(entity) = q_stick.get_single() {
            commands.entity(entity).insert(Animator::new(tween));
        }
        let tween = Tween::new(
            EaseFunction::QuarticInOut,
            Duration::from_millis(1),
            TransformPositionLens {
                start: vec3(0., 0.65, 0.),
                end: vec3(0., 0.65, 0.),
            },
        );
        if let Ok(entity) = q_stick_in.get_single() {
            commands.entity(entity).insert(Animator::new(tween));
        }
    }
}

// pub fn ball_catch_sensor_collding(
//     q_sensor: Query<(Entity, &CollidingEntities), With<BallCatchSensor>>,
//     q_ball: Query<&Ball>,
// ) {
//     for (_entity, colliding_entities) in &q_sensor {
//         for entity in colliding_entities.iter() {
//             // info!("colliding_entities {entity:?}");
//             if let Ok(ball) = q_ball.get(*entity) {
//                 // info!("colliding ball {:?}", ball.0);
//             }
//         }
//     }
// }

pub fn ball_holder_last_collding(
    mut commands: Commands,
    q_last_holder: Query<(Entity, &CollidingEntities), With<BallOutletGuideHolderLast>>,
    q_ball: Query<Entity, (With<Ball>, Without<PickedStatic>)>,
) {
    for (_entity, colliding_entities) in &q_last_holder {
        for entity in colliding_entities.iter() {
            // info!("colliding_entities {entity:?}");
            if let Ok(entity) = q_ball.get(*entity) {
                info!("colliding ball ");
                commands
                    .entity(entity)
                    .insert(PickedStatic)
                    .insert(RigidBody::Static);
            }
        }
    }
}

pub fn ball_picked_static(
    mut commands: Commands,
    q_picked_static: Query<(Entity, &CollidingEntities), With<PickedStatic>>,
    q_ball: Query<Entity, (With<Ball>, Without<PickedStatic>)>,
) {
    for (_entity, colliding_entities) in &q_picked_static {
        for entity in colliding_entities.iter() {
            // info!("colliding_entities {entity:?}");
            if let Ok(entity) = q_ball.get(*entity) {
                // info!("colliding ball {:?}", ball.0);
                commands
                    .entity(entity)
                    .insert(PickedStatic)
                    .insert(RigidBody::Static);
            }
        }
    }
}

pub fn pool_ball_cnt_zero_sensor(
    mut config: ResMut<GameConfig>,
    q_sensor: Query<(Entity, &CollidingEntities), With<PoolBallCntSensor>>,
    q_ball: Query<&Ball>,
    mut ew: EventWriter<GameStepFinishEvent>,
) {
    if config.is_running {
        if config.is_pool_ball_cnt_sensor {
            for (_entity, colliding_entitiles) in &q_sensor {
                let mut ball_cnt = 0;
                for entity in colliding_entitiles.iter() {
                    if let Ok(_) = q_ball.get(*entity) {
                        ball_cnt += 1;
                    }
                }

                if ball_cnt == 0 {
                    config.is_pool_ball_cnt_sensor = false;
                    ew.send(GameStepFinishEvent::new(STEP_POOL_BALL_ZERO));
                }
            }
        }
    }
}

pub fn ball_release_sensor(
    mut config: ResMut<GameConfig>,
    q_sensor: Query<(Entity, &CollidingEntities), With<BallReleaseSensor>>,
    q_ball: Query<&Ball>,
    mut ew: EventWriter<GameStepFinishEvent>,
) {
    if config.is_running {
        if config.is_ball_release_sensor {
            for (_entity, colliding_entitiles) in &q_sensor {
                let mut is_ball_released = true;

                for entity in colliding_entitiles.iter() {
                    if let Ok(_) = q_ball.get(*entity) {
                        is_ball_released = false;
                    }
                }

                if is_ball_released {
                    config.is_ball_release_sensor = false;
                    ew.send(GameStepFinishEvent::new(STEP_BALL_RELEASE_DONE));
                }
            }
        }
    }
}

pub fn er_ball_catch(mut er: EventReader<GameStepStartEvent>, mut config: ResMut<GameConfig>) {
    for GameStepStartEvent { event_id, .. } in er.read() {
        if *event_id == STEP_BALL_CATCH {
            config.is_catching = true;
        }
    }
}

pub fn ball_catch(
    mut commands: Commands,
    mut config: ResMut<GameConfig>,
    q_sensor: Query<(Entity, &CollidingEntities), With<BallCatchSensor>>,
    q_ball: Query<(Entity, &Transform, &Ball), With<Ball>>,
) {
    if config.is_running {
        if config.is_catching {
            for (_entity, colliding_entities) in &q_sensor {
                for entity in colliding_entities.iter() {
                    if let Ok((entity, transform, ball)) = q_ball.get(*entity) {
                        config.is_catching = false;
                        let tween = Tween::new(
                            EaseFunction::QuadraticInOut,
                            Duration::from_millis(100),
                            TransformPositionLens {
                                start: transform.translation,
                                end: vec3(0., -0.9, 0.),
                            },
                        )
                        .with_completed_event(TWEEN_BALL_CATCH_END);
                        commands
                            .entity(entity)
                            .insert(RigidBody::Static)
                            .insert(Catched)
                            .insert(Picked)
                            .insert(Animator::new(tween));
                        info!("catched!! {:?}", ball.0);
                        config.picked_ball.push(ball.0);
                        return;
                    }
                }
            }
        }
    }
}

pub fn er_ball_release(
    mut commands: Commands,
    mut er: EventReader<GameStepStartEvent>,
    q_catched_ball: Query<(Entity, &Transform), (With<Ball>, With<Catched>)>,
) {
    for GameStepStartEvent { event_id, .. } in er.read() {
        if *event_id == STEP_BALL_RELEASE {
            if let Ok((entity, transform)) = q_catched_ball.get_single() {
                commands
                    .entity(entity)
                    .remove::<Catched>()
                    .insert(Restitution::new(0.))
                    // .insert(CollisionMargin(0.001))
                    .insert(RigidBody::Dynamic);

                let mut impulse = ExternalImpulse::default();
                impulse.apply_impulse(Vec3::NEG_Z * 0.002);
                commands.entity(entity).insert(impulse);
            }
        }
    }
}

pub fn er_pool_outlet_cover_open(
    mut commands: Commands,
    mut er: EventReader<GameStepStartEvent>,
    q_cover: Query<Entity, With<PoolOutletCover>>,
) {
    for GameStepStartEvent { event_id, .. } in er.read() {
        if *event_id == STEP_POOL_OUTLET_OPEN_START {
            if let Ok(entity) = q_cover.get_single() {
                let tween = Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_millis(500),
                    TransformPositionLens {
                        start: vec3(-0.86, 0.1, 0.0),
                        end: vec3(-0.86, -0.1, 0.0),
                    },
                )
                .with_completed_event(TWEEN_POOL_OUTLET_OPEN_END);
                commands.entity(entity).insert(Animator::new(tween));
            }
        }
    }
}

pub fn er_pool_outlet_cover_close(
    mut commands: Commands,
    mut er: EventReader<GameStepStartEvent>,
    q_cover: Query<Entity, With<PoolOutletCover>>,
) {
    for GameStepStartEvent { event_id, .. } in er.read() {
        if *event_id == STEP_POOL_OUTLET_CLOSE_START {
            if let Ok(entity) = q_cover.get_single() {
                let tween = Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_millis(500),
                    TransformPositionLens {
                        start: vec3(-0.86, -0.1, 0.0),
                        end: vec3(-0.86, 0.1, 0.0),
                    },
                )
                .with_completed_event(TWEEN_POOL_OUTLET_CLOSE_END);
                commands.entity(entity).insert(Animator::new(tween));
            }
        }
    }
}

/// true: dynamic
/// false: static
pub fn er_ball_rigid_change(
    mut commands: Commands,
    q_ball: Query<Entity, With<Ball>>,
    mut er: EventReader<GameStepStartEvent>,
) {
    for GameStepStartEvent { event_id, .. } in er.read() {
        match *event_id {
            STEP_BALL_RIGID_TO_DYNAMIC => {
                for entity in &q_ball {
                    commands.entity(entity).insert(RigidBody::Dynamic);
                }
            }
            STEP_BALL_RIGID_TO_STATIC => {
                for entity in &q_ball {
                    commands.entity(entity).insert(RigidBody::Static);
                }
            }
            _ => {}
        }
    }
}

pub fn draw_stick_rigid_change(
    mut commands: Commands,
    mut er: EventReader<GameStepStartEvent>,
    q_stick: Query<Entity, With<BallDrawStick>>,
    q_stick_in: Query<Entity, With<BallDrawStickIn>>,
) {
    for GameStepStartEvent { event_id, .. } in er.read() {
        if *event_id == STEP_BALL_STICK_RIGID_TO_STATIC {
            if let Ok(entity) = q_stick.get_single() {
                commands.entity(entity).insert(RigidBody::Static);
            }
            if let Ok(entity) = q_stick_in.get_single() {
                commands.entity(entity).insert(RigidBody::Static);
            }
        } else if *event_id == STEP_BALL_STICK_RIGID_TO_EMPTY {
            if let Ok(entity) = q_stick.get_single() {
                commands.entity(entity).remove::<RigidBody>();
            }
            if let Ok(entity) = q_stick_in.get_single() {
                commands.entity(entity).remove::<RigidBody>();
            }
        }
    }
}

pub fn ball_mixer_rotate(
    mut commands: Commands,
    mut er: EventReader<GameStepStartEvent>,
    q_mixer: Query<(Entity, &AngularVelocity), With<BallMixer>>,
) {
    for evt in er.read() {
        match evt {
            GameStepStartEvent {
                event_id: STEP_BALL_MIXER_ROTATE,
                data: Some(GameStepData::Float(speed)),
            } => {
                if let Ok((entity, av)) = q_mixer.get_single() {
                    let tween = Tween::new(
                        EaseFunction::QuarticInOut,
                        Duration::from_millis(2000),
                        MyAngularVelocityYLens {
                            start: av.y,
                            end: *speed,
                        },
                    )
                    .with_completed_event(TWEEN_BALL_MIXER_ROTATE_END);
                    commands.entity(entity).insert(Animator::new(tween));
                }
            }
            _ => {}
        }
    }
}

pub fn er_game_run(
    mut er: EventReader<GameRunEvent>,
    mut config: ResMut<GameConfig>,
    mut ew: EventWriter<GameStepFinishEvent>,
) {
    for _ in er.read() {
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        {
            config.is_running = true;
            ew.send(GameStepFinishEvent::new(STEP_GAME_RUN_COMMAND));
            config.running_cnt += 1;
        }
        #[cfg(any(target_os = "ios", target_os = "android"))]
        {
            use crate::ffi::ffi_fn::admob_interstitial_is_ready;
            admob_interstitial_is_ready();
        }
    }
}

pub fn er_ffi_ad(
    mut er: EventReader<FfiEvents>,
    mut config: ResMut<GameConfig>,
    mut ew: EventWriter<GameStepFinishEvent>,
) {
    for evt in er.read() {
        match evt.clone() {
            FfiEvents::Ad(AdFfi::AdmobInterstitial(InterstitailAdEvents::ShowFail(msg))) => {
                config.is_running = true;
                ew.send(GameStepFinishEvent::new(STEP_GAME_RUN_COMMAND));
                config.running_cnt += 1;
            }
            FfiEvents::Ad(AdFfi::AdmobInterstitial(InterstitailAdEvents::Dismissed)) => {
                config.is_running = true;
                ew.send(GameStepFinishEvent::new(STEP_GAME_RUN_COMMAND));
                config.running_cnt += 1;
                admob_interstitial_load()
            }
            FfiEvents::Ad(AdFfi::AdmobInterstitial(InterstitailAdEvents::IsReady(is_ready))) => {
                if is_ready {
                    info!("interstitail is ready");
                    let mut rng = rand::thread_rng();
                    let random_value: u8 = rng.gen_range(0..=100);
                    if random_value <= config.show_ad_weight {
                        info!("interstitail show");
                        admob_interstitial_show();
                        config.show_ad_weight = 0;
                    } else {
                        config.is_running = true;
                        ew.send(GameStepFinishEvent::new(STEP_GAME_RUN_COMMAND));
                        config.running_cnt += 1;
                        config.show_ad_weight += 20;
                        info!(
                            "interstitail not show add weight {:>}",
                            config.show_ad_weight
                        );
                    }
                } else {
                    info!("interstitail is not ready");
                    admob_interstitial_load();
                    config.is_running = true;
                    ew.send(GameStepFinishEvent::new(STEP_GAME_RUN_COMMAND));
                    config.running_cnt += 1;
                }
            }
            _ => {}
        }
    }
}

// pub fn er_game_end(mut er: EventReader<GameEndEvent>) {
//     for _ in er.read() {
//         //
//     }
// }

// pub fn er_game_reset(mut er: EventReader<GameResetEvent>) {
//     for _ in er.read() {
//         //
//     }
// }

/// 1. open pooloutlet
/// 2. ball rigid to dynamic
/// check pool ball zero
/// close pooloutlet

pub fn game_run_step_finish(
    mut er: EventReader<GameStepFinishEvent>,
    mut config: ResMut<GameConfig>,
    mut ew_step_start: EventWriter<GameStepStartEvent>,
    mut ew_game_end: EventWriter<GameEndEvent>,
) {
    if config.is_running {
        for GameStepFinishEvent { event_id, .. } in er.read() {
            match *event_id {
                STEP_GAME_RUN_COMMAND => {
                    config.is_ball_release_sensor = false;
                    config.is_pool_ball_cnt_sensor = false;
                    config.is_catching = false;
                    config.picked_ball = vec![];
                    ew_step_start.send(GameStepStartEvent::new(STEP_POOL_OUTLET_OPEN_START));
                }
                STEP_POOL_OUTLET_OPEN_END => {
                    config.is_pool_ball_cnt_sensor = true;
                    ew_step_start.send(GameStepStartEvent::new(STEP_BALL_RIGID_TO_DYNAMIC));
                }
                STEP_POOL_BALL_ZERO => {
                    ew_step_start.send(GameStepStartEvent::new(STEP_POOL_OUTLET_CLOSE_START));
                }
                STEP_POOL_OUTLET_CLOSE_END => {
                    ew_step_start.send(GameStepStartEvent::new_with_data(
                        STEP_BALL_MIXER_ROTATE,
                        GameStepData::Float(11.),
                    ));
                    ew_step_start.send(GameStepStartEvent::new(STEP_DRAW_STICK_DOWN));
                }
                STEP_BALL_MIXER_ROTATE_END => {
                    // MIXER ROTATE를 자주하게되는데 이러면 이벤트가 계속발생하므로 사용은 x
                    // ew_stick_down.send(DrawStickDownEvent);
                }
                STEP_DRAW_STICK_DOWN_END => {
                    ew_step_start.send(GameStepStartEvent::new_with_data(
                        STEP_BALL_MIXER_ROTATE,
                        GameStepData::Float(1.),
                    ));
                    ew_step_start.send(GameStepStartEvent::new(STEP_BALL_CATCH));
                    ew_step_start.send(GameStepStartEvent::new(STEP_BALL_STICK_RIGID_TO_EMPTY));
                }
                STEP_BALL_CATCH_DONE => {
                    ew_step_start.send(GameStepStartEvent::new(STEP_BALL_STICK_RIGID_TO_STATIC));
                    ew_step_start.send(GameStepStartEvent::new(STEP_DRAW_STICK_UP));
                }
                STEP_DRAW_STICK_UP_END => {
                    ew_step_start.send(GameStepStartEvent::new(STEP_INNER_DRAW_STICK_UP));
                }
                STEP_INNER_DRAW_STICK_UP_END => {
                    config.is_ball_release_sensor = true;
                    ew_step_start.send(GameStepStartEvent::new(STEP_BALL_RELEASE));
                }
                STEP_BALL_RELEASE_DONE => {
                    info!("ball release done!");
                    ew_step_start.send(GameStepStartEvent::new(STEP_INNER_DRAW_STICK_DOWN));
                }
                STEP_INNER_DRAW_STICK_DOWN_END => {
                    //JUDGE
                    if config.picked_ball.len() < config.rule_taken_ball as usize {
                        // KEEP GOING
                        ew_step_start.send(GameStepStartEvent::new(STEP_DRAW_STICK_DOWN));
                        ew_step_start.send(GameStepStartEvent::new_with_data(
                            STEP_BALL_MIXER_ROTATE,
                            GameStepData::Float(11.),
                        ));
                    } else {
                        // END
                        // todo rotate speed down is must worked before step
                        info!("end! picked ball is {:?}", config.picked_ball);
                        ew_step_start.send(GameStepStartEvent::new_with_data(
                            STEP_BALL_MIXER_ROTATE,
                            GameStepData::Float(1.),
                        ));

                        config.is_running = false;

                        ew_game_end.send(GameEndEvent);
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn tcb_to_step_convert(
    mut er: EventReader<TweenCompleted>,
    mut ew: EventWriter<GameStepFinishEvent>,
) {
    for TweenCompleted { user_data, .. } in er.read() {
        // info!("entity {entity:?}");
        match *user_data {
            TWEEN_DRAW_STICK_UP_END => {
                ew.send(GameStepFinishEvent::new(STEP_DRAW_STICK_UP_END));
            }
            TWEEN_POOL_OUTLET_OPEN_END => {
                ew.send(GameStepFinishEvent::new(STEP_POOL_OUTLET_OPEN_END));
            }
            TWEEN_POOL_OUTLET_CLOSE_END => {
                ew.send(GameStepFinishEvent::new(STEP_POOL_OUTLET_CLOSE_END));
            }
            TWEEN_BALL_MIXER_ROTATE_END => {
                ew.send(GameStepFinishEvent::new(STEP_BALL_MIXER_ROTATE_END));
            }
            TWEEN_DRAW_STICK_DOWN_END => {
                ew.send(GameStepFinishEvent::new(STEP_DRAW_STICK_DOWN_END));
            }
            TWEEN_INNER_DRAW_STICK_UP_END => {
                ew.send(GameStepFinishEvent::new(STEP_INNER_DRAW_STICK_UP_END));
            }
            TWEEN_INNER_DRAW_STICK_DOWN_END => {
                ew.send(GameStepFinishEvent::new(STEP_INNER_DRAW_STICK_DOWN_END));
            }
            TWEEN_BALL_CATCH_END => {
                ew.send(GameStepFinishEvent::new(STEP_BALL_CATCH_DONE));
            }
            _ => {}
        }
    }
}

// pub fn play_ball_sound(
//     mut collision_event_reader: EventReader<CollisionStarted>,
//     q_ball_collidings: Query<(Entity, &CollidingEntities), With<Ball>>,
//     q_ball: Query<&Ball>,
//     my_assets: Res<MyAsstes>,
//     audio: Res<bevy_kira_audio::Audio>,
// ) {
//     for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
//         // println!(
//         //     "Entities {:?} and {:?} are colliding",
//         //     contacts.entity1, contacts.entity2,
//         // );
//         let mut is_ball1 = false;
//         let mut is_ball2 = false;
//         if let Ok(_) = q_ball.get(*entity1) {
//             is_ball1 = true;
//         }
//         if let Ok(_) = q_ball.get(*entity2) {
//             is_ball2 = true;
//         }

//         if is_ball1 && is_ball2 {
//             // info!("ball colliding");
//             // audio.play(my_assets.mp3_ballsound.clone());
//         }
//     }
//     // for (e, c) in &q_ball_collidings {
//     //     for e in c.iter() {}
//     //     //
//     // }
//     // audio.play(my_assets.mp3_ballsound.clone());
// }
