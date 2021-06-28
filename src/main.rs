use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;


fn main() {
    App::build()
        .init_resource::<MainState>()
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .add_system(gravity_system.system().chain(motion_update_system.system()))
        .run();
}


#[derive(Bundle)]
struct Planet {
    transform: GlobalTransform,
    velocity: Velocity,
    force: ResultantForce,
    mass: Mass,
    body: Sphere,
}

struct Mass(f32);
struct Sphere { r: f32 }

struct Velocity(Vec2);
struct ResultantForce(Vec2);


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


fn gravity_system(mut grav_query: Query<(
    &mut ResultantForce,
    &Mass,
    &GlobalTransform,
)>) {

}


fn setup(
    mut state: ResMut<MainState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    add_planet(&mut commands, &mut state, Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0), 5000.0, 10.0);
    add_planet(&mut commands, &mut state, Vec2::new(0.0, -100.0), Vec2::new(10.0, 0.0), 5000.0, 100.0);
}

fn add_planet(
    commands: &mut Commands,
    state: &mut MainState,
    position: Vec2, velocity: Vec2, mass: f32, radius: f32
) {
    let circle = shapes::Circle {
        radius,
        center: [0.0, 0.0].into()
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &circle,
            ShapeColors::outlined(Color::NONE, Color::WHITE),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(5.0),
            },
            Transform::from_xyz(0.0, 0.0, 0.0)
        ))
        .insert_bundle(Planet {
            transform: GlobalTransform::from_xyz(position.x, position.y, 0.0),
            velocity: Velocity(velocity),
            force: ResultantForce(Vec2::default()),
            mass: Mass(mass),
            body: Sphere { r: radius },
        });

    state.planet_count += 1;
}


#[derive(Default)]
struct MainState {
    planet_count: u32,
}
