extern crate image;
extern crate nalgebra;
extern crate rays;

fn main() {
    use nalgebra::Point3;
    use rays::camera::Camera;
    use rays::light::Light;
    use rays::surface::{Surface, Sphere};

    let surfaces: &[Box<Surface>] =
        &[Box::new(Sphere::new(Point3::new(0.0, 0.0, 500.0), 100.0)),
          Box::new(Sphere::new(Point3::new(150.0, 0.0, 350.0), 50.0))];
    let lights = &[Light::new(Point3::new(500.0, 0.0, 0.0))];

    let camera = Camera::new(500, 500);
    let img = camera.draw(surfaces, lights);
    img.save("images/test.png").unwrap();
}
