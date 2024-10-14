use crate::{assets::AssetsPlugin, ffi::FfiPlugin, game::GamePlugin, ui::MyUiPlugin};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use bevy_mod_picking::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_tweening::TweeningPlugin;
use states::{Loading, MyStates};

pub mod states;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(MyStates::Load(Loading::Loading));

        app.add_plugins(PanOrbitCameraPlugin)
            .add_plugins(AudioPlugin)
            .add_plugins(TweeningPlugin)
            .add_plugins(DefaultPickingPlugins)
            .add_plugins(PhysicsPlugins::default());

        app.add_plugins(AssetsPlugin)
            .add_plugins(GamePlugin)
            .add_plugins(MyUiPlugin)
            .add_plugins(FfiPlugin)
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
    let mut panorbit = PanOrbitCamera::default();
    // panorbit.yaw_upper_limit = Some(20.);
    panorbit.zoom_upper_limit = Some(20.);
    panorbit.zoom_lower_limit = Some(4.);
    // panorbit.modifier_pan = Some(KeyCode::ShiftLeft);
    panorbit.pan_sensitivity = 0.;

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(8.0, 5.0, 0.0).looking_at(Vec3::Y * 2.0, Vec3::Y),
            ..default()
        },
        panorbit,
    ));
}
