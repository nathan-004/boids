use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite_render::{Wireframe2dConfig, Wireframe2dPlugin};

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
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        #[cfg(not(target_arch = "wasm32"))]
        Wireframe2dPlugin::default(),
    ))
    .add_systems(Startup, setup);
    #[cfg(not(target_arch = "wasm32"))]
    //app.add_systems(Update, toggle_wireframe);
    app.run();
}

const X_EXTENT: f32 = 900.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let shapes = [
        meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        )),
        meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        ))
    ];
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform {
                translation: Vec3::new(
                    // Position X répartie sur l'écran
                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                    0.0,
                    0.0,
                ),
                rotation: Quat::from_rotation_z(3.14 / 2.0), // Angle en radians
                scale: Vec3::splat(1.0),
            },
        ));

    }
}