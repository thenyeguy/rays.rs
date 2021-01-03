use clap::{clap_app, value_t};
use indicatif::{ProgressBar, ProgressStyle};
use rays::prelude::*;

struct Logger {
    progress_bar: ProgressBar,
    prof_file: String,
}

impl Logger {
    fn new(renderer: &Renderer, prof_file: Option<&str>) -> Self {
        let progress_bar = ProgressBar::new(renderer.height as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("    [{elapsed_precise}] {wide_bar} {percent}%    "),
        );
        Logger {
            progress_bar,
            prof_file: prof_file.map_or(String::new(), |s| s.into()),
        }
    }

    fn on_start(&self) {
        println!("Rendering image...");
        self.progress_bar.enable_steady_tick(100 /* ms */);
        if !self.prof_file.is_empty() {
            cpuprofiler::PROFILER
                .lock()
                .unwrap()
                .start(self.prof_file.as_str())
                .unwrap();
        }
    }

    fn on_finish(&self) {
        self.progress_bar.finish();
        if !self.prof_file.is_empty() {
            cpuprofiler::PROFILER.lock().unwrap().stop().unwrap();
            println!("Profile saved to: {}", self.prof_file);
        }
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
        (@arg profile: --profile +takes_value
            "the (optional) file to write profiling information to")
        (@arg scene: +required "the scene to render")
        (@arg output: +required "the output file")
    )
    .get_matches();

    let renderer = Renderer {
        width: value_t!(args, "width", u32).unwrap_or(100),
        height: value_t!(args, "height", u32).unwrap_or(100),
        fov: value_t!(args, "fov", u32).unwrap_or(45),
        samples_per_pixel: value_t!(args, "samples", u32).unwrap_or(500),
        max_reflections: value_t!(args, "reflections", u32).unwrap_or(5),
    };

    let profile = args.value_of("profile");
    let scene_file = args.value_of("scene").unwrap();
    let output = args.value_of("output").unwrap();

    let scene = load_scene(scene_file).unwrap_or_else(|e| {
        println!("Error loading scene: {}", e);
        std::process::exit(1);
    });

    let logger = Logger::new(&renderer, profile);
    logger.on_start();
    let img = renderer.render(&scene, &logger);
    logger.on_finish();

    img.save(&output).unwrap_or_else(|e| {
        println!("Could not write file: {}", e);
        std::process::exit(1);
    });
    println!("Wrote final image to: {}", output);
}
