use glam::Vec2;

use crate::curves::{CubicBezier, Curve, Line, QuadBezier};

/// Sample points in the range (0,1]
pub trait Sampler {
    fn sample_cubic(&self, curve: CubicBezier) -> Vec<Vec2>;

    fn sample_quad(&self, curve: QuadBezier) -> Vec<Vec2> {
        self.sample_cubic(CubicBezier::from(curve))
    }

    fn sample_line(&self, line: Line) -> Vec<Vec2> {
        return vec![line.p1];
    }
}

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
