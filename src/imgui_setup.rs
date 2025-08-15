use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::{HiDpiMode, WinitPlatform};

pub struct ImGuiSetup {
    pub imgui: imgui::Context,
    pub imnodes: imnodes::Context,
    pub imnodes_editor: imnodes::EditorContext,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
}

impl ImGuiSetup {
    pub fn new_with_device(
        window: &winit::window::Window,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_format: wgpu::TextureFormat,
    ) -> Self {
        // imguiの初期化
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        let imnodes = imnodes::Context::new();
        let imnodes_editor = imnodes.create_editor();

        let mut platform = WinitPlatform::init(&mut imgui);
        platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Default);

        let hidpi_factor = window.scale_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData {
                config: Some(imgui::FontConfig {
                    oversample_h: 1,
                    pixel_snap_h: true,
                    size_pixels: font_size,
                    ..Default::default()
                }),
            }]);

        let renderer_config = RendererConfig {
            texture_format: surface_format,
            ..Default::default()
        };

        let renderer = Renderer::new(&mut imgui, device, queue, renderer_config);

        Self {
            imgui,
            imnodes,
            imnodes_editor,
            platform,
            renderer,
        }
    }
}
