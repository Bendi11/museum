use bevy::window::WindowResized;

use super::*;
use super::scene::*;

/// Set up the museum scene with all walls and interactable objects
pub fn setup(
    mut windows: ResMut<Windows>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut light: ResMut<AmbientLight>,
    resources: Res<GlobalResources>,
    asset_server: Res<AssetServer>,
) {
    light.color = Color::WHITE;
    light.brightness = 1.2;
    windows.get_primary_mut().map(|window| {
        window.set_resizable(true);
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    });
    let window = windows.primary();

    let font = asset_server.get_handle("fonts/times-new-roman.ttf");

    let protest_image_txt = tombstone(
        &mut commands,
        window,
        font.clone(), 
        "Pro - Union Protestors",
        "March 22, 2021",
        "Lucy Nicholson",
r#"
Pictured are people in Los Angles protesting against the controversial results of a failed attempt to unionize the Amazon fulfillment center BHM1 in Alabama. Many were angered by Amazon's distribution of anti-union flyers to the workers and attempts to disrupt the vote by confusing uninformed voters at BHM1, arguing that it amounted to modern day union busting. In response to the Amazon's alleged interference in the election, the warehouse was granted another union election by the National Labor Relations Board. 
"#,
        "Nicholson, Lucy. “People Protest in Support of the Unionizing Efforts of the Alabama Amazon Workers, in Los Angeles, California, March 22, 2021.” How Amazon Fought the Union Drive in Alabama, CNBC, 16 Apr. 2021, https://www.cnbc.com/2021/04/16/how-amazon-fought-the-union-drive-in-alabama.html."
    );

    let art_txt = tombstone(
        &mut commands,
        window,
        font.clone(),
        "Amazon Labor Union",
        "April 3, 2022",
        "Randall Enos",
r#"
Depicted is a figure wearing a shirt labelled 'Smalls', referring to labor union leader Chris Smalls attacking a Goliathan figure. The upside-down orange arrow holds special significance as a pro-union symbol, representing Amazon's smiley face arrow turned into a frown. This cartoon shows the unionization effort as a strike from the working class against the goliathan giant of Amazon.
"#,
        "Enos, Randall. “Amazon Labor Union.” Cagle Cartoons, 3 Apr. 2022, https://caglecartoons.com/sku/261682. "
    );

    let mlk_speech_txt = tombstone(
        &mut commands,
        window,
        font.clone(),
        "All Labor Has Dignity",
        "March 18, 1968",
        "Martin Luther King Jr.",
r#"
Martin Luther King Jr. is possibly the most well known civil rights activist in the U.S., however his contributions to the labor movement often go unnoticed. In his "All Labor Has Dignity" speech, he speaks to striking sanitation workers in Memphis, Tennessee protesting dangerous working conditions and poor pay.
"#,
        "“The 50th Anniversary of Martin Luther King, Jr.'s ‘All Labor Has Dignity.’” Beacon Broadside: A Project of Beacon Press, 18 Mar. 2018, https://www.beaconbroadside.com/broadside/2018/03/the-50th-anniversary-of-martin-luther-king-jrs-all-labor-has-dignity.html."
    );

    let teacher_txt = tombstone(
        &mut commands,
        window,
        font.clone(),
        "Wear Red 4 Ed",
        "2022",
        "Union Strong",
r#"
A shirt in the style of the popular 'Red 4 Ed' movement. The item shows support for teacher's unions- protesting unlivable wages paid to teachers. The statement 'My Other Job Paid for This Shirt refers to the oft-quoted statistic that 1 in 5 teachers must take another job to support themselves financially.
"#,
        "Unionstrongshirts. “Wear Red 4 Ed.” Unionstrongshirts, https://unionstrongshirts.com/products/wear-red-4-ed."
    );

    let reagan_txt = tombstone(
        &mut commands,
        window,
        font.clone(),
        "Reagan ATC Strike Interview",
        "August 3, 1981",
        "Ronald Reagan",
r#"
Following a strike by air traffic controllers across the nation in response to unsafe conditions. The Professional Air Traffic Controllers Organization organized a strike, demanding better pay and post-retirement benefits, as well as a reduced 32-hour work week. Reagan delivered a press conference, declaring PATCO a 'peril to national safety' and ordering their immediate return to work or 'termination'. 
"#,
        "Reagan, Ronald. Remarks and Q &amp; A with Reporters on the Air Traffic ... - Youtube. https://www.youtube.com/watch?v=j3ZTCPJ39LA."
    );

    let news_txt = tombstone(
        &mut commands,
        window,
        font.clone(),
        "Senate Defeats Labor Bill Veto",
        "June 23, 1947",
        "Elmira Star-Gazette",
r#"
On June 23, 1947 Congress overrode president Truman's veto of the Taft-Hartley Act following a wave of postwar labor strikes. The act was designed to limit the power of unions to organize strikes and form contractual 'closed shop' agreements with employers requiring the employer to hire union members exlusively. In addition, the executive branch was given the ability to obtain an injuction requiring that a strike be broken if the strike was deemed a threat to national health and safety. Truman vehemently vetoed this act, and labor leaders condemned the act as a 'slave labor bill'.
"#,
        "Washington (AP). “Overrider Vote Wins 68-25 With Measure Becoming Law.” Elmira Star-Gazette, 23 June 1947, p. 1."
    );

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
    const PEDESTAL_HEIGHT: f32 = 1.;

    SceneBuilder::new()
        .with_wall(wall(a, f)
            .with_texture(resources.blue_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(a, c)
            .with_texture(resources.blue_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(b, d)
            .with_texture(resources.blue_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(c, k)
            .with_texture(resources.intro_wall.clone())
            .with_tiles(-1., 1.)
            .with_cull(Face::Front)
        )
        .with_wall(wall(d, bp)
            .with_texture(resources.blue_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(e, i)
            .with_texture(resources.blue_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(e, bq)
            .with_texture(resources.blue_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(f, i)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.eggshell_wall.clone())
        )
        .with_wall(wall(g, z)
            .with_texture(resources.red_trimmed_wall.clone())
            .autotile_len()
            .with_cull(Face::Back)
        )
        .with_wall(wall(k, bf)
            .with_collision(false)
            .with_offset(WALL_HEIGHT - CEILING_OFFSET)
            .with_height(CEILING_OFFSET * 2.)
            .with_texture(resources.concrete.clone())
            .with_tiles(1., 0.2)
        )
        .with_wall(wall(k, o)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.concrete.clone())
            .with_tiles(0.5, 1.)
            .with_cull(Face::Front)
        )
        .with_wall(wall(m, ae)
            .with_texture(resources.red_trimmed_wall.clone())
            .autotile_len()
            .with_cull(Face::Front)
        )
        .with_wall(wall(n, p)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.concrete.clone())
            .with_cull(Face::Back)
            .with_tiles(0.5, 1.)
        )
        .with_wall(wall(n, bf)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.concrete.clone())
            .with_tiles(0.5, 1.)
            .with_cull(Face::Front)
        )
        .with_wall(wall(o, q)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.concrete.clone())
            .with_cull(Face::Front)
            .with_tiles(0.5, 1.)
        )
        .with_wall(wall(p, v)
            .with_collision(false)
            .with_offset(WALL_HEIGHT)
            .with_height(CEILING_OFFSET)
            .with_texture(resources.concrete.clone())
            .with_tiles(4., 0.2)
            .with_cull(Face::Back)
        )
        .with_wall(wall(p, bt)
            .with_texture(resources.concrete.clone())
            .with_tiles(0.3, 1.)
            .with_cull(Face::Back)
        )
        .with_wall(wall(q, w)
            .with_collision(false)
            .with_offset(WALL_HEIGHT)
            .with_height(CEILING_OFFSET)
            .with_texture(resources.concrete.clone())
            .with_tiles(4., 0.2)
            .with_cull(Face::Front)
        )
        .with_wall(wall(q, bu)
            .with_texture(resources.concrete.clone())
            .with_tiles(0.3, 1.)
            .with_cull(Face::Front)
        )
        .with_wall(wall(v, x)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.concrete.clone())
            .with_cull(Face::Back)
            .with_tiles(0.3, 1.)

        )
        .with_wall(wall(v, bv)
            .with_texture(resources.concrete.clone())
            .with_tiles(0.3, 1.)
            .with_cull(Face::Front)
        )
        .with_wall(wall(w, y)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.concrete.clone())
            .with_tiles(0.3, 1.)
            .with_cull(Face::Front)

        )
        .with_wall(wall(w, bw)
            .with_texture(resources.concrete.clone())
            .with_tiles(0.3, 1.)
            .with_cull(Face::Back)
        )
        .with_wall(wall(x, ab)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.concrete.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(y, ac)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.concrete.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(z, bx)
            .with_height(WALL_HEIGHT)
            .with_texture(resources.red_trimmed_wall.clone())
            .autotile_len()
            .with_cull(Face::Back)
        )
        .with_wall(wall(z, ae)
            .with_collision(false)
            .with_offset(WALL_HEIGHT + CEILING_OFFSET)
            .with_height(CEILING_OFFSET)
            .with_texture(resources.concrete.clone())
            .with_tiles(10., 0.2)
            .with_cull(Face::Back)
        )
        .with_wall(wall(ab, ag)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.eggshell_wall.clone())
            .with_tiles(0.2, 1.)
            .with_cull(Face::Back)
        )
        .with_wall(wall(ac, ah)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.eggshell_wall.clone())
            .with_tiles(0.2, 1.)
            .with_cull(Face::Front)
        )
        .with_wall(wall(af, ag)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.blue_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(af, al)
            .with_texture(resources.blue_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(ah, ai)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.blue_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(ai, aq)
            .with_texture(resources.blue_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(aj, am)
            .with_texture(resources.blue_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(aj, bz)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.green_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(aj, cb)
            .with_height(CEILING_OFFSET)
            .with_offset(WALL_HEIGHT)
            .with_collision(false)
            .with_cull(Face::Front)
        )
        .with_wall(wall(ak, ap)
            .with_texture(resources.blue_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(ak, ca)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.green_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(ak, cc)
            .with_height(CEILING_OFFSET)
            .with_offset(WALL_HEIGHT)
            .with_collision(false)
            .with_cull(Face::Back)
        )
        .with_wall(wall(al, am)
            .with_texture(resources.blue_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(an, ay)
            .with_texture(resources.eggshell_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(an, ao)
            .with_height(CEILING_OFFSET)
            .with_offset(WALL_HEIGHT)
            .with_collision(false)
            .with_cull(Face::Back)
        )
        .with_wall(wall(ao, ar)
            .with_texture(resources.eggshell_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(ap, aq)
            .with_texture(resources.blue_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(ar, at)
            .with_texture(resources.eggshell_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(r#as, av)
            .with_height(WALL_HEIGHT - 0.9)
        )
        .with_wall(wall(at, ax)
            .with_texture(resources.eggshell_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(au, az)
            .with_height(WALL_HEIGHT - 0.9)
            .with_texture(resources.josh_exit.clone())
            .with_tiles(-1., 1.)
        )
        .with_wall(wall(aw, bb)
            .with_height(WALL_HEIGHT - 0.9)
        )
        .with_wall(wall(ax, be)
            .with_texture(resources.red_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(ay, bc)
            .with_texture(resources.eggshell_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(bc, cf)
            .with_texture(resources.red_trimmed_wall.clone())
            .with_tiles(0.2, 1.)
            .with_cull(Face::Back)
        )
        .with_wall(wall(bc, cd)
            .with_height(CEILING_OFFSET)
            .with_offset(WALL_HEIGHT - CEILING_OFFSET)
            .with_collision(false)
            .with_texture(resources.eggshell_wall.clone())
            .with_tiles(2., 0.2)
            .with_cull(Face::Back)
        )
        .with_wall(wall(bd, be)
            .with_texture(resources.red_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(bf, bq)
            .with_texture(resources.blue_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(bg, bk)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.concrete.clone())
            .with_tiles(0.3, 1.)
            .with_cull(Face::Front)
        )
        .with_wall(wall(bg, bi)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.concrete.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(bh, bl)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.concrete.clone())
            .with_tiles(0.3, 1.)
            .with_cull(Face::Back)
        )
        .with_wall(wall(bh, bj)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.concrete.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(bi, bm)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.concrete.clone())
            .with_tiles(0.3, 1.)
            .with_cull(Face::Back)
        )
        .with_wall(wall(bj, bn)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.concrete.clone())
            .with_tiles(0.3, 1.)
            .with_cull(Face::Front)
        )
        .with_wall(wall(bk, bm)
            .with_texture(resources.red_trimmed_wall.clone())
            .autotile_len()
            .with_cull(Face::Front)
        )
        .with_wall(wall(bl, bn)
            .with_texture(resources.red_trimmed_wall.clone())
            .autotile_len()
            .with_cull(Face::Back)
        )
        .with_wall(wall(bo, bp)
            .with_texture(resources.blue_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(br, bt)
            .with_texture(resources.red_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(bs, bu)
            .with_texture(resources.red_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(bs, m)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.eggshell_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(bv, bx)
            .with_texture(resources.red_trimmed_wall.clone())
            .autotile_len()
            .with_cull(Face::Front)
        )
        .with_wall(wall(bw, by)
            .with_texture(resources.red_trimmed_wall.clone())
            .autotile_len()
            .with_cull(Face::Back)
        )
        .with_wall(wall(by, ae)
            .with_height(WALL_HEIGHT)
            .with_texture(resources.red_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(bz, an)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.green_trimmed_wall.clone())
            .with_cull(Face::Back)
        )
        .with_wall(wall(ca, ao)
            .with_height(WALL_HEIGHT + CEILING_OFFSET)
            .with_texture(resources.green_trimmed_wall.clone())
            .with_cull(Face::Front)
        )
        .with_wall(wall(ce, bd)
            .with_texture(resources.red_trimmed_wall.clone())
            .autotile_len()
            .with_cull(Face::Back)
        )

        .with_wall(wall(ce, cf)
            .with_texture(resources.red_trimmed_wall.clone())
            .autotile_len()
            .with_cull(Face::Front)
        )
        
        .with_floor(
            FloorBuilder::new(ba, be)
                .with_offset(WALL_HEIGHT - CEILING_OFFSET)
                .with_texture(resources.ceiling_panel.clone())
                .autotile()
                .with_cull(Face::Front)
        )
        .with_floor(
            FloorBuilder::new(ad, am)
                .with_offset(WALL_HEIGHT + CEILING_OFFSET)
                .with_texture(resources.ceiling_panel.clone())
                .autotile()
                .with_cull(Face::Back)
        )
        .with_floor(
            FloorBuilder::new(af, am)
                .with_offset(WALL_HEIGHT)
                .with_texture(resources.ceiling_panel.clone())
                .autotile()
                .with_cull(Face::Front)
        )
        .with_floor(
            FloorBuilder::new(ai, ap)
                .with_offset(WALL_HEIGHT)
                .with_texture(resources.ceiling_panel.clone())
                .autotile()
                .with_cull(Face::Back)
        )
        .with_floor(
            FloorBuilder::new(a, k)
                .with_offset(WALL_HEIGHT - CEILING_OFFSET)
                .with_texture(resources.ceiling_panel.clone())
                .with_brightness(0.8)
                .autotile()
                .with_cull(Face::Front)
        )
        .with_floor(
            FloorBuilder::new(g, aa)
                .with_offset(WALL_HEIGHT)
                .with_texture(resources.ceiling_panel.clone())
                .autotile()
                .with_cull(Face::Front)
        )
        .with_floor(
            FloorBuilder::new(l, ae)
                .with_offset(WALL_HEIGHT)
                .with_texture(resources.ceiling_panel.clone())
                .autotile()
                .with_cull(Face::Front)
        )

        .with_floor(
            FloorBuilder::new(an, cd)
                .with_texture(resources.wood_slat_roof.clone())
                .with_brightness(0.3)
                .with_offset(WALL_HEIGHT)
                .autotile()
                .with_cull(Face::Front)
        )

        .with_floor(
            FloorBuilder::new(ba, be)
                .with_texture(resources.oak_floor.clone())
                .with_brightness(0.5)
                .autotile()
                .with_cull(Face::Back)
        )
        
        .with_floor(
            FloorBuilder::new(af, aq)
                .with_texture(resources.red_tile_floor.clone())
                .autotile()
                .with_cull(Face::Back)
        )
        .with_floor(
            FloorBuilder::new(an, cd)
                .with_texture(resources.linoleum_floor.clone())
                .with_brightness(0.4)
                .autotile()
                .with_cull(Face::Back)
        )
        .with_floor(
            FloorBuilder::new(h, ad)
                .with_texture(resources.flagstone_floor.clone())
                .with_offset(0.0001)
                .autotile()
                .with_cull(Face::Back)
        )
        .with_floor(
            FloorBuilder::new(m, z)
                .with_texture(resources.oak_floor.clone())
                .autotile()
                .with_cull(Face::Front)
        )
        .with_floor(
            FloorBuilder::new(a, k)
                .with_texture(resources.birch_floor.clone())
                .with_brightness(0.6)
                .autotile()
                .with_cull(Face::Back)
        )
        .with_floor(
            FloorBuilder::new(aa, ap)
                .with_texture(resources.tile_floor.clone())
                .with_brightness(0.7)
                .with_offset(0.001)
                .autotile()
                .with_cull(Face::Back)
        )
        
        .with_floor(
            FloorBuilder::new((100., 100.), (-100., -100.))
                .with_offset(10.)
                .with_texture(resources.sky.clone())
                .with_cull(Face::Front)
        )

        .with_wall(
            wall((bj.0 - 0.5, bj.1), (bj.0, bj.1 - 0.5))
            .with_texture(resources.job_iden.clone())
            .with_height(2.1)
            .with_transparency(true)
        )
    
        .with_wall(
            wall((g.0 + 0.01, g.1 + 2.5), (g.0 + 0.01, g.1 + 3.))
                .with_texture(resources.tombstone.clone())
                .with_cull(Face::Back)
                .with_height(0.25)
                .with_offset(WALL_HEIGHT / 2. - 0.75)
                .with_transparency(false)
                .with_action(InteractableAction::Tombstone { text: protest_image_txt})
                .with_collision(false)
        )
        .with_wall(
            wall((g.0 + 0.01, g.1 + 4.), (g.0 + 0.01, g.1 + 8.))
                .with_texture(resources.protest_image.clone())
                .with_cull(Face::Back)
                .with_height(2.25)
                .with_offset(WALL_HEIGHT / 2. - 0.7)
                .with_collision(false)
        )
        .with_wall(
            wall((m.0 - 0.01, m.1 + 1.), (m.0 - 0.01, m.1 + 4.))
                .with_texture(resources.art.clone())
                .with_cull(Face::Front)
                .with_height(1.98)
                .with_offset(WALL_HEIGHT / 2. - 0.9)
                .with_collision(false)
        )
        .with_wall(
            wall((m.0 - 0.01, m.1 + 4.5), (m.0 - 0.01, m.1 + 5.))
                .with_texture(resources.tombstone.clone())
                .with_cull(Face::Front)
                .with_height(0.25)
                .with_offset(WALL_HEIGHT / 2. - 0.5)
                .with_action(InteractableAction::Tombstone { text: art_txt })
                .with_collision(false)
                .with_tiles(-1., 1.)
        )
        
        .with_wall(
            wall((af.0 + 0.01, af.1 + 0.5), (af.0 + 0.01, af.1 + 4.5))
                .with_height(2.25)
                .with_offset(WALL_HEIGHT / 2. - 1.)
                .with_collision(false)
                .with_texture(resources.mlk.clone())
        )
        .with_wall(
            wall((af.0 + 0.01, af.1 + 4.8), (af.0 + 0.01, af.1 + 5.3))
                .with_height(0.25)
                .with_cull(Face::Back)
                .with_collision(false)
                .with_offset(WALL_HEIGHT / 2. - 0.4)
                .with_action(InteractableAction::Tombstone { text: mlk_speech_txt })
                .with_texture(resources.tombstone.clone())
        )
        .with_wall(
            wall((al.0 + 3.2, al.1 - 0.1), (al.0 + 3.70, al.1 - 0.1))
                .with_texture(resources.headphones.clone())
                .with_transparency(true)
                .with_height(0.5)
                .with_offset(WALL_HEIGHT / 2. - 0.25)
                .with_action(InteractableAction::Audio { source: resources.mlk_speech.clone() })
                .with_cull(Face::Back)
        )

        .with_wall(
            wall((z.0 + 0.01, z.1 - 3.), (z.0 + 0.01, z.1 - 3.9))
                .with_texture(resources.teacher_shirt.clone())
                .with_cull(Face::Front)
                .with_height(1.)
                .with_offset(WALL_HEIGHT / 2. - 0.5)
                .with_tiles(-1., 1.)
                .with_transparency(true)
                .with_collision(false)
        )
        .with_wall(
            wall((z.0 + 0.01, z.1 - 3.2), (z.0 + 0.01, z.1 - 3.7))
                .with_height(0.25)
                .with_cull(Face::Front)
                .with_collision(false)
                .with_texture(resources.tombstone.clone())
                .with_offset(WALL_HEIGHT / 2. - 0.9)
                .with_tiles(-1., 1.)
                .with_action(InteractableAction::Tombstone { text: teacher_txt })
        )

        .with_wall(
            wall((ap.0 + 1., ap.1 - 0.01), (ap.0 + 1.5, ap.1 - 0.01))
                .with_height(0.25)
                .with_cull(Face::Back)
                .with_collision(false)
                .with_texture(resources.tombstone.clone())
                .with_offset(WALL_HEIGHT / 2. - 0.7)
                .with_action(InteractableAction::Tombstone { text: reagan_txt })
        )
        .with_wall(
            wall((ai.0 - 0.01, ai.1 + 2.), (ai.0 - 0.01, ai.1 + 5.))
                .with_height(2.0625)
                .with_texture(resources.reagan.clone())
                .with_collision(false)
                .with_tiles(-1., 1.)
                .with_cull(Face::Front)
                .with_offset(WALL_HEIGHT / 2. - 1.03)
        )
        .with_wall(
            wall((ai.0 - 4., ai.1 + 0.01), (ai.0 - 3.5, ai.1 + 0.01))
                .with_texture(resources.headphones.clone())
                .with_height(0.5)
                .with_collision(false)
                .with_action(InteractableAction::Audio { source: resources.reagan_audio.clone() })
                .with_cull(Face::Front)
                .with_offset(WALL_HEIGHT / 2. - 0.25)
                .with_transparency(true)
        )

        /* Pedestal for newspaper */
        .with_wall(
            wall((10., 29.), (12., 29.))
                .with_cull(Face::Front)
                .with_height(PEDESTAL_HEIGHT)
                .with_texture(resources.velvet.clone())
        )
        .with_wall(
            wall((10., 29.), (10., 27.))
                .with_cull(Face::Back)
                .with_height(PEDESTAL_HEIGHT)
                .with_texture(resources.velvet.clone())
        )
        .with_wall(
            wall((10., 27.), (12., 27.))
                .with_cull(Face::Back)
                .with_height(PEDESTAL_HEIGHT)
                .with_texture(resources.velvet.clone())
        )
        .with_wall(
            wall((12., 27.), (12., 29.))
                .with_cull(Face::Back)
                .with_height(PEDESTAL_HEIGHT)
                .with_texture(resources.velvet.clone())
        )
        .with_floor(
            FloorBuilder::new((10., 29.), (12., 27.))
                .with_cull(Face::Front)
                .with_texture(resources.news.clone())
                .with_offset(PEDESTAL_HEIGHT)
                .with_tiles(-1., 1.)
        )
        .with_wall(
            wall((10.75, 26.999), (11.25, 26.999))
                .with_texture(resources.tombstone.clone())
                .with_offset(PEDESTAL_HEIGHT / 2. + 0.1)
                .with_height(0.25)
                .with_cull(Face::Back)
                .with_action(InteractableAction::Tombstone { text: news_txt })
                .with_collision(false)
        )
        
        .finish(&mut commands, &mut meshes, &mut materials);
    
    //Spawn the player
    commands
        .spawn_bundle(LookTransformBundle {
            transform: LookTransform {
                eye: Vec3::new(4., PLAYER_HEIGHT, 3.),
            target: Vec3::new(5., PLAYER_HEIGHT, 0.),
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
        .insert(Player::default());
    
    commands.spawn_bundle(UiCameraBundle::default());

    // Text with one section
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Percent(50.),
                    left: Val::Percent(50.),
                    ..default()
                },
                ..default()
            },
            text: Text::with_section(
                "[e] Read",
                TextStyle {
                    font: asset_server.load("fonts/times-new-roman.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(Visibility { is_visible: true, })
        .insert(InteractText);
}

/// Create a new tombstone with all required sections
pub fn tombstone(
    commands: &mut Commands,
    window: &Window,
    font: Handle<Font>,
    title: &str,
    date: &str,
    creator: &str,
    summary: &str,
    source: &str,
) -> Entity {
    let text_color = Color::rgb(0.2, 0.2, 0.2);

    commands.spawn_bundle(TextBundle {
        style: Style {
            //align_self: AlignSelf::Center,
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Percent(35.),
                left: Val::Percent(5.),
                ..default()
            },
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::FlexEnd,
            flex_wrap: FlexWrap::Wrap,
            max_size: Size::new(Val::Px(window.width() - (window.width() * 0.2)), Val::Px(window.height())),
            ..default()
        },
        text: Text {
            sections: vec![
                TextSection {
                    style: TextStyle { font: font.clone(), font_size: 48., color:  text_color},
                    value: format!("{}, {}\n\n", title, date),
                },
                TextSection {
                    style: TextStyle { font: font.clone(), font_size: 32., color: text_color},
                    value: format!("{}\n\n", creator),
                },
                TextSection {
                    style: TextStyle { font: font.clone(), font_size: 24., color: text_color},
                    value: format!("{}\n", summary),
                },
                TextSection {
                    style: TextStyle { font: font.clone(), font_size: 16., color: text_color},
                    value: source.to_owned(),
                }
            ],
            alignment: TextAlignment { vertical: VerticalAlign::Top, horizontal: HorizontalAlign::Left }
        },
        ..default()
    })
    .insert(Visibility { is_visible: false})
    .id()
}

/// System to update text maximum sizes based on window size, used because maximum size in 
/// percents doesn't work in Bevy 0.7
pub fn set_text_sizes(
    mut resized: EventReader<WindowResized>,
    mut texts: Query<&mut Style>,
) {
    for event in resized.iter() {
        for mut text in texts.iter_mut() {
            text.max_size = Size::new(Val::Px(event.width - (event.width * 0.2)), Val::Px(event.height));
        }
    }
}
