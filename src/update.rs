use nannou::prelude::*;
// use crate::model::Model;
use crate::Model;

pub fn update(app: &App, model: &mut Model, _update: Update) {

    let mut changes_vec: Vec<Vec2> = Vec::new();

    for b in &model.flock {

        let separation_vec = b.get_separate_vec(&model.flock);
        let alignment_vec = b.get_alignment_vector(&model.flock);
        let cohesion_vec = b.get_cohesion_vec(&model.flock);
        let cursor_vec = b.get_cursor_vec(app.mouse.position());

        changes_vec.push(separation_vec*8.0 + cursor_vec*0.25 + alignment_vec*0.25 + cohesion_vec*0.25);

    }

    for (boid, change) in model.flock.iter_mut().zip(changes_vec.iter_mut()) {
        boid.update(*change);
        boid.wrap(&model.window_rect);
    }

}