use std::collections::{BTreeMap, HashMap};

use egui::{vec2, FontData, FontDefinitions, FontFamily, Pos2, Rect};
use egui_extras::install_image_loaders;
use rand::random_range;
use turingrs::{parser::parse_turing_machine, turing_machine::{TuringExecutionStep, TuringExecutor, TuringMachineExecutor}};
use::turingrs::turing_machine::TuringMachine;

use crate::{turing::{State, Turing}, ui::{self, constant::Constant}};

pub struct TuringApp {
    pub turing: TuringMachineExecutor,
    pub input: String,
    pub code: String,
    pub graph_rect: Rect,
    pub is_stable: bool,
    pub node : HashMap<u8, State>,
    pub selected_node : Option<u8>,
    pub selected_transition : Option<u8>,
    pub current_step: TuringExecutionStep,
    pub count: usize,
    pub is_accepted: Option<bool>
}

impl Default for TuringApp {
    fn default() -> Self {
        let tm = TuringMachine::new(0);
        let mut pos: Pos2 = Pos2::ZERO;
        let mut hash = HashMap::new();
        for (name, index) in tm.name_index_hashmap.iter() {
            hash.insert(index.clone(), State {
                name: name.clone(),
                position: pos,
                ..Default::default()
            });
            pos = (pos.to_vec2() + vec2(200.0, 0.0)).to_pos2();
        }
        let tm = TuringMachineExecutor::new(tm, "".to_string()).unwrap();
        let step = TuringExecutionStep::new(0);
        Self {
            turing: tm,
            graph_rect: Rect::ZERO,
            is_stable: true,
            input: String::from(""),
            code: String::from(""),
            node: hash,
            selected_node : None,
            selected_transition : None,
            current_step: step,
            count: 0,
            is_accepted: None
        }
    }
}

impl TuringApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext) -> Self {

        // cc.egui_ctx.set_visuals(Visuals {
        //     // window_fill: Constant::BACKGROUND,
        //     // panel_fill: Constant::BACKGROUND,
        //     // extreme_bg_color: Constant::BACKGROUND2,
        //     ..Default::default()
        // });
        
        // cc.egui_ctx.set_debug_on_hover(true);

        load_font(cc);

        Default::default()
    }

    pub fn compile(&mut self) {
        let tm = parse_turing_machine(self.code.clone()).unwrap();
        self.turing = TuringMachineExecutor::new(tm, "".to_string()).unwrap();
        self.node = HashMap::new();
        let mut pos: Pos2 = Pos2::ZERO;
        for (name, index) in self.turing.get_turing_machine().name_index_hashmap.iter() {
            self.node.insert(index.clone(), State {
                name: name.clone(),
                position: pos,
                ..Default::default()
            });
            pos = (pos.to_vec2() + vec2(200.0, random_range(-100.0..100.0))).to_pos2();
        }
        self.current_step = TuringExecutionStep::new(self.turing.turing_machine.k);
        self.count = 0;
        self.is_accepted = None;
    }

    pub fn update_input(& mut self) {
        let tm = parse_turing_machine(self.code.clone()).unwrap();
        self.turing = TuringMachineExecutor::new(tm, self.input.clone()).unwrap();
        self.current_step = self.turing.as_iter().next().unwrap();
        self.count = 0;
        self.is_accepted = None;
    }

    pub fn next(& mut self) {
        match self.turing.as_iter().next() {
            Some(x) => {
                self.current_step = x;
                self.count += 1;
            },
            None => self.is_accepted = Some(self.turing.turing_machine.get_state(self.turing.get_state_pointer()).is_final),
        }
    }
}

impl eframe::App for TuringApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        install_image_loaders(ctx);

        ui::show(self, ctx);
    }
}

fn load_font(cc: &eframe::CreationContext<'_>) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "Roboto".into(),
        FontData::from_static(include_bytes!("../assets/fonts/Roboto.ttf")).into(),
    );

    let mut newfam = BTreeMap::new();
    newfam.insert(FontFamily::Name("Roboto".into()), vec!["Roboto".to_owned()]);
    fonts.families.append(&mut newfam);

    cc.egui_ctx.set_fonts(fonts);
}
