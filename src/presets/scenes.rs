use presets::scene_builder::*;
use presets::scene_builder::Color::*;
use presets::scene_builder::Mat::*;
use scene::Scene;

pub fn by_name(name: &str) -> Option<Scene> {
    match name {
        "cornell_box" => Some(cornell_box()),
        _ => None,
    }
}

pub fn cornell_box() -> Scene {
    let xl = -10.0;
    let xh = 10.0;
    let yl = 0.0;
    let yh = 20.0;
    let zl = 0.0;
    let zh = 20.0;

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
        .sphere((-3.0, 3.0, 13.0), 3.0, Specular(White))
        .sphere((4.0, 4.0, 10.0), 4.0, Diffuse(White))
        .quad(bbl, btl, btr, bbr, Diffuse(White))
        .quad(bbl, btl, ftl, fbl, Diffuse(Red))
        .quad(bbr, btr, ftr, fbr, Diffuse(Green))
        .quad(bbl, bbr, fbr, fbl, Diffuse(White))
        .quad(btl, btr, ftr, ftl, Light(White))
        .camera((0.0, 15.0, -20.0), (0.0, -0.2, 1.0))
        .build()
}
