use std::ops::Range;

use rand::{Rng, SeedableRng};

#[derive(clap::Args, Debug, Copy, Clone)]
pub struct Opts {
    /// The number of random weights to generate
    #[arg(short, long, default_value_t = 300)]
    n_weights: usize,
    /// The maximum weight value
    #[arg(short = 'M', long, default_value_t = 100)]
    max_weight: usize,
    /// The minimum weight value
    #[arg(short, long, default_value_t = 1)]
    min_weight: usize,
}

pub fn get(seed: u64, opts: Opts) -> impl Iterator<Item = usize> {
    Iter {
        remaining: opts.n_weights,
        range: opts.min_weight..opts.max_weight + 1,
        rng: rand_chacha::ChaCha12Rng::seed_from_u64(seed),
    }
}

#[derive(Debug)]
struct Iter {
    remaining: usize,
    range: Range<usize>,
    rng: rand_chacha::ChaCha12Rng,
}

impl Iterator for Iter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        Some(self.rng.random_range(self.range.clone()))
    }
}
