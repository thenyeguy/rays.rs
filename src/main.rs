use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use rays::prelude::*;
use structopt::StructOpt;

struct LoadProgress(ProgressBar);

impl LoadProgress {
    fn new() -> Self {
        let bar = ProgressBar::new_spinner()
            .with_prefix("[1/3]")
            .with_message("Loading scene")
            .with_style(
                ProgressStyle::default_spinner()
                    .tick_strings(&["   ", ".  ", ".. ", "...", "   "])
                    .template(
                        " {prefix:.cyan.bold} {msg}{spinner} ({elapsed_precise})"
                    ),
            );
        bar.enable_steady_tick(500 /*ms*/);
        LoadProgress(bar)
    }
}

impl Drop for LoadProgress {
    fn drop(&mut self) {
        self.0.set_style(
            ProgressStyle::default_spinner()
                .template(" {prefix:.green.bold} {msg} ({elapsed_precise})"),
        );
        self.0.finish_with_message("Load complete");
    }
}

struct RenderProgress(ProgressBar);

impl RenderProgress {
    fn new(image_width: u32) -> Self {
        let bar = ProgressBar::new(image_width as u64)
            .with_prefix("[2/3]")
            .with_message("Rendering image")
            .with_style(
                ProgressStyle::default_bar()
                    .template(concat!(
                        " {prefix:.cyan.bold} {msg}",
                        "   {bar:50.dim/black.bright}",
                        "   {percent}% ({elapsed_precise}) "
                    ))
                    .progress_chars("━╾╶"),
            );
        bar.enable_steady_tick(100 /*ms*/);
        RenderProgress(bar)
    }

    fn tick(&self) {
        self.0.inc(1);
    }
}

impl Drop for RenderProgress {
    fn drop(&mut self) {
        self.0.set_style(
            ProgressStyle::default_bar()
                .template(" {prefix:.green.bold} {msg} ({elapsed_precise})"),
        );
        self.0.finish_with_message("Render complete");
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
    let renderer = Renderer {
        width: app.width,
        height: app.height,
        samples_per_pixel: app.samples,
        max_reflections: app.reflections,
    };

    let scene = {
        let _progress = LoadProgress::new();
        load_scene(&app.scene).unwrap_or_else(|e| {
            println!("Error loading scene: {}", e);
            std::process::exit(1);
        })
    };

    let img = {
        let progress = RenderProgress::new(renderer.width);
        renderer.render(&scene, || progress.tick())
    };

    img.save(&app.output).unwrap_or_else(|e| {
        println!("Could not write file: {}", e);
        std::process::exit(1);
    });
    println!(
        " {} Saved image to {}",
        style("[3/3]").green().bold(),
        app.output
    );

    report_statistics();
    report_traces();
}
