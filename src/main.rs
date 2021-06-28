use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;


fn main() {
    App::build()
        .init_resource::<MainState>()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .run();
}


#[derive(Bundle)]
struct Planet {
    m: Mass,
    body: Sphere,
}

struct Mass(f32);
struct Sphere { r: f32 }


fn setup(
    mut state: ResMut<MainState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    add_planet(commands, state, 0.0, 0.0, 5000.0, 100.0);
}

fn add_planet(
    mut commands: Commands,
    mut state: ResMut<MainState>,
    x: f32, y: f32, mass: f32, radius: f32
) {
    let circle = shapes::Circle {
        radius,
        center: [x, y].into()
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &circle,
            ShapeColors::outlined(Color::NONE, Color::WHITE),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(5.0),
            },
            Transform::from_xyz(x, y, 0.0)
        ))
        .insert_bundle(Planet {
            m: Mass(mass),
            body: Sphere { r: radius },
        });
}


#[derive(Default)]
struct MainState {
    circle_handle: Handle<ColorMaterial>,
    planet_count: u32,
}
