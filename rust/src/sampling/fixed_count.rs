use glam::Vec2;

use crate::{
    curves::{CubicBezier, Curve},
    sampling::Sampler,
};

pub struct FixedCountSampler(pub u32);

impl Sampler for FixedCountSampler {
    fn sample_cubic(&self, curve: CubicBezier) -> Vec<Vec2> {
        let n = self.0;
        (1..=n)
            .map(move |i| {
                let t = i as f32 / n as f32;
                curve.eval(t)
            })
            .collect()
    }
}
