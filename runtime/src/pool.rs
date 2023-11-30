use rayon::{ThreadPool, ThreadPoolBuildError, ThreadPoolBuilder};

pub fn create_pool(size: usize) -> Result<ThreadPool, ThreadPoolBuildError> {
    let pool = ThreadPoolBuilder::new().num_threads(size).build();
    pool
}
