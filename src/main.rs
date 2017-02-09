#[macro_use]
extern crate clap;
extern crate image;
extern crate nalgebra;
extern crate palette;
extern crate rays;

fn main() {
    use nalgebra::{Vector3, Point3};
    use palette::Rgb;
    use rays::camera::Camera;
    use rays::light::Light;
    use rays::scene::Scene;
    use rays::surface::{Plane, Sphere};
    use std::error::Error;
    use std::time::Instant;

    let matches = clap_app!(rays =>
        (version: "0.1")
        (author: "Michael Nye <thenyeguy@gmail.com>")
        (about: "Ray tracer in Rust")
        (@arg width: -w --width +takes_value "image width (pixels)")
        (@arg height: -h --height +takes_value "image height (pixels)")
        (@arg fov: --fov +takes_value "field of view (degrees)")
        (@arg output: +required "image file to render into")
    )
        .get_matches();
    let width = value_t!(matches, "width", u32).unwrap_or(1000);
    let height = value_t!(matches, "height", u32).unwrap_or(1000);
    let fov = value_t!(matches, "fov", u32).unwrap_or(45);
    let output = matches.value_of("output").unwrap();

    let scene = Scene {
        surfaces: vec![Box::new(Sphere::new(Point3::new(0.0, 0.0, 20.0), 2.0)),
                       Box::new(Sphere::new(Point3::new(3.0, 1.0, 15.0), 1.0)),
                       Box::new(Plane::new(Point3::new(0.0, 2.0, 0.0),
                                           Vector3::new(0.0, 1.0, 0.0)))],
        lights: vec![Light::new(Point3::new(10.0, -1.0, 0.0),
                                Rgb::new(1.0, 0.7, 0.2))],
    };

    let now = Instant::now();
    let camera = Camera::new(width, height, fov);
    let img = camera.draw(&scene);
    println!("Rendering took {} seconds.", now.elapsed().as_secs());

    match img.save(output) {
        Ok(()) => println!("Wrote final image to {}", output),
        Err(e) => {
            println!("Could not write file: {}", e.description());
            std::process::exit(1);
        }
    }
}
