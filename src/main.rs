extern crate clap;
extern crate image;
extern crate nalgebra;
extern crate rays;

fn main() {
    use nalgebra::Point3;
    use rays::camera::Camera;
    use rays::light::Light;
    use rays::surface::{Surface, Sphere};
    use std::error::Error;

    let matches = clap::App::new("rays")
        .version("0.1")
        .about("Ray Tracer in Rust")
        .author("Michael Nye")
        .arg(clap::Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("Image file to write rendered result to.")
            .takes_value(true))
        .get_matches();

    let surfaces: &[Box<Surface>] =
        &[Box::new(Sphere::new(Point3::new(0.0, 0.0, 500.0), 100.0)),
          Box::new(Sphere::new(Point3::new(150.0, 0.0, 350.0), 50.0))];
    let lights = &[Light::new(Point3::new(500.0, 0.0, 0.0))];

    let camera = Camera::new(500, 500);
    let img = camera.draw(surfaces, lights);

    let output = matches.value_of("output").unwrap_or("images/test.png");
    if let Err(e) = img.save(output) {
        println!("Could not write file: {}", e.description());
        std::process::exit(1);
    }
}
