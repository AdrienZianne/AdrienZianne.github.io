use std::collections::hash_map::Entry;
use std::collections::HashMap;

use egui::{response, CornerRadius, Margin, StrokeKind};
use egui::{
    Color32, FontId, Frame, Id, Label, Pos2, Rect, Response, RichText, Scene, Sense, SidePanel,
    Stroke, Ui, Vec2,
    epaint::{CubicBezierShape, PathShape, QuadraticBezierShape},
    vec2,
};
use itertools::Itertools;
use turingrs::turing_machine::TuringExecutor;
use turingrs::turing_state::{TuringDirection, TuringTransition};

use crate::{
    TuringApp,
    ui::turing::State,
    utils::{self, direction},
};

use super::constant::Constant;
use super::turing::Transition;

/// Show the graph part of the gui
pub fn ui(app: &mut TuringApp, ui: &mut Ui) {

    SidePanel::left(Id::new("Graph"))
        .frame(Frame {
            fill: Constant::BACKGROUND2,
            outer_margin: Margin {
                right: 15,
                ..Default::default()
            },
            ..Default::default()
        })
        .resizable(true)
        .min_width(100.0)
        .max_width(ui.available_width() - 115.0)
        .default_width(ui.available_width() / 1.5)
        .show_inside(ui, |ui| {

            // rect initialization for Scene resize/moving
            let mut inner_rect = Rect::NAN;
            let mut scene_rect = app.graph_rect;

            // apply force on node
            let (center, is_stable) = apply_organic_force(app);
            app.is_stable = is_stable;

            // scene for graph resize/move
            let response = Scene::new()
                .show(ui, &mut scene_rect, |ui| {


                    // group transition by source and target state
                    let mut transitions: HashMap<(u8,u8), Vec<(TuringTransition, &Transition)>> = HashMap::new();

                    for (index, _state) in app.states_hash.iter() {

                        for t in _state.transitions.iter() {
                            let x = app.turing.turing_machine.states[*index as usize].get_transition(t.id).clone();

                            match transitions.entry((*index, x.index_to_state)) {
                                Entry::Occupied(mut e) => { e.get_mut().push((x,t)); },
                                Entry::Vacant(e) => { e.insert(vec![(x,t)]); },
                            }
                        }
                    }

                    // draw group of transitions
                    for ((from,to), trans) in transitions {
                        let force_switch = app.turing.get_turing_machine().get_transition_index(from, to).is_some() && from > to;

                        let mut rules: Vec<(bool, &Transition)> = vec![];
                        for (tt, t) in trans.iter() {
                            rules.push((app.current_step.transition_taken == *tt, t));
                        }

                        let source_pos = app.states_hash.get(&from).unwrap().position;
                        let target_pos = app.states_hash.get(&to).unwrap().position;

                        // println!("{}->{} == {:?}", from, to, &trans);
                            
                        let clicked = draw_transition(
                            ui,
                            source_pos,
                            target_pos,
                            &rules,
                            center,
                            force_switch,
                            if app.selected_transition.is_some_and(|(f,t)| f == from && t == to) {Constant::SELECTED} else {Constant::ARROW},
                            (from, to)
                        );

                        if let Some(ti) = clicked {
                            app.selected_transition = Some(ti);
                            app.selected_node = None;
                        }
                    }

                    let mut responses: Vec<(Response, u8)> = vec![];

                    // draw nodes
                    for (index, state) in app.states_hash.iter_mut() {
                        let pos = state.position;

                        let response = draw_node(
                            ui,
                            pos,
                            50.0,
                            state.color,
                            if app.selected_node.is_some_and(|x| x == *index) {
                                Constant::SELECTED
                            } else {
                                state.color
                            },
                            &state.name,
                            app.turing.get_state_pointer() == *index
                        );

                        responses.push((response, *index));
                    }

                    for (response, index) in responses {
                        // if node clicked
                        if response.clicked() {

                            // if there is a node already selected
                            if let Some(from) = app.selected_node {

                                let transition = TuringTransition::new(
                                    vec![],
                                    TuringDirection::None,
                                    vec![],
                                );

                                let ts = transition.to_string();

                                let x = app.turing.turing_machine.append_rule_state(from, transition, index).expect("unable to add rule");

                                app.states_hash.get_mut(&from).unwrap().transitions.push(Transition { text: ts, id: x });

                                app.is_stable = false;

                                app.selected_node = None;
                            } else {
                                app.selected_node = Some(index);
                                app.selected_transition = None;
                            }
                        }

                        if response.dragged() {
                            app.states_hash.get_mut(&index).unwrap().position = response.interact_pointer_pos().unwrap();
                        }
                    }

                    inner_rect = ui.min_rect();
                })
                .response;

            // save state of Scene
            app.graph_rect = scene_rect;

            // if graph canvas clicked
            if response.clicked() {

                // if double clicked, try to center
                if response.double_clicked() {
                    app.graph_rect = inner_rect;
                } 
                else {

                    // Unselect whatever is selected
                    if app.selected_node.is_some() || app.selected_transition.is_some() {
                        app.selected_node = None;
                        app.selected_transition = None;
                    } 
                    else {
                        // create new state
                        // TODO need to make focus to textedit for name
                        let text = String::from("Test");
                        let index = app.turing.turing_machine.add_state(&text);
                        app.states_hash.insert(
                            index,
                            State::new_at_pos(
                                text,
                                response
                                    .interact_pointer_pos()
                                    .expect("No pointer pos found"),
                            ),
                        );
                        app.is_stable = false;
                    }
                }
            }
        });

    if !app.is_stable {
        ui.ctx().request_repaint();
    }
}

