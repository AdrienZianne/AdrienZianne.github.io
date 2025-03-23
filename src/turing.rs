
use std::default;

use eframe::egui::{Color32, Pos2};
use egui::Vec2;
use rand::{random, random_range};


type NodeIndex = usize;
type EdgeIndex = usize;

pub struct Turing {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub selected: Option<NodeIndex>,
    pub code: String,
    pub input: String,
    pub rubindex: Vec<usize>
}

#[derive(PartialEq)]
pub struct State {
    pub name: String,
    pub position: Pos2,
    pub color: Color32,
}

#[derive(Default)]
pub struct Transition {
    pub text: String,
}

#[derive(PartialEq)]
pub struct Node {
    pub state: State,
    pub edge: Option<EdgeIndex>,
}

pub struct Edge {
    pub transition: Transition,
    pub source: NodeIndex,
    pub target: NodeIndex,
    pub next: Option<EdgeIndex>,
}

impl Default for Turing {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            selected: None,
            code: String::new(),
            input: String::new(),
            rubindex: vec![0,0]
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            name: String::from("Test"),
            position: Pos2::new(random_range(0.0..1.0), random_range(0.0..1.0)),
            color: Color32::from_rgb(random(), random(), random()),
        }
    }
}

impl State {

    pub fn new_at_pos(name: String, position: Pos2) -> State {
        State {
            name: name,
            position: position,
            color: Color32::from_rgb(random(), random(), random()),
        }
    }
}

impl Turing {

    pub fn add_state(&mut self, state: State) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(Node {
            state: state,
            edge: None,
        });
        index
    }

    pub fn add_transition(&mut self, transition: Transition, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node = &mut self.nodes[source];
        self.edges.push(Edge {
            transition: transition,
            source: source,
            target: target,
            next: node.edge,
        });
        node.edge = Some(edge_index);
    }

    pub fn remove_state(&mut self, index: NodeIndex) {
        let mut edge_to_remove: Vec<EdgeIndex> = Vec::new();
        for (i, e) in self.edges.iter_mut().enumerate() {
            if e.target == index {
                edge_to_remove.push(i);
            }

            if e.target > index {
                e.target -= 1;
            }

            if e.source > index {
                e.source -= 1;
            }
        }

        for e in edge_to_remove {
            self.remove_transition(e);
        }

        let node = &self.nodes[index];
        if node.edge.is_some() {
            let mut edge_index = node.edge.unwrap();
            loop {
                let next_edge = self.edges[edge_index].next;
                self.remove_transition(edge_index);
                if next_edge.is_none() {
                    break;
                } else {
                    edge_index = next_edge.unwrap();
                }
            }
        }

        self.nodes.remove(index);
    }

    pub fn remove_transition(&mut self, index: EdgeIndex) {
        for n in self.nodes.iter_mut() {
            if n.edge.is_some() {
                if n.edge.unwrap() == index {
                    n.edge = self.edges[index].next;
                } else if n.edge.unwrap() > index {
                    n.edge = Some(n.edge.unwrap() - 1);
                }
            }
        }

        self.edges.remove(index);
        for e in self.edges.iter_mut() {
            if e.next.is_some() {
                if e.next.unwrap() == index {
                    e.next = None;
                } else if e.next.unwrap() > index {
                    e.next = Some(e.next.unwrap() - 1);
                }
            }
        }
    }

    pub fn is_adjacent(&mut self, n1: NodeIndex, n2: NodeIndex) -> usize {
        let mut c = 0;
        if self.transition_exist(n1, n2).is_some() { c+= 1}
        if self.transition_exist(n2, n1).is_some() { c+=1 }
        c
    }

    pub fn transition_exist(&mut self, n1: NodeIndex, n2: NodeIndex) -> Option<EdgeIndex> {
        let mut res : Option<EdgeIndex> = None;
        let n1 = &self.nodes[n1];

        let mut edge = n1.edge;
        loop {
            if edge.is_none() {
                break;
            }
            if self.edges[edge.unwrap()].target == n2 {
                res = Some(edge.unwrap());
            }
            edge = self.edges[edge.unwrap()].next;
        }
        res
    }
}
