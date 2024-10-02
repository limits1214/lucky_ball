use avian3d::prelude::*;
use bevy::{
    gltf::{GltfMesh, GltfNode},
    math::vec3,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use bevy_color::palettes::css::SILVER;
use std::f32::consts::PI;

use crate::assets::resources::MyAsstes;
#[derive(Component)]
pub struct Shape;

const SHAPES_X_EXTENT: f32 = 14.0;
const EXTRUSION_X_EXTENT: f32 = 16.0;
const Z_EXTENT: f32 = 5.0;
pub fn test_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //
    my_assets: Res<MyAsstes>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_node: Res<Assets<GltfNode>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let shapes = [
        meshes.add(Cuboid::default()),
        meshes.add(Tetrahedron::default()),
        meshes.add(Capsule3d::default()),
        meshes.add(Torus::default()),
        meshes.add(Cylinder::default()),
        meshes.add(Cone::default()),
        meshes.add(ConicalFrustum::default()),
        meshes.add(Sphere::default().mesh().ico(5).unwrap()),
        meshes.add(Sphere::default().mesh().uv(32, 18)),
    ];

    let extrusions = [
        meshes.add(Extrusion::new(Rectangle::default(), 1.)),
        meshes.add(Extrusion::new(Capsule2d::default(), 1.)),
        meshes.add(Extrusion::new(Annulus::default(), 1.)),
        meshes.add(Extrusion::new(Circle::default(), 1.)),
        meshes.add(Extrusion::new(Ellipse::default(), 1.)),
        meshes.add(Extrusion::new(RegularPolygon::default(), 1.)),
        meshes.add(Extrusion::new(Triangle2d::default(), 1.)),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform::from_xyz(
                    -SHAPES_X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * SHAPES_X_EXTENT,
                    2.0,
                    Z_EXTENT / 2.,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            Shape,
        ));
    }

    let num_extrusions = extrusions.len();

    for (i, shape) in extrusions.into_iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform::from_xyz(
                    -EXTRUSION_X_EXTENT / 2.
                        + i as f32 / (num_extrusions - 1) as f32 * EXTRUSION_X_EXTENT,
                    2.0,
                    -Z_EXTENT / 2.,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            Shape,
        ));
    }

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10)),
    //     material: materials.add(Color::from(SILVER)),
    //     ..default()
    // });

    //////

    // commands.spawn((
    //     SceneBundle {
    //         scene: my_assets.luckyball.clone(),
    //         transform: Transform {
    //             translation: vec3(0., 10., 0.),
    //             scale: vec3(10., 10., 10.),
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     RigidBody::Static,
    // ));
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
                            // transform: transform.with_scale(vec3(10., 10., 10.)), // .with_translation(vec3(0., 10., 0.)),
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
                            // transform: transform.with_scale(vec3(10., 10., 10.)), // .with_translation(vec3(0., 10., 0.)),
                            ..default()
                        })
                        // .insert(RigidBody::Static)
                        // .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        .insert(Name::new("BallInletCover"));
                } else if node_name == "BallMixer" {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: mat_handle,
                            transform,
                            // transform: transform.with_scale(vec3(10., 10., 10.)), // .with_translation(vec3(0., 10., 0.)),
                            ..default()
                        })
                        .insert(RigidBody::Kinematic)
                        .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        .insert(BallMixer)
                        .insert(AngularVelocity(vec3(0., 1., 0.)))
                        .insert(Name::new("BallMixer"));
                } else if node_name == "BallInletGuide2" {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: mat_handle,
                            transform,
                            // transform: transform.with_scale(vec3(10., 10., 10.)), // .with_translation(vec3(0., 10., 0.)),
                            ..default()
                        })
                        .insert(RigidBody::Static)
                        .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        .insert(Name::new("BallInletCover"));
                } else if node_name == "pool" {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: mat_handle,
                            transform,
                            // transform: transform.with_scale(vec3(10., 10., 10.)), // .with_translation(vec3(0., 10., 0.)),
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
                            // transform: transform.with_scale(vec3(10., 10., 10.)), // .with_translation(vec3(0., 10., 0.)),
                            ..default()
                        })
                        .insert(RigidBody::Static)
                        .insert(Collider::trimesh_from_mesh(mesh).unwrap())
                        .insert(Name::new("BottomSupport"));
                } else if node_name == "BallTmp1_1"
                    || node_name == "BallTmp1_2"
                    || node_name == "BallTmp1_3"
                    || node_name == "BallTmp1_4"
                    || node_name == "BallTmp1_5"
                    || node_name == "BallTmp1_6"
                    || node_name == "BallTmp1_7"
                    || node_name == "BallTmp1_8"
                    || node_name == "BallTmp1_9"
                    || node_name == "BallTmp1_10"
                    || node_name == "BallTmp1_11"
                    || node_name == "BallTmp1_12"
                    || node_name == "BallTmp1_13"
                    || node_name == "BallTmp1_14"
                    || node_name == "BallTmp2_1"
                    || node_name == "BallTmp2_2"
                    || node_name == "BallTmp2_3"
                    || node_name == "BallTmp2_4"
                    || node_name == "BallTmp2_5"
                    || node_name == "BallTmp2_6"
                    || node_name == "BallTmp2_7"
                    || node_name == "BallTmp2_8"
                    || node_name == "BallTmp2_9"
                    || node_name == "BallTmp2_10"
                    || node_name == "BallTmp2_11"
                    || node_name == "BallTmp2_12"
                    || node_name == "BallTmp2_13"
                    || node_name == "BallTmp2_14"
                    || node_name == "BallTmp3_1"
                    || node_name == "BallTmp3_2"
                    || node_name == "BallTmp3_3"
                    || node_name == "BallTmp3_4"
                    || node_name == "BallTmp3_5"
                    || node_name == "BallTmp3_6"
                    || node_name == "BallTmp3_7"
                    || node_name == "BallTmp3_8"
                    || node_name == "BallTmp3_9"
                    || node_name == "BallTmp3_10"
                    || node_name == "BallTmp3_11"
                    || node_name == "BallTmp3_12"
                    || node_name == "BallTmp3_13"
                    || node_name == "BallTmp3_14"
                    || node_name == "BallTmp4_1"
                    || node_name == "BallTmp4_2"
                    || node_name == "BallTmp4_3"
                    || node_name == "BallTmp4_4"
                    || node_name == "BallTmp4_5"
                    || node_name == "BallTmp4_6"
                    || node_name == "BallTmp4_7"
                    || node_name == "BallTmp4_8"
                    || node_name == "BallTmp4_9"
                    || node_name == "BallTmp4_10"
                    || node_name == "BallTmp4_11"
                    || node_name == "BallTmp4_12"
                    || node_name == "BallTmp4_13"
                    || node_name == "BallTmp4_14"
                    || node_name == "BallTmp5_1"
                    || node_name == "BallTmp5_2"
                    || node_name == "BallTmp5_3"
                    || node_name == "BallTmp5_4"
                    || node_name == "BallTmp5_5"
                    || node_name == "BallTmp5_6"
                    || node_name == "BallTmp5_7"
                    || node_name == "BallTmp5_8"
                    || node_name == "BallTmp5_9"
                    || node_name == "BallTmp5_10"
                    || node_name == "BallTmp5_11"
                    || node_name == "BallTmp5_12"
                    || node_name == "BallTmp5_13"
                    || node_name == "BallTmp5_14"
                {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: mat_handle,
                            transform,
                            // transform: transform.with_scale(vec3(1., 1., 1.)),
                            ..default()
                        })
                        .insert(RigidBody::Dynamic)
                        // .insert(Collider::trimesh_from_mesh(mesh).unwrap());
                        .insert(Collider::sphere(1.))
                        .insert(Name::new("BallTmp"));
                }
            }
        }
    }

    // commands.spawn((
    //     PbrBundle {
    //         mesh: meshes.add(Sphere::default()),
    //         material: materials.add(StandardMaterial::default()),
    //         transform: Transform::from_xyz(0., 25.0, 0.),
    //         ..default()
    //     },
    //     RigidBody::Dynamic,
    //     Collider::sphere(0.5),
    // ));
}
pub fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

#[derive(Component)]
pub struct BallMixer;

pub fn ball_mixer_rotate(mut q_mixer: Query<&mut Transform, With<BallMixer>>, time: Res<Time>) {
    for mut tr in &mut q_mixer {
        info!("zzz {:?}, {:?}", tr.rotation.y, time.elapsed_seconds());
        tr.rotation.y = time.elapsed_seconds();
    }
}
