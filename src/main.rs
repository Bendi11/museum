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
        texture::{CompressedImageFormats, ImageType},
    }, audio::AudioSink,
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
        .init_resource::<GlobalResources>()
        //.insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(LookTransformPlugin)
        .add_startup_system(load_resources.before(setup::setup))
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
    tombstones: Query<&Interactable, (Without<InteractText>, Without<ExitPrompt>)>,
    mut texts: Query<&mut Visibility, (Without<InteractText>, Without<ExitPrompt>)>,
    mut interact_text: Query<(&mut Visibility, &mut Text), (With<InteractText>, Without<ExitPrompt>)>,
    time: Res<Time>,
    sinks: Res<Assets<AudioSink>>,
    audio: Res<Audio>,
    mut exit_prompt: Query<&mut Visibility, With<ExitPrompt>>,
) {
    let sensitivity = 0.1 * time.delta_seconds();
    for (mut camera, mut player, mut smoother) in players.iter_mut() {
        if let Some(txt) = player.viewed_text {
            if kb.just_released(KeyCode::E) {
                exit_prompt
                    .iter_mut()
                    .for_each(|mut prompt| prompt.is_visible = false);
                texts.get_mut(txt).unwrap().is_visible = false;
                player.viewed_text = None;
                camera.eye = player.old_eye;
                camera.target = player.old_target;
                *smoother = Smoother::new(0.7);
            } else {
                exit_prompt
                    .iter_mut()
                    .for_each(|mut prompt| prompt.is_visible = true);
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
            } 
            if kb.pressed(KeyCode::S) {
                movement.y -= movespeed;
            }

            if kb.pressed(KeyCode::D) {
                movement.x -= movespeed;
            }
            if kb.pressed(KeyCode::A) {
                movement.x += movespeed;
            }
            let speed = movement.length();
            let movement_3d = Vec3::new(movement.x, 0., movement.y);

            let mut pos = camera.eye + (movement_3d.x * rot_x + movement_3d.y * rot_y + movement_3d.z * rot_z);
            let mut pos2d = Vec2::new(pos.x, pos.z);
            const BOB_AMOUNT: f32 = 0.05;
            const BOB_SPEED: f32 = 0.2;

            if movement != Vec2::default() {
                match player.up {
                    true => match player.cam_height >= BOB_AMOUNT {
                        true => player.up = false,
                        false => player.cam_height += BOB_SPEED * time.delta_seconds(),
                    },
                    false => match player.cam_height <= -BOB_AMOUNT {
                        true => player.up = true,
                        false => player.cam_height -= BOB_SPEED * time.delta_seconds(),
                    }
                }
                
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
                            pos2d -= unit * speed;
                        }
                    }
                }
            } else {
                if player.cam_height.abs() >= 0.01 {
                    player.cam_height -= BOB_SPEED * player.cam_height.signum() * time.delta_seconds();
                }
            }

            let (mut interact_visibility, mut interact_text) = interact_text.get_single_mut().unwrap();
            
            pos.x = pos2d.x;
            pos.y = PLAYER_HEIGHT + player.cam_height;
            pos.z = pos2d.y;
            
            angles.assert_not_looking_up();
            camera.eye = pos;
            camera.target = camera.eye + camera.radius() * angles.unit_vector();
            player.old_eye = camera.eye;
            player.old_target = camera.target;
            
            interact_visibility.is_visible = false;

            for interactable in tombstones.iter() {
                if interactable.point.distance(pos2d) < interactable.radius {
                    if kb.just_released(KeyCode::E) {
                        match &interactable.action {
                            InteractableAction::Tombstone { text } => {
                                texts.get_mut(*text).unwrap().is_visible = true; 
                                player.viewed_text = Some(*text);
                                camera.eye = Vec3::new(4., 1., 5.5);
                                camera.target = Vec3::new(4., 1., 6.);
                                *smoother = Smoother::new(0.);
                            },
                            InteractableAction::Audio { source } => {
                                if let Some((sink, playing_source)) = &player.playing_audio {
                                    if playing_source == source {
                                        sinks.get(sink).map(|sink| match sink.is_paused() {
                                            true => sink.play(),
                                            false => sink.pause(),
                                        });
                                        break;
                                    }
                                    sinks.get(sink).map(AudioSink::stop);
                                }
                                let sink = audio.play(source.clone());        
                                let sink = sinks.get_handle(sink);
                                player.playing_audio = Some((sink, source.clone()));
                            },
                            _ => (),
                        }
                    } else {
                        interact_visibility.is_visible = true;
                        interact_text.sections[0].value = match &interactable.action {
                            InteractableAction::Tombstone { .. } => "[e] Read",
                            InteractableAction::Audio { source } => match player.playing_audio
                                .as_ref()
                                .map_or(false, |(sink, src)| src.id == source.id && !sinks.get(sink).unwrap().is_paused()) {
                                false => "[e] Play Audio",
                                true => "[e] Pause Audio",
                            },
                            InteractableAction::Tooltip(tip) => tip,
                        }.to_owned();
                    }
                    break;
                }
            }
        }
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

/// Various global state items 
pub struct GlobalState {
    pub interact_text: Entity,
}

