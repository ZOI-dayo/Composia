use crate::widget::Widget;

pub struct VerticalGradient { pub top:[f32;4], pub bottom:[f32;4] }
impl Widget for VerticalGradient {
    fn name(&self)->&str{"VGrad"}
    fn draw_into(&mut self,w:u32,h:u32,p:&mut [u8]){
        for y in 0..h { let t = y as f32 / (h.max(1)-1) as f32; let mut row_color=[0.0f32;4];
            for i in 0..4 { row_color[i] = self.top[i]*(1.0-t)+self.bottom[i]*t; }
            let row_start = (y*w*4) as usize; let row_slice=&mut p[row_start..row_start+(w*4) as usize];
            for px in row_slice.chunks_exact_mut(4) { px[0]=(row_color[0]*255.0) as u8; px[1]=(row_color[1]*255.0) as u8; px[2]=(row_color[2]*255.0) as u8; px[3]=(row_color[3]*255.0) as u8; }
        }
    }
}
