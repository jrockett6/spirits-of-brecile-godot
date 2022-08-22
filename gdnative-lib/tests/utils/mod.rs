use futures::prelude::*;
use tokio::runtime::{Builder, Runtime};

pub struct Utils {
    runtime: Runtime,
}

impl Default for Utils {
    fn default() -> Self {
        Self {
            runtime: Builder::new_current_thread()
                .enable_io()
                .enable_time()
                .build()
                .unwrap(),
        }
    }
}
impl Utils {
    pub fn go<F: Future>(f: F) {
        let rt = Utils::default().runtime;
        rt.block_on(f);
    }
}
