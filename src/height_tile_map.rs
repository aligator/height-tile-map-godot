use bevy::prelude::*;

use crate::open_ttd_mapper::OpenTTDMapper;

pub struct HeightTileMapPlugin;

impl Plugin for HeightTileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_height_tile_map)
            .add_systems(Update, update_height_tile_map);
    }
}

#[derive(Component)]
pub struct HeightTileMap {
    pub heights: Vec<Vec<i32>>,
    pub tile_heights: Vec<Vec<i32>>,
    pub size: usize,
    pub max_height: i32,
    pub shift: Vec2,
}

#[derive(Component)]
pub struct TileLayer {
    pub height: i32,
}

#[derive(Component)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub height: i32,
}

fn spawn_height_tile_map(
    mut commands: Commands,
    query: Query<&HeightTileMap, Added<HeightTileMap>>,
) {
    for height_tile_map in query.iter() {
        // Create tile layers for each height
        for height in 0..height_tile_map.max_height {
            let layer_entity = commands
                .spawn((
                    TileLayer { height },
                    Transform::from_xyz(0.0, height_tile_map.shift.y * height as f32, 0.0),
                    GlobalTransform::default(),
                ))
                .id();

            // Spawn tiles for this layer
            for x in 0..height_tile_map.tile_heights.len() {
                for y in 0..height_tile_map.tile_heights[x].len() {
                    if height_tile_map.tile_heights[x][y] == height {
                        // Create a simple colored tile
                        let tile_color = get_tile_color(x, y, height_tile_map);

                        commands
                            .spawn((
                                Tile { x, y, height },
                                Transform::from_xyz(
                                    (x as f32 - height_tile_map.size as f32 / 2.0) * 32.0,
                                    (y as f32 - height_tile_map.size as f32 / 2.0) * 32.0,
                                    height as f32,
                                ),
                                GlobalTransform::default(),
                            ))
                            .set_parent(layer_entity);
                    }
                }
            }
        }
    }
}

fn update_height_tile_map(// Add any update logic here if needed
) {
    // This system can be used for real-time updates
}

fn get_tile_color(x: usize, y: usize, height_tile_map: &HeightTileMap) -> Color {
    // Get the slope type for this tile
    let mapper = OpenTTDMapper::new();
    let heights_at = mapper.get_heights(&height_tile_map.heights, x, y);
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

    // Return different colors based on slope type
    match bitmask {
        0b00000 => Color::rgb(0.2, 0.8, 0.2), // Flat - green
        0b00001 => Color::rgb(0.3, 0.7, 0.2), // W slope
        0b00010 => Color::rgb(0.4, 0.6, 0.2), // S slope
        0b00011 => Color::rgb(0.5, 0.5, 0.2), // SW slope
        0b00100 => Color::rgb(0.6, 0.4, 0.2), // E slope
        0b00101 => Color::rgb(0.7, 0.3, 0.2), // EW slope
        0b00110 => Color::rgb(0.8, 0.2, 0.2), // SE slope
        0b00111 => Color::rgb(0.9, 0.1, 0.2), // WSE slope
        0b01000 => Color::rgb(0.1, 0.9, 0.2), // N slope
        0b01001 => Color::rgb(0.2, 0.8, 0.3), // NW slope
        0b01010 => Color::rgb(0.3, 0.7, 0.4), // NS slope
        0b01011 => Color::rgb(0.4, 0.6, 0.5), // NWS slope
        0b01100 => Color::rgb(0.5, 0.5, 0.6), // NE slope
        0b01101 => Color::rgb(0.6, 0.4, 0.7), // ENW slope
        0b01110 => Color::rgb(0.7, 0.3, 0.8), // SEN slope
        0b11000 => Color::rgb(0.8, 0.2, 0.9), // Steep N
        0b10010 => Color::rgb(0.9, 0.1, 1.0), // Steep S
        0b10001 => Color::rgb(1.0, 0.0, 0.8), // Steep W
        0b10100 => Color::rgb(0.8, 0.0, 1.0), // Steep E
        _ => Color::rgb(0.5, 0.5, 0.5),       // Default gray
    }
}
