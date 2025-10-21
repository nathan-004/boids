use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]

use std::f32::consts::PI;
use rand::Rng;

#[derive(Component)]
struct Boid {
    posx: f32,
    posy: f32,
    direction: f32,
    speed: f32,
}

impl Boid {
    fn update_position(&mut self) {
        let (s, c) = self.direction.sin_cos(); // sin, cos
        self.posx += self.speed * s;
        self.posy += self.speed * c;
        let mut rng = rand::rng();
        self.direction += rng.random_range(-0.05..0.05);
    }

    fn constrain_position(&mut self, width: f32, height: f32) {
        let (s, c) = self.direction.sin_cos();
        let mut cur_vy:f32 = self.speed * c;
        let mut cur_vx:f32 = self.speed * s;

        if self.posx >= width {
            self.posx = width;
            cur_vx = -cur_vx;
        } else if self.posx <= -width {
            self.posx = -width;
            cur_vx = -cur_vx;
        }

        if self.posy >= height {
            self.posy = height;
            cur_vy = -cur_vy;
        } else if self.posy <= -height {
            self.posy = -height;
            cur_vy = -cur_vy;
        }

        self.direction = cur_vx.atan2(cur_vy);
        self.direction = (self.direction + PI).rem_euclid(2.0 * PI) - PI;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_boids)
        .add_systems(Update, keyboard_inputs)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Caméra 2D
    commands.spawn(Camera2d);
    let mut rng = rand::rng();
    
    for _ in 0..5 {
        // Création d'un mesh triangle
        let triangle = meshes.add(Triangle2d::new(
            Vec2::Y * 20.0,
            Vec2::new(-15.0, -15.0),
            Vec2::new(15.0, -15.0),
        ));

        // Ajout du boid à l'écran
        commands.spawn((
            Boid {
                posx: 0.0,
                posy: 0.0,
                direction: rng.random_range(-1.0..1.0),
                speed: 2.0,
            },
            Mesh2d(triangle),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    }
}

fn update_boids(mut query: Query<(&mut Transform, &mut Boid)>, window_query: Query<&Window>) {
    let window = window_query.single().unwrap();
    let width = window.resolution.width() / 2.0;
    let height = window.resolution.height() / 2.0;


    for (mut transform, mut boid) in &mut query {
        // Mise à jour logique
        boid.update_position();
        boid.constrain_position(width, height);

        // Mise à jour graphique
        transform.translation.x = boid.posx;
        transform.translation.y = boid.posy;
        transform.rotation = Quat::from_rotation_z(-boid.direction);
    }
}

fn keyboard_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Boid>,
) {
    let turn_speed = 3.0; // radians par seconde
    let accel = 50.0;     // unités de vitesse par seconde

    let mut turn_delta = 0.0;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        turn_delta += turn_speed * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        turn_delta -= turn_speed * time.delta_secs();
    }

    let mut speed_delta = 0.0;
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        speed_delta += accel * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        speed_delta -= accel * time.delta_secs();
    }

    if turn_delta == 0.0 && speed_delta == 0.0 {
        return;
    }

    for mut boid in &mut query {
        boid.direction += turn_delta;
        boid.speed = (boid.speed + speed_delta).max(0.0); // pas de vitesse négative

        // normaliser l'angle dans [-PI, PI] pour éviter débordements
        boid.direction = (boid.direction + PI).rem_euclid(2.0 * PI) - PI;
    }
}