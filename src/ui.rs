use imgui::Condition;
use imnodes::CoordinateSystem;

pub fn render_ui(
    ui: &imgui::Ui,
    value: &mut usize,
    choices: &[&'static str; 2],
    imnodes: &mut imnodes::Context,
) {
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


    let mut context = imnodes.create_editor();
    context.set_as_current_editor();
    let mut id_generator = context.new_identifier_generator();

    let outer_scope = imnodes::editor(&mut context, |mut editor| {
        editor.add_node(id_generator.next_node(), |mut node| {
            node.add_titlebar(|| ui.text("simple node :)"));

            node.add_input(
                id_generator.next_input_pin(),
                imnodes::PinShape::Circle,
                || ui.text("input"),
            );

            node.add_output(
                id_generator.next_output_pin(),
                imnodes::PinShape::QuadFilled,
                || ui.text("output"),
            );
        });
        let node2 = id_generator.next_node();
        editor.add_node(node2, |mut node| {
            node.add_titlebar(|| ui.text("simple node2 :)"));

            node.add_input(
                id_generator.next_input_pin(),
                imnodes::PinShape::Circle,
                || ui.text("input"),
            );

            node.add_output(
                id_generator.next_output_pin(),
                imnodes::PinShape::QuadFilled,
                || ui.text("output"),
            );
        });
        node2.set_draggable(true);
        node2.set_position(100.0, 100.0, CoordinateSystem::EditorSpace);
    });
    if let Some(link) = outer_scope.links_created() {
        println!("Link created: {:?}", link);
    }
    if let Some(dropped_link_id) = outer_scope.get_dropped_link() {
        println!("Link dropped: {:?}", dropped_link_id);
    }
}
