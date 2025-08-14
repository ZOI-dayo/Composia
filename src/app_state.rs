use wgpu::util::DeviceExt; // WGPU の補助トレイト（バッファ初期化など）
use crate::geometry::{quad_vertices, Vertex}; // 四角形ジオメトリと頂点レイアウト
use crate::widget::{WidgetInstance, sample_widget_instances}; // Widget インスタンス生成関数

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct WidgetUniform {
    pub transform: [[f32;4];4], // 64 bytes
    pub color: [f32;4],         // 16 bytes
    pub size_border: [f32;4],   // (w_px, h_px, inset_px, thickness_px) 16 bytes
    pub margins: [f32;4],       // (left, right, top, bottom) 16 bytes
} // 112 bytes total

pub struct GpuWidget {
    #[allow(dead_code)] pub widget_index: usize, // 対応するウィジェット番号（将来の参照用）
    pub uniform_buf: wgpu::Buffer,              // 個別ユニフォームバッファ
    pub bind_group: wgpu::BindGroup,            // 描画時にセットするバインドグループ
}

pub struct AppState {
    pub widgets: Vec<WidgetInstance>,           // CPU 側のウィジェット + レイアウト矩形
    pub gpu_widgets: Vec<GpuWidget>,            // GPU リソースを持った対応配列
    pub bind_group_layout: wgpu::BindGroupLayout, // ユニフォーム用レイアウト
    pub render_pipeline: wgpu::RenderPipeline,  // 描画パイプライン
    pub vertex_buffer: wgpu::Buffer,            // 四角形頂点バッファ
    pub index_buffer: wgpu::Buffer,             // インデックスバッファ
    pub num_indices: u32,                       // 描画インデックス数
    pub last_frame_time: std::time::Instant,    // 前フレーム時刻（Δt計算）
}

