pub mod setup;
pub mod scene;

use bevy::{
    ecs::system::EntityCommands,
    input::{
        mouse::{MouseButtonInput, MouseMotion},
        ElementState,
    },
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_resource::{AddressMode, FilterMode, Face},
        texture::{CompressedImageFormats, ImageType}, render_phase::Draw,
    },
};
use smooth_bevy_cameras::{
    LookAngles, LookTransform, LookTransformBundle, LookTransformPlugin, Smoother,
};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Museum".to_owned(),
            present_mode: bevy::window::PresentMode::Fifo,
            ..Default::default()
        })
        .init_resource::<Textures>()
        //.insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(LookTransformPlugin)
        .add_startup_system(load_textures.before(setup::setup))
        .add_startup_system(setup::setup)
        .add_system(input)
        .add_system(setup::set_text_sizes)
        .run();
}


/// System running every update used to update the player position and camera angles, as well as 
/// check if the player is aiming at something that is interactable
fn input(
    mut mouse: EventReader<MouseMotion>,
    mut mb: EventReader<MouseButtonInput>,
    kb: Res<Input<KeyCode>>,
    mut players: Query<(&mut LookTransform, &mut Player, &mut Smoother)>,
    mut windows: ResMut<Windows>,
    objects: Query<&LineCollider>,
    tombstones: Query<&Readable, Without<InteractText>>,
    mut text: Query<&mut Visibility, Without<InteractText>>,
    mut interact_text: Query<(&mut Visibility, &mut Text), With<InteractText>>,
    time: Res<Time>,
) {
    let sensitivity = 0.1 * time.delta_seconds();
    for (mut camera, mut player, mut smoother) in players.iter_mut() {
        if let Some(txt) = player.viewed_text { 
            if kb.just_released(KeyCode::E) {
                text.get_mut(txt).unwrap().is_visible = false;
                player.viewed_text = None;
                camera.eye = player.old_eye;
                camera.target = player.old_target;
                *smoother = Smoother::new(0.7);
            }
            break
        }

        if let Some(dir) = camera.look_direction() {
            let mut angles = LookAngles::from_vector(dir);
            let yaw_rot = Quat::from_axis_angle(Vec3::Y, angles.get_yaw());
            let rot_x = yaw_rot * Vec3::X;
            let rot_y = yaw_rot * Vec3::Y;
            let rot_z = yaw_rot * Vec3::Z;

            for event in mouse.iter() {
                angles.add_pitch(-event.delta.y * sensitivity);
                angles.add_yaw(-event.delta.x * sensitivity);
            }

            if angles.get_pitch() == 0. {
                angles.set_pitch(0.01);
            }
            if angles.get_yaw() == 0. {
                angles.set_yaw(0.01);
            }

            let movespeed = 5.5 * time.delta_seconds();
            let mut movement = Vec2::default();
            
            if kb.pressed(KeyCode::W) {
                movement.y += movespeed;
            } else if kb.pressed(KeyCode::S) {
                movement.y -= movespeed;
            }

            if kb.pressed(KeyCode::D) {
                movement.x -= movespeed;
            } else if kb.pressed(KeyCode::A) {
                movement.x += movespeed;
            }
            let speed = movement.length();
            let movement_3d = Vec3::new(movement.x, 0., movement.y);

            let mut pos = camera.eye + (movement_3d.x * rot_x + movement_3d.y * rot_y + movement_3d.z * rot_z);
            let mut pos2d = Vec2::new(pos.x, pos.z);
            if movement != Vec2::default() {
                for object in objects.iter() {
                    let dot = ( ( (pos2d.x - object.from.x) * (object.to.x - object.from.x)) + ((pos2d.y - object.from.y) * (object.to.y - object.from.y))) / (object.len.powi(2));
                    let closest = Vec2::new(
                        object.from.x + (dot * (object.to.x - object.from.x)),
                        object.from.y + (dot * (object.to.y - object.from.y))
                    );

                    let d_from = closest.distance(object.from);
                    let d_to = closest.distance(object.to);

                    const ERROR: f32 = 0.5;
                    if d_from + d_to >= object.len - ERROR && d_from + d_to <= object.len + ERROR {
                        let collision_distance = closest.distance(pos2d);
                        if collision_distance <= PLAYER_RADIUS {
                            let unit = (closest - pos2d).normalize();
                            pos2d -= unit * speed;//collision_distance;
                        }
                    }
                }
            }
            pos.x = pos2d.x;
            pos.z = pos2d.y;
            
            angles.assert_not_looking_up();
            camera.eye = pos;
            camera.target = camera.eye + camera.radius() * angles.unit_vector();
            player.old_eye = camera.eye;
            player.old_target = camera.target;
            
            let (mut interact_visibility, _) = interact_text.get_single_mut().unwrap();
            interact_visibility.is_visible = false;
            //let target2d = Vec2::new(camera.target.x, camera.target.z);

            const INTERACT_RADIUS: f32 = 2.5;
            for readable in tombstones.iter() {
                if readable.point.distance(pos2d) < INTERACT_RADIUS {
                    if kb.just_released(KeyCode::E) {
                        text.get_mut(readable.text).unwrap().is_visible = true; 
                        player.viewed_text = Some(readable.text);
                        camera.eye = Vec3::new(4., 1., 5.5);
                        camera.target = Vec3::new(4., 1., 6.);
                        *smoother = Smoother::new(0.);
                    } else {
                        interact_visibility.is_visible = true;
                    }
                    break;
                }
            }
        }
    }

    for event in mb.iter() {
        if event.button == MouseButton::Left && event.state == ElementState::Released {
            windows.get_primary_mut().map(|window| {
                window.set_cursor_lock_mode(!window.cursor_locked());
                window.set_cursor_visibility(!window.cursor_visible());
            });
        }
    }
}

