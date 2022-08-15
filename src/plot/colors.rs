use plotters::prelude::RGBAColor;
use plotters::prelude::RGBColor;
use plotters::prelude::ShapeStyle;
use plotters::prelude::GREEN;
use plotters::style::Color;

pub const GRAY: RGBColor = RGBColor(100, 100, 100);

pub fn black_stroke() -> ShapeStyle {
    ShapeStyle {
        color: RGBAColor(0, 0, 0, 1.0),
        filled: true,
        stroke_width: 1,
    }
}

pub fn lighter_stroke() -> ShapeStyle {
    ShapeStyle {
        color: GREEN.mix(1.0),
        filled: true,
        stroke_width: 1,
    }
}
