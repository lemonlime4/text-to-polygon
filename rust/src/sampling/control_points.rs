///! debugging use
use glam::Vec2;

use crate::{
    curves::{CubicBezier, QuadBezier},
    sampling::Sampler,
};

pub struct ControlPointsSampler<const TO_CUBIC: bool = false>;

impl<const TO_CUBIC: bool> Sampler for ControlPointsSampler<TO_CUBIC> {
    fn sample_cubic(&self, curve: CubicBezier) -> Vec<Vec2> {
        vec![curve.c0, curve.c1, curve.p1]
    }

    fn sample_quad(&self, curve: QuadBezier) -> Vec<Vec2> {
        if TO_CUBIC {
            self.sample_cubic(curve.into())
        } else {
            vec![curve.c, curve.p1]
        }
    }
}
