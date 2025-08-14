use crate::widget::Widget;

pub struct SolidColor(pub [f32;4]);
impl Widget for SolidColor { fn name(&self)->&str{"Solid"} fn desired_color(&self)->[f32;4]{ self.0 } }
