use bevy::prelude::*;
use rand::Rng;

mod diamond_square;
mod height_map_resource;
mod open_ttd_mapper;
mod open_ttd_renderer;

use height_map_resource::{DefaultHeightMapPlugin, HeightMapConfig};

/// Marker component for terrain tiles that need to be updated when height map changes
#[derive(Component)]
struct TerrainTile;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(HeightMapConfig {
            size: 129,
            roughness: 15.0,
            max_height: 20,
            seed: 12345,
        })
        .add_plugins(DefaultHeightMapPlugin::default())
        .add_systems(Startup, setup_camera)
        // Runtime regeneration systems - change config and map auto-regenerates!
        .add_systems(
            Update,
            (
                regenerate_on_spacebar,
                change_roughness_on_keys,
                change_seed_on_enter,
            ),
        )
        .run();
}

/// Setup system that only creates the camera - terrain is handled by the plugin
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 0.0), // Y inverted for Bevy coordinate system
    ));

    println!("🎮 Controls:");
    println!("  SPACE  - Generate new random map");
    println!("  E/Q    - Decrease/Increase terrain roughness");
    println!("  ENTER  - Cycle through terrain presets");
    println!("  Maps regenerate automatically when you change settings!");
}

/// Example system: Press SPACE to regenerate map with random seed
fn regenerate_on_spacebar(keys: Res<ButtonInput<KeyCode>>, mut config: ResMut<HeightMapConfig>) {
    if keys.just_pressed(KeyCode::Space) {
        config.seed = rand::thread_rng().gen(); // Random seed
        println!("🎲 Generating new map with random seed: {}", config.seed);
    }
}

/// Example system: Use +/- keys to change terrain roughness
fn change_roughness_on_keys(keys: Res<ButtonInput<KeyCode>>, mut config: ResMut<HeightMapConfig>) {
    if keys.just_pressed(KeyCode::KeyE) {
        // + key
        config.roughness = (config.roughness + 2.0).min(50.0);
        println!("⛰️  Increased roughness to: {}", config.roughness);
    }
    if keys.just_pressed(KeyCode::KeyQ) {
        // - key
        config.roughness = (config.roughness - 2.0).max(1.0);
        println!("🏔️  Decreased roughness to: {}", config.roughness);
    }
}

/// Example system: Press ENTER to cycle through predefined map styles
fn change_seed_on_enter(keys: Res<ButtonInput<KeyCode>>, mut config: ResMut<HeightMapConfig>) {
    if keys.just_pressed(KeyCode::Enter) {
        // Cycle through some interesting predefined configurations
        match config.seed {
            12345 => {
                *config = HeightMapConfig {
                    size: 129,
                    roughness: 25.0,
                    max_height: 30,
                    seed: 54321,
                };
                println!("🏔️  Switched to mountainous terrain");
            }
            54321 => {
                *config = HeightMapConfig {
                    size: 129,
                    roughness: 5.0,
                    max_height: 8,
                    seed: 99999,
                };
                println!("🏞️  Switched to gentle hills");
            }
            _ => {
                *config = HeightMapConfig {
                    size: 129,
                    roughness: 15.0,
                    max_height: 20,
                    seed: 12345,
                };
                println!("🗻  Switched to default terrain");
            }
        }
    }
}