// Draw the states of the turing machine
fn draw_node(
    ui: &mut Ui,
    pos: Pos2,
    size: f32,
    color: Color32,
    stroke_color: Color32,
    text: &str,
    is_current: bool
) -> Response {
    let rect = Rect::from_center_size(pos, (size, size).into());

    ui.painter()
        .circle(pos, size / 2.0, color, Stroke::new(3.0, stroke_color));

    let mut text = RichText::new(text)
        .font(FontId {
            family: egui::FontFamily::Name("Roboto-regular".into()),
            size: 14.0,
        })
        .color(Color32::BLACK);

    if is_current {text = text.underline()}
    let label = Label::new(text);

    ui.put(rect, label);

    ui.allocate_rect(rect, Sense::click_and_drag())
}

// Draw the transition of the turing machine
fn draw_transition(
    ui: &mut Ui,
    source: Pos2,
    target: Pos2,
    transitions: &Vec<(bool, &Transition)>,
    graph_center: Vec2,
    reverse: bool,
    color: Color32,
    transition_id: (u8, u8)
) -> Option<(u8, u8)> {
    let mut delta = (if reverse {
        source - target
    } else {
        target - source
    })
    .rot90()
    .normalized();
    let center = vec2((source.x + target.x) / 2.0, (source.y + target.y) / 2.0);

    // ui.painter().circle((center + delta * 20.0).to_pos2(), 1.0, Color32::YELLOW, Stroke::NONE);
    // ui.painter().circle((center - delta * 20.0).to_pos2(), 1.0, Color32::CYAN, Stroke::NONE);

    if ((center + delta * 20.0) - graph_center).length()
        < ((center - delta * 20.0) - graph_center).length() - 0.1
    {
        delta = -delta
    }

    if reverse {
        delta = -delta
    }

    let pointy;
    let pos;

    if source == target {
        delta = direction(graph_center.to_pos2(), source).normalized();

        let points = [
            source,
            (source + (delta - delta.rot90()) * 100.0),
            (source + (delta + delta.rot90()) * 100.0),
            target,
        ];
        ui.painter().add(CubicBezierShape::from_points_stroke(
            points,
            false,
            Color32::TRANSPARENT,
            Stroke::new(1.0, color),
        ));

        pointy = cubicbeziercurve(points, 1.0 - 15.0 / Constant::L);
        pos = center + delta * Vec2::new(120.0, 100.0);

        for i in 0..10 {
            ui.painter().circle(
                cubicbeziercurve(points, 1.0 / i as f32).to_pos2(),
                1.0,
                Color32::CYAN,
                Stroke::NONE,
            );
        }
    } else {
        let points = [source, (center + delta * 30.0).to_pos2(), target];

        ui.painter().add(QuadraticBezierShape::from_points_stroke(
            points,
            false,
            Color32::TRANSPARENT,
            Stroke::new(1.0, color),
        ));

        pointy = quadraticbeziercurve(points,  1.0 - 25.0 / Constant::L);
        pos = center + delta * Vec2::new(50.0, 35.0);
    }

    // paint arrow
    let vec = (pointy - target.to_vec2()).normalized();
    let triangles: Vec<Pos2> = vec![
        (pointy + vec * 10.0 - vec.rot90() * 5.0).to_pos2(),
        (pointy + vec * 10.0 + vec.rot90() * 5.0).to_pos2(),
        pointy.to_pos2(),
    ];

    ui.painter().add(PathShape::convex_polygon(
        triangles,
        color,
        Stroke::NONE,
    ));

    let mut clicked: Option<(u8, u8)> = None;

    for (i,(select, t)) in transitions.iter().enumerate() {
        
        let txt = if *select {RichText::new(&t.text).color(Constant::SELECTED)} else { RichText::new(&t.text)};
        // paint text
        let rect = ui.put(
                Rect::from_center_size((pos + vec2(0.0, i as f32 * 15.0)).to_pos2(), Vec2::new(0.0, 20.0)),
                Label::new(txt).extend().selectable(false),
            ).rect;

        let response = ui
            .allocate_rect(rect,
            Sense::click()
        );

        ui.painter().rect_stroke(response.rect, CornerRadius::ZERO, Stroke::new(1.0, Color32::CYAN), StrokeKind::Inside);
        if response.clicked() {
            clicked = Some(transition_id);
            println!("{} {} {} {:?}", response.interact_pointer_pos().unwrap(), rect, t.text, transition_id);
        }
    };
    
    clicked
}

