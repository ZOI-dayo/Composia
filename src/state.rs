use imgui_wgpu::Renderer;
use imgui_winit_support::WinitPlatform;
use std::time::Instant;
use crate::graphics::GraphicsState;
use crate::imgui_setup::ImGuiSetup;
use crate::ui::render_ui;

pub struct State {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub imgui: imgui::Context,
    pub imnodes: imnodes::Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub last_frame: Instant,
    pub value: usize,
    pub choices: [&'static str; 2],
}

impl State {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();

        // GraphicsStateを初期化
        let graphics_state = GraphicsState::new(&window).await;

        // ImGuiSetupを初期化
        let imgui_setup = ImGuiSetup::new_with_device(
            &window,
            &graphics_state.device,
            &graphics_state.queue,
            graphics_state.config.format,
        );

        let last_frame = Instant::now();

        Self {
            surface: graphics_state.surface,
            device: graphics_state.device,
            queue: graphics_state.queue,
            config: graphics_state.config,
            size,
            imgui: imgui_setup.imgui,
            imnodes: imgui_setup.imnodes,
            platform: imgui_setup.platform,
            renderer: imgui_setup.renderer,
            last_frame,
            value: 0,
            choices: ["test test this is 1", "test test this is 2"],
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width.max(1);
            self.config.height = new_size.height.max(1);
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.imgui.io_mut().update_delta_time(now - self.last_frame);
        self.last_frame = now;
    }

    pub fn render(&mut self, window: &winit::window::Window) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.platform
            .prepare_frame(self.imgui.io_mut(), &window)
            .expect("Failed to prepare frame");
        
        let ui = self.imgui.frame();

        // UIを描画
        render_ui(&ui, &mut self.value, &self.choices, &mut self.imnodes);

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        self.platform.prepare_render(&ui, &window);
        let draw_data = self.imgui.render();

        self.renderer
            .render(&draw_data, &self.queue, &self.device, &mut render_pass)
            .expect("Rendering failed");

        drop(render_pass);
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
