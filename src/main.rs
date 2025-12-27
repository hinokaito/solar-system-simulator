use bevy::{
    core_pipeline::tonemapping::Tonemapping, post_process::bloom::Bloom, prelude::*, render::view::Hdr
};

// ========================================
// Constants
// ========================================

// === 天文学単位 ===
const AU_KM: f32 = 149_597_870.0;
const AU_UNITS: f32 = 23_455.0; 

// === 単位変換（地球半径スケール） ===
const KM_PER_UNIT: f32 = AU_KM / AU_UNITS;
const UNIT_S_TO_KMH: f32 = KM_PER_UNIT * 3600.0;

// === [相対サイズ, x, y, z] ===
const SUN:     [f32; 4] =   [109.0, 0.0 * AU_UNITS,   0.0, 0.0];
const MERCURY: [f32; 4] =   [0.38,  0.32 * AU_UNITS,  0.0, 0.0];
const VENUS:   [f32; 4] =   [0.95,  0.72 * AU_UNITS,  0.0, 0.0];
const EARTH:   [f32; 4] =   [1.00,  1.00 * AU_UNITS,  0.0, 0.0];
const MARS:    [f32; 4] =   [0.53,  1.52 * AU_UNITS,  0.0, 0.0];
const JUPITER: [f32; 4] =   [11.21, 5.20 * AU_UNITS,  0.0, 0.0];
const SATURN:  [f32; 4] =   [9.45,  9.54 * AU_UNITS,  0.0, 0.0];
const URANUS:  [f32; 4] =   [4.01,  19.19 * AU_UNITS, 0.0, 0.0];
const NEPTUNE: [f32; 4] =   [3.88,  30.07 * AU_UNITS, 0.0, 0.0];


// ========================================
// Components
// ========================================

#[derive(Component)]
struct Sun;

#[derive(Component)]
struct Mercury;

#[derive(Component)]
struct Venus;

#[derive(Component)]
struct Earth;

#[derive(Component)]
struct Mars;

#[derive(Component)]
struct Jupiter;

#[derive(Component)]
struct Saturn;

#[derive(Component)]
struct Uranus;

#[derive(Component)] 
struct Neptune;


// ========================================
// Systems
// ========================================

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // ========================================
    // 天体
    // ========================================

    // テクスチャマップ
    let sun_texture = asset_server.load("sun.png");
    let mercury_texture = asset_server.load("mercury.png");
    let venus_texture = asset_server.load("venus.png");
    let earth_texture = asset_server.load("earth.png");
    let mars_texture = asset_server.load("mars.png");
    let jupiter_texture = asset_server.load("jupiter.png");
    let saturn_texture = asset_server.load("saturn.png");
    let uranus_texture = asset_server.load("uranus.png");
    let neptune_texture = asset_server.load("neptune.png");

    // 天体メッシュ
    let celestial_body_mesh = meshes.add(Sphere::new(1.0));

    // 太陽
    commands.spawn((
        Sun,
        Mesh3d(celestial_body_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(sun_texture),
            emissive: LinearRgba::rgb(256.0, 60.0, 15.0),
            ..default()
        })),
        Transform:: from_xyz(SUN[1], SUN[2], SUN[3]).with_scale(Vec3::splat(SUN[0]))
    ));

    // 水星
    commands.spawn((
        Mercury,
        Mesh3d(celestial_body_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(mercury_texture),
            ..default()
        })),
        Transform:: from_xyz(MERCURY[1], MERCURY[2], MERCURY[3]).with_scale(Vec3::splat(MERCURY[0]))
    ));

    // 金星
    commands.spawn((
        Venus,
        Mesh3d(celestial_body_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(venus_texture),
            ..default()
        })),
        Transform:: from_xyz(VENUS[1], VENUS[2], VENUS[3]).with_scale(Vec3::splat(VENUS[0]))
    ));

    // 地球
    commands.spawn((
        Earth,
        Mesh3d(celestial_body_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(earth_texture),
            ..default()
        })),
        Transform:: from_xyz(EARTH[1], EARTH[2], EARTH[3]).with_scale(Vec3::splat(EARTH[0]))
    ));

    // 火星
    commands.spawn((
        Mars,
        Mesh3d(celestial_body_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(mars_texture),
            ..default()
        })),
        Transform:: from_xyz(MARS[1], MARS[2], MARS[3]).with_scale(Vec3::splat(MARS[0]))
    ));

    // 木星
    commands.spawn((
        Jupiter,
        Mesh3d(celestial_body_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(jupiter_texture),
            ..default()
        })),
        Transform:: from_xyz(JUPITER[1], JUPITER[2], JUPITER[3]).with_scale(Vec3::splat(JUPITER[0]))
    ));

    // 土星
    commands.spawn((
        Saturn,
        Mesh3d(celestial_body_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(saturn_texture),
            ..default()
        })),
        Transform:: from_xyz(SATURN[1], SATURN[2], SATURN[3]).with_scale(Vec3::splat(SATURN[0]))
    ));

    // 天王星
    commands.spawn((
        Uranus,
        Mesh3d(celestial_body_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(uranus_texture),
            ..default()
        })),
        Transform:: from_xyz(URANUS[1], URANUS[2], URANUS[3]).with_scale(Vec3::splat(URANUS[0]))
    ));

    // 海王星
    commands.spawn((
        Neptune,
        Mesh3d(celestial_body_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(neptune_texture),
            ..default()
        })),
        Transform:: from_xyz(NEPTUNE[1], NEPTUNE[2], NEPTUNE[3]).with_scale(Vec3::splat(NEPTUNE[0]))
    ));

    // ========================================
    // カメラ・ライト
    // ========================================

    // カメラ
    commands.spawn((
        Camera3d::default(),
        Hdr,
        Tonemapping::TonyMcMapface,
        Bloom {
            intensity: 0.1,
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            far: 1_000_000.0,
            ..default()
        }),
        Transform::from_xyz(1000.0, 0.0, 0.0)
            .looking_at(Vec3::ZERO, Vec3::Y)
    ));

    // ライト
    commands.spawn((
        PointLight {
            intensity: 25_000_000_000_000.0,
            range: 1_000_000.0,
            radius: 109.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));
}


// ========================================
// App Entry Point
// ========================================s

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}