#[macro_use]
extern crate clap;
extern crate image;
extern crate nalgebra;
extern crate palette;
extern crate rays;

fn main() {
    use rays::prelude::*;
    use std::error::Error;
    use std::time::Instant;

    let matches = clap_app!(rays =>
        (version: "0.1")
        (author: "Michael Nye <thenyeguy@gmail.com>")
        (about: "Ray tracer in Rust")
        (@arg width: -w --width +takes_value "image width (pixels)")
        (@arg height: -h --height +takes_value "image height (pixels)")
        (@arg fov: --fov +takes_value "field of view (degrees)")
        (@arg samples: --samples +takes_value "number of samples per pixel")
        (@arg reflections: --reflections +takes_value
            "maximum number of reflections per sample")
        (@arg scene: +required "the scene to render")
    )
        .get_matches();

    let renderer = Renderer {
        width: value_t!(matches, "width", u32).unwrap_or(100),
        height: value_t!(matches, "height", u32).unwrap_or(100),
        fov: value_t!(matches, "fov", u32).unwrap_or(45),
        samples_per_pixel: value_t!(matches, "samples", u32).unwrap_or(50),
        max_reflections: value_t!(matches, "reflections", u32).unwrap_or(5),
    };

    let scene_name = matches.value_of("scene").unwrap();
    let scene = match scene_name {
        "basic_spheres" => scenes::basic_spheres(),
        "pyramid" => scenes::pyramid(),
        "sphere_in_room" => scenes::sphere_in_room(),
        _ => {
            println!("Invalid scene name: {}", scene_name);
            std::process::exit(1);
        }
    };

    let start = Instant::now();
    let img = renderer.render(&scene);
    println!("Rendering took {} seconds.", start.elapsed().as_secs());

    let output = format!("images/{}_{}x{}.png",
                         scene_name,
                         renderer.width,
                         renderer.height);
    match img.save(&output) {
        Ok(()) => println!("Wrote final image to {}", output),
        Err(e) => {
            println!("Could not write file: {}", e.description());
            std::process::exit(1);
        }
    }
}
