# Boids
This program is a Rust implementation of a boids flocking algorithm.<br>
For graphics, it uses nannou, which can be read about at [*this link*](https://docs.rs/nannou/latest/nannou/prelude/index.html)
The article [*Simulating Flocking with the Boids Algorithm*](https://medium.com/fragmentblog/simulating-flocking-with-the-boids-algorithm-92aef51b9e00) was used to provide a foundation of how boid flocks work.
In the article, it's stated that boids follow three main principles:

## 1. Separation
Steer to avoid contact with flockmates

## 2. Alignment
Steer towards the average heading of flockmates

## 3. Cohesion
Steer to move towards the average position of flockmates

## The Math
The essential pseudocode for the flock system is as follows:

### Update Flock
FOR boid in boids<br>
    vec1 = separation(boid) // avoid other birds<br>
    vec2 = alignment(boid) // match heading of flock<br>
    vec3 = cohesion(boid) // move towards center of flock<br>
    // could add more rules here<br>
    // vec4, ..., vecX<br>

    vec_final = vec1 + vec2 + vec3 + ... + vecX<br>

    boid.velocity += finalVector    // adjust heading<br>
    boid.position += boid.velocity // update position<br>

### Get Separation Vector 
  separation(b1: Boid)<br>
    distance = X // some number depending on how close you want to allow them to get<br>
    result = <0,0><br>
    FOR b2 in boids<br>
      IF b1 != b2<br>
        IF dist(b1.position, b2.position) < distance<br>
          result -= (b2.position - b1.position)<br>
    RETURN result<br>

### Get Alignment Vector
  alignment(b1: Boid) <br>
    velo = <0,0><br>
    FOR b2 in boids<br>
      IF b1 != b2<br>
        velo += b2.velocity<br>
    velo = velo / (boids.len - 1)<br>
    RETURN (velo - b1.velocity) / X // X is some number depending on how quickly you want them to turn<br>
    
### Get Cohesion Vector
  cohesion(b1: Boid)<br>
    loc = <0,0><br>
    FOR b2 in boids<br>
      IF b1 != b2<br>
        loc += b2.position<br>
    loc = loc / (boids.len - 1)  <br>      
    RETURN (loc - b1.position) / X // X is some number depending on how quickly you want them to turn<br>


## Ways to Improve

### Efficiency
- Merge the three get_vec functions. This would reduce how many times I'd need to iterate through the flock; instead of one time for each vector (3 total) it would be one time for 3 vectors.

### Functionality
- Implement the predator boid
- Fine-tune steering process
- Add live adjustments to the number of boids
- Add obstacles instead of a blank window

