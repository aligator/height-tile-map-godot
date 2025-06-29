use bevy::prelude::*;
use rand::Rng;

mod diamond_square;
mod height_map_resource;
mod open_ttd_mapper;
mod open_ttd_renderer;

use diamond_square::DiamondSquareGenerator;
use height_map_resource::{
    DefaultHeightMapBundle, DefaultHeightMapPlugin, HeightMapConfig, HeightMapEntity, TerrainTile,
};
use open_ttd_mapper::OpenTTDMapper;
use open_ttd_renderer::{OpenTTDRenderer, OpenTTDRendererConfig};

/// Marker components to distinguish different maps
#[derive(Component)]
struct MainMap;

#[derive(Component)]
struct SecondMap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(DefaultHeightMapPlugin::default())
        .add_systems(Startup, (setup_camera, spawn_maps))
        // Runtime regeneration systems - change config on entities!
        .add_systems(
            Update,
            (
                move_camera,
                zoom_camera,
                regenerate_on_spacebar,
                change_roughness_on_keys,
                change_seed_on_enter,
                toggle_map_on_tab,
            ),
        )
        .run();
}

/// Setup system that only creates the camera
fn setup_camera(mut commands: Commands) {
    // Position camera to view isometric terrain better
    // Terrain will be rendered around (0,0) in isometric coordinates
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 1000.0), // Position camera to see terrain
    ));

    println!("📷 Camera positioned at (0, 0, 1000) to view terrain");
    println!("🎮 Controls:");
    println!("  WASD   - Move camera (W/S = up/down, A/D = left/right)");
    println!("  CTRL   - Zoom out (Z-axis)");
    println!("  SHIFT  - Zoom in (Z-axis)");
    println!("  SPACE  - Generate new random map for main map");
    println!("  E/Q    - Decrease/Increase terrain roughness for main map");
    println!("  ENTER  - Cycle through terrain presets for main map");
    println!("  TAB    - Toggle 2nd-map visibility");
    println!("  Multiple maps regenerate independently!");
}

/// Setup function to spawn height map entities
fn spawn_maps(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    println!("🌍 Spawning maps...");

    // Create renderer configs
    let grass_config =
        OpenTTDRendererConfig::default_grass(&asset_server, &mut texture_atlas_layouts);

    // Example: Custom config with different parameters
    let custom_config = OpenTTDRendererConfig::from_assets(
        &asset_server,
        &mut texture_atlas_layouts,
        "grass_sheet.png", // Could be "stone_sheet.png", "snow_sheet.png", etc.
        UVec2::new(64, 63),
        (19, 1),
    )
    .with_tile_size(Vec2::new(64.0, 32.0)) // Custom tile positioning
    .with_texture_size(Vec2::new(64.0, 63.0)) // Custom texture size
    .with_height_shift(Vec2::new(0.0, -8.0)); // Custom 3D effect

    // Main map with default grass texture
    let main_map = commands
        .spawn((
            DefaultHeightMapBundle::new(
                HeightMapConfig {
                    size: 129,
                    roughness: 15.0,
                    max_height: 20,
                    seed: 12345,
                },
                DiamondSquareGenerator::default(),
                OpenTTDMapper::default(),
                OpenTTDRenderer::new(grass_config),
            ),
            MainMap, // Custom marker for the main map
            Name::new("Main Map"),
        ))
        .id();

    // Second map with custom renderer configuration
    let second_map = commands
        .spawn((
            DefaultHeightMapBundle::new(
                HeightMapConfig {
                    size: 65,       // Smaller size
                    roughness: 5.0, // Less rough
                    max_height: 8,  // Lower height
                    seed: 54321,    // Different seed
                },
                DiamondSquareGenerator::default(),
                OpenTTDMapper::default(),
                OpenTTDRenderer::new(custom_config),
            )
            .with_transform(Transform::from_xyz(3000.0, 3000.0, 100.0)), // Offset position
            SecondMap, // Custom marker for the second map
            Name::new("Second Map"),
            Visibility::Visible, // Start visible
        ))
        .id();

    println!(
        "🗺️  Spawned main map (entity {:?}) and mini-map (entity {:?})",
        main_map, second_map
    );
    println!("📦 Map entities should now be processed by height map generation systems...");
}

/// Example system: Press SPACE to regenerate main map with random seed
fn regenerate_on_spacebar(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut HeightMapConfig, (With<HeightMapEntity>, With<MainMap>)>,
) {
    if keys.just_pressed(KeyCode::Space) {
        if let Ok(mut config) = query.single_mut() {
            config.seed = rand::thread_rng().gen(); // Random seed
            println!(
                "🎲 Generating new main map with random seed: {}",
                config.seed
            );
        }
    }
}

