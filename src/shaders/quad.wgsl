struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) local_pos: vec2<f32>, // quad local space (-0.5..0.5)
};

struct WidgetUniform {
    transform: mat4x4<f32>,
    color: vec4<f32>,
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
    // Draw white border slightly inset; fill with widget color elsewhere.
    let half_size = 0.5;
    let border_inset = 0.04; // inset from actual edge to place border line region
    let border_thickness = 0.01; // thickness of border band
    let ax = abs(in.local_pos.x);
    let ay = abs(in.local_pos.y);
    let outer_limit = half_size - border_inset;
    let inner_limit = outer_limit - border_thickness;
    let is_in_border_band = (ax > inner_limit && ax <= outer_limit) || (ay > inner_limit && ay <= outer_limit);
    if (is_in_border_band) { return vec4<f32>(1.0,1.0,1.0,1.0); }
    return widget.color;
}
