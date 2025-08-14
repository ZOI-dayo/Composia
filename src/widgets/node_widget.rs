use crate::widget::Widget;

// NodeWidget: 複数の角丸矩形ノードを RGBA バッファへソフト描画するウィジェット
// 座標系: ウィジェット左下を (0,0) とする中心座標 pos (x:右+, y:上+)
// size_px: (幅, 高さ) [px]
// color: RGBA (0.0-1.0)
#[derive(Clone, Debug)]
pub struct Node { pub pos:[f32;2], pub size_px:[f32;2], pub color:[f32;4] }

pub struct NodeWidget { pub nodes: Vec<Node>, pub background:[f32;4] }
impl NodeWidget {
	pub fn new(bg:[f32;4])->Self { Self { nodes:Vec::new(), background:bg } }
	pub fn with_nodes(bg:[f32;4], nodes:Vec<Node>)->Self { Self { nodes, background:bg } }
}

impl Widget for NodeWidget {
	fn name(&self)->&str { "Nodes" }
	fn draw_into(&mut self,w:u32,h:u32,p:&mut [u8]){
		println!("NodeWidget::draw_into {}x{} p:{}", w, h, p.len());
		// 背景一括塗り (既存内容は上書き)
		let bg=self.background;
		for px in p.chunks_exact_mut(4){ px[0]=(bg[0]*255.0) as u8; px[1]=(bg[1]*255.0) as u8; px[2]=(bg[2]*255.0) as u8; px[3]=(bg[3]*255.0) as u8; }
		if self.nodes.is_empty(){ return; }
		const R:f32=8.0; // 角丸半径の上限 (px)
		for node in &self.nodes {
			// 中心座標
			let cx = node.pos[0];
			let cy = node.pos[1];
			// 半幅/半高 (最低 1px) と角丸半径決定
			let half_w=(node.size_px[0].max(0.0)*0.5).max(1.0);
			let half_h=(node.size_px[1].max(0.0)*0.5).max(1.0);
			let r=R.min(half_w.min(half_h));
			// core_*: 角丸部分を除いた矩形コア領域 (十字形) の半幅/半高
			let core_w=half_w-r; let core_h=half_h-r;
			// 対象 AABB を整数ピクセルに (境界内へ再クランプ)
			let x0=(cx-half_w).floor().max(0.0) as i32; let x1=(cx+half_w).ceil().min(w as f32 -1.0) as i32;
			let y0=(cy-half_h).floor().max(0.0) as i32; let y1=(cy+half_h).ceil().min(h as f32 -1.0) as i32;
			for py in y0..=y1 { for px_i in x0..=x1 {
				// 中心からの距離 (対称性利用のため絶対値)
				let dx=(px_i as f32 - cx).abs(); let dy=(py as f32 - cy).abs();
				// 角丸矩形: 中央の十字 (core) + 4 隅を円弧で補完
				let inside_core = (dx <= core_w && dy <= half_h) || (dy <= core_h && dx <= half_w);
				let mut alpha=0.0;
				if inside_core { alpha=1.0; } else {
					// 円弧領域: core を越えた部分の距離を計算
					let qx=(dx-core_w).max(0.0); let qy=(dy-core_h).max(0.0);
					let dist=(qx*qx+qy*qy).sqrt();
					// 端 1px で簡易フェード (アンチエイリアス的緩和)
					if dist<=r { alpha=1.0-((dist-(r-1.0)).clamp(0.0,1.0)); }
				}
				if alpha<=0.0 { continue; }
				// ピクセル位置 (RGBA 4byte)
				let idx=((py as u32 * w + px_i as u32)*4) as usize;
				let rgba=&mut p[idx..idx+4];
				// アルファ合成 (src over dst)。dst は既に bg か前ノードの結果。
				let src=node.color; let src_a=(src[3]*alpha).clamp(0.0,1.0); let inv=1.0-src_a;
				let dr=rgba[0] as f32/255.0; let dg=rgba[1] as f32/255.0; let db=rgba[2] as f32/255.0; let da=rgba[3] as f32/255.0;
				let out_a=src_a + da*inv; // A = a_s + a_d*(1-a_s)
				let out_r=(src[0]*src_a + dr*da*inv)/out_a.max(1e-6);
				let out_g=(src[1]*src_a + dg*da*inv)/out_a.max(1e-6);
				let out_b=(src[2]*src_a + db*da*inv)/out_a.max(1e-6);
				rgba[0]=(out_r*255.0) as u8; rgba[1]=(out_g*255.0) as u8; rgba[2]=(out_b*255.0) as u8; rgba[3]=(out_a*255.0) as u8;
			}}
		}
	}
}