const PLAYER_RADIUS: f32 = 0.32;
const PLAYER_HEIGHT: f32 = 1.6;

/// Marker component specifying that a collision object is controlled with the keyboard and mouse
#[derive(Component, Default)]
struct Player {
    /// Viewed text entity
    viewed_text: Option<Entity>,
    /// The currently playing audio track
    playing_audio: Option<(Handle<AudioSink>, Handle<AudioSource>)>,
    
    /// Used to restore state after exiting the read dialogue
    old_eye: Vec3,
    /// Used to restore state after exiting the read dialogue
    old_target: Vec3,
    /// Camera offset used to add head bobbing
    cam_height: f32,
    /// If head bob is travelling up or down
    up: bool,
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

/// Action that can be taken when interacting with something
#[derive(Clone)]
pub enum InteractableAction {
    Tombstone {
        text: Entity,
    },
    Audio {
        source: Handle<AudioSource>,
    },
    Tooltip(&'static str),
}

/// Any interactable object
#[derive(Component)]
pub struct Interactable {
    pub point: Vec2,
    pub radius: f32,
    pub action: InteractableAction,
}

#[derive(Component)]
pub struct InteractText;

#[derive(Component)]
pub struct ExitPrompt;

#[derive(Default)]
pub struct GlobalResources {
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
    mlk: Handle<Image>,
    mlk_speech: Handle<AudioSource>,
    headphones: Handle<Image>,
    teacher_shirt: Handle<Image>,
    velvet: Handle<Image>,
    news: Handle<Image>,
    josh_exit: Handle<Image>,
    matt_exit: Handle<Image>,
    ben_exit: Handle<Image>,
    intro_wall: Handle<Image>,
    other_intro_wall: Handle<Image>,
    reagan: Handle<Image>,
    reagan_audio: Handle<AudioSource>,
    cesar_chavez: Handle<Image>,
    protestors: Handle<Image>,
    modern_protestors: Handle<Image>,
    works_cited: Handle<Image>,
    job_iden: Handle<Image>,
}

/// Load all textures and set their repeat mode
fn load_resources(
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
    mut resources: ResMut<GlobalResources>
) {
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

    resources.birch_floor = load(include_bytes!("../assets/birch-floor.png"));
    resources.blue_trimmed_wall = load(include_bytes!("../assets/blue-trimmed-wall.png"));
    resources.red_trimmed_wall = load(include_bytes!("../assets/red-trimmed-wall.png"));
    resources.ceiling_panel = load(include_bytes!("../assets/ceiling-panel.png"));
    resources.sky = load(include_bytes!("../assets/sky.png"));
    resources.flagstone_floor = load(include_bytes!("../assets/flagstone-floor.png"));
    resources.limestone_wall = load(include_bytes!("../assets/limestone-wall.png"));
    resources.wood_slat_roof = load(include_bytes!("../assets/wood-slat-roof.png"));
    resources.eggshell_wall = load(include_bytes!("../assets/eggshell-wall.png"));
    resources.linoleum_floor = load(include_bytes!("../assets/linoleum-floor.png"));
    resources.concrete = load(include_bytes!("../assets/concrete.png"));
    resources.oak_floor = load(include_bytes!("../assets/oak-floor.png"));
    resources.tile_floor = load(include_bytes!("../assets/tile-floor.png"));
    resources.green_trimmed_wall = load(include_bytes!("../assets/green-trimmed-wall.png"));
    resources.red_tile_floor = load(include_bytes!("../assets/red-tile-floor.png"));
    resources.job_iden = load(include_bytes!("../assets/job-iden.png"));
    resources.barrier = load(include_bytes!("../assets/barrier.png"));
    resources.tombstone = load(include_bytes!("../assets/tombstone.png"));
    resources.protest_image = load(include_bytes!("../assets/protest-image.png"));
    resources.art = load(include_bytes!("../assets/art.png"));
    resources.mlk = load(include_bytes!("../assets/martin-luther-king-jr.png"));
    resources.headphones = load(include_bytes!("../assets/headphones.png"));
    resources.teacher_shirt = load(include_bytes!("../assets/starbucks.png"));
    resources.velvet = load(include_bytes!("../assets/velvet.png"));
    resources.news = load(include_bytes!("../assets/news.png"));
    resources.josh_exit = load(include_bytes!("../assets/josh-exit.png"));
    resources.matt_exit = load(include_bytes!("../assets/matt-exit.png"));
    resources.ben_exit = load(include_bytes!("../assets/ben-exit.png"));
    resources.intro_wall = load(include_bytes!("../assets/intro-wall.png"));
    resources.reagan = load(include_bytes!("../assets/reagan.png"));
    resources.cesar_chavez = load(include_bytes!("../assets/cesar.png"));
    resources.other_intro_wall = load(include_bytes!("../assets/other-intro.png"));
    resources.protestors = load(include_bytes!("../assets/protestors.png"));
    resources.works_cited = load(include_bytes!("../assets/works-cited.png"));
    resources.modern_protestors = load(include_bytes!("../assets/modern-protestors.png"));
    
    resources.mlk_speech = asset_server.load("sound/mlk-speech.ogg");
    resources.reagan_audio = asset_server.load("sound/reagan.ogg");
}
