use bevy::prelude::*;

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
) {
    // 太陽
    commands.spawn((
        Sun,
        Transform:: from_xyz(SUN[1], SUN[2], SUN[3]).with_scale(Vec3::splat(SUN[0]))
    ));

    // 水星
    commands.spawn((
        Mercury,
        Transform:: from_xyz(MERCURY[1], MERCURY[2], MERCURY[3]).with_scale(Vec3::splat(MERCURY[0]))
    ));

    // 金星
    commands.spawn((
        Venus,
        Transform:: from_xyz(VENUS[1], VENUS[2], VENUS[3]).with_scale(Vec3::splat(VENUS[0]))
    ));

    // 地球
    commands.spawn((
        Earth,
        Transform:: from_xyz(EARTH[1], EARTH[2], EARTH[3]).with_scale(Vec3::splat(EARTH[0]))
    ));

    // 火星
    commands.spawn((
        Mars,
        Transform:: from_xyz(MARS[1], MARS[2], MARS[3]).with_scale(Vec3::splat(MARS[0]))
    ));

    // 木星
    commands.spawn((
        Jupiter,
        Transform:: from_xyz(JUPITER[1], JUPITER[2], JUPITER[3]).with_scale(Vec3::splat(JUPITER[0]))
    ));

    // 土星
    commands.spawn((
        Saturn,
        Transform:: from_xyz(SATURN[1], SATURN[2], SATURN[3]).with_scale(Vec3::splat(SATURN[0]))
    ));

    // 天王星
    commands.spawn((
        Uranus,
        Transform:: from_xyz(URANUS[1], URANUS[2], URANUS[3]).with_scale(Vec3::splat(URANUS[0]))
    ));

    // 海王星
    commands.spawn((
        Neptune,
        Transform:: from_xyz(NEPTUNE[1], NEPTUNE[2], NEPTUNE[3]).with_scale(Vec3::splat(NEPTUNE[0]))
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