use crate::widget::Widget;

#[derive(Clone, Debug)]
pub struct Node { pub pos:[f32;2], pub size_frac:[f32;2], pub color:[f32;4] }

pub struct NodeWidget { pub nodes: Vec<Node>, pub background:[f32;4] }
impl NodeWidget {
	pub fn new(bg:[f32;4])->Self { Self { nodes:Vec::new(), background:bg } }
	pub fn with_nodes(bg:[f32;4], nodes:Vec<Node>)->Self { Self { nodes, background:bg } }
}

impl Widget for NodeWidget {
	fn name(&self)->&str { "Nodes" }
	fn draw_into(&mut self,w:u32,h:u32,p:&mut [u8]){
		let bg=self.background;
		for px in p.chunks_exact_mut(4){ px[0]=(bg[0]*255.0) as u8; px[1]=(bg[1]*255.0) as u8; px[2]=(bg[2]*255.0) as u8; px[3]=(bg[3]*255.0) as u8; }
		if self.nodes.is_empty(){ return; }
		const R:f32=8.0;
		for node in &self.nodes {
			let cx=node.pos[0].clamp(0.0,1.0)*(w as f32 -1.0);
			let cy=node.pos[1].clamp(0.0,1.0)*(h as f32 -1.0);
			let half_w=(node.size_frac[0].max(0.0)*w as f32*0.5).max(1.0);
			let half_h=(node.size_frac[1].max(0.0)*h as f32*0.5).max(1.0);
			let r=R.min(half_w.min(half_h));
			let core_w=half_w-r; let core_h=half_h-r;
			let x0=(cx-half_w).floor().max(0.0) as i32; let x1=(cx+half_w).ceil().min(w as f32 -1.0) as i32;
			let y0=(cy-half_h).floor().max(0.0) as i32; let y1=(cy+half_h).ceil().min(h as f32 -1.0) as i32;
			for py in y0..=y1 { for px_i in x0..=x1 {
				let dx=(px_i as f32 - cx).abs(); let dy=(py as f32 - cy).abs();
				let inside_core = dx <= core_w && dy <= half_h || dy <= core_h && dx <= half_w;
				let mut alpha=0.0;
				if inside_core { alpha=1.0; } else {
					let qx=(dx-core_w).max(0.0); let qy=(dy-core_h).max(0.0);
					let dist=(qx*qx+qy*qy).sqrt();
					if dist<=r { alpha=1.0-((dist-(r-1.0)).clamp(0.0,1.0)); }
				}
				if alpha<=0.0 { continue; }
				let idx=((py as u32 * w + px_i as u32)*4) as usize;
				let rgba=&mut p[idx..idx+4];
				let src=node.color; let src_a=(src[3]*alpha).clamp(0.0,1.0); let inv=1.0-src_a;
				let dr=rgba[0] as f32/255.0; let dg=rgba[1] as f32/255.0; let db=rgba[2] as f32/255.0; let da=rgba[3] as f32/255.0;
				let out_a=src_a + da*inv;
				let out_r=(src[0]*src_a + dr*da*inv)/out_a.max(1e-6);
				let out_g=(src[1]*src_a + dg*da*inv)/out_a.max(1e-6);
				let out_b=(src[2]*src_a + db*da*inv)/out_a.max(1e-6);
				rgba[0]=(out_r*255.0) as u8; rgba[1]=(out_g*255.0) as u8; rgba[2]=(out_b*255.0) as u8; rgba[3]=(out_a*255.0) as u8;
			}}
		}
	}
}

