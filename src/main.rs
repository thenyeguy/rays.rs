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
    use std::time::Instant;

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
        &[Box::new(Sphere::new(Point3::new(0.0, 0.0, 1000.0), 200.0)),
          Box::new(Sphere::new(Point3::new(300.0, 0.0, 700.0), 100.0))];
    let lights = &[Light::new(Point3::new(1000.0, 0.0, 0.0))];

    let now = Instant::now();
    let camera = Camera::new(1000, 1000);
    let img = camera.draw(surfaces, lights);
    println!("Rendering took {} seconds", now.elapsed().as_secs());

    let output = matches.value_of("output").unwrap_or("images/test.png");
    if let Err(e) = img.save(output) {
        println!("Could not write file: {}", e.description());
        std::process::exit(1);
    }
}
