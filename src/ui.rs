use imgui::Condition;

// Import NodeInfo, PinInfo, PinType if they are defined in crate::node_editor
use crate::node_editor::{InputPinInfo, NodeInfo, OutputPinInfo, PinType};

pub fn render_ui(
    ui: &imgui::Ui,
    value: &mut usize,
    choices: &[&'static str; 2],
    node_state: &mut crate::node_editor::NetworkState,
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
            
            if ui.button("Add Node") {
                node_state.nodes.push(NodeInfo{
                    id: node_state.id_gen.next_node(),
                    title: format!("New Node {}", node_state.nodes.len() + 1),
                    inputs: vec![InputPinInfo {
                        id: node_state.id_gen.next_input_pin(),
                        pin_type: PinType::Integer,
                        body: Box::new(|ui| ui.text("input")),
                    }],
                    outputs: vec![OutputPinInfo {
                        id: node_state.id_gen.next_output_pin(),
                        pin_type: PinType::Integer,
                        body: Box::new(|ui| ui.text("output")),
                    }],
                });
            }
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
            node_state.imnodes_editor.set_as_current_editor();

            let outer_scope = imnodes::editor(&mut node_state.imnodes_editor, |mut editor| {
                for ele in &node_state.nodes {
                    editor.add_node(ele.id, |mut node| {
                        node.add_titlebar(|| ui.text(&ele.title));
                        for input in &ele.inputs {
                            node.add_input(input.id, imnodes::PinShape::Circle, || (input.body)(ui));
                        }
                        for output in &ele.outputs {
                            node.add_output(output.id, imnodes::PinShape::QuadFilled, || (output.body)(ui));
                        }
                    });
                }
                for link in node_state.links.iter() {
                    editor.add_link(link.0, link.2, link.1);
                }
            });
            if let Some(link) = outer_scope.links_created() {
                node_state.links.push((
                    node_state.id_gen.next_link(),
                    link.start_pin,
                    link.end_pin,
                ));
            }
            if let Some(dropped_link_id) = outer_scope.get_dropped_link() {
                println!("Link dropped: {:?}", dropped_link_id);
            }
        });
}
