use glam::Vec2;

use crate::{
    curves::{CubicBezier, Curve},
    sampling::Sampler,
};

pub struct FixedCountSampler {
    count: u32,
}

impl FixedCountSampler {
    pub fn new(count: u32) -> Self {
        Self { count }
    }
}

impl Sampler for FixedCountSampler {
    fn sample_cubic(&self, curve: CubicBezier) -> Vec<Vec2> {
        (1..=self.count)
            .map(move |i| {
                let t = i as f32 / self.count as f32;
                curve.eval(t)
            })
            .collect()
    }
}
