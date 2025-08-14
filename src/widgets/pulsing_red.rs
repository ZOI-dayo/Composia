use crate::widget::Widget;

pub struct PulsingRed { pub t:f32 }
impl Widget for PulsingRed { fn name(&self)->&str{"PulseRed"} fn update(&mut self,dt:f32){ self.t+=dt; } fn draw_into(&mut self,w:u32,h:u32,p:&mut [u8]){ let r=((self.t*0.7).sin()*0.5+0.5) as f32; for y in 0..h { for x in 0..w { let o=((y*w+x)*4) as usize; p[o]=(r*255.0) as u8; p[o+1]=30; p[o+2]=30; p[o+3]=255; } } }}
