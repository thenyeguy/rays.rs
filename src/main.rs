use indicatif::{ProgressBar, ProgressStyle};
use rays::prelude::*;
use structopt::StructOpt;

struct Logger {
    progress_bar: ProgressBar,
    prof_file: Option<String>,
}

impl Logger {
    fn new(renderer: &Renderer, prof_file: &Option<String>) -> Self {
        let progress_bar = ProgressBar::new(renderer.width as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("    [{elapsed_precise}] {wide_bar} {percent}%    "),
        );
        Logger {
            progress_bar,
            prof_file: prof_file.clone(),
        }
    }
}

impl RenderProgress for Logger {
    fn on_render_start(&self) {
        println!("Rendering image...");
        self.progress_bar.reset_elapsed();
        if let Some(ref file) = self.prof_file {
            cpuprofiler::PROFILER
                .lock()
                .unwrap()
                .start(file.as_str())
                .unwrap();
        }
    }

    fn on_col_done(&self) {
        self.progress_bar.inc(1);
    }

    fn on_render_done(&self) {
        self.progress_bar.finish();
        if let Some(ref file) = self.prof_file {
            cpuprofiler::PROFILER.lock().unwrap().stop().unwrap();
            println!("Profile saved to: {}", file);
        }
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
    /// An optional file to write profiling information to
    #[structopt(long)]
    profile: Option<String>,
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
    let logger = Logger::new(&renderer, &app.profile);
    let img = renderer.render(&scene, &logger);

    report_statistics();

    img.save(&app.output).unwrap_or_else(|e| {
        println!("Could not write file: {}", e);
        std::process::exit(1);
    });
    println!("Wrote final image to: {}", app.output);
}
