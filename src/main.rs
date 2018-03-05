#[macro_use]
extern crate clap;
extern crate image;
extern crate indicatif;
extern crate nalgebra;
extern crate palette;
extern crate rays;

use indicatif::{ProgressBar, ProgressStyle};
use rays::prelude::*;
use std::error::Error;

struct Logger {
    progress_bar: ProgressBar,
}

impl Logger {
    fn new(renderer: &Renderer) -> Self {
        let progress_bar = ProgressBar::new(renderer.height as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("    [{elapsed_precise}] {wide_bar} {percent}%    "),
        );
        Logger {
            progress_bar: progress_bar,
        }
    }

    fn on_start(&self) {
        println!("Rendering image...");
        self.progress_bar.enable_steady_tick(100 /* ms */);
    }

    fn on_finish(&self) {
        self.progress_bar.finish();
    }
}

impl RenderProgress for Logger {
    fn on_row_done(&self) {
        self.progress_bar.inc(1);
    }
}

fn main() {
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
    let scene = match scenes::by_name(scene_name) {
        Some(scene) => scene,
        None => {
            println!("Invalid scene name: {}", scene_name);
            std::process::exit(1);
        }
    };

    let output = format!(
        "images/{}_{}x{}.png",
        scene_name, renderer.width, renderer.height
    );

    let logger = Logger::new(&renderer);
    logger.on_start();
    let img = renderer.render(&scene, &logger);
    logger.on_finish();

    match img.save(&output) {
        Ok(()) => println!("Wrote final image to: {}", output),
        Err(e) => {
            println!("Could not write file: {}", e.description());
            std::process::exit(1);
        }
    }
}
