use imgui::Ui;
use imnodes::{IdentifierGenerator, InputPinId};

pub struct NetworkState {
    pub imnodes_editor: imnodes::EditorContext,
    pub id_gen: IdentifierGenerator,
    pub nodes: Vec<NodeInfo>,
    pub links: Vec<(imnodes::LinkId, imnodes::OutputPinId, imnodes::InputPinId)>,
}

impl NetworkState {
    pub fn new(imnodes_context: &imnodes::Context) -> Self {
        let imnodes_editor = imnodes_context.create_editor();
        let id_gen = imnodes_editor.new_identifier_generator();
        Self {
            imnodes_editor,
            id_gen,
            nodes: Vec::new(),
            links: Vec::new(),
        }
    }
}

pub struct NodeInfo {
    pub id: imnodes::NodeId,
    pub title: String,
    pub inputs: Vec<InputPinInfo>,
    pub outputs: Vec<OutputPinInfo>,
}

pub enum PinType {
    Integer,
}

pub struct InputPinInfo {
    pub id: InputPinId,
    #[allow(dead_code)] // Will be used for type checking in the future
    pub pin_type: PinType,
    pub body: Box<dyn Fn(&Ui)>,
}

pub struct OutputPinInfo {
    pub id: imnodes::OutputPinId,
    #[allow(dead_code)] // Will be used for type checking in the future  
    pub pin_type: PinType,
    pub body: Box<dyn Fn(&Ui)>,
}