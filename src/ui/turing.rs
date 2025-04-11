use egui::{Color32, Pos2};
use rand::{random, random_range};

#[derive(PartialEq, Debug)]
pub struct State {
    pub name: String,
    pub position: Pos2,
    pub color: Color32,
    pub transitions: Vec<Transition>,
}

#[derive(Default, PartialEq, Debug)]
pub struct Transition {
    pub text: String,
    pub id: u8
}


impl Default for State {
    fn default() -> Self {
        Self {
            name: String::from("name"),
            position: Pos2::new(random_range(0.0..1.0), random_range(0.0..1.0)),
            color: Color32::from_rgb(random(), random(), random()),
            transitions: vec![]
        }
    }
}

impl State {

    pub fn new_at_pos(name: String, position: Pos2) -> State {
        State {
            name: name,
            position: position,
            color: Color32::from_rgb(random(), random(), random()),
            transitions: vec![]
        }
    }
}