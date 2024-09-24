mod model;
mod update;
mod view;
mod boids;

use model::Model;

fn main() {
    nannou::app(model::model)      
        .update(update::update)  
        .run();
}

