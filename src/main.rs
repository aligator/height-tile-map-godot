use bevy::prelude::*;
use rand::prelude::*;
use rand::rngs::StdRng;

mod diamond_square;
mod open_ttd_mapper;

use diamond_square::DiamondSquareGenerator;
use open_ttd_mapper::OpenTTDMapper;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Camera positioned like in Godot world.tscn: position Vector2(175, 728), zoom 0.4
    commands.spawn((
        Camera2d,
        Transform::from_xyz(175.0, -728.0, 0.0), // Y inverted for Bevy coordinate system
    ));

    println!("Generating OpenTTD-style height map...");

    // Create height tile map with parameters from world.tscn HeightTileMap2
    let mut rng = StdRng::seed_from_u64(42);
    let size = 128 + 1; // size = 128 in world.tscn, +1 for corner heights
    let roughness = 10.0; // roughness = 10.0 in world.tscn
    let max_height = 16;

    let generator = DiamondSquareGenerator::new();
    let heights = generator.generate(size, roughness, max_height, &mut rng);

    let mapper = OpenTTDMapper::new();
    let tile_heights = mapper.build(&heights);

    // Print the height map (first few rows only due to size)
    println!("Height map (first 10x10):");
    for y in 0..10.min(heights[0].len()) {
        for x in 0..10.min(heights.len()) {
            print!("{:2} ", heights[x][y]);
        }
        println!();
    }

    println!("\nTile heights (lowest corner, first 10x10):");
    for y in 0..10.min(tile_heights[0].len()) {
        for x in 0..10.min(tile_heights.len()) {
            print!("{:2} ", tile_heights[x][y]);
        }
        println!();
    }

    // Load grass texture and create texture atlas layout
    let grass_texture: Handle<Image> = asset_server.load("grass_sheet.png");

    // Create texture atlas layout (19 tiles, each 64x63 pixels)
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(64, 63),
        19, // 19 tiles in a row
        1,  // 1 row
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Tile configuration matching Godot's TileMap system
    // In Godot, isometric transformation is handled by the TileSet, not manual calculation
    let tile_size = Vec2::new(64.0, 32.0); // From grassTiles.tres tile_size
    let texture_size = Vec2::new(64.0, 63.0); // Actual texture size

    // OpenTTD-style height shift - each layer is shifted by this amount
    let height_shift = Vec2::new(0.0, -8.0);

    // Create tiles exactly like in Godot: group by height layers and apply shift per layer
    // Use regular grid positioning (Godot's TileMap handles isometric transformation automatically)

    // Group tiles by their height (like Godot's tile_maps array)
    let mut tiles_by_height: std::collections::HashMap<i32, Vec<(usize, usize, usize)>> =
        std::collections::HashMap::new();

    for x in 0..tile_heights.len() {
        for y in 0..tile_heights[x].len() {
            let tile_height = tile_heights[x][y]; // This is the lowest corner height
            let sprite_index = get_sprite_index(x, y, &heights, &mapper);

            tiles_by_height
                .entry(tile_height)
                .or_insert_with(Vec::new)
                .push((x, y, sprite_index));
        }
    }

    // Debug: Print how many tiles are at each height
    println!("\nTiles per height layer:");
    for height in 0..max_height {
        if let Some(tiles) = tiles_by_height.get(&height) {
            println!("Height {}: {} tiles", height, tiles.len());
        } else {
            println!("Height {}: 0 tiles", height);
        }
    }

    // Now render each height layer with its own transform (like Godot's tile_maps)
    for height in 0..max_height {
        if let Some(tiles) = tiles_by_height.get(&height) {
            // Apply the height-based transform shift (like Godot's tile_map.set_transform)
            let layer_shift = height_shift * height as f32;

            println!("Rendering height {} with shift {:?}", height, layer_shift);

            for &(x, y, sprite_index) in tiles {
                // Isometric transformation (like Godot's isometric TileSet)
                // Convert grid coordinates to isometric coordinates
                let iso_x = (x as f32 - y as f32) * tile_size.x / 2.0;
                let iso_y = (x as f32 + y as f32) * tile_size.y / 2.0;

                // Apply layer transform shift
                let final_pos = Vec2::new(iso_x, iso_y) + layer_shift;

                commands.spawn((
                    Sprite {
                        image: grass_texture.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index: sprite_index,
                        }),
                        custom_size: Some(texture_size),
                        ..default()
                    },
                    Transform::from_xyz(
                        final_pos.x,
                        -final_pos.y,  // Y inverted for Bevy coordinate system
                        height as f32, // Z-index for proper layering
                    ),
                ));
            }
        }
    }

    // Analyze slope distribution (sample only)
    let mut slope_counts = std::collections::HashMap::new();
    let sample_size = 50.min(heights.len() - 1);

    for x in 0..sample_size {
        for y in 0..sample_size.min(heights[x].len() - 1) {
            let heights_at = mapper.get_heights(&heights, x, y);
            let lowest = mapper.get_lowest(&heights_at);

            // Calculate slope bitmask
            let mut bitmask = 0;

            // Check for steep slopes
            for i in 0..heights_at.len() {
                if heights_at[i] == lowest && heights_at[(i + 2) % 4] == lowest + 2 {
                    bitmask |= 0b10000;
                    break;
                }
            }

            // Check if all corners are the same height (flat)
            if heights_at[0] == heights_at[1]
                && heights_at[0] == heights_at[2]
                && heights_at[0] == heights_at[3]
            {
                bitmask = 0;
            } else {
                // Set corner flags
                if heights_at[0] > lowest {
                    bitmask |= 0b01000; // CORNER_N
                }
                if heights_at[1] > lowest {
                    bitmask |= 0b00100; // CORNER_E
                }
                if heights_at[2] > lowest {
                    bitmask |= 0b00010; // CORNER_S
                }
                if heights_at[3] > lowest {
                    bitmask |= 0b00001; // CORNER_W
                }
            }

            *slope_counts.entry(bitmask).or_insert(0) += 1;
        }
    }

    println!("\nSlope distribution (sample):");
    for (bitmask, count) in slope_counts {
        println!("Slope {:05b}: {} tiles", bitmask, count);
    }

    println!("\nOpenTTD-style height tile map generated successfully!");
    println!("Size: {}x{}, Roughness: {}", size - 1, size - 1, roughness);
    println!("Camera positioned at (175, 728) with proper tile layering!");
}

