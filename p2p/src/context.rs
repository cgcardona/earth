use futures_cpupool::CpuPool;

/// Network context.
pub struct Context {
    pool: CpuPool,
}

impl Context {
    pub fn new(pool: CpuPool) -> Self {
        Context { pool: pool }
    }
}
