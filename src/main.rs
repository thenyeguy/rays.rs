extern crate image;
extern crate nalgebra;
extern crate rays;

use image::RgbImage;
use nalgebra::{Point3};
use rays::camera::Camera;
use rays::surface::{Surface, Sphere};

fn draw_console(img: &RgbImage, sample_rate: u32) {
    let width = img.width() / sample_rate;
    let height = img.height() / sample_rate;
    print!("┌"); for _ in 0..width { print!("──"); } print!("┐\n");
    for y in 0..height {
        print!("│");
        for x in 0..width {
            if img.get_pixel(sample_rate*x as u32, sample_rate*y as u32)[0] > 0 {
                print!("xx");
            } else {
                print!("  ");
            }
        }
        print!("│\n");
    }
    print!("└"); for _ in 0..width { print!("──"); } print!("┘\n");
}

#[cfg(not(test))]
fn main() {
    let surfaces: Vec<Box<Surface>> = vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, 100.0), 5.0)),
    ];

    let camera = Camera::new(25, 25);
    let img = camera.draw(&surfaces);
    draw_console(&img, 1);
}
