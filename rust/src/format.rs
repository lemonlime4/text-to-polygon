use glam::Vec2;
use std::fmt;

pub struct PolygonFormatter<'a>(pub &'a [Vec2]);

impl fmt::Display for PolygonFormatter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let points = self.0;
        let precision = 5;

        f.write_str("\\operatorname{polygon}\\left(\\left[")?;

        for x in points.iter().map(|v| v.x) {
            if x.is_nan() {
                f.write_str("0\\infinity,")?;
            } else {
                write!(f, "{x:.0$},", precision)?;
            }
        }

        f.write_str("0\\infty\\right],\\left[")?;

        for y in points.iter().map(|v| v.y) {
            if y.is_nan() {
                f.write_str("0\\infinity,")?;
            } else {
                write!(f, "{y:.0$},", precision)?;
            }
        }

        f.write_str("0\\infty\\right]\\right)")?;
        Ok(())
    }
}
