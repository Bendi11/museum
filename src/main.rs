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
    let b = (8., 0.);
    let c = (14., 0.);
    let d = (8., 1.);
    let e = (8., 5.);
    let f = (0., 6.);
    let g = (1., 6.);
    let h = (7., 6.);
    let i = (8., 6.);
    //let j = (13., 6.);
    let k = (14., 6.);
    let l = (15., 6.);
    let m = (21., 6.);
    let n = (7., 7.);
    let o = (15., 7.);
    let p = (7., 9.);
    let q = (15., 9.);
    let r = (7., 13.);
    let s = (15., 13.);
    let t = (7., 17.);
    let u = (15., 17.);
    let v = (7., 21.);
    let w = (15., 21.);
    let x = (7., 22.);
    let y = (15., 22.);
    let z = (1., 24.);
    let aa = (7., 24.);
    let ab = (9., 24.);
    let ac = (13., 24.);
    let ad = (15., 24.);
    let ae = (21., 24.);
    let af = (1., 25.);
    let ag = (9., 25.);
    let ah = (13., 25.);
    let ai = (21., 25.);
    let aj = (7., 28.);
    let ak = (15., 28.);
    let al = (1., 31.);
    let am = (7., 31.);
    let an = (8., 31.);
    let ao = (14., 31.);
    let ap = (15., 31.);
    let aq = (21., 31.);
    let ar = (16., 33.);
    let r#as = (23., 33.);
    let at = (31., 33.);
    let au = (19., 36.);
    let av = (23., 36.);
    let aw = (27., 36.);
    let ax = (33., 37.);
    let ay = (16., 39.);
    let az = (19., 39.);
    let ba = (23., 39.);
    let bb = (27., 39.);
    let bc = (29., 39.);
    let bd = (23., 45.);
    let be = (33., 45.);
    let bf = (9., 6.);
    let bg = (7.5, 13.);
    let bh = (14.5, 13.);
    let bi = (7.5, 17.);
    let bj = (14.5, 17.);
    let bk = (6.5, 13.);
    let bl = (15.5, 13.);
    let bm = (6.5, 17.);
    let bn = (15.5, 17.);
    let bo = (9., 0.);
    let bp = (9., 1.);
    let bq = (9., 5.);
    let br = (6.5, 6.);
    let bs = (15.5, 6.);
    let bt = (6.5, 9.);
    let bu = (15.5, 9.);
    let bv = (6.5, 21.);
    let bw = (15.5, 21.);
    let bx = (6.5, 24.);
    let by = (15.5, 24.);
    let bz = (8., 28.);
    let ca = (14., 28.);
    let cb = (7., 25.);
    let cc = (15., 25.);
    let cd = (33., 39.);
    let ce = (23., 39.5);
    let cf = (29., 39.5);

    const CEILING_OFFSET: f32 = 0.4;

    SceneBuilder::new()
        .with_wall(wall(a, f)
            .with_texture(textures.blue_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(a, c)
            .with_texture(textures.blue_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(b, d)
            .with_texture(textures.blue_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(c, k)
            .with_texture(textures.blue_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(d, bp)
            .with_texture(textures.blue_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(e, i)
            .with_texture(textures.blue_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(e, bq)
            .with_texture(textures.blue_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(f, i)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.eggshell_wall.clone())
        )
        .with_wall(wall(g, z)
            .with_texture(textures.red_trimmed_wall.clone())
            .autotile_len()
            .with_cull(Face::Back)
        )
        .with_wall(wall(k, bf)
            .with_collision(false)
            .with_offset(WALL_HEIGHT - CEILING_OFFSET)
            .with_height(CEILING_OFFSET * 2.)
            .with_texture(textures.concrete.clone())
            .with_tiles(1., 0.2)
        )
        .with_wall(wall(k, o)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.concrete.clone())
            .with_tiles(0.5, 1.)
            .with_cull(Face::Front)
        )
        .with_wall(wall(m, ae)
            .with_texture(textures.red_trimmed_wall.clone())
            .autotile_len()
            .with_cull(Face::Front)
        )
        .with_wall(wall(n, p)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.concrete.clone())
            .with_cull(Face::Back)
            .with_tiles(0.5, 1.)
        )
        .with_wall(wall(n, bf)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.concrete.clone())
            .with_tiles(0.5, 1.)
            .with_cull(Face::Front)
        )
        .with_wall(wall(o, q)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.concrete.clone())
            .with_cull(Face::Front)
            .with_tiles(0.5, 1.)
        )
        .with_wall(wall(p, v)
            .with_collision(false)
            .with_offset(WALL_HEIGHT)
            .with_height(CEILING_OFFSET)
            .with_texture(textures.concrete.clone())
            .with_tiles(4., 0.2)
            .with_cull(Face::Back)
        )
        .with_wall(wall(p, bt)
            .with_texture(textures.concrete.clone())
            .with_tiles(0.3, 1.)
            .with_cull(Face::Back)
        )
        .with_wall(wall(q, w)
            .with_collision(false)
            .with_offset(WALL_HEIGHT)
            .with_height(CEILING_OFFSET)
            .with_texture(textures.concrete.clone())
            .with_tiles(4., 0.2)
            .with_cull(Face::Front)
        )
        .with_wall(wall(q, bu)
            .with_texture(textures.concrete.clone())
            .with_tiles(0.3, 1.)
            .with_cull(Face::Front)
        )
        .with_wall(wall(v, x)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.concrete.clone())
            .with_cull(Face::Back)
            .with_tiles(0.3, 1.)

        )
        .with_wall(wall(v, bv)
            .with_texture(textures.concrete.clone())
            .with_tiles(0.3, 1.)
            .with_cull(Face::Front)
        )
        .with_wall(wall(w, y)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.concrete.clone())
            .with_tiles(0.3, 1.)
            .with_cull(Face::Front)

        )
        .with_wall(wall(w, bw)
            .with_texture(textures.concrete.clone())
            .with_tiles(0.3, 1.)
            .with_cull(Face::Back)
        )
        .with_wall(wall(x, ab)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.concrete.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(y, ac)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.concrete.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(z, bx)
            .with_height(WALL_HEIGHT)
            .with_texture(textures.red_trimmed_wall.clone())
            .autotile_len()
            .with_cull(Face::Back)
        )
        .with_wall(wall(z, ae)
            .with_collision(false)
            .with_offset(WALL_HEIGHT + CEILING_OFFSET)
            .with_height(CEILING_OFFSET)
            .with_texture(textures.concrete.clone())
            .with_tiles(10., 0.2)
            .with_cull(Face::Back)
        )
        .with_wall(wall(ab, ag)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.eggshell_wall.clone())
            .with_tiles(0.2, 1.)
            .with_cull(Face::Back)
        )
        .with_wall(wall(ac, ah)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.eggshell_wall.clone())
            .with_tiles(0.2, 1.)
            .with_cull(Face::Front)
        )
        .with_wall(wall(af, ag)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.blue_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(af, al)
            .with_texture(textures.blue_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(ah, ai)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.blue_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(ai, aq)
            .with_texture(textures.blue_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(aj, am)
            .with_texture(textures.blue_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(aj, bz)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.green_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(aj, cb)
            .with_height(CEILING_OFFSET)
            .with_offset(WALL_HEIGHT)
            .with_collision(false)
            .with_cull(Face::Front)
        )
        .with_wall(wall(ak, ap)
            .with_texture(textures.blue_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(ak, ca)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.green_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(ak, cc)
            .with_height(CEILING_OFFSET)
            .with_offset(WALL_HEIGHT)
            .with_collision(false)
            .with_cull(Face::Back)
        )
        .with_wall(wall(al, am)
            .with_texture(textures.blue_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(an, ay)
            .with_texture(textures.eggshell_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(an, ao)
            .with_height(CEILING_OFFSET)
            .with_offset(WALL_HEIGHT)
            .with_collision(false)
            .with_cull(Face::Back)
        )
        .with_wall(wall(ao, ar)
            .with_texture(textures.eggshell_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(ap, aq)
            .with_texture(textures.blue_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(ar, at)
            .with_texture(textures.eggshell_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(r#as, av)
            .with_height(WALL_HEIGHT - 0.9)
        )
        .with_wall(wall(at, ax)
            .with_texture(textures.eggshell_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(au, az)
            .with_height(WALL_HEIGHT - 0.9)
        )
        .with_wall(wall(aw, bb)
            .with_height(WALL_HEIGHT - 0.9)
        )
        .with_wall(wall(ax, be)
            .with_texture(textures.red_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(ay, bc)
            .with_texture(textures.eggshell_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(bc, cf)
            .with_texture(textures.red_trimmed_wall.clone())
            .with_tiles(0.2, 1.)
            .with_cull(Face::Back)
        )
        .with_wall(wall(bc, cd)
            .with_height(CEILING_OFFSET)
            .with_offset(WALL_HEIGHT - CEILING_OFFSET)
            .with_collision(false)
            .with_texture(textures.eggshell_wall.clone())
            .with_tiles(2., 0.2)
            .with_cull(Face::Back)
        )
        .with_wall(wall(bd, be)
            .with_texture(textures.red_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(bf, bq)
            .with_texture(textures.blue_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(bg, bk)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.concrete.clone())
            .with_tiles(0.3, 1.)
            .with_cull(Face::Front)
        )
        .with_wall(wall(bg, bi)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.concrete.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(bh, bl)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.concrete.clone())
            .with_tiles(0.3, 1.)
            .with_cull(Face::Back)
        )
        .with_wall(wall(bh, bj)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.concrete.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(bi, bm)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.concrete.clone())
            .with_tiles(0.3, 1.)
            .with_cull(Face::Back)
        )
        .with_wall(wall(bj, bn)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.concrete.clone())
            .with_tiles(0.3, 1.)
            .with_cull(Face::Front)
        )
        .with_wall(wall(bk, bm)
            .with_texture(textures.red_trimmed_wall.clone())
            .autotile_len()
            .with_cull(Face::Front)
        )
        .with_wall(wall(bl, bn)
            .with_texture(textures.red_trimmed_wall.clone())
            .autotile_len()
            .with_cull(Face::Back)
        )
        .with_wall(wall(bo, bp)
            .with_texture(textures.blue_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(br, bt)
            .with_texture(textures.red_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(bs, bu)
            .with_texture(textures.red_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(bs, m)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.eggshell_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(bv, bx)
            .with_texture(textures.red_trimmed_wall.clone())
            .autotile_len()
            .with_cull(Face::Front)
        )
        .with_wall(wall(bw, by)
            .with_texture(textures.red_trimmed_wall.clone())
            .autotile_len()
            .with_cull(Face::Back)
        )
        .with_wall(wall(by, ae)
            .with_height(WALL_HEIGHT)
            .with_texture(textures.red_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(bz, an)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.green_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(ca, ao)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(textures.green_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(ce, bd)
            .with_texture(textures.red_trimmed_wall.clone())
            .autotile_len()
            .with_cull(Face::Back)
        )

        .with_wall(wall(ce, cf)
            .with_texture(textures.red_trimmed_wall.clone())
            .autotile_len()
            .with_cull(Face::Front)
        )
        
        .with_floor(
            FloorBuilder::new(ba, be)
                .with_offset(WALL_HEIGHT - CEILING_OFFSET)
                .with_texture(textures.ceiling_panel.clone())
                .autotile()
                .with_cull(Face::Front)
        )
        .with_floor(
            FloorBuilder::new(ad, am)
                .with_offset(WALL_HEIGHT + CEILING_OFFSET)
                .with_texture(textures.ceiling_panel.clone())
                .autotile()
                .with_cull(Face::Back)
        )
        .with_floor(
            FloorBuilder::new(af, am)
                .with_offset(WALL_HEIGHT)
                .with_texture(textures.ceiling_panel.clone())
                .autotile()
                .with_cull(Face::Front)
        )
        .with_floor(
            FloorBuilder::new(ai, ap)
                .with_offset(WALL_HEIGHT)
                .with_texture(textures.ceiling_panel.clone())
                .autotile()
                .with_cull(Face::Back)
        )
        .with_floor(
            FloorBuilder::new(a, k)
                .with_offset(WALL_HEIGHT - CEILING_OFFSET)
                .with_texture(textures.ceiling_panel.clone())
                .with_brightness(0.8)
                .autotile()
                .with_cull(Face::Front)
        )
        .with_floor(
            FloorBuilder::new(g, aa)
                .with_offset(WALL_HEIGHT)
                .with_texture(textures.ceiling_panel.clone())
                .autotile()
                .with_cull(Face::Front)
        )
        .with_floor(
            FloorBuilder::new(l, ae)
                .with_offset(WALL_HEIGHT)
                .with_texture(textures.ceiling_panel.clone())
                .autotile()
                .with_cull(Face::Front)
        )

        .with_floor(
            FloorBuilder::new(an, cd)
                .with_texture(textures.wood_slat_roof.clone())
                .with_brightness(0.3)
                .with_offset(WALL_HEIGHT)
                .autotile()
                .with_cull(Face::Front)
        )

        .with_floor(
            FloorBuilder::new(ba, be)
                .with_texture(textures.oak_floor.clone())
                .with_brightness(0.5)
                .autotile()
                .with_cull(Face::Back)
        )
        
        .with_floor(
            FloorBuilder::new(af, aq)
                .with_texture(textures.red_tile_floor.clone())
                .autotile()
                .with_cull(Face::Back)
        )
        .with_floor(
            FloorBuilder::new(an, cd)
                .with_texture(textures.linoleum_floor.clone())
                .with_brightness(0.4)
                .autotile()
                .with_cull(Face::Back)
        )
        .with_floor(
            FloorBuilder::new(h, ad)
                .with_texture(textures.flagstone_floor.clone())
                .with_offset(0.0001)
                .autotile()
                .with_cull(Face::Back)
        )
        .with_floor(
            FloorBuilder::new(m, z)
                .with_texture(textures.oak_floor.clone())
                .autotile()
                .with_cull(Face::Front)
        )
        .with_floor(
            FloorBuilder::new(a, k)
                .with_texture(textures.birch_floor.clone())
                .with_brightness(0.6)
                .autotile()
                .with_cull(Face::Back)
        )
        .with_floor(
            FloorBuilder::new(aa, ap)
                .with_texture(textures.tile_floor.clone())
                .with_brightness(0.7)
                .with_offset(0.001)
                .autotile()
                .with_cull(Face::Back)
        )
        
        .with_floor(
            FloorBuilder::new((100., 100.), (-100., -100.))
                .with_offset(10.)
                .with_texture(textures.sky.clone())
                .with_cull(Face::Front)
        )

        .with_wall(
            wall((bj.0 - 0.5, bj.1), (bj.0, bj.1 - 0.5))
            .with_texture(textures.job_iden.clone())
            .with_height(2.1)
            .with_transparency(true)
        )
        .finish(&mut commands, &mut meshes, &mut materials);
    
    //Spawn the player
    commands
        .spawn_bundle(LookTransformBundle {
            transform: LookTransform {
                eye: Vec3::new(2., 1.25, 1.5),
            target: Vec3::new(-2., 1.25, 1.5),
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
            let yaw_rot = Quat::from_axis_angle(Vec3::Y, angles.get_yaw());
            let rot_x = yaw_rot * Vec3::X;
            let rot_y = yaw_rot * Vec3::Y;
            let rot_z = yaw_rot * Vec3::Z;

            for event in mouse.iter() {
                angles.add_pitch(-event.delta.y * SENSITIVITY);
                angles.add_yaw(-event.delta.x * SENSITIVITY);
            }

            if angles.get_pitch() == 0. {
                angles.set_pitch(0.01);
            }
            if angles.get_yaw() == 0. {
                angles.set_yaw(0.01);
            }

            const MOVESPEED: f32 = 0.1;
            let mut movement = Vec2::default();
            
            if kb.pressed(KeyCode::W) {
                movement.y += MOVESPEED;
            } else if kb.pressed(KeyCode::S) {
                movement.y -= MOVESPEED;
            }

            if kb.pressed(KeyCode::D) {
                movement.x -= MOVESPEED;
            } else if kb.pressed(KeyCode::A) {
                movement.x += MOVESPEED;
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

const PLAYER_RADIUS: f32 = 0.32;

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

const WALL_HEIGHT: f32 = 3.7;

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
    /// Wether or not to enable transparency
    transparent: bool,
    /// What side to cull, optional
    cull: Option<Face>,
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
        }
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
        command
    }
}

#[derive(Default)]
struct Textures {
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
    concrete: Handle<Image>,
    ceiling_panel: Handle<Image>,
    wood_slat_roof: Handle<Image>,
    sky: Handle<Image>,
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
}
