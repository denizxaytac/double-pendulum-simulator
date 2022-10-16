use nannou::prelude::*;

const WINDOW_HEIGHT: u32 = 720;
const WINDOW_WIDTH: u32 = 1440;
const G: f32 = 1.0; // gravity
const DAMPENING: f32 = 0.0001;
use std::f32::consts::PI;

fn main() {
    nannou::app(model).size(WINDOW_WIDTH, WINDOW_HEIGHT).update(update).run();
}

struct Model {
    _window: window::Id,
    pub a1: f32, // angle
    pub m1: f32, // mass
    pub l1: f32, // length
    pub v1: f32, // velocity

    pub a2: f32, // angle
    pub m2: f32, // mass
    pub l2: f32, // length
    pub v2: f32, // velocity
}

impl Model{
    pub fn new(window: nannou::prelude::WindowId) -> Model{
        return Model{
            _window: window,
            a1: PI / 2.0, 
            m1: 40.0,
            l1: 200.0,
            v1: 0.0,


            a2: PI / 2.0, 
            m2: 40.0,
            l2: 200.0,
            v2: 0.0,
        }
    }
}

fn model(app: &App) -> Model {
    let _window = app.new_window().resizable(false).title("double-pendulum sim").view(view).build().unwrap();
    return Model::new(_window);
}

fn update(_app: &App, m: &mut Model, _update: Update) {
    let num1 = -G * (2.0 * m.m1 + m.m2) * m.a1.sin()
                - m.m2 * G * (m.a1 - 2.0 * m.a2).sin()
                - 2.0 * (m.a1 - m.a2).sin() * m.m2
                * (m.v2 * m.v2 * m.l2) + (m.v1 * m.v1 * m.l1) * (m.a1 - m.a2).cos();

    let den1 = m.l1 * (2.0 * m.m1 + m.m2 - m.m2 * (2.0 * m.a1 - 2.0 * m.a2).cos());
    
    let num2 = 2.0 * (m.a1 - m.a2).sin()
                * (m.v1 * m.v1 * m.l1 * (m.m1 + m.m2)
                    + G * (m.m1 + m.m2) * m.a1.cos()
                    + m.v2 * m.v2 * m.l2 * m.m2 * (m.a1 - m.a2).cos());
    let den2 = m.l2 * (2.0 * m.m1 + m.m2 - m.m2 * (2.0 * m.a1 - 2.0 * m.a2).cos());

    m.v1 += num1 / den1;
    m.v2 += num2 / den2;
    m.a1 += m.v1;
    m.a2 += m.v2;
}

fn view(app: &App, _model: &Model, frame: Frame) {
    // for converting from cartesian to coords starting from top-left
    let mut draw = app.draw();
    draw = draw.x_y(-(WINDOW_WIDTH as f32) / 2.0, WINDOW_HEIGHT as f32 / 2.0);
    draw = draw.x_y(WINDOW_WIDTH  as f32 / 2.0, -50.0);
    draw = draw.scale_y(-1.0);
    //
    draw.background().color(BLACK);

    println!("angle1: {} - angle2: {}", _model.a1, _model.a2);

    // drawing first pend
    let start_point = pt2(0.0, 0.0);
    let x1 = _model.a1.sin() * _model.l1;
    let y1 = _model.a1.cos() * _model.l1;
    let end_point = pt2(x1, y1);

    draw.line()
        .start(start_point)
        .end(end_point)
        .weight(4.0)
        .color(STEELBLUE);
    draw.ellipse().color(STEELBLUE).w(_model.m1).h(_model.m1).x_y(x1, y1);

    // drawing second pend
    let start_point = pt2(x1, y1);
    let x2 = (_model.a2.sin() * _model.l2) + x1;
    let y2 = (_model.a2.cos() * _model.l2) + y1;
    let end_point = pt2(x2, y2);

    draw.line()
        .start(start_point)
        .end(end_point)
        .weight(4.0)
        .color(STEELBLUE);
    draw.ellipse().color(STEELBLUE).w(_model.m2).h(_model.m2).x_y(x2, y2);

    // apply changes
    draw.to_frame(app, &frame).unwrap();

}
