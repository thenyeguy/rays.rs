use presets::scene_builder::*;
use presets::scene_builder::Color::*;
use presets::scene_builder::Mat::*;
use scene::Scene;

pub fn sphere_room() -> Scene {
    let xl = -10.0;
    let xh = 10.0;
    let yl = -8.0;
    let yh = 12.0;
    let zl = 20.0;
    let zh = 40.0;

    // {back,front}{bottom,top}{left,right} corners
    let bbl = (xl, yl, zh);
    let btl = (xl, yh, zh);
    let bbr = (xh, yl, zh);
    let btr = (xh, yh, zh);
    let fbl = (xl, yl, zl);
    let ftl = (xl, yh, zl);
    let fbr = (xh, yl, zl);
    let ftr = (xh, yh, zl);

    SceneBuilder::new()
        .sphere((-3.0, -5.0, 33.0), 3.0, Specular(White))
        .sphere((4.0, -4.0, 30.0), 4.0, Diffuse(White))
        .quad(bbl, btl, btr, bbr, Diffuse(White))
        .quad(bbl, btl, ftl, fbl, Diffuse(Red))
        .quad(bbr, btr, ftr, fbr, Diffuse(Blue))
        .quad(bbl, bbr, fbr, fbl, Diffuse(White))
        .quad(btl, btr, ftr, ftl, Light(White))
        .build()
}
