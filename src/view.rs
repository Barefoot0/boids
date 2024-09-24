use nannou::prelude::*;
use crate::model::Model;

pub fn view(app: &App, model: &Model, frame: Frame){ // underscores to indicate unused

    let draw = app.draw();
    let mut should_draw_bg = true;

    if should_draw_bg {
        draw.background().color(GRAY);
        should_draw_bg = false;
    }
    
    // draw shapes and graphics here

    for boid in &model.flock {
        boid.show(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}