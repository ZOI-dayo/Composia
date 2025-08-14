use std::f32::consts::PI; // 円周率定数

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

/// UI / 表示要素を表すための最小限のトレイト。
pub trait Widget {
    /// 識別やデバッグ表示に使う名前を返す。
    fn name(&self) -> &str;
    /// 現在望む RGBA 色を返す（レンダリング用）。
    fn desired_color(&self) -> [f32;4];
    /// 経過時間 dt を用いた状態更新。デフォルトは何もしない。
    fn update(&mut self, _dt: f32) {}
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
/// 固定色の青ウィジェット
pub struct BlueWidget; impl Widget for BlueWidget { fn name(&self)->&str{"Blue"} fn desired_color(&self)->[f32;4]{[0.2,0.7,0.9,1.0]} }
/// 時間経過で赤成分が周期変化するウィジェット
pub struct RedWidget { pub t: f32 } impl Widget for RedWidget { fn name(&self)->&str{"Red"} fn desired_color(&self)->[f32;4]{ let r=(self.t*0.7).sin()*0.5+0.5; [r,0.1,0.1,1.0]} fn update(&mut self,dt:f32){ self.t+=dt; }}

/// デモ用に複数のウィジェットインスタンスを作成して返す。
pub fn sample_widget_instances() -> Vec<WidgetInstance> {
    vec![
        // 左半分を埋める (上だけ余白 20px)
        WidgetInstance::with_margin(
            Box::new(BlueWidget),
            FractionRect { x:0.0, y:0.0, w:0.5, h:1.0 },
            Margins { left:0.0, right:0.0, top:20.0, bottom:0.0 }
        ),
        // 右上 40% (四辺 8px)
        WidgetInstance::with_margin(
            Box::new(RedWidget{t:0.0}),
            FractionRect { x:0.5, y:0.0, w:0.5, h:0.4 },
            Margins { left:8.0, right:8.0, top:8.0, bottom:8.0 }
        ),
        // 右下残り (左右 24px 下 12px)
        WidgetInstance::with_margin(
            Box::new(BlueWidget),
            FractionRect { x:0.5, y:0.4, w:0.5, h:0.6 },
            Margins { left:24.0, right:24.0, top:0.0, bottom:12.0 }
        ),
        // 画面中央オーバーレイ (マージン無し)
        WidgetInstance::new(Box::new(RedWidget{t:PI}), FractionRect { x:0.25, y:0.25, w:0.5, h:0.5 }),
    ]
}
