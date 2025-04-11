use std::{
    collections::{BTreeMap, HashMap}, fs::File, path::PathBuf
};

use poll_promise::Promise;
use rfd::FileHandle;
use ::turingrs::turing_machine::TuringMachine;
use egui::{
    FontData, FontDefinitions, FontFamily, Pos2, Rect, Stroke, Visuals, style::Selection,
    vec2,
};
use egui_extras::install_image_loaders;
use rand::random_range;
use turingrs::{
    parser::parse_turing_machine,
    turing_machine::{TuringExecutionStep, TuringExecutor, TuringMachineExecutor},
};

use crate::ui::{self, turing::{State, Transition}};

pub struct TuringApp {
    pub turing: TuringMachineExecutor,
    pub input: String,
    pub code: String,
    pub graph_rect: Rect,
    pub is_stable: bool,
    pub states_hash: HashMap<u8, State>,
    pub selected_node: Option<u8>,
    pub selected_transition: Option<(u8, u8)>,
    pub current_step: TuringExecutionStep,
    pub count: usize,
    pub is_accepted: Option<bool>,
    pub promise: Option<Promise<Option<PathBuf>>>,
    pub promise_wasm: Option<Promise<Option<FileHandle>>>
}

impl Default for TuringApp {
    fn default() -> Self {
        let tm = TuringMachine::new(0);
        let mut pos: Pos2 = Pos2::ZERO;
        let mut hash = HashMap::new();
        for (name, index) in tm.name_index_hashmap.iter() {
            hash.insert(
                index.clone(),
                State {
                    name: name.clone(),
                    position: pos,
                    ..Default::default()
                },
            );
            pos = (pos.to_vec2() + vec2(200.0, 0.0)).to_pos2();
        }
        let (tm, cs) = TuringMachineExecutor::new(tm, "".to_string()).unwrap();
        Self {
            turing: tm,
            graph_rect: Rect::ZERO,
            is_stable: true,
            input: "".to_string(),
            code: "".to_string(),
            states_hash: hash,
            selected_node: None,
            selected_transition: None,
            current_step: cs,
            count: 0,
            is_accepted: None,
            promise: None,
            promise_wasm: None,
        }
    }
}

impl TuringApp {
    /// Called once before the first frame.
    pub fn new(cc: &'_ eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals {
            selection: Selection {
                stroke: Stroke::NONE,
                ..Default::default()
            },
            ..Default::default()
        });

        // cc.egui_ctx.set_debug_on_hover(true);

        load_font(cc);

        Default::default()
    }


    /// Compile the code by creating a new TuringMachine and TuringMachineExecutor and updating the graph
    pub fn compile(&mut self) {
        let tm = parse_turing_machine(self.code.clone()).unwrap();
        (self.turing, self.current_step) = TuringMachineExecutor::new(tm, "".to_string()).unwrap();
        self.states_hash = HashMap::new();
        let mut pos: Pos2 = Pos2::ZERO;
        for (name, index) in self.turing.get_turing_machine().name_index_hashmap.iter() {

            // set up transitions for graph
            let mut transitions = vec![];
            for (i, t) in self.turing.get_turing_machine().get_state(*index).transitions.iter().enumerate() {
                transitions.push(Transition {
                    text: t.to_string(),
                    id: i as u8,
                });
            }

            // set up states for graph
            self.states_hash.insert(
                index.clone(),
                State {
                    name: name.clone(),
                    position: pos,
                    transitions: transitions,
                    ..Default::default()
                }
            );

            // increment position to avoid collision or huge amount of force at initialisation
            pos = (pos.to_vec2() + vec2(200.0, random_range(-100.0..100.0))).to_pos2();
        }
        self.current_step = TuringExecutionStep::new(self.turing.turing_machine.k);
        self.count = 0;
        self.is_accepted = None;
    }


    /// Try to convert the graph to code. if impossible display error
    pub fn apply_graph() {

    }


    /// Update the input string 
    /// 
    /// TODO lock the graph to prevent modification during execution
    pub fn update_input(&mut self) {
        let tm = self.turing.turing_machine.clone();
        (self.turing, self.current_step) =
            TuringMachineExecutor::new(tm, self.input.clone()).unwrap();
        self.count = 0;
        self.is_accepted = None;
    }

    /// Go to next state by following available transition if exist
    pub fn next(&mut self) {
        match self.turing.as_iter().next() {
            Some(x) => {
                self.current_step = x;
                self.count += 1;
            }
            None => {
                self.is_accepted = Some(
                    self.turing
                        .turing_machine
                        .get_state(self.turing.get_state_pointer())
                        .is_final,
                )
            }
        }
    }
}


/// Entry point of UI with update function
impl eframe::App for TuringApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        install_image_loaders(ctx);

        ui::show(self, ctx);
    }
}


/// Load the necessary font for the application
fn load_font(cc: &eframe::CreationContext<'_>) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "Roboto".into(),
        FontData::from_static(include_bytes!("../assets/fonts/Roboto.ttf")).into(),
    );
    fonts.font_data.insert(
        "Roboto-regular".into(),
        FontData::from_static(include_bytes!("../assets/fonts/Roboto-Regular.ttf")).into(),
    );

    let mut newfam = BTreeMap::new();
    newfam.insert(FontFamily::Name("Roboto".into()), vec!["Roboto".to_owned()]);
    newfam.insert(
        FontFamily::Name("Roboto-regular".into()),
        vec!["Roboto-regular".to_owned()],
    );
    fonts.families.append(&mut newfam);

    cc.egui_ctx.set_fonts(fonts);
}
