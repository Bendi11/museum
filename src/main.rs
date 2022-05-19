//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{
    ecs::system::EntityCommands,
    input::{
        mouse::{MouseButtonInput, MouseMotion},
        ElementState,
    },
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_resource::{AddressMode, FilterMode},
        texture::{CompressedImageFormats, ImageType},
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
        .add_startup_system(load_textures.before(setup))
        .add_startup_system(setup)
        .add_system(input)
        .run();
}

fn setup(
    mut windows: ResMut<Windows>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut light: ResMut<AmbientLight>,
    textures: Res<Textures>,
) {
    light.color = Color::WHITE;
    light.brightness = 1.2;
    windows.get_primary_mut().map(|window| {
        window.set_resizable(true);
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    });

    let wall = |p1: (f32, f32), p2: (f32, f32)| WallBuilder::new(p1, p2);

    let a = (0., 0.);
    let b = (0., -2.);
    let c = (-3., -2.);
    let d = (-3., 0.);
    let e = (-3., -0.5);
    let f = (-3., -1.5);
    let g = (-9., 0.);
    let h = (-9., -3.);
    let i = (-3., -3.);
    let j = (-8., -3.);
    let k = (-4., -3.);
    let l = (-2., -3.);
    let m = (-10., -3.);
    let n = (1., -3.);
    let o = (-13., -3.);
    let p = (-10., -5.);
    let q = (-2., -5.);
    //let r = (-10., )

    let tall = 1.2;

    SceneBuilder::new()
        .with_wall(
            wall(a, g)
                .with_texture(textures.blue_trimmed_wall.clone())
                .with_height(tall)
                .autotile_len(),
        )
        .with_wall(
            wall(a, b)
                .with_texture(textures.blue_trimmed_wall.clone())
                .autotile_len(),
        )
        .with_wall(
            wall(b, c)
                .with_texture(textures.blue_trimmed_wall.clone())
                .autotile_len(),
        )
        .with_wall(
            wall(d, e)
                .with_height(tall)
                .with_texture(textures.blue_trimmed_wall.clone())
                .autotile_len(),
        )
        .with_wall(
            wall(e, f)
                .with_height(0.2)
                .with_offset(tall - 0.2)
                .with_collision(false)
                .with_texture(textures.blue_trimmed_wall.clone()),
        )
        .with_wall(wall(g, h).with_height(tall))
        .with_wall(
            wall(i, f)
                .with_height(1.2)
                .with_texture(textures.blue_trimmed_wall.clone())
                .with_height(1.2)
                .autotile_len(),
        )
        .with_wall(wall(j, o).with_height(tall))
        .with_wall(wall(k, n).with_height(tall))
        
        .with_wall(wall(m, p).with_height(tall))
        .with_wall(wall(l, q).with_height(tall))

        .with_floor(
            FloorBuilder::new(a, h)
                .with_texture(textures.birch_floor.clone())
                .autotile(),
        )
        .with_floor(
            FloorBuilder::new(a, c)
                .with_offset(1.)
                .with_texture(textures.ceiling_panel.clone())
                .autotile(),
        )
        .with_floor(
            FloorBuilder::new(d, h)
                .with_offset(tall)
                .with_texture(textures.ceiling_panel.clone())
                .autotile(),
        )

        
        .with_floor(
            FloorBuilder::new((100., 100.), (-100., -100.))
                .with_offset(10.)
                .with_texture(textures.sky.clone())
        )
        .finish(&mut commands, &mut meshes, &mut materials);

    commands
        .spawn_bundle(LookTransformBundle {
            transform: LookTransform {
                eye: Vec3::new(-1.5, 0.4, -1.),
                target: Vec3::new(-1.5, 0.3, -2.),
            },
            smoother: Smoother::new(0.7),
        })
        .insert_bundle(PerspectiveCameraBundle {
            perspective_projection: PerspectiveProjection {
                fov: 1.22173,
                ..Default::default()
            },
            ..default()
        })
        .insert(Player);
}

