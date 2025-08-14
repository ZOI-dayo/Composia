// ジオメトリ関連ユーティリティ（現状 GPU デバイスとの直接やり取りはなし）

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
/// 2D 位置のみを持つ最小限の頂点構造体
pub struct Vertex { pub position: [f32; 2] }

/// 単純な正方形(中心原点, 一辺 1.0) の頂点/インデックス配列を生成する関数。
/// 戻り値: (頂点配列, インデックス配列)
pub fn quad_vertices() -> (Vec<Vertex>, Vec<u16>) {
    let v = vec![
        Vertex { position: [-0.5, -0.5] },
        Vertex { position: [ 0.5, -0.5] },
        Vertex { position: [ 0.5,  0.5] },
        Vertex { position: [-0.5,  0.5] },
    ];
    let i = vec![0, 1, 2, 2, 3, 0];
    (v, i)
}

impl Vertex {
    /// シェーダ側 location=0 に (float2) で渡す頂点属性定義
    pub const ATTRS: [wgpu::VertexAttribute;1] = [wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x2, offset: 0, shader_location: 0 }];
    /// wgpu パイプラインに登録する頂点バッファレイアウト
    pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, // 1頂点のバイト幅
        step_mode: wgpu::VertexStepMode::Vertex, // 頂点毎に進める
        attributes: &Self::ATTRS,
    };
}
