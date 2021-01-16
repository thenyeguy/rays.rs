#[allow(unused_variables)]
pub fn start(filename: &str) {
    #[cfg(feature = "profile")]
    {
        cpuprofiler::PROFILER
            .lock()
            .unwrap()
            .start(filename)
            .unwrap();
    }
}

pub fn end() {
    #[cfg(feature = "profile")]
    {
        cpuprofiler::PROFILER.lock().unwrap().stop().unwrap();
    }
}
