use nannou::prelude::*;
use rand::Rng;
use std::ptr;

// This will make easier to implement other boid types in the future
pub enum BoidType { 
    Predator,
    Prey,
}

impl BoidType {
    /*
    Boid types will be differentiated by:
    color, view range, speed, and size.
    */
    fn color(&self) -> Rgb8 {
        match self {
            BoidType::Predator => DARKBLUE,
            BoidType::Prey => BLACK,
        }
    }

    fn view_dist(&self) -> f32 {
        match self {
            BoidType::Predator => 100.0,
            BoidType::Prey => 100.0,
        }
    }

    fn max_speed(&self) -> f32 {
        match self {
            BoidType::Predator => 7.0,
            BoidType::Prey => 1000.0,
        }
    }

    fn size(&self) -> (f32, f32) { // returns (width, height)
        match self {
            BoidType::Predator => (10.0, 13.0),
            BoidType::Prey => (20.0,30.0),
        }
    }
}


// This is the main struct of a boid
pub struct Boid {
    
    // descriptive attributes:
    pub color: Rgb8,
    pub height: f32,
    pub width: f32,

    // movement attributes:
    pub position: Vec2,
    pub velocity: Vec2, // Vec2 for 2d movement
    
    // limiting attributes:
    pub max_speed: f32,
    pub min_speed: f32,
    pub view_dist: f32,
    pub avoid_range: f32,
}

impl Boid {

    // Constructor
    pub fn new(bt: BoidType, x_pos: f32, y_pos: f32, ) -> Boid { 
        let color = bt.color();
        let (width, height) = bt.size();
        let position = Vec2::new(x_pos,y_pos);
        let velocity = Vec2::new(rand::thread_rng().gen_range(-1..1) as f32, rand::thread_rng().gen_range(-1..1) as f32);
        let max_speed = bt.max_speed();
        let min_speed = 3.0;
        let view_dist = bt.view_dist();
        let avoid_range = 60.0;
        Boid {
            color,
            height,
            width,
            position,
            velocity,
            max_speed,
            min_speed,
            view_dist,
            avoid_range,
        }
    } // Constructor done

    // The UI guide git does something very different for sep vec
    pub fn get_separate_vec(&self, neighbours: &Vec<Boid>) -> Vec2 {
        let sum = neighbours.iter().fold(Vec2::ZERO, |mut acc, boid| {
            // if !ptr::eq(self, boid) {
            if self.position != boid.position {
                if self.position.distance(boid.position) < self.avoid_range {
                    acc -= (boid.position - self.position).normalize()/(boid.position - self.position).length();
                }
            }
            acc
        });
        return sum
    }

    pub fn get_alignment_vector(&self, neighbours: &Vec<Boid>) -> Vec2 {
        let mut sum = neighbours.iter().fold(Vec2::new(0.0,0.0), |mut acc: Vec2, boid| {
            if (self.position.distance(boid.position) < self.view_dist) && (!ptr::eq(self, boid)) {acc += boid.velocity;}
            acc
        });

        let len = if neighbours.len() > 1 {neighbours.len() - 1} else {1};
        sum = sum/(len as f32);
        return (sum - self.velocity).normalize_or_zero()
    }

    pub fn get_cohesion_vec(&self, neighbours: &Vec<Boid>) -> Vec2 {
        let pos_avg = neighbours.iter().fold(Vec2::ZERO, |mut sum, boid| {
            if self.position.distance(boid.position) < self.view_dist {sum += boid.position}
            sum
        }); // use a closure making sum an accumulator and add each boid position

        if pos_avg == Vec2::ZERO {return pos_avg}
        else{return (pos_avg/(neighbours.len() as f32) - self.position).normalize()}
    }

    pub fn get_cursor_vec(&self, target: Vec2) -> Vec2 {
        let goal = (target - self.position).normalize_or_zero(); // vector pointing to target
        return (goal -  self.velocity).clamp_length(self.min_speed, self.max_speed) // velocity change
    }

    pub fn update(&mut self, update_vec: Vec2){ 
        self.velocity += update_vec;
        self.velocity = self.velocity.clamp_length(self.min_speed, self.max_speed);        
        self.position += self.velocity;
        // self.acceleration *= 0.0;        
    }

    pub fn show(&self, draw: &Draw){
        draw.tri().w_h(self.height, self.width).xy(self.position).rotate(self.velocity.normalize().angle()).color(self.color);
        // draw.ellipse().radius(self.width/2.0).xy(self.position).color(self.color);
    }

    pub fn wrap(&mut self, win: &Rect) {
        let pad = 10.0;

        self.position.x = match self.position.x {
            x if x>win.right()+pad => win.left()-pad,
            x if x<win.left()-pad => win.right()+pad,
            x => x,
        }; 

        self.position.y = match self.position.y {
            y if y>win.top()+pad => win.bottom()-pad,
            y if y<win.bottom()-pad => win.top()+pad,
            y => y,
        }; 
    }

} // End of boid implementation
