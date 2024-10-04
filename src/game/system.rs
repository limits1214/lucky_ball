use avian3d::prelude::*;
use bevy::{
    gltf::{GltfMesh, GltfNode},
    math::vec3,
    prelude::*,
};
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween};
use std::time::Duration;

use crate::{
    assets::resources::MyAsstes,
    game::{
        component::{Ball, BallDrawStick, BallDrawStickIn, BallMixer, BallOutletGuideHolderLast},
        constant::BALL_NAMES,
    },
};

use super::{
    component::{BallCatchSensor, Catched, Picked, PickedStatic},
    event::{
        BallCatchDoneEvent, BallCatchEvent, BallMixerRotateEvent, BallReleaseEvent,
        BallRigidChange, DrawInnerStickDownEvent, DrawInnerStickUpEvent, DrawStickDownEvent,
        DrawStickRigidChangeEvent, DrawStickUpEvent,
    },
    resource::GameConfig,
};

pub fn spawn_balls(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //
    my_assets: Res<MyAsstes>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_node: Res<Assets<GltfNode>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
) {
    if let Some(gltf) = assets_gltf.get(my_assets.luckyball.id()) {
        struct Balls {
            node_name: String,
            transform: Transform,
            number: u8,
            mesh_handle: Handle<Mesh>,
            mat_handle: Handle<StandardMaterial>,
        }

        let mut balls: Vec<Balls> = Vec::new();

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

                for (name, num) in BALL_NAMES {
                    if name == node_name {
                        balls.push(Balls {
                            node_name: name.to_owned(),
                            transform,
                            number: num,
                            mat_handle: mat_handle.clone(),
                            mesh_handle: mesh_handle.clone(),
                        });
                    }
                }
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
                .insert(Name::new(node_name));
        }
    }
}

pub fn spawn_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //
    my_assets: Res<MyAsstes>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_node: Res<Assets<GltfNode>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
) {
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                // shadows_enabled: true,
                intensity: 10_000_000.,
                range: 100.0,
                // shadow_depth_bias: 0.2,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 16.0, 0.0),
            ..default()
        },
        Name::new("PointLight"),
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
                        .insert(AngularVelocity(vec3(0., 0., 0.)))
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
}

