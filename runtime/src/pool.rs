use rayon::{ThreadPool, ThreadPoolBuildError, ThreadPoolBuilder};

pub fn create_pool(size: usize) -> Result<ThreadPool, ThreadPoolBuildError> {
    ThreadPoolBuilder::new().num_threads(size).build()
}