fn input(
    mut mouse: EventReader<MouseMotion>,
    mut mb: EventReader<MouseButtonInput>,
    kb: Res<Input<KeyCode>>,
    mut players: Query<&mut LookTransform, With<Player>>,
    mut windows: ResMut<Windows>,
    objects: Query<&LineCollider, Without<Player>>,
) {
    const SENSITIVITY: f32 = 0.01;
    for mut camera in players.iter_mut() {
        if let Some(dir) = camera.look_direction() {
            let mut angles = LookAngles::from_vector(dir);
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

            let mut pos = camera.eye + (movement.x * rot_x + movement.y * rot_y + movement.z * rot_z);
            let mut pos2d = Vec2::new(pos.x, pos.z);
            if movement != Vec3::default() {
                for object in objects.iter() {
                    let dot = ( ( (pos2d.x - object.from.x) * (object.to.x - object.from.x)) + ((pos2d.y - object.from.y) * (object.to.y - object.from.y))) / (object.len.powi(2));
                    let closest = Vec2::new(
                        object.from.x + (dot * (object.to.x - object.from.x)),
                        object.from.y + (dot * (object.to.y - object.from.y))
                    );

                    let d_from = closest.distance(object.from);
                    let d_to = closest.distance(object.to);

                    const ERROR: f32 = 0.1;
                    if d_from + d_to >= object.len - ERROR && d_from + d_to <= object.len + ERROR {
                        let collision_distance = closest.distance(pos2d);
                        if collision_distance <= PLAYER_RADIUS {
                            let unit = (closest - pos2d).normalize();
                            pos2d -= unit * collision_distance;
                        }
                    }
                }
            }
            pos.x = pos2d.x;
            pos.z = pos2d.y;
            
            camera.eye = pos;
            camera.target = camera.eye + camera.radius() * angles.unit_vector();
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

const PLAYER_RADIUS: f32 = 0.16;

/// Marker component specifying that a collision object is controlled with the keyboard and mouse
#[derive(Component)]
struct Player;


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

/// Current scene state
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum State {
    Loading,
    Main,
}

/// Structure for constructing the map
struct SceneBuilder {
    /// All walls in the scene
    walls: Vec<WallBuilder>,
    /// All floors / ceilings in the scene
    floors: Vec<FloorBuilder>,
}

impl SceneBuilder {
    /// Create a new scene builder that contains all data needed to add walls and floors to the
    /// scene
    pub fn new() -> Self {
        Self {
            walls: vec![],
            floors: vec![],
        }
    }

    /// Add a wall to this scene
    pub fn with_wall(mut self, wall: WallBuilder) -> Self {
        self.walls.push(wall);
        self
    }

    /// Add a floor to this scene
    pub fn with_floor(mut self, floor: FloorBuilder) -> Self {
        self.floors.push(floor);
        self
    }

    /// Finish building the scene and add all walls and floors
    pub fn finish<'w, 's>(
        self,
        commands: &mut Commands<'w, 's>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        self.walls.iter().for_each(|wall| {
            wall.build(commands, meshes, materials);
        });
        self.floors.iter().for_each(|floor| {
            floor.build(commands, meshes, materials);
        });
    }
}

/// Structure containing all data needed for a wall
struct WallBuilder {
    /// If this wall has collision
    collision: bool,
    /// Height offset of the wall
    h_off: f32,
    /// Height of the wall
    height: f32,
    /// Texture of the wall
    texture: Option<Handle<Image>>,
    /// Color of the wall
    color: Color,
    /// Where the wall begins
    from: Vec2,
    /// Where the wall ends
    to: Vec2,
    /// How many times to repeat the applied texture in the X coordinate
    tiles_wide: f32,
    /// How many times to repeat the applied texture in the Y coordinate
    tiles_tall: f32,
}

/// A rectangle floor
struct FloorBuilder {
    /// Position that the rectangle begins from
    from: Vec2,
    /// Position that the rectangle ends at
    to: Vec2,
    /// Texture of the floor, if any
    texture: Option<Handle<Image>>,
    /// Color of the floor
    color: Color,
    /// Height offset of the floor
    height: f32,
    /// How many times to repeat the applied texture in the X coordinate
    tiles_wide: f32,
    /// How many times to repeat the applied texture in the Y coordinate
    tiles_tall: f32,
}

impl FloorBuilder {
    /// Create a new wall with height of 1, collision enabled, and untextured gray
    pub fn new(
        from: (impl Into<f32>, impl Into<f32>),
        to: (impl Into<f32>, impl Into<f32>),
    ) -> Self {
        let from = Vec2::new(from.0.into(), from.1.into());
        let to = Vec2::new(to.0.into(), to.1.into());
        Self {
            height: 0.,
            texture: None,
            color: Color::default() * 0.7,
            from,
            to,
            tiles_wide: 1.,
            tiles_tall: 1.,
        }
    }

    /// Set how many times to repeat the applied texture in X and Y coordinates
    pub fn with_tiles(mut self, width: f32, height: f32) -> Self {
        self.tiles_wide = width;
        self.tiles_tall = height;
        self
    }

    /// Calculate texture repetitions based on size of the floor
    pub fn autotile(mut self) -> Self {
        self.tiles_wide = (self.from - self.to).x.abs();
        self.tiles_tall = (self.from - self.to).y.abs();
        self
    }

    /// Add a texture to this wall
    pub fn with_texture(mut self, texture: Handle<Image>) -> Self {
        self.texture = Some(texture);
        self
    }

    /// Add a height offset from the ground
    pub fn with_offset(mut self, off: f32) -> Self {
        self.height = off;
        self
    }

    /// Set the color of this wall
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Add the wall to the scene
    pub fn build<'w, 's, 'a>(
        &self,
        commands: &'a mut Commands<'w, 's>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> EntityCommands<'w, 's, 'a> {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let verts = vec![
            [self.from.x, self.height, self.from.y], //bl
            [self.to.x, self.height, self.from.y],   //br
            [self.from.x, self.height, self.to.y],   //tl
            [self.to.x, self.height, self.to.y],     //tr
        ];

        let direction = self.from - self.to;
        let norm = Vec2::new(-direction.y, direction.x);

        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 0., 1.]; 4]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![
                [0., 0.],
                [self.tiles_wide, 0.],
                [0., self.tiles_tall],
                [self.tiles_wide, self.tiles_tall],
            ],
        );

        mesh.set_indices(Some(Indices::U16(vec![0, 2, 1, 3, 1, 2])));

        let mut command = commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh.clone()),
            material: materials.add(StandardMaterial {
                base_color: self.color,
                base_color_texture: self.texture.clone(),
                cull_mode: None,
                unlit: true,
                ..Default::default()
            }),
            ..default()
        });

        command
    }
}

