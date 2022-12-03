use std::time::{Duration, Instant};

use crypto_hashes::digest::Digest;
use rand::RngCore;
use pcg::Pcg;

use log::*;

pub struct HashBencher {
  data: Vec<u8>,
  iters: usize,
}

impl HashBencher {
  pub fn new(size: usize, iters: usize) -> HashBencher {
    let mut data = Vec::with_capacity(size);
    data.resize(size, 0);
    let mut rng = Pcg::default();
    rng.fill_bytes(data.as_mut_slice());
    HashBencher { data, iters }
  }

  /// Measure a hash and return duration
  pub fn measure_hash<H: Digest>(&self, mut hash: H) -> Duration {
    let start = Instant::now();
    for _i in 0..self.iters {
      hash.update(&self.data);
    }
    let result = hash.finalize();
    let end = Instant::now();
    let hash = hex::encode(result);
    info!("hashed to {}", hash);
    end.duration_since(start)
  }

  /// Get the total bytes to hash.
  pub fn total_bytes(&self) -> usize {
    self.data.len() * self.iters
  }

  /// Convert a duration to a rate.
  pub fn duration_to_rate(&self, duration: &Duration) -> f64 {
    let bytes = self.total_bytes() as f64;
    let secs = duration.as_secs_f64();
    bytes / secs
  }
}