/// Various global state items 
pub struct GlobalState {
    pub interact_text: Entity,
}

const PLAYER_RADIUS: f32 = 0.32;

/// Marker component specifying that a collision object is controlled with the keyboard and mouse
#[derive(Component, Default)]
struct Player {
    /// Viewed text entity
    viewed_text: Option<Entity>,
    
    /// Used to restore state after exiting the read dialogue
    old_eye: Vec3,
    /// Used to restore state after exiting the read dialogue
    old_target: Vec3,
}


/// Collider component that specifies a wall has collision
#[derive(Component)]
pub struct LineCollider {
    /// Start of the line segment
    from: Vec2,
    /// End of the line segment
    to: Vec2,
    /// Cached length of the collider
    len: f32,
}

/// Tombtone with text about a museum piece 
#[derive(Component)]
pub struct Readable {
    /// Description of the museum piece containing the Text and Visible components
    pub text: Entity,
    /// Position of the interactable object
    pub point: Vec2,
}

#[derive(Component)]
pub struct InteractText;


#[derive(Default)]
pub struct Textures {
    birch_floor: Handle<Image>,
    oak_floor: Handle<Image>,
    flagstone_floor: Handle<Image>,
    tile_floor: Handle<Image>,
    red_tile_floor: Handle<Image>,
    linoleum_floor: Handle<Image>,
    blue_trimmed_wall: Handle<Image>,
    red_trimmed_wall: Handle<Image>,
    green_trimmed_wall: Handle<Image>,
    limestone_wall: Handle<Image>,
    eggshell_wall: Handle<Image>,
    barrier: Handle<Image>,
    concrete: Handle<Image>,
    ceiling_panel: Handle<Image>,
    wood_slat_roof: Handle<Image>,
    sky: Handle<Image>,
    tombstone: Handle<Image>,
    protest_image: Handle<Image>,
    art: Handle<Image>,
    job_iden: Handle<Image>,
}

/// Load all textures and set their repeat mode
fn load_textures(mut images: ResMut<Assets<Image>>, mut textures: ResMut<Textures>) {
    let mut load = |buf: &[u8]| {
        let mut image = Image::from_buffer(
            buf,
            ImageType::MimeType("image/png"),
            CompressedImageFormats::empty(),
            true,
        )
        .expect("Failed to load texture");
        image.sampler_descriptor.mag_filter = FilterMode::Nearest;
        image.sampler_descriptor.min_filter = FilterMode::Nearest;
        image.sampler_descriptor.address_mode_u = AddressMode::Repeat;
        image.sampler_descriptor.address_mode_v = AddressMode::Repeat;
        image.sampler_descriptor.address_mode_w = AddressMode::Repeat;

        images.add(image)
    };

    textures.birch_floor = load(include_bytes!("../assets/birch-floor.png"));
    textures.blue_trimmed_wall = load(include_bytes!("../assets/blue-trimmed-wall.png"));
    textures.red_trimmed_wall = load(include_bytes!("../assets/red-trimmed-wall.png"));
    textures.ceiling_panel = load(include_bytes!("../assets/ceiling-panel.png"));
    textures.sky = load(include_bytes!("../assets/sky.png"));
    textures.flagstone_floor = load(include_bytes!("../assets/flagstone-floor.png"));
    textures.limestone_wall = load(include_bytes!("../assets/limestone-wall.png"));
    textures.wood_slat_roof = load(include_bytes!("../assets/wood-slat-roof.png"));
    textures.eggshell_wall = load(include_bytes!("../assets/eggshell-wall.png"));
    textures.linoleum_floor = load(include_bytes!("../assets/linoleum-floor.png"));
    textures.concrete = load(include_bytes!("../assets/concrete.png"));
    textures.oak_floor = load(include_bytes!("../assets/oak-floor.png"));
    textures.tile_floor = load(include_bytes!("../assets/tile-floor.png"));
    textures.green_trimmed_wall = load(include_bytes!("../assets/green-trimmed-wall.png"));
    textures.red_tile_floor = load(include_bytes!("../assets/red-tile-floor.png"));
    textures.job_iden = load(include_bytes!("../assets/job-iden.png"));
    textures.barrier = load(include_bytes!("../assets/barrier.png"));
    textures.tombstone = load(include_bytes!("../assets/tombstone.png"));
    textures.protest_image = load(include_bytes!("../assets/protest-image.png"));
    textures.art = load(include_bytes!("../assets/art.png"));
}