pub fn draw_stick_up_event(
    mut er: EventReader<DrawStickUpEvent>,
    mut commands: Commands,
    q_stick: Query<Entity, With<BallDrawStick>>,
    q_stick_in: Query<Entity, With<BallDrawStickIn>>,
    q_catched_ball: Query<(Entity, &Transform), (With<Ball>, With<Catched>)>,
) {
    for _ in er.read() {
        let tween = Tween::new(
            EaseFunction::QuarticInOut,
            Duration::from_secs(2),
            TransformPositionLens {
                start: vec3(0., -1.85, 0.),
                end: vec3(0., 0.001, 0.),
            },
        );
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

pub fn draw_stick_down_event(
    mut er: EventReader<DrawStickDownEvent>,
    mut commands: Commands,
    q_stick: Query<Entity, With<BallDrawStick>>,
    q_stick_in: Query<Entity, With<BallDrawStickIn>>,
) {
    for _ in er.read() {
        let tween = Tween::new(
            EaseFunction::QuarticInOut,
            Duration::from_secs(2),
            TransformPositionLens {
                start: vec3(0., 0.001, 0.),
                end: vec3(0., -1.85, 0.),
            },
        );
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

pub fn draw_inner_stick_up_event(
    mut er: EventReader<DrawInnerStickUpEvent>,
    mut commands: Commands,
    q_stick_in: Query<Entity, With<BallDrawStickIn>>,
    q_catched_ball: Query<(Entity, &Transform), (With<Ball>, With<Catched>)>,
) {
    for _ in er.read() {
        let tween = Tween::new(
            EaseFunction::QuarticInOut,
            Duration::from_millis(500),
            TransformPositionLens {
                start: vec3(0., 0.65, 0.),
                end: vec3(0., 0.73, 0.),
            },
        );
        if let Ok(entity) = q_stick_in.get_single() {
            commands.entity(entity).insert(Animator::new(tween));
        }

        if let Ok((entity, transform)) = q_catched_ball.get_single() {
            let tween = Tween::new(
                EaseFunction::QuarticInOut,
                Duration::from_millis(500),
                TransformPositionLens {
                    start: transform.translation,
                    end: vec3(0., 1.1, 0.),
                },
            );
            commands.entity(entity).insert(Animator::new(tween));
        }
    }
}

pub fn draw_inner_stick_down_event(
    mut er: EventReader<DrawInnerStickDownEvent>,
    mut commands: Commands,
    q_stick_in: Query<Entity, With<BallDrawStickIn>>,
) {
    for _ in er.read() {
        let tween = Tween::new(
            EaseFunction::QuarticInOut,
            Duration::from_millis(500),
            TransformPositionLens {
                start: vec3(0., 0.73, 0.),
                end: vec3(0., 0.65, 0.),
            },
        );
        if let Ok(entity) = q_stick_in.get_single() {
            commands.entity(entity).insert(Animator::new(tween));
        }
    }
}

pub fn ball_catch_sensor_collding(
    q_sensor: Query<(Entity, &CollidingEntities), With<BallCatchSensor>>,
    q_ball: Query<&Ball>,
) {
    for (_entity, colliding_entities) in &q_sensor {
        for entity in colliding_entities.iter() {
            // info!("colliding_entities {entity:?}");
            if let Ok(ball) = q_ball.get(*entity) {
                // info!("colliding ball {:?}", ball.0);
            }
        }
    }
}

pub fn ball_holder_last_collding(
    mut commands: Commands,
    q_last_holder: Query<(Entity, &CollidingEntities), With<BallOutletGuideHolderLast>>,
    q_ball: Query<Entity, With<Ball>>,
) {
    for (_entity, colliding_entities) in &q_last_holder {
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

pub fn er_ball_catch(mut er: EventReader<BallCatchEvent>, mut config: ResMut<GameConfig>) {
    for _ in er.read() {
        config.is_catching = true;
    }
}

pub fn ball_catch(
    mut commands: Commands,
    mut config: ResMut<GameConfig>,
    q_sensor: Query<(Entity, &CollidingEntities), With<BallCatchSensor>>,
    q_ball: Query<(Entity, &Transform, &Ball), With<Ball>>,
    mut ew: EventWriter<BallCatchDoneEvent>,
) {
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
                    );
                    commands
                        .entity(entity)
                        .insert(RigidBody::Static)
                        .insert(Catched)
                        .insert(Picked)
                        .insert(Animator::new(tween));
                    ew.send(BallCatchDoneEvent);
                    info!("catched!! {:?}", ball.0);
                }
            }
        }
    }
}

pub fn er_ball_release(
    mut commands: Commands,
    mut er: EventReader<BallReleaseEvent>,
    q_catched_ball: Query<(Entity, &Transform), (With<Ball>, With<Catched>)>,
) {
    for _ in er.read() {
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

/// true: dynamic
/// false: static
pub fn er_ball_rigid_change(
    mut commands: Commands,
    q_ball: Query<Entity, With<Ball>>,
    mut er: EventReader<BallRigidChange>,
) {
    for evt in er.read() {
        for entity in &q_ball {
            if evt.0 {
                commands.entity(entity).insert(RigidBody::Dynamic);
            } else {
                commands.entity(entity).insert(RigidBody::Static);
            }
        }
    }
}

pub fn draw_stick_rigid_change(
    mut commands: Commands,
    mut er: EventReader<DrawStickRigidChangeEvent>,
    q_stick: Query<Entity, With<BallDrawStick>>,
    q_stick_in: Query<Entity, With<BallDrawStickIn>>,
) {
    for evt in er.read() {
        if evt.0 {
            if let Ok(entity) = q_stick.get_single() {
                commands.entity(entity).insert(RigidBody::Static);
            }
            if let Ok(entity) = q_stick_in.get_single() {
                commands.entity(entity).insert(RigidBody::Static);
            }
        } else {
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
    mut er_mixer_rotate: EventReader<BallMixerRotateEvent>,
    q_mixer: Query<Entity, With<BallMixer>>,
) {
    for evt in er_mixer_rotate.read() {
        if let Ok(entity) = q_mixer.get_single() {
            commands
                .entity(entity)
                .insert(AngularVelocity(vec3(0., evt.0, 0.)));
        }
    }
}