/// Example system: Use E/Q keys to change terrain roughness of main map
fn change_roughness_on_keys(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut HeightMapConfig, (With<HeightMapEntity>, With<MainMap>)>,
) {
    if let Ok(mut config) = query.single_mut() {
        if keys.just_pressed(KeyCode::KeyE) {
            config.roughness = (config.roughness + 2.0).min(50.0);
            println!("⛰️  Increased main map roughness to: {}", config.roughness);
        }
        if keys.just_pressed(KeyCode::KeyQ) {
            config.roughness = (config.roughness - 2.0).max(1.0);
            println!("🏔️  Decreased main map roughness to: {}", config.roughness);
        }
    }
}

/// Example system: Press ENTER to cycle through predefined map styles for main map
fn change_seed_on_enter(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut HeightMapConfig, (With<HeightMapEntity>, With<MainMap>)>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        if let Ok(mut config) = query.single_mut() {
            // Cycle through some interesting predefined configurations
            match config.seed {
                12345 => {
                    *config = HeightMapConfig {
                        size: 129,
                        roughness: 25.0,
                        max_height: 30,
                        seed: 54321,
                    };
                    println!("🏔️  Switched main map to mountainous terrain");
                }
                54321 => {
                    *config = HeightMapConfig {
                        size: 129,
                        roughness: 5.0,
                        max_height: 8,
                        seed: 99999,
                    };
                    println!("🏞️  Switched main map to gentle hills");
                }
                _ => {
                    *config = HeightMapConfig {
                        size: 129,
                        roughness: 15.0,
                        max_height: 20,
                        seed: 12345,
                    };
                    println!("🗻  Switched main map to default terrain");
                }
            }
        }
    }
}

/// Example system: Press TAB to toggle mini-map visibility
fn toggle_map_on_tab(
    keys: Res<ButtonInput<KeyCode>>,
    minimap_query: Query<Entity, (With<HeightMapEntity>, With<SecondMap>)>,
    mut terrain_query: Query<(Entity, &mut Visibility, &TerrainTile)>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        if let Ok(minimap_entity) = minimap_query.single() {
            // Find current visibility state of mini-map tiles
            let mut current_visibility = Visibility::Visible;
            let mut found_tile = false;

            // Check current state by looking at first mini-map terrain tile
            for (_, visibility, terrain_tile) in terrain_query.iter() {
                if terrain_tile.map_entity == minimap_entity {
                    current_visibility = *visibility;
                    found_tile = true;
                    break;
                }
            }

            if !found_tile {
                println!("⚠️  No terrain tiles found for mini-map");
                return;
            }

            // Toggle to opposite visibility
            let new_visibility = match current_visibility {
                Visibility::Visible => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
                Visibility::Inherited => Visibility::Hidden,
            };

            // Apply new visibility to all mini-map terrain tiles
            let mut toggled_count = 0;
            for (_, mut visibility, terrain_tile) in terrain_query.iter_mut() {
                if terrain_tile.map_entity == minimap_entity {
                    *visibility = new_visibility;
                    toggled_count += 1;
                }
            }

            match new_visibility {
                Visibility::Hidden => println!("🙈 Mini-map hidden ({} tiles)", toggled_count),
                Visibility::Visible => println!("👁️  Mini-map visible ({} tiles)", toggled_count),
                _ => {}
            }
        }
    }
}

/// Camera movement system - WASD for movement
fn move_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = camera_query.single_mut() {
        let mut movement = Vec3::ZERO;
        let speed = 500.0; // Movement speed

        // Horizontal movement (WASD)
        if keys.pressed(KeyCode::KeyW) {
            movement.y += speed;
        }
        if keys.pressed(KeyCode::KeyS) {
            movement.y -= speed;
        }
        if keys.pressed(KeyCode::KeyA) {
            movement.x -= speed;
        }
        if keys.pressed(KeyCode::KeyD) {
            movement.x += speed;
        }

        // Apply movement
        transform.translation += movement * time.delta_secs();
    }
}

/// Camera zoom system - Z to zoom in, X to zoom out
fn zoom_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = camera_query.single_mut() {
        let mut zoom_delta = 0.0;
        let zoom_speed = 2.0; // Zoom speed

        // Shift to zoom in (smaller scale = closer view)
        if keys.pressed(KeyCode::ShiftLeft) {
            zoom_delta -= zoom_speed * time.delta_secs();
        }
        // Ctrl to zoom out (larger scale = farther view)
        if keys.pressed(KeyCode::ControlLeft) {
            zoom_delta += zoom_speed * time.delta_secs();
        }

        if zoom_delta != 0.0 {
            // Apply zoom by modifying the transform scale
            let new_scale = (transform.scale.x + zoom_delta).clamp(0.1, 5.0);
            transform.scale = Vec3::new(new_scale, new_scale, 1.0);
            println!("🔍 Camera zoom: {:.2}", new_scale);
        }
    }
}
