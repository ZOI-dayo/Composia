use imgui::Condition;
use imnodes::{CoordinateSystem, LinkId};

pub fn render_ui(
    ui: &imgui::Ui,
    value: &mut usize,
    choices: &[&'static str; 2],
    editor: &mut imnodes::EditorContext,
    links: &mut Vec<(imnodes::LinkId, imnodes::OutputPinId, imnodes::InputPinId)>,
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

    ui.window("Node Editor")
        .size([600.0, 400.0], Condition::FirstUseEver)
        .build(|| {
            editor.set_as_current_editor();
            let mut id_generator = editor.new_identifier_generator();

            let outer_scope = imnodes::editor(editor, |mut editor| {
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
                for link in links.iter() {
                    editor.add_link(link.0, link.2, link.1);
                }
            });
            if let Some(link) = outer_scope.links_created() {
                links.push((
                    id_generator.next_link(),
                    link.start_pin,
                    link.end_pin,
                ));
            }
            if let Some(dropped_link_id) = outer_scope.get_dropped_link() {
                println!("Link dropped: {:?}", dropped_link_id);
            }
        });
}
