#[macro_use]
extern crate clap;
extern crate image;
extern crate nalgebra;
extern crate palette;
extern crate rays;

fn print_time(duration: std::time::Duration) {
    let secs = duration.as_secs();
    let hours = secs / 3600;
    let mins = (secs % 3600) / 60;
    let secs = secs % 60;
    println!("Rendering took: {}:{:02}:{:02}", hours, mins, secs);
}

fn main() {
    use rays::prelude::*;
    use std::error::Error;
    use std::time::Instant;

    let args = clap_app!(rays =>
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
    ).get_matches();

    let renderer = Renderer {
        width: value_t!(args, "width", u32).unwrap_or(100),
        height: value_t!(args, "height", u32).unwrap_or(100),
        fov: value_t!(args, "fov", u32).unwrap_or(45),
        samples_per_pixel: value_t!(args, "samples", u32).unwrap_or(50),
        max_reflections: value_t!(args, "reflections", u32).unwrap_or(5),
    };

    let scene_name = args.value_of("scene").unwrap();
    let scene = match scene_name {
        "sphere_room" => scenes::sphere_room(),
        _ => {
            println!("Invalid scene name: {}", scene_name);
            std::process::exit(1);
        }
    };

    let start = Instant::now();
    let img = renderer.render(&scene);
    print_time(start.elapsed());

    let output = format!("images/{}_{}x{}.png",
                         scene_name,
                         renderer.width,
                         renderer.height);
    match img.save(&output) {
        Ok(()) => println!("Wrote final image to: {}", output),
        Err(e) => {
            println!("Could not write file: {}", e.description());
            std::process::exit(1);
        }
    }
}
