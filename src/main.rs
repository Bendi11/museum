//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{prelude::*, input::{mouse::MouseMotion, keyboard::KeyboardInput, ElementState}};
use bevy_asset_loader::{AssetLoader, AssetCollection};
use smooth_bevy_cameras::{LookAngles, LookTransform, LookTransformBundle, LookTransformPlugin, Smoother};

fn main() {
    let mut app = App::new();
    AssetLoader::new(State::Loading)
        .continue_to_state(State::Main)
        .with_collection::<Textures>()
        .build(&mut app);
    app.add_state(State::Loading)
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(LookTransformPlugin)
        .insert_resource(MouseGrabbed(true))
        .add_system_set(SystemSet::on_enter(State::Main).with_system(setup))
        .add_system_set(SystemSet::on_update(State::Main)
            .with_system(input.before(kb_events))
            .with_system(kb_events)
        )
        .run();
}

fn setup(
    mut windows: ResMut<Windows>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    textures: Res<Textures>,
) {
    windows.iter_mut().for_each(|window| {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    });

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(textures.floor.clone().into()),//Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(1., 0.25, 1.),
        ..default()
    });
    /*commands.spawn_bundle(LookTransformBundle {
        transform: LookTransform {
            eye: Vec3::new(1., 0.25, 1.),
            target: Vec3::X
        },
        smoother: Smoother::new(0.7),
    })
    .insert_bundle(PerspectiveCameraBundle::default())
    .insert(Player);*/
}

fn kb_events(
    mut events: EventReader<KeyboardInput>,
    mut windows: ResMut<Windows>,
    mut grabbed: ResMut<MouseGrabbed>,
) {
    for event in events.iter() {
        if event.state == ElementState::Released && event.key_code == Some(KeyCode::Escape) {
            grabbed.0 = !grabbed.0;
            let window = windows.primary_mut();
            window.set_cursor_lock_mode(grabbed.0);
            window.set_cursor_visibility(!grabbed.0);
        } 
    }
}

fn input(
    mut mouse: EventReader<MouseMotion>,
    kb: Res<Input<KeyCode>>,
    mut cameras: Query<&mut LookTransform>,
) {
    const SENSITIVITY: f32 = 0.01;
    for mut camera in cameras.iter_mut() {
        let mut angles = LookAngles::from_vector(camera.look_direction().unwrap_or_default());
        for event in mouse.iter() {
            angles.add_pitch(-event.delta.y * SENSITIVITY);
            angles.add_yaw(-event.delta.x * SENSITIVITY);
        }
        camera.target = camera.eye + 1. * camera.radius() * angles.unit_vector();

        let yaw_rot = Quat::from_axis_angle(Vec3::Y, angles.get_yaw());
        let rot_x = yaw_rot * Vec3::X;
        let rot_y = yaw_rot * Vec3::Y;
        let rot_z = yaw_rot * Vec3::Z;
    
        const MOVESPEED: f32 = 0.02;
        let mut movement = Vec3::default();
        if kb.pressed(KeyCode::W) {
            movement.z += MOVESPEED 
        }
        if kb.pressed(KeyCode::A) {
            movement.x += MOVESPEED;
        }
        if kb.pressed(KeyCode::S) {
            movement.z -= MOVESPEED;
        }
        if kb.pressed(KeyCode::D) {
            movement.x -= MOVESPEED;
        }

        camera.eye += movement.x * rot_x + movement.y * rot_y + movement.z * rot_z;
        camera.target = camera.eye + camera.radius() * angles.unit_vector();
    }

}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct MouseGrabbed(bool);

/// Current scene state
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum State {
    Loading,
    Main
}

#[derive(AssetCollection)]
struct Textures {
    #[asset(path="floor.png")]
    floor: Handle<Image>
}
