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
    /// 親側レイアウト情報 (ウィジェットのグローバル左上 px / サイズ px / 全体サーフェス px) を通知。
    fn set_layout(&mut self, _origin_px:[f32;2], _widget_size_px:[f32;2], _surface_px:[f32;2]) {}
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

// 個別 widget 実装モジュール
pub mod widgets {
    pub use crate::widgets::solid_color::SolidColor;
    pub use crate::widgets::vertical_gradient::VerticalGradient;
    pub use crate::widgets::noise::Noise;
    pub use crate::widgets::pulsing_red::PulsingRed;
    pub use crate::widgets::node_widget::{NodeWidget, Node};
}

/// デモ用に複数のウィジェットインスタンスを作成して返す。
pub fn sample_widget_instances() -> Vec<WidgetInstance> {
    vec![
        WidgetInstance::with_margin(
            Box::new(widgets::VerticalGradient{ top:[0.2,0.7,0.9,1.0], bottom:[0.0,0.0,0.2,1.0]}),
            FractionRect { x:0.0, y:0.0, w:0.5, h:1.0 },
            Margins { left:0.0, right:0.0, top:20.0, bottom:0.0 }
        ),
        WidgetInstance::with_margin(
            Box::new(widgets::PulsingRed{t:0.0}),
            FractionRect { x:0.5, y:0.0, w:0.5, h:0.4 },
            Margins { left:8.0, right:8.0, top:8.0, bottom:8.0 }
        ),
        WidgetInstance::with_margin(
            Box::new(widgets::Noise::new(42)),
            FractionRect { x:0.5, y:0.4, w:0.5, h:0.6 },
            Margins { left:24.0, right:24.0, top:0.0, bottom:12.0 }
        ),
    // NodeWidget (px ベース): ノード中心座標 / サイズは NodeWidget の描画領域 左下 を (0,0) とするピクセル値
        // ここではウィジェット想定サイズ  (後で実際の割当 px に依存) に対し 400x300 程度を想定したデモ値を配置
        WidgetInstance::new(Box::new(widgets::NodeWidget::with_nodes([0.05,0.05,0.08,0.25], vec![
            // pos:[x,y] (中心, 左下原点), size_px:[w,h]
            widgets::Node { pos:[ 60.0,  60.0], size_px:[140.0, 80.0], color:[1.0,0.35,0.35,0.95]},
            widgets::Node { pos:[180.0, 160.0], size_px:[110.0,110.0], color:[0.30,0.85,0.45,0.85]},
            widgets::Node { pos:[300.0, 120.0], size_px:[100.0, 80.0], color:[0.30,0.45,1.0,0.90]},
            widgets::Node { pos:[340.0, 100.0], size_px:[130.0,100.0], color:[1.0,0.82,0.25,0.88]},
            widgets::Node { pos:[280.0, 220.0], size_px:[ 70.0, 70.0], color:[0.9,0.2,1.0,0.9]},
        ])), FractionRect { x:0.25, y:0.25, w:0.5, h:0.5 }),
    ]
}
