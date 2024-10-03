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
        component::{Ball, BallDrawStick, BallDrawStickIn, BallMixer},
        constant::BALL_NAMES,
    },
};

use super::{
    component::BallCatchSensor,
    event::{BallMixerRotateEvent, BallRigidChange, DrawStickDownEvent, DrawStickUpEvent},
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
            // info!("node name: {:?}", node_name);
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
                        // .insert(RigidBody::Static)
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
                        // .insert(RigidBody::Static)
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
                        .insert(RigidBody::Static)
                        .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        .insert(Name::new("BottomSupport"));
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
) {
    for _ in er.read() {
        let tween = Tween::new(
            EaseFunction::QuarticInOut,
            Duration::from_secs(3),
            TransformPositionLens {
                start: vec3(0., -2., 0.),
                end: vec3(0., 0., 0.),
            },
        );
        if let Ok(entity) = q_stick.get_single() {
            commands.entity(entity).insert(Animator::new(tween));
        }
        let tween = Tween::new(
            EaseFunction::QuarticInOut,
            Duration::from_secs(3),
            TransformPositionLens {
                start: vec3(0., -2. + 0.65, 0.),
                end: vec3(0., 0.65, 0.),
            },
        );
        if let Ok(entity) = q_stick_in.get_single() {
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
            Duration::from_secs(3),
            TransformPositionLens {
                start: vec3(0., 0., 0.),
                end: vec3(0., -1.85, 0.),
            },
        );
        if let Ok(entity) = q_stick.get_single() {
            commands.entity(entity).insert(Animator::new(tween));
        }
        let tween = Tween::new(
            EaseFunction::QuarticInOut,
            Duration::from_secs(3),
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

pub fn ball_catch_sensor_collding(
    q_sensor: Query<(Entity, &CollidingEntities), With<BallCatchSensor>>,
) {
    for (_entity, colliding_entities) in &q_sensor {
        for e in colliding_entities.iter() {
            for entity in colliding_entities.iter() {
                info!("colliding_entities {entity:?}");
            }
            info!("=====");
            // if let Ok(_) = q_lotto_ball.get(*e) {
            //     let mut impulse = ExternalImpulse::default();
            //     impulse.apply_impulse(Vec3::new(0.0, 0.01, 0.0));
            //     commands.entity(*e).insert(impulse);
            // }
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
