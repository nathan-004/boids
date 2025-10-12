use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite_render::{Wireframe2dConfig, Wireframe2dPlugin};

use std::f32::consts::PI;

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
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        #[cfg(not(target_arch = "wasm32"))]
        Wireframe2dPlugin::default(),
    ))
    .add_systems(Startup, setup)
    .add_systems(Update, update_boids);

    #[cfg(not(target_arch = "wasm32"))]
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Caméra 2D
    commands.spawn(Camera2d);

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
            direction: PI / 2.0,
            speed: 2.0,
        },
        Mesh2d(triangle),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn update_boids(mut query: Query<(&mut Transform, &mut Boid)>) {
    for (mut transform, mut boid) in &mut query {
        // Mise à jour logique
        boid.update_position();

        // Mise à jour graphique
        transform.translation.x = boid.posx;
        transform.translation.y = boid.posy;
        transform.rotation = Quat::from_rotation_z(-boid.direction);
    }
}
