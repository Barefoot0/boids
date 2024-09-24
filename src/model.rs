use nannou::prelude::*;
use rand::Rng;
use crate::boids::Boid;

// This function handles the application's state

pub struct Model {
    pub flock: Vec<Boid>,
    pub window_rect: Rect,
}

pub fn model(app: &App) -> Model {
    let window_id = app.new_window().view(crate::view::view).build().unwrap();
    let window = app.window(window_id).unwrap();
    let mut flock: Vec<Boid> = Vec::new();

    // populate the flock with boids
    for _ in 0..80 { // CHOOSE HOW MANY BOIDS YOU WANT HERE
        let x = rand::thread_rng().gen_range(window.rect().left()..window.rect().right()) as f32;
        let y = rand::thread_rng().gen_range(window.rect().bottom()..window.rect().top()) as f32;
        let boid = Boid::new(crate::boids::BoidType::Prey, x, y);
        flock.push(boid);
    }

    let window_rect = window.rect();

    Model {flock,
           window_rect}
}
