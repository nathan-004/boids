use bevy::prelude::*;

struct Boid {
    posx: f32,
    posy: f32,
    direction: f32,
    speed: f32,
}

impl Boid {
    fn new() -> Self {
        Boid {
            posx: 1.0,
            posy: 1.0,
            direction: 1.0,
            speed: 1.0,
        }
    }

    fn update_position(&mut self) {
        let (s, c) = self.direction.sin_cos(); // sin, cos
        self.posx += self.speed * c;
        self.posy += self.speed * s;
    }
}

fn main() {
    App::new().run();
}