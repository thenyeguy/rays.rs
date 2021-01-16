use indicatif::{ProgressBar, ProgressStyle};
use rays::prelude::*;
use structopt::StructOpt;

struct Logger {
    progress_bar: ProgressBar,
}

impl Logger {
    fn new(renderer: &Renderer) -> Self {
        let progress_bar = ProgressBar::new(renderer.width as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("    [{elapsed_precise}] {wide_bar} {percent}%    "),
        );
        Logger { progress_bar }
    }
}

impl RenderProgress for Logger {
    fn on_render_start(&self) {
        println!("Rendering image...");
        self.progress_bar.enable_steady_tick(100 /*ms*/);
        self.progress_bar.reset_elapsed();
    }

    fn on_col_done(&self) {
        self.progress_bar.inc(1);
    }

    fn on_render_done(&self) {
        self.progress_bar.finish();
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "rays", about = "A ray tracer in Rust")]
struct App {
    /// Image width, in pixels
    #[structopt(short, long, default_value = "100")]
    width: u32,
    /// Image height, in pixels
    #[structopt(short, long, default_value = "100")]
    height: u32,
    /// Number of samples per pixel
    #[structopt(short, long, default_value = "500")]
    samples: u32,
    /// Maximum number of reflections per sample
    #[structopt(long, default_value = "5")]
    reflections: u32,
    /// The scene to render
    scene: String,
    /// The output image file
    output: String,
}

fn main() {
    let app = App::from_args();

    println!("Loading scene...");
    let scene = load_scene(&app.scene).unwrap_or_else(|e| {
        println!("Error loading scene: {}", e);
        std::process::exit(1);
    });

    let renderer = Renderer {
        width: app.width,
        height: app.height,
        samples_per_pixel: app.samples,
        max_reflections: app.reflections,
    };
    let logger = Logger::new(&renderer);
    let img = renderer.render(&scene, &logger);

    report_statistics();

    img.save(&app.output).unwrap_or_else(|e| {
        println!("Could not write file: {}", e);
        std::process::exit(1);
    });
    println!("Wrote final image to: {}", app.output);
}
