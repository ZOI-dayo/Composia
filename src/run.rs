use winit::{event::{Event, WindowEvent}, event_loop::EventLoop, window::WindowBuilder}; // winit によるウィンドウ & イベント
use crate::app_state::AppState; // アプリケーション状態管理

/// メインの非同期ランループを開始し、ウィンドウ・WGPU を初期化して描画/更新を行う。
pub async fn run() {
    let event_loop = EventLoop::new().unwrap(); // 新しいイベントループ
    let window: &'static winit::window::Window = { // 'static にするため Box::leak でリーク（終了まで保持）
        let boxed = Box::new(WindowBuilder::new().build(&event_loop).unwrap());
        Box::leak(boxed)
    };

    let size = window.inner_size();
    let instance = wgpu::Instance::default(); // WGPU インスタンス生成
    let surface = instance.create_surface(window).unwrap(); // Window から描画サーフェス生成
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions { // 利用可能な GPU アダプタを選択
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor { // デバイス & キュー取得
        label: None,
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::default(),
    }, None).await.unwrap();

    let surface_caps = surface.get_capabilities(&adapter); // 対応フォーマット/モードを問い合わせ
    let surface_format = surface_caps.formats[0];
    let mut config = wgpu::SurfaceConfiguration { // スワップチェーン設定
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config); // 初期設定を適用

    let mut app = AppState::new(&device, surface_format); // アプリ状態作成
    app.ensure_gpu_widgets(&device);                      // GPU リソース初期化

    use winit::event_loop::ControlFlow;
    if let Err(e) = event_loop.run(|event, elwt| { // イベントループ開始
    elwt.set_control_flow(ControlFlow::Poll); // 常にポーリングして描画
        match event {
            Event::WindowEvent { event: WindowEvent::Resized(new_size), .. } => { // リサイズ時に再設定
                config.width = new_size.width;
                config.height = new_size.height;
                surface.configure(&device, &config);
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => { // 閉じる要求
                elwt.exit();
            }
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => { // 再描画要求
                let frame = match surface.get_current_texture() { // 現在のフレーム取得 (失敗時再設定)
                    Ok(frame) => frame,
                    Err(_) => { surface.configure(&device, &config); surface.get_current_texture().unwrap() }
                };
                let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default()); // テクスチャビュー
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Render Encoder") }); // コマンドエンコーダ
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { // レンダーパス開始
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store },
                        })],
                        depth_stencil_attachment: None,
                        occlusion_query_set: None,
                        timestamp_writes: None,
                    });
                    app.render(&mut rpass); // ウィジェット描画
                }
                queue.submit(Some(encoder.finish())); // GPU にサブミット
                frame.present(); // 画面に表示
            }
            Event::AboutToWait => { // 待機直前にアップデートと次フレーム要求
                app.update(&device, &queue, (config.width, config.height)); // 状態更新 (デバイス参照を追加)
                window.request_redraw(); // 再描画要求発行
            }
            _ => {}
        }
    }) { eprintln!("Event loop exited with error: {e}"); }
}
