use crate::{asset::AssetsPlugin, ffi::FfiPlugin, game::GamePlugin, ui::MyUiPlugin};
// use avian3d::prelude::*;
use bevy::prelude::*;
// use bevy_framepace::{FramepaceSettings, Limiter};
// use bevy_kira_audio::AudioPlugin;
use bevy_mod_picking::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_rapier3d::prelude::*;
use bevy_tweening::TweeningPlugin;
use states::{Loading, MyStates};
pub mod states;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(MyStates::Load(Loading::Loading));

        let mut config = RapierConfiguration::new(1.);
        // config.timestep_mode = TimestepMode::Fixed {
        //     dt: 1. / 60.,
        //     substeps: 1,
        // };
        app.add_plugins(PanOrbitCameraPlugin)
            // .add_plugins(AudioPlugin)
            .add_plugins(TweeningPlugin)
            .add_plugins(DefaultPickingPlugins)
            // .add_plugins(PhysicsPlugins::new(PostUpdate))
            // .add_plugins(bevy_framepace::FramepacePlugin)
            // .insert_resource(Time::new_with(Physics::fixed_hz(144.0)))
            // .insert_resource(Time::<Physics>::default().with_relative_speed(2.0))
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .insert_resource(config)
            // .insert_resource(SubstepCount(12))
            ;

        // app.add_systems(Startup, |mut settings: ResMut<FramepaceSettings>| {
        //     settings.limiter = Limiter::from_framerate(30.);
        // });

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
            .add_plugins(RapierDebugRenderPlugin::default())
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
