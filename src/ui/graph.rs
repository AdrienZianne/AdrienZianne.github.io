use std::collections::HashMap;
use std::ptr;
use std::{sync::Arc, time::Duration};

use egui::Margin;
use egui::{
    Color32, FontId, Frame, Id, Label, Pos2, Rect, Response, RichText, Scene, Sense, SidePanel,
    Stroke, Ui, Vec2,
    epaint::{CubicBezierShape, PathShape, QuadraticBezierShape},
    mutex::Mutex,
    vec2,
};
use turingrs::turing_machine::{TuringExecutor, TuringMachine};
use turingrs::turing_state::{TuringDirection, TuringTransition};

use crate::{
    TuringApp,
    turing::State,
    utils::{self, direction},
};

use super::constant::Constant;

// show the graph part of the gui
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
            // update ui for organic force
            let ofl = Arc::new(Mutex::new(OrganicForceLoop::default()));
            ofl.lock().ctx = Some(ui.ctx().clone());
            // let ofl_clone = ofl.clone();
            // std::thread::spawn(move || update_graph(ofl_clone));

            let mut inner_rect = Rect::NAN;

            // apply force on node
            let (center, is_stable) = apply_organic_force(app);
            app.is_stable = is_stable;

            let response = Scene::new()
                .show(ui, &mut app.graph_rect, |ui| {
                    // ui.painter().circle(center.to_pos2(), 1.0, Color32::RED, Stroke::NONE);

                    let mut transitions: Vec<(u8, TuringTransition)> = Vec::new();

                    for (index, _state) in app.node.iter() {
                        let mut x = app.turing.get_turing_machine().get_state(*index).transitions
                            .clone().iter().map(|x| (*index, x.clone())).collect::<Vec<_>>();
                        transitions.append(&mut x);
                    }

                    // draw transitions
                    for (from, t) in transitions {
                        let to = t.index_to_state;
                        let force_switch =
                            app.turing.get_turing_machine().get_transition_index(from, to).is_some() && from > to;

                        draw_transition(
                            ui,
                            app.node.get(&from).unwrap().position,
                            app.node.get(&to).unwrap().position,
                            "place holder\nplace holder",
                            center,
                            1.0 - 25.0 / 150.0,
                            force_switch,
                        );
                    }

                    for (index, state) in app.node.iter_mut() {
                        let pos = state.position;
                        let response = draw_node(
                            ui,
                            pos,
                            50.0,
                            state.color,
                            if app.selected_node.is_some_and(|x| x == *index) {
                                Color32::LIGHT_GRAY
                            } else {
                                Color32::DARK_GRAY
                            },
                            &state.name,
                        );

                        // if node clicked
                        if response.clicked() {
                            // if there is a node already selected
                            if app.selected_node.is_some() {
                                let from = app.selected_node.clone().unwrap();

                                let transition = TuringTransition::new(
                                    vec!['ç', 'ç', 'ç'],
                                    TuringDirection::Right,
                                    vec![
                                        ('ç', TuringDirection::Right),
                                        ('ç', TuringDirection::Right),
                                    ],
                                );

                                let _ = app.turing.turing_machine.append_rule_state(from, transition, *index);
                                // app.turing.add_transition(
                                //     Transition {
                                //         text: String::from("ç,ç -> R,ç,R"),
                                //     },
                                //     app.old_turing.selected.unwrap(),
                                //     i,
                                // );
                                app.is_stable = false;

                                app.selected_node = None;
                            } else {
                                app.selected_node = Some(*index);
                            }
                        }

                        if response.dragged() {
                            state.position = response.interact_pointer_pos().unwrap();
                        }
                    }

                    // // draw the nodes
                    // for i in 0..app.old_turing.nodes.len() {
                    //     let node = &app.old_turing.nodes[i];
                    //     let pos = node;
                    //     let response = draw_node(
                    //         ui,
                    //         pos,
                    //         50.0,
                    //         node.state.color,
                    //         if app.old_turing.selected.is_some_and(|x| x == i) {
                    //             Color32::LIGHT_GRAY
                    //         } else {
                    //             Color32::DARK_GRAY
                    //         },
                    //         &node.state.name,
                    //     );

                    //     // if node clicked
                    //     if response.clicked() {
                    //         // if there is a node already selected
                    //         if app.old_turing.selected.is_some() {
                    //             match app.old_turing.transition_exist(app.old_turing.selected.unwrap(), i) {
                    //                 Some(e) => app.old_turing.remove_transition(e),
                    //                 None => {
                    //                     app.old_turing.add_transition(
                    //                         Transition {
                    //                             text: String::from("ç,ç -> R,ç,R"),
                    //                         },
                    //                         app.old_turing.selected.unwrap(),
                    //                         i,
                    //                     );
                    //                     app.is_stable = false;
                    //                 }
                    //             }

                    //             app.old_turing.selected = None;
                    //         } else {
                    //             app.old_turing.selected = Some(i);
                    //         }
                    //     }

                    //     if response.dragged() {
                    //         app.old_turing.nodes[i] =
                    //             response.interact_pointer_pos().unwrap();
                    //     }
                    // }

                    inner_rect = ui.min_rect();
                })
                .response;

            // if graph canvas clicked
            if response.clicked() {
                if response.double_clicked() {
                    app.graph_rect = inner_rect;
                } else {
                    if app.selected_node.is_some() || app.selected_transition.is_some() {
                        app.selected_node = None;
                        app.selected_transition = None;
                    } else {
                        let text = String::from("Test");
                        let index = app.turing.turing_machine.add_state(&text);
                        app.node.insert(
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
) -> Response {
    let rect = Rect::from_center_size(pos, (size, size).into());

    ui.painter()
        .circle(pos, size / 2.0, color, Stroke::new(3.0, stroke_color));

    let label = Label::new(
        RichText::new(text)
            .font(FontId {
                family: egui::FontFamily::Name("Roboto".into()),
                size: 10.0,
            })
            .color(Color32::BLACK),
    );

    ui.put(rect, label);

    ui.allocate_rect(rect, Sense::click_and_drag())
}

// Draw the transition of the turing machine
fn draw_transition(
    ui: &mut Ui,
    source: Pos2,
    target: Pos2,
    text: &str,
    graph_center: Vec2,
    arrow_pos: f32,
    reverse: bool,
) {
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
    let t = arrow_pos;

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
            Stroke::new(1.0, Constant::ARROW),
        ));

        pointy = cubicbeziercurve(points, t);

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
            Stroke::new(1.0, Constant::ARROW),
        ));

        pointy = quadraticbeziercurve(points, t);
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
        Constant::ARROW,
        Stroke::NONE,
    ));

    // paint text
    let pos = center + delta * Vec2::new(50.0, 30.0);
    if ui
        .put(
            Rect::from_center_size(pos.to_pos2(), Vec2::new(0.0, 30.0)),
            Label::new(text).extend(),
        )
        .clicked()
    {
        println!("WTF")
    }
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

#[derive(Default)]
struct OrganicForceLoop {
    ctx: Option<egui::Context>,
}

// force the update of the graph part every x ms
fn update_graph(ofl: Arc<Mutex<OrganicForceLoop>>) {
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
        let ctx = &ofl.lock().ctx;
        match ctx {
            Some(x) => x.request_repaint(),
            None => panic!("Error"),
        }
    }
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
                app.node.get(i).unwrap().position,
                app.node.get(j).unwrap().position,
            );
            let direction = utils::direction(
                app.node.get(i).unwrap().position,
                app.node.get(j).unwrap().position,
            );

            if adj {
                force = utils::attract_force(
                    app.node.get(i).unwrap().position,
                    app.node.get(j).unwrap().position,
                )
            } else {
                force = -utils::rep_force(
                    app.node.get(i).unwrap().position,
                    app.node.get(j).unwrap().position,
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
        let n = app.node.get_mut(i).unwrap();
        n.position += new_delta.get(i).unwrap().to_vec2();
        center += n.position.to_vec2();
    }
    (center / k as f32, max_force < 0.01)
}
