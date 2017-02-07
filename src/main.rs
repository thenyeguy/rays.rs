extern crate clap;
extern crate image;
extern crate nalgebra;
extern crate rays;

fn main() {
    use nalgebra::Point3;
    use rays::camera::Camera;
    use rays::light::Light;
    use rays::scene::Scene;
    use rays::surface::Sphere;
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

    let scene = Scene {
        surfaces: vec![Box::new(Sphere::new(Point3::new(0.0, 0.0, 1000.0),
                                            200.0)),
                       Box::new(Sphere::new(Point3::new(300.0, 0.0, 700.0),
                                            100.0))],
        lights: vec![Light::new(Point3::new(1000.0, 0.0, 0.0))],
    };

    let now = Instant::now();
    let camera = Camera::new(1000, 1000);
    let img = camera.draw(&scene);
    println!("Rendering took {} seconds.", now.elapsed().as_secs());

    let out_file = matches.value_of("output").unwrap_or("images/test.png");
    match img.save(out_file) {
        Ok(()) => println!("Wrote final image to {}", out_file),
        Err(e) => {
            println!("Could not write file: {}", e.description());
            std::process::exit(1);
        }
    }
}
