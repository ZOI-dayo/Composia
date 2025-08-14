use crate::widget::Widget; use rand::RngCore; use rand::SeedableRng;

pub struct Noise { pub rng: rand::rngs::SmallRng }
impl Noise { pub fn new(seed:u64)->Self{ Self{ rng: rand::rngs::SmallRng::seed_from_u64(seed)} } }
impl Widget for Noise {
    fn name(&self)->&str{"Noise"}
    fn update(&mut self,_dt:f32){}
    fn draw_into(&mut self,w:u32,h:u32,p:&mut [u8]){ let n=(w as usize)*(h as usize); for i in 0..n { let v=(self.rng.next_u32()&0xFF) as u8; let o=i*4; p[o]=v; p[o+1]=v; p[o+2]=v; p[o+3]=255; } }
}
