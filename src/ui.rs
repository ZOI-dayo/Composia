use imgui::Condition;

pub fn render_ui(ui: &imgui::Ui, value: &mut usize, choices: &[&'static str; 2]) {
    ui.window("Hello world")
        .size([300.0, 110.0], Condition::FirstUseEver)
        .build(|| {
            ui.text_wrapped("Hello world!");
            ui.text_wrapped("こんにちは世界！");
            if ui.button(choices[*value]) {
                *value += 1;
                *value %= 2;
            }

            ui.button("This...is...imgui-rs!");
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos[0], mouse_pos[1]
            ));
        });
}