// return a point on the curve of a quadratic bezier
fn quadraticbeziercurve(points: [Pos2; 3], t: f32) -> Vec2 {
    let x = (1.0 - t).powi(2) * points[0].x
        + 2.0 * (1.0 - t) * t * points[1].x
        + t.powi(2) * points[2].x;
    let y = (1.0 - t).powi(2) * points[0].y
        + 2.0 * (1.0 - t) * t * points[1].y
        + t.powi(2) * points[2].y;
    Vec2::new(x, y)
}

// return a point on the curve of a cubic bezier
fn cubicbeziercurve(points: [Pos2; 4], t: f32) -> Vec2 {
    let x = (1.0 - t).powi(3) * points[0].x
        + 3.0 * (1.0 - t).powi(2) * t * points[1].x
        + 3.0 * (1.0 - t) * t.powi(2) * points[2].x
        + t.powi(3) * points[3].x;
    let y = (1.0 - t).powi(3) * points[0].y
        + 3.0 * (1.0 - t).powi(2) * t * points[1].y
        + 3.0 * (1.0 - t) * t.powi(2) * points[2].y
        + t.powi(3) * points[3].y;
    Vec2::new(x, y)
}

// apply a force on the node for organic layout
fn apply_organic_force(app: &mut TuringApp) -> (Vec2, bool) {
    let k = app.turing.get_turing_machine().name_index_hashmap.len();

    let mut new_delta: HashMap<u8, Pos2> = HashMap::new();

    let mut max_force = 0.0;

    for (name, i) in app.turing.get_turing_machine().name_index_hashmap.iter() {
        // println!("{} {}", name, i);
        let mut final_force: Vec2 = Vec2::ZERO;
        let mut force: f32;

        for (name_o, j) in app.turing.get_turing_machine().name_index_hashmap.iter() {
            if j == i {
                continue;
            }

            let adj = app.turing.get_turing_machine().get_transition_index(*i, *j).is_some() 
                || app.turing.get_turing_machine().get_transition_index(*j, *i).is_some();

            let distance = utils::distance(
                app.states_hash.get(i).unwrap().position,
                app.states_hash.get(j).unwrap().position,
            );
            let direction = utils::direction(
                app.states_hash.get(i).unwrap().position,
                app.states_hash.get(j).unwrap().position,
            );

            if adj {
                force = utils::attract_force(
                    app.states_hash.get(i).unwrap().position,
                    app.states_hash.get(j).unwrap().position,
                )
            } else {
                force = -utils::rep_force(
                    app.states_hash.get(i).unwrap().position,
                    app.states_hash.get(j).unwrap().position,
                ) * if distance >= Constant::L - 0.1 {
                    0.0
                } else {
                    1.0
                }
            }

            final_force += direction * force;


            // println!(
            //     "names {}:{} adj {} dir {} force {} final {} pos {}",
            //     name,
            //     name_o,
            //     adj,
            //     direction,
            //     force,
            //     final_force,
            //     app.node.get(i).unwrap().position
            // );
        }

        if final_force.length() > max_force {
            max_force = final_force.length();
        }

        new_delta.insert(*i, final_force.to_pos2());
    }

    let mut center: Vec2 = Vec2::ZERO;
    for (name, i) in app.turing.get_turing_machine().name_index_hashmap.iter() {
        let n = app.states_hash.get_mut(i).unwrap();
        n.position += new_delta.get(i).unwrap().to_vec2();
        center += n.position.to_vec2();
    }
    (center / k as f32, max_force < 0.01)
}