impl WallBuilder {
    /// Create a new wall with height of 1, collision enabled, and untextured gray
    pub fn new(
        from: (impl Into<f32>, impl Into<f32>),
        to: (impl Into<f32>, impl Into<f32>),
    ) -> Self {
        let from = Vec2::new(from.0.into(), from.1.into());
        let to = Vec2::new(to.0.into(), to.1.into());
        let color = 0.4 + 0.2 * ((from - to).angle_between(Vec2::Y) / (std::f32::consts::PI * 2.));
        Self {
            collision: true,
            h_off: 0.,
            height: 1.,
            texture: None,
            color: Color::rgb(color, color, color),
            from,
            to,
            tiles_tall: 1.,
            tiles_wide: 1.,
        }
    }

    /// Set the amount of times to repeat the applied texture
    pub fn with_tiles(mut self, wide: f32, tall: f32) -> Self {
        self.tiles_wide = wide;
        self.tiles_tall = tall;
        self
    }

    /// Calculate how many times to repeat the tile based on height and length of the wall
    pub fn autotile(mut self) -> Self {
        self.tiles_wide = self.from.distance(self.to);
        self.tiles_tall = self.height;
        self
    }

    /// Calculate how many times to repeat the texture based on length, while always using one
    /// tile's height for the height
    pub fn autotile_len(mut self) -> Self {
        self.tiles_wide = self.from.distance(self.to);
        self
    }

    /// Add a texture to this wall
    pub fn with_texture(mut self, texture: Handle<Image>) -> Self {
        self.texture = Some(texture);
        let color = 0.6
            + 0.2 * ((self.from - self.to).angle_between(Vec2::Y) / (std::f32::consts::PI * 2.));

        self.color = Color::WHITE * color;
        self
    }

    /// Add a height offset from the ground
    pub fn with_offset(mut self, off: f32) -> Self {
        self.h_off = off;
        self
    }

    /// Set the height of this wall
    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set the color of this wall
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set the collision for this object
    pub fn with_collision(mut self, collision: bool) -> Self {
        self.collision = collision;
        self
    }

    /// Add the wall to the scene
    pub fn build<'w, 's, 'a>(
        &self,
        commands: &'a mut Commands<'w, 's>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> EntityCommands<'w, 's, 'a> {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let verts = vec![
            [self.from.x, self.h_off, self.from.y],               //bl
            [self.to.x, self.h_off, self.to.y],                   //br
            [self.from.x, self.height + self.h_off, self.from.y], //tl
            [self.to.x, self.height + self.h_off, self.to.y],     //tr
        ];

        let direction = self.from - self.to;
        let norm = Vec2::new(-direction.y, direction.x);

        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[norm.x, 0., norm.y]; 4]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![
                [self.tiles_wide, self.tiles_tall],
                [0., self.tiles_tall],
                [self.tiles_wide, 0.],
                [0., 0.],
            ],
        );

        mesh.set_indices(Some(Indices::U16(vec![0, 2, 1, 3, 1, 2])));

        let mut command = commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh.clone()),
            material: materials.add(StandardMaterial {
                base_color: self.color,
                base_color_texture: self.texture.clone(),
                cull_mode: None,
                unlit: true,
                ..Default::default()
            }),
            ..default()
        });

        if self.collision {
            command
                .insert(LineCollider {
                    from: self.from,
                    to: self.to,
                    len: self.from.distance(self.to),
                });
        }

        command
    }
}

#[derive(Default)]
struct Textures {
    birch_floor: Handle<Image>,
    blue_trimmed_wall: Handle<Image>,
    ceiling_panel: Handle<Image>,
    sky: Handle<Image>,
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
    textures.ceiling_panel = load(include_bytes!("../assets/ceiling-panel.png"));
    textures.sky = load(include_bytes!("../assets/sky.png"));
}
