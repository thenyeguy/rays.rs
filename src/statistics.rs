use std::sync::atomic::AtomicUsize;

pub static RAYS_CAST: AtomicUsize = AtomicUsize::new(0);
pub static BOUNDING_BOX_TESTS: AtomicUsize = AtomicUsize::new(0);
pub static TRIANGLE_TESTS: AtomicUsize = AtomicUsize::new(0);

#[macro_export]
macro_rules! increment_statistic {
    ($stat:expr) => {
        if cfg!(feature = "statistics") {
            $stat.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    };
}

macro_rules! print_statistic {
    ($label:expr,$stat:expr) => {
        let count = $stat.load(std::sync::atomic::Ordering::SeqCst);
        println!("  {:<20} {}", $label, count);
    };
}

pub fn report_statistics() {
    if cfg!(feature = "statistics") {
        println!();
        println!("Statistics:");
        print_statistic!("Rays cast:", RAYS_CAST);
        print_statistic!("Bounding box tests:", BOUNDING_BOX_TESTS);
        print_statistic!("Triangle tests:", TRIANGLE_TESTS);
    }
}
