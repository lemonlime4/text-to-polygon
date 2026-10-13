use glam::Vec2;

use crate::{curves::CubicBezier, sampling::Sampler};

pub struct SubdivisionSampler {
    pub flatness_tolerance: f32,
}

impl Sampler for SubdivisionSampler {
    fn sample_cubic(&self, curve: CubicBezier) -> Vec<Vec2> {
        let mut curves = vec![curve];
        let mut points = Vec::new();
        while let Some(curve) = curves.pop() {
            if curve.flatness() < self.flatness_tolerance {
                points.push(curve.p1);
            } else {
                let (curve0, curve1) = curve.split(0.5);
                curves.push(curve1);
                curves.push(curve0);
            }
        }
        points
    }
}

impl CubicBezier {
    fn flatness(&self) -> f32 {
        self.p0.midpoint(self.c1).distance(self.c0) + self.c0.midpoint(self.p1).distance(self.c1)
    }

    fn split(&self, t: f32) -> (Self, Self) {
        let a0 = self.p0.lerp(self.c0, t);
        let a1 = self.c0.lerp(self.c1, t);
        let a2 = self.c1.lerp(self.p1, t);
        let b0 = a0.lerp(a1, t);
        let b1 = a1.lerp(a2, t);
        let mid = b0.lerp(b1, t);
        (
            Self {
                p0: self.p0,
                c0: a0,
                c1: b0,
                p1: mid,
            },
            Self {
                p0: mid,
                c0: b1,
                c1: a2,
                p1: self.p1,
            },
        )
    }
}