fn get_sprite_index(x: usize, y: usize, heights: &[Vec<i32>], mapper: &OpenTTDMapper) -> usize {
    let heights_at = mapper.get_heights(heights, x, y);
    let lowest = mapper.get_lowest(&heights_at);

    // Calculate slope bitmask
    let mut bitmask = 0;

    // Check for steep slopes
    for i in 0..heights_at.len() {
        if heights_at[i] == lowest && heights_at[(i + 2) % 4] == lowest + 2 {
            bitmask |= 0b10000;
            break;
        }
    }

    // Check if all corners are the same height (flat)
    if heights_at[0] == heights_at[1]
        && heights_at[0] == heights_at[2]
        && heights_at[0] == heights_at[3]
    {
        bitmask = 0;
    } else {
        // Set corner flags
        if heights_at[0] > lowest {
            bitmask |= 0b01000; // CORNER_N
        }
        if heights_at[1] > lowest {
            bitmask |= 0b00100; // CORNER_E
        }
        if heights_at[2] > lowest {
            bitmask |= 0b00010; // CORNER_S
        }
        if heights_at[3] > lowest {
            bitmask |= 0b00001; // CORNER_W
        }
    }

    // Map bitmask to sprite index (OpenTTD style)
    match bitmask {
        0b00000 => 0,  // SLOPE_FLAT
        0b00001 => 1,  // SLOPE_W
        0b00010 => 2,  // SLOPE_S
        0b00011 => 3,  // SLOPE_SW
        0b00100 => 4,  // SLOPE_E
        0b00101 => 5,  // SLOPE_EW
        0b00110 => 6,  // SLOPE_SE
        0b00111 => 7,  // SLOPE_WSE
        0b01000 => 8,  // SLOPE_N
        0b01001 => 9,  // SLOPE_NW
        0b01010 => 10, // SLOPE_NS
        0b01011 => 11, // SLOPE_NWS
        0b01100 => 12, // SLOPE_NE
        0b01101 => 13, // SLOPE_ENW
        0b01110 => 14, // SLOPE_SEN
        0b11000 => 15, // SLOPE_STEEP_N
        0b10010 => 16, // SLOPE_STEEP_S
        0b10001 => 17, // SLOPE_STEEP_W
        0b10100 => 18, // SLOPE_STEEP_E
        _ => 0,        // Default to flat
    }
}
