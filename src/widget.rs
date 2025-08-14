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
pub struct WidgetInstance { pub widget: Box<dyn Widget>, pub frac: FractionRect }

impl WidgetInstance {
    pub fn new(widget: Box<dyn Widget>, frac: FractionRect) -> Self { Self { widget, frac: frac.clamp() } }
}

// Example Widgets --------------------------------------------------------------
/// 固定色の青ウィジェット
pub struct BlueWidget; impl Widget for BlueWidget { fn name(&self)->&str{"Blue"} fn desired_color(&self)->[f32;4]{[0.2,0.7,0.9,1.0]} }
/// 時間経過で赤成分が周期変化するウィジェット
pub struct RedWidget { pub t: f32 } impl Widget for RedWidget { fn name(&self)->&str{"Red"} fn desired_color(&self)->[f32;4]{ let r=(self.t*0.7).sin()*0.5+0.5; [r,0.1,0.1,1.0]} fn update(&mut self,dt:f32){ self.t+=dt; }}

/// デモ用に複数のウィジェットインスタンスを作成して返す。
pub fn sample_widget_instances() -> Vec<WidgetInstance> {
    vec![
        // 左半分を埋める
        WidgetInstance::new(Box::new(BlueWidget), FractionRect { x:0.0, y:0.0, w:0.5, h:1.0 }),
        // 右上 40%
        WidgetInstance::new(Box::new(RedWidget{t:0.0}), FractionRect { x:0.5, y:0.0, w:0.5, h:0.4 }),
        // 右下残り
        WidgetInstance::new(Box::new(BlueWidget), FractionRect { x:0.5, y:0.4, w:0.5, h:0.6 }),
        // 画面中央オーバーレイ
        WidgetInstance::new(Box::new(RedWidget{t:PI}), FractionRect { x:0.25, y:0.25, w:0.5, h:0.5 }),
    ]
}
