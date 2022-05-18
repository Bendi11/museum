//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{prelude::{*, shape::Quad}, input::{mouse::{MouseMotion, MouseButtonInput}, keyboard::KeyboardInput, ElementState}, render::{mesh::{PrimitiveTopology, Indices}, render_resource::Face}, window::WindowMode};
use bevy_asset_loader::{AssetLoader, AssetCollection};
use smooth_bevy_cameras::{LookAngles, LookTransform, LookTransformBundle, LookTransformPlugin, Smoother};
use impacted::CollisionShape;

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
        .add_system_to_stage(CoreStage::PostUpdate, update_transforms)
        .add_system_set(SystemSet::on_enter(State::Main).with_system(setup))
        .add_system_set(SystemSet::on_update(State::Main)
            .with_system(input)
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
    windows.get_primary_mut().map(|window| {
        window.set_resizable(true);
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    });

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(StandardMaterial {
                base_color_texture: Some(textures.floor.clone()),
                metallic: 0.,
                perceptual_roughness: 0.9,
                ..Default::default()
        }),
        ..default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add({
            let mut material = StandardMaterial::from(Color::rgb(0.8, 0.7, 0.6));
            material.metallic = 0.2;
            material.perceptual_roughness = 0.5;
            material.reflectance = 0.9;
            material
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    }).insert(CollisionShape::new_rectangle(1., 1.));
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
    commands.spawn_bundle(LookTransformBundle {
        transform: LookTransform {
            eye: Vec3::new(1., 0.25, 1.),
            target: Vec3::X
        },
        smoother: Smoother::new(0.7),
    })
        .insert_bundle(PerspectiveCameraBundle {
            perspective_projection: PerspectiveProjection { fov: 1.22173, ..Default::default() },
            ..default()
        })
        .insert(CollisionShape::new_circle(PLAYER_RADIUS))
        .insert(Player);
}

fn input(
    mut mouse: EventReader<MouseMotion>,
    mut mb: EventReader<MouseButtonInput>,
    kb: Res<Input<KeyCode>>,
    mut players: Query<(&mut LookTransform, &CollisionShape), With<Player>>,
    mut windows: ResMut<Windows>,
    objects: Query<&CollisionShape, Without<Player>>,
) {
    const SENSITIVITY: f32 = 0.01;
    for (mut camera, shape) in players.iter_mut() {
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
            movement.z += MOVESPEED;
        } else if kb.pressed(KeyCode::S) {
            movement.z -= MOVESPEED;
        }

        if kb.pressed(KeyCode::D) {
            movement.x -= MOVESPEED;
        } else if kb.pressed(KeyCode::A) {
            movement.x += MOVESPEED;
        }
        
        camera.eye += movement.x * rot_x + movement.y * rot_y + movement.z * rot_z;
        let moved = shape.clone().with_transform(impacted::Transform::from_translation([camera.eye.x, camera.eye.z]));
        for collision in objects.iter().filter_map(|obj| moved.contact_with(obj)) {
            camera.eye.x += collision.normal[0] * collision.penetration;
            camera.eye.z += collision.normal[1] * collision.penetration;
        }
        camera.target = camera.eye + camera.radius() * angles.unit_vector();
    }

    for event in mb.iter() {
        if event.button == MouseButton::Left && event.state == ElementState::Released {
            windows.get_primary_mut().map(|window| {
                window.set_cursor_lock_mode(true);
                window.set_cursor_visibility(false);
            }); 
        }
    }

}

const PLAYER_RADIUS: f32 = 0.16;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct StaticGeom;

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
    floor: Handle<Image>,
}

fn update_transforms(mut shapes: Query<(&mut CollisionShape, &GlobalTransform), (Changed<GlobalTransform>, Without<StaticGeom>)>) {
    for (mut shape, transform) in shapes.iter_mut() {
        shape.set_transform(impacted::Transform::from_scale_angle_translation(
            [transform.scale.x, transform.scale.z],
            transform.rotation.to_axis_angle().1,
            [transform.translation.x, transform.translation.z]
        ));
    }
}
