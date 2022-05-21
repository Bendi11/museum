use super::*;


/// Structure for constructing the map
pub struct SceneBuilder {
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

pub const WALL_HEIGHT: f32 = 3.7;

/// Structure containing all data needed for a wall
pub struct WallBuilder {
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
    /// Wether or not to enable transparency
    transparent: bool,
    /// What side to cull, optional
    cull: Option<Face>,
    /// What text to display as a tombstone
    txt: Option<Entity>,
}

/// A rectangle floor
pub struct FloorBuilder {
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
    /// What side to cull while rendering
    cull: Option<Face>,
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
            cull: None,
        }
    }
    
    /// Set what side to cull while rendering
    pub fn with_cull(mut self, cull: Face) -> Self {
        self.cull = Some(cull);
        self
    }
    
    /// Set the brightness of this floor's texture
    pub fn with_brightness(self, brightness: f32) -> Self {
        self.with_color(Color::rgb(brightness, brightness, brightness))
    }

    /// Set how many times to repeat the applied texture in X and Y coordinates
    pub fn with_tiles(mut self, width: f32, height: f32) -> Self {
        self.tiles_wide = width;
        self.tiles_tall = height;
        self
    }

    /// Calculate texture repetitions based on size of the floor
    pub fn autotile(mut self) -> Self {
        self.tiles_wide = (self.from - self.to).x.abs() / 2.;
        self.tiles_tall = (self.from - self.to).y.abs() / 2.;
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

        let command = commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh.clone()),
            material: materials.add(StandardMaterial {
                base_color: self.color,
                base_color_texture: self.texture.clone(),
                cull_mode: self.cull,
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
        let color = 0.4 + 0.2 * ((from - to).angle_between(Vec2::Y) / (std::f32::consts::PI * 2.)).abs();
        Self {
            collision: true,
            h_off: 0.,
            height: WALL_HEIGHT,
            texture: None,
            color: Color::rgb(color, color, color),
            from,
            to,
            tiles_tall: 1.,
            tiles_wide: 1.,
            transparent: false,
            cull: None,
            txt: None,
        }
    }
    
    /// Add a text entity to display for this wall when interacted
    pub fn with_text(mut self, txt: Entity) -> Self {
        self.txt = Some(txt);
        self
    }
    
    /// Set the cull mode of this wall
    pub fn with_cull(mut self, cull: Face) -> Self {
        self.cull = Some(cull);
        self
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
        self.tiles_wide = self.from.distance(self.to) / 2.;
        self
    }

    /// Add a texture to this wall
    pub fn with_texture(mut self, texture: Handle<Image>) -> Self {
        self.texture = Some(texture);
        let color = 0.6
            + 0.2 * ((self.from - self.to).angle_between(Vec2::Y) / (std::f32::consts::PI * 2.)).abs();

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
    
    /// Enable or disable transparency for this wall's texture
    pub fn with_transparency(mut self, transparency: bool) -> Self {
        self.transparent = transparency;
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
                cull_mode: self.cull,
                unlit: true,
                alpha_mode: if self.transparent { AlphaMode::Blend } else { AlphaMode::Opaque },
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
        if let Some(text) = self.txt {
            command
                .insert(Readable {
                    text,
                    point: (self.from + self.to) / 2.
                });
        }

        command
    }
}

