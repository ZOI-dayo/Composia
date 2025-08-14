struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) local_pos: vec2<f32>, // quad local space (-0.5..0.5)
};

struct WidgetUniform {
    transform: mat4x4<f32>,
    color: vec4<f32>,
    size_border: vec4<f32>, // (w_px, h_px, inset_px, thickness_px)
    margins: vec4<f32>,     // (left, right, top, bottom) in px of the OUTER rect (for debug or future use)
};

@group(0) @binding(0) var<uniform> widget: WidgetUniform;

@vertex
fn vs_main(@location(0) position: vec2<f32>) -> VertexOut {
    var out: VertexOut;
    let pos = vec4<f32>(position, 0.0, 1.0);
    out.position = widget.transform * pos;
    out.local_pos = position; // pass through
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    // ピクセル単位で一定のボーダーを描画する。
    // local_pos は -0.5..0.5 の正規化（スクエア）空間。
    let w_px = widget.size_border.x;
    let h_px = widget.size_border.y;
    let inset_px = widget.size_border.z;
    let thickness_px = widget.size_border.w;

    // 各軸でエッジからの距離（ピクセル換算）を計算
    let dist_x_px = (0.5 - abs(in.local_pos.x)) * w_px;
    let dist_y_px = (0.5 - abs(in.local_pos.y)) * h_px;

    // 小さすぎるウィジェット対策: inset を半分のサイズ未満にクランプ
    let inset_x = min(inset_px, 0.5 * w_px);
    let inset_y = min(inset_px, 0.5 * h_px);

    let in_band_x = dist_x_px <= inset_x && dist_x_px > inset_x - thickness_px;
    let in_band_y = dist_y_px <= inset_y && dist_y_px > inset_y - thickness_px;
    if (in_band_x || in_band_y) { return vec4<f32>(1.0,1.0,1.0,1.0); }
    return widget.color;
}
