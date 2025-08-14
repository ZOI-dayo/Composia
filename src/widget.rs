// use std::f32::consts::PI; // (以前のデモで使用) 今は未使用

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FractionRect { pub x: f32, pub y: f32, pub w: f32, pub h: f32 } // 0..1 の正規化座標
impl FractionRect {
    pub fn clamp(mut self) -> Self { // 範囲外を丸める簡易ガード
        self.x = self.x.clamp(0.0, 1.0); self.y = self.y.clamp(0.0, 1.0);
        self.w = self.w.max(0.0).min(1.0 - self.x);
        self.h = self.h.max(0.0).min(1.0 - self.y);
        self
    }
}

/// UI / 表示要素: CPU 側でピクセルを生成し親へ渡す能力を持つ。
pub trait Widget {
    /// 識別やデバッグ表示用の名前。
    fn name(&self) -> &str;
    /// 背景のベース色（テクスチャとブレンド）。
    fn desired_color(&self) -> [f32;4] { [0.0,0.0,0.0,1.0] }
    /// 経過時間 dt に基づく内部更新。
    fn update(&mut self, _dt: f32) {}
    /// 与えられた RGBA8 ピクセルバッファ (width*height*4) を塗りつぶす。
    /// width,height は親側の割当て。実装側はサイズ依存描画を行う。
    fn draw_into(&mut self, _width: u32, _height: u32, _pixels: &mut [u8]) {
        // 既定では単色 desired_color で塗る
        let c = self.desired_color();
        for px in _pixels.chunks_exact_mut(4) { px[0]=(c[0]*255.0) as u8; px[1]=(c[1]*255.0) as u8; px[2]=(c[2]*255.0) as u8; px[3]=(c[3]*255.0) as u8; }
    }
}

/// Widget とそのレイアウト（正規化 0..1 矩形）をペアにした構造体。
/// 実ピクセルサイズへは毎フレーム変換されるためウィンドウリサイズに追従する。
/// ピクセル単位のマージン（左/右/上/下 それぞれ独立）
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Margins { pub left: f32, pub right: f32, pub top: f32, pub bottom: f32 }
impl Margins { pub const ZERO: Self = Self { left:0.0, right:0.0, top:0.0, bottom:0.0 }; }

/// 1 ウィジェット + その正規化レイアウト矩形 + マージン
pub struct WidgetInstance { pub widget: Box<dyn Widget>, pub frac: FractionRect, pub margin: Margins }

impl WidgetInstance {
    /// マージン無し（ゼロ）で生成
    pub fn new(widget: Box<dyn Widget>, frac: FractionRect) -> Self { Self { widget, frac: frac.clamp(), margin: Margins::ZERO } }
    /// 明示的マージン付きで生成
    pub fn with_margin(widget: Box<dyn Widget>, frac: FractionRect, margin: Margins) -> Self { Self { widget, frac: frac.clamp(), margin } }
}

// Example Widgets --------------------------------------------------------------
/// 固定色塗り (基底実装そのまま)
pub struct SolidColor(pub [f32;4]);
impl Widget for SolidColor { fn name(&self)->&str{"Solid"} fn desired_color(&self)->[f32;4]{ self.0 } }

/// 垂直グラデーション (top -> bottom)
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

/// ランダムノイズ (毎フレーム更新)
pub struct Noise { pub rng: rand::rngs::SmallRng }
impl Noise { pub fn new(seed:u64)->Self{ use rand::SeedableRng; Self{ rng: rand::rngs::SmallRng::seed_from_u64(seed)} } }
impl Widget for Noise {
    fn name(&self)->&str{"Noise"}
    fn update(&mut self,_dt:f32){ /* could animate parameters */ }
    fn draw_into(&mut self,w:u32,h:u32,p:&mut [u8]){
    use rand::RngCore; let n = (w as usize)*(h as usize);
    for i in 0..n { let v: u8 = (self.rng.next_u32() & 0xFF) as u8; let o=i*4; p[o]=v; p[o+1]=v; p[o+2]=v; p[o+3]=255; }
    }
}

/// 時間で赤成分を変化させる動的サンプル
pub struct PulsingRed { pub t:f32 }
impl Widget for PulsingRed { fn name(&self)->&str{"PulseRed"} fn update(&mut self,dt:f32){ self.t+=dt; } fn draw_into(&mut self,w:u32,h:u32,p:&mut [u8]){
    let r=((self.t*0.7).sin()*0.5+0.5) as f32; for y in 0..h { for x in 0..w { let o = ((y*w+x)*4) as usize; p[o]=(r*255.0) as u8; p[o+1]=30; p[o+2]=30; p[o+3]=255; }} }}

/// デモ用に複数のウィジェットインスタンスを作成して返す。
pub fn sample_widget_instances() -> Vec<WidgetInstance> {
    vec![
        WidgetInstance::with_margin(
            Box::new(VerticalGradient{ top:[0.2,0.7,0.9,1.0], bottom:[0.0,0.0,0.2,1.0]}),
            FractionRect { x:0.0, y:0.0, w:0.5, h:1.0 },
            Margins { left:0.0, right:0.0, top:20.0, bottom:0.0 }
        ),
        WidgetInstance::with_margin(
            Box::new(PulsingRed{t:0.0}),
            FractionRect { x:0.5, y:0.0, w:0.5, h:0.4 },
            Margins { left:8.0, right:8.0, top:8.0, bottom:8.0 }
        ),
        WidgetInstance::with_margin(
            Box::new(Noise::new(42)),
            FractionRect { x:0.5, y:0.4, w:0.5, h:0.6 },
            Margins { left:24.0, right:24.0, top:0.0, bottom:12.0 }
        ),
        WidgetInstance::new(Box::new(SolidColor([0.1,0.2,0.6,1.0])), FractionRect { x:0.25, y:0.25, w:0.5, h:0.5 }),
    ]
}