impl AppState {
    pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> Self {
        let (vertices, indices) = quad_vertices();          // 1枚の四角形メッシュを取得
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),      // CPU 配列→GPU バッファ転送
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),       // インデックスデータ
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = indices.len() as u32;             // 描画する三角形数 * 3

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("widget BGL"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0, // ユニフォーム一つだけの単純レイアウト
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                count: None,
            }],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("Widget Shader"), source: wgpu::ShaderSource::Wgsl(include_str!("shaders/quad.wgsl").into()) }); // WGSL を読み込み
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline layout"),
            bind_group_layouts: &[&bind_group_layout], // 1 つの BGL を使う
            push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: "vs_main", buffers: &[Vertex::LAYOUT], compilation_options: Default::default() }, // 頂点入力設定
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: "fs_main", targets: &[Some(wgpu::ColorTargetState { format: surface_format, blend: Some(wgpu::BlendState::REPLACE), write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),
            primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::TriangleList, strip_index_format: None, front_face: wgpu::FrontFace::Ccw, cull_mode: Some(wgpu::Face::Back), polygon_mode: wgpu::PolygonMode::Fill, unclipped_depth: false, conservative: false }, // 基本的なラスタライズ設定
            depth_stencil: None, // 深度不要（2D）
            multisample: wgpu::MultisampleState { count: 1, mask: !0, alpha_to_coverage_enabled: false },
            multiview: None,
        });

    // 初期ウィジェット群（正規化レイアウト）。サイズ依存しないため仮サイズ不要。
    let widgets: Vec<WidgetInstance> = sample_widget_instances();
        let gpu_widgets = Vec::new();                         // GPU 側は後で確保

        Self { widgets, gpu_widgets, bind_group_layout, render_pipeline, vertex_buffer, index_buffer, num_indices, last_frame_time: std::time::Instant::now() }
    }

    pub fn ensure_gpu_widgets(&mut self, device: &wgpu::Device) {
        while self.gpu_widgets.len() < self.widgets.len() { // 新規ウィジェット分だけ GPU リソース確保
            let idx = self.gpu_widgets.len();
            let initial = WidgetUniform { transform: glam::Mat4::IDENTITY.to_cols_array_2d(), color: [0.0;4], size_border: [0.0;4], margins: [0.0;4] }; // 初期は透明
            let buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("widget uniform"), contents: bytemuck::bytes_of(&initial), usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST }); // 後で上書き可能
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("widget bind group"), layout: &self.bind_group_layout, entries: &[wgpu::BindGroupEntry { binding: 0, resource: buf.as_entire_binding() }] });
            self.gpu_widgets.push(GpuWidget { widget_index: idx, uniform_buf: buf, bind_group }); // 登録
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, surface_size: (u32,u32)) {
        let now = std::time::Instant::now();
        let dt = (now - self.last_frame_time).as_secs_f32(); // 経過秒数
        self.last_frame_time = now;                          // 次回へ更新
        for inst in &mut self.widgets { inst.widget.update(dt); }         // 各ウィジェットのロジック更新
        self.layout_and_upload(queue, surface_size);         // レイアウト計算 + GPU へ書き込み
    }

    fn layout_and_upload(&mut self, queue: &wgpu::Queue, surface_size: (u32,u32)) {
        let (sw, sh) = (surface_size.0 as f32, surface_size.1 as f32); // 画面サイズ（ピクセル→f32）
        // 将来的にレイアウトエンジンで rect を再計算する場合はここで widgets[..].rect を更新する。
        for (inst, gpu) in self.widgets.iter().zip(self.gpu_widgets.iter()) {
            // 正規化 0..1 の矩形を現在のウィンドウサイズへ拡大（外側矩形）
            let outer_x = inst.frac.x * sw; let outer_y = inst.frac.y * sh; let outer_w = inst.frac.w * sw; let outer_h = inst.frac.h * sh;
            // 各方向マージン（過剰ならクランプ）
            let ml = inst.margin.left.min(outer_w * 0.5).max(0.0);
            let mr = inst.margin.right.min(outer_w * 0.5).max(0.0);
            let mt = inst.margin.top.min(outer_h * 0.5).max(0.0);
            let mb = inst.margin.bottom.min(outer_h * 0.5).max(0.0);
            // 内側（描画領域）
            let x = outer_x + ml; // 左端
            let y = outer_y + mt; // 上端（ウィンドウ座標系で上から）
            let w = (outer_w - ml - mr).max(0.0);
            let h = (outer_h - mt - mb).max(0.0);
            // NDC 変換用 中心とスケール
            let tx = (x + w * 0.5) / sw * 2.0 - 1.0; // 中心 X (NDC)
            let ty = 1.0 - (y + h * 0.5) / sh * 2.0; // 中心 Y (NDC, 反転)
            let sx = (w / sw) * 2.0; // NDC スケール
            let sy = (h / sh) * 2.0; // NDC スケール
            let transform = glam::Mat4::from_scale_rotation_translation(
                glam::vec3(sx, sy, 1.0),
                glam::Quat::IDENTITY,
                glam::vec3(tx, ty, 0.0),
            );
            // ピクセルベースのボーダー指定: 固定値（後で設定画面で変更可能にする想定）
            const BORDER_INSET_PX: f32 = 16.0;     // 外枠から内側へのオフセット
            const BORDER_THICKNESS_PX: f32 = 4.0;  // ボーダーの太さ
            let size_border = [w, h, BORDER_INSET_PX, BORDER_THICKNESS_PX];
            let uni = WidgetUniform { transform: transform.to_cols_array_2d(), color: inst.widget.desired_color(), size_border, margins: [ml, mr, mt, mb] };
            queue.write_buffer(&gpu.uniform_buf, 0, bytemuck::bytes_of(&uni));
        }
    }

    pub fn render<'r>(&'r self, rpass: &mut wgpu::RenderPass<'r>) {
        rpass.set_pipeline(&self.render_pipeline); // 使用するパイプライン
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..)); // 共通頂点バッファ
        rpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // インデックス設定
        for gw in &self.gpu_widgets { // 各ウィジェットごとにユニフォームを差し替えて描画
            rpass.set_bind_group(0, &gw.bind_group, &[]);
            rpass.draw_indexed(0..self.num_indices, 0, 0..1); // 1 インスタンス描画
        }
    }
}
