use std::time::Instant;

use spdlog::info;

pub struct Startup(Instant);

impl Default for Startup {
    fn default() -> Self {
        Startup(Instant::now())
    }
}

impl Drop for Startup {
    fn drop(&mut self) {
        let total_us = self.0.elapsed().as_micros() as u32;
        let ms = total_us / 1000;
        let us = total_us % 1000;
        if ms > 0 {
            info!("Initialization finished, cost: {ms}.{us:03}ms",);
        } else {
            info!("Initialization finished, cost: {us}us",);
        }
    }
}
