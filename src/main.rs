pub mod gravity;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use glsl_layout::vec2;
use glsl_layout::float;


fn main() {
    use gravity::{gravity_job_receiver_system, gravity_job_sender_system};

    App::build()
        .init_resource::<MainState>()
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .add_system(
            gravity_job_sender_system.system()
                .chain(gravity_job_receiver_system.system())
                .chain(motion_update_system.system())
        )
        .run();
}



pub struct Mass(f32);
pub struct Sphere { r: f32 }

pub struct Velocity(Vec2);
pub struct ResultantForce(Vec2);
pub struct Planet;


// Updates forces on objects and applies velocity
fn motion_update_system(
    time: Res<Time>,
    mut motion_query: Query<(
        &mut ResultantForce,
        &mut Velocity,
        &mut GlobalTransform,
        &Mass,
    )>
) {
    let dt = time.delta_seconds();

    for (mut res_force, mut vel, mut pos, mass) in motion_query.iter_mut() {
        // First turn the force -> velocity
        // F = ma, a = F/m, dv = a dt
        let a = res_force.0/mass.0;
        vel.0 += a * dt;
        res_force.0 = Vec2::default();  // Reset resultant force for next update

        // Update position based on velocity
        // dr = v dt
        let dr = vel.0 * dt;
        pos.translation += Vec3::new(dr.x, dr.y, 0.0);
    }
}





fn setup(
    mut state: ResMut<MainState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    add_planet(&mut commands, &mut state, Vec2::new(0.0, 100.0), Vec2::new(10.0, 0.0), 5000.0, 10.0);
    add_planet(&mut commands, &mut state, Vec2::new(0.0, -100.0), Vec2::new(-10.0, 0.0), 5000.0, 100.0);
}

fn add_planet(
    commands: &mut Commands,
    state: &mut MainState,
    position: Vec2, velocity: Vec2, mass: f32, radius: f32
) {
    let circle = shapes::Circle {
        radius,
        ..Default::default()
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &circle,
            ShapeColors::outlined(Color::NONE, Color::WHITE),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(5.0),
            },
            Transform::default(),
        ))
        .insert(Transform::from_xyz(position.x, position.y, 0.0))
        .insert(Velocity(velocity))
        .insert(ResultantForce(Vec2::default()))
        .insert(Mass(mass))
        .insert(Sphere { r: radius })
        .insert(Planet);
}


#[derive(Default)]
struct MainState {
    planet_count: u32,
}
