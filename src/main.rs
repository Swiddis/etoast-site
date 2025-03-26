extern crate nannou;
mod geom;

use std::f32::consts::SQRT_2;

use geom::{Segment, intersects};
use nannou::{color::Gradient, prelude::*};
use rand::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const SCALE: f32 = 10.0;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    graph: Vec<Segment>,
    /// Track living lines, and remove any lines which definitely have no attachment types
    living: Vec<usize>,
    living_ctr: usize,
    done: usize,
}

fn model(_app: &App) -> Model {
    Model {
        graph: vec![
            (pt2(-1.0, -1.0), pt2(1.0, -1.0)),
            (pt2(-1.0, -1.0), pt2(1.0, 1.0)),
            (pt2(1.0, -1.0), pt2(1.0, 1.0)),
        ],
        living: vec![0, 1, 2],
        living_ctr: 3,
        done: 0,
    }
}

fn as_segments(seg: &Segment, attach_type: u8) -> (Segment, Segment) {
    assert!((0..6).contains(&attach_type));
    match attach_type {
        0 => (
            (seg.0, seg.0 + (seg.1 - seg.0).rotate(PI / 2.0)),
            (seg.1, seg.0 + (seg.1 - seg.0).rotate(PI / 2.0)),
        ),
        1 => (
            (seg.0, seg.0 + (seg.1 - seg.0).rotate(-PI / 2.0)),
            (seg.1, seg.0 + (seg.1 - seg.0).rotate(-PI / 2.0)),
        ),
        2 => (
            (seg.0, seg.1 + (seg.0 - seg.1).rotate(PI / 2.0)),
            (seg.1, seg.1 + (seg.0 - seg.1).rotate(PI / 2.0)),
        ),
        3 => (
            (seg.0, seg.1 + (seg.0 - seg.1).rotate(-PI / 2.0)),
            (seg.1, seg.1 + (seg.0 - seg.1).rotate(-PI / 2.0)),
        ),
        4 => (
            (seg.0, seg.0 + (seg.1 - seg.0).rotate(PI / 4.0) / SQRT_2),
            (seg.1, seg.0 + (seg.1 - seg.0).rotate(PI / 4.0) / SQRT_2),
        ),
        5 => (
            (seg.0, seg.0 + (seg.1 - seg.0).rotate(-PI / 4.0) / SQRT_2),
            (seg.1, seg.0 + (seg.1 - seg.0).rotate(-PI / 4.0) / SQRT_2),
        ),
        _ => unreachable!(),
    }
}

fn is_legal(segment: &Segment, rect: &Rect) -> bool {
    (segment.0 - segment.1).length() >= 0.8
        && (rect.contains(segment.0 * SCALE) && rect.contains(segment.1 * SCALE))
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let mut rng = rand::rng();

    let mut attach_types: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
    let attachment = match model.living.choose(&mut rng) {
        Some(a) => *a,
        None => {
            model.done += 1;
            return;
        }
    };
    attach_types.shuffle(&mut rng);

    let attach_line = model.graph[attachment];

    for atype in attach_types {
        let (new_s1, new_s2) = as_segments(&attach_line, atype);

        if !is_legal(&new_s1, &app.window_rect()) || !is_legal(&new_s2, &app.window_rect()) {
            continue;
        }

        if !model
            .graph
            .par_iter()
            .any(|s| intersects(s, &new_s1) || intersects(s, &new_s2))
        {
            model.graph.push(new_s1);
            model.graph.push(new_s2);

            model.living.push(model.living_ctr);
            model.living.push(model.living_ctr + 1);
            model.living_ctr += 2;
            break;
        }
    }

    // This line no longer has any possible attachments (both sides occupied)
    model
        .living
        .remove(model.living.binary_search(&attachment).unwrap());
}

fn point_color_at(pt: &Vec2, window: &Rect) -> LinSrgb {
    let grad = Gradient::new(vec![
        lin_srgb(
            0xff as f32 / 255.0,
            0xcf as f32 / 255.0,
            0x67 as f32 / 255.0,
        ),
        lin_srgb(
            0xd3 as f32 / 255.0,
            0x32 as f32 / 255.0,
            0x1d as f32 / 255.0,
        ),
    ]);
    grad.get(
        (2.0 * pt.y - window.bottom()) / window.h() + (0.35 * pt.x - window.left()) / window.w(),
    )
}

fn view(app: &App, model: &Model, frame: Frame) {
    let window = app.window_rect();

    let draw = app.draw();
    // draw.background().color(BLACK);

    for idx in model.living.iter().rev().take(2) {
        let (a, b) = model.graph[*idx];
        let points: Vec<(Vec2, LinSrgb)> = (0..50)
            .map(|i| {
                let pt = SCALE * a.lerp(b, i as f32 / 50.0);
                (pt, point_color_at(&pt, &window))
            })
            .collect();
        draw.ellipse()
            .radius(0.15 * SCALE)
            .xy(points[0].0)
            .color(point_color_at(&points[0].0, &window));
        draw.ellipse()
            .radius(0.15 * SCALE)
            .xy(points[points.len() - 1].0)
            .color(point_color_at(&points[1].0, &window));
        draw.polyline().weight(0.2 * SCALE).points_colored(points);
    }

    draw.to_frame(app, &frame).unwrap();

    if model.done == 1 {
        let file_path = captured_frame_path(app);
        app.main_window().capture_frame(file_path);
    }
}

fn captured_frame_path(app: &App) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join("output")
        .join("result")
        .with_extension("png")
}
