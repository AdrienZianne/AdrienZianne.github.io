use std::ptr;
use std::{sync::Arc, time::Duration};

use egui::{
    Color32, FontId, Frame, Id, Label, Pos2, Rect, Response, RichText, Scene, Sense, SidePanel,
    Stroke, Ui, Vec2,
    epaint::{CubicBezierShape, PathShape, QuadraticBezierShape},
    mutex::Mutex,
    vec2,
};

use crate::{
    TuringApp,
    turing::{Node, State, Transition},
    utils::{self, direction},
};

use super::constant::Constant;

// show the graph part of the gui
pub fn ui(app: &mut TuringApp, ui: &mut Ui) {
    SidePanel::left(Id::new("Graph"))
        .frame(Frame {
            fill: Constant::BACKGROUND2,
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
            let ofl_clone = ofl.clone();
            // std::thread::spawn(move || update_graph(ofl_clone));

            let mut inner_rect = Rect::NAN;

            // apply force on node
            let (center, is_stable) = apply_organic_force(app);
            app.is_stable = is_stable;

            let response = Scene::new()
                .show(ui, &mut app.graph_rect, |ui| {
                    // ui.painter().circle(center.to_pos2(), 1.0, Color32::RED, Stroke::NONE);

                    // draw transitions
                    for i in 0..app.turing.edges.len() {
                        let force_switch = app
                            .turing
                            .is_adjacent(app.turing.edges[i].source, app.turing.edges[i].target)
                            > 1
                            && app.turing.edges[i].source > app.turing.edges[i].target;

                        let edge = &app.turing.edges[i];
                        let source = &app.turing.nodes[edge.source];
                        let target = &app.turing.nodes[edge.target];

                        draw_transition(
                            ui,
                            source,
                            target,
                            &edge.transition.text,
                            center,
                            1.0 - 25.0 / 150.0,
                            force_switch,
                        );
                    }

                    // draw the nodes
                    for i in 0..app.turing.nodes.len() {
                        let node = &app.turing.nodes[i];
                        let zoomed_pos = node.state.position;
                        let response = draw_node(
                            ui,
                            zoomed_pos,
                            50.0,
                            node.state.color,
                            if app.turing.selected.is_some_and(|x| x == i) {
                                Color32::LIGHT_GRAY
                            } else {
                                Color32::DARK_GRAY
                            },
                            &node.state.name,
                        );

                        // if node clicked
                        if response.clicked() {
                            // if there is a node already selected
                            if app.turing.selected.is_some() {
                                match app.turing.transition_exist(app.turing.selected.unwrap(), i) {
                                    Some(e) => app.turing.remove_transition(e),
                                    None => {
                                        app.turing.add_transition(
                                            Transition {
                                                text: String::from("ç,ç -> R,ç,R"),
                                            },
                                            app.turing.selected.unwrap(),
                                            i,
                                        );
                                        app.is_stable = false;
                                    }
                                }

                                app.turing.selected = None;
                            } else {
                                app.turing.selected = Some(i);
                            }
                        }

                        if response.dragged() {
                            app.turing.nodes[i].state.position =
                                response.interact_pointer_pos().unwrap();
                        }
                    }

                    inner_rect = ui.min_rect();
                })
                .response;

            // if graph canvas clicked
            if response.clicked() {
                if response.double_clicked() {
                    app.graph_rect = inner_rect;
                } else {
                    if app.turing.selected.is_some() {
                        app.turing.selected = None;
                    } else {
                        app.turing.add_state(State::new_at_pos(
                            String::from("test"),
                            response
                                .interact_pointer_pos()
                                .expect("No pointer pos found"),
                        ));
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
    source: &Node,
    target: &Node,
    text: &str,
    graph_center: Vec2,
    arrow_pos: f32,
    reverse: bool,
) {
    let mut delta = (if reverse {
        source.state.position - target.state.position
    } else {
        target.state.position - source.state.position
    })
    .rot90()
    .normalized();
    let center = vec2(
        (source.state.position.x + target.state.position.x) / 2.0,
        (source.state.position.y + target.state.position.y) / 2.0,
    );

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
        delta = direction(graph_center.to_pos2(), source.state.position).normalized();

        let points = [
            source.state.position,
            (source.state.position + (delta - delta.rot90()) * 100.0),
            (source.state.position + (delta + delta.rot90()) * 100.0),
            target.state.position,
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
        let points = [
            source.state.position,
            (center + delta * 30.0).to_pos2(),
            target.state.position,
        ];

        ui.painter().add(QuadraticBezierShape::from_points_stroke(
            points,
            false,
            Color32::TRANSPARENT,
            Stroke::new(1.0, Constant::ARROW),
        ));

        pointy = quadraticbeziercurve(points, t);
    }

    let vec = (pointy - target.state.position.to_vec2()).normalized();
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
    let k = app.turing.nodes.len();

    let mut new_delta = vec![Pos2::default(); k];

    let mut max_force = 0.0;
    for i in 0..k {
        let mut final_force: Vec2 = Vec2::ZERO;
        let mut force: f32;
        for j in 0..k {
            if j == i {
                continue;
            }

            let adj = app.turing.is_adjacent(i, j);
            let distance = utils::distance(
                app.turing.nodes[i].state.position,
                app.turing.nodes[j].state.position,
            );
            let direction = utils::direction(
                app.turing.nodes[i].state.position,
                app.turing.nodes[j].state.position,
            );

            if adj > 0 {
                force = utils::attract_force(
                    app.turing.nodes[i].state.position,
                    app.turing.nodes[j].state.position,
                )
            } else {
                force = -utils::rep_force(
                    app.turing.nodes[i].state.position,
                    app.turing.nodes[j].state.position,
                ) * if distance >= Constant::L - 0.1 {
                    0.0
                } else {
                    1.0
                }
            }

            final_force += direction * force;
        }

        if final_force.length() > max_force {
            max_force = final_force.length();
        }

        new_delta[i] = final_force.to_pos2();
    }

    let mut center: Vec2 = Vec2::ZERO;
    for i in 0..k {
        let n = &mut app.turing.nodes[i];
        n.state.position += new_delta[i].to_vec2();
        center += n.state.position.to_vec2();
    }
    (center / k as f32, max_force < 0.01)
}
