use crate::{assets::AssetsPlugin, game::GamePlugin};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_tweening::TweeningPlugin;
use states::{Loading, MyStates};

pub mod states;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(MyStates::Load(Loading::Loading));

        app.add_plugins(PanOrbitCameraPlugin)
            .add_plugins(TweeningPlugin)
            .add_plugins(PhysicsPlugins::default());

        app.add_plugins(AssetsPlugin)
            .add_plugins(GamePlugin)
            .insert_resource(ClearColor(Color::srgb(0.53, 0.81, 0.92)));

        app.add_systems(Startup, camera_light_setup);

        #[cfg(feature = "inspector")]
        {
            use super::inspector::InspectorPlugin;
            app.add_plugins(InspectorPlugin)
                // .add_plugins(PhysicsDebugPlugin::default())
                ;
        }
    }
}

fn camera_light_setup(mut commands: Commands) {
    // commands.spawn((
    //     PointLightBundle {
    //         point_light: PointLight {
    //             shadows_enabled: true,
    //             ..default()
    //         },
    //         transform: Transform::from_xyz(0.0, 7.0, 0.0),
    //         ..default()
    //     },
    //     Name::new("point light"),
    // ));

    // directional 'sun' light
    // commands.spawn((
    //     DirectionalLightBundle {
    //         directional_light: DirectionalLight {
    //             illuminance: light_consts::lux::OVERCAST_DAY,
    //             shadows_enabled: true,
    //             ..default()
    //         },
    //         transform: Transform {
    //             translation: Vec3::new(0.0, 2.0, 0.0),
    //             rotation: Quat::from_rotation_x(-PI / 4.),
    //             ..default()
    //         },
    //         // The default cascade config is designed to handle large scenes.
    //         // As this example has a much smaller world, we can tighten the shadow
    //         // bounds for better visual quality.
    //         cascade_shadow_config: CascadeShadowConfigBuilder {
    //             first_cascade_far_bound: 4.0,
    //             maximum_distance: 10.0,
    //             ..default()
    //         }
    //         .into(),
    //         ..default()
    //     },
    //     Name::new("directional light"),
    // ));

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-0.0, 10.0, 10.0).looking_at(Vec3::Y * 2.0, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}
