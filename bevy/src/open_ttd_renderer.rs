use bevy::prelude::*;
use std::collections::HashMap;

use crate::{
    diamond_square::HeightMap,
    height_map_resource::{HeightMapConfig, TerrainRenderer, TerrainTile},
    open_ttd_mapper::{OpenTTDMapper, TileHeights},
};

/// Configuration for OpenTTD-style terrain rendering
/// Contains all the assets and parameters needed for rendering
#[derive(Clone, Debug)]
pub struct OpenTTDRendererConfig {
    /// The texture handle for the terrain sprite sheet
    pub texture: Handle<Image>,
    /// The texture atlas layout handle
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
    /// Logical tile size for positioning
    pub tile_size: Vec2,
    /// Actual texture size for rendering
    pub texture_size: Vec2,
    /// Height shift per layer (for 3D effect)
    pub height_shift: Vec2,
}

impl OpenTTDRendererConfig {
    /// Create a new config with pre-loaded assets
    pub fn new(texture: Handle<Image>, texture_atlas_layout: Handle<TextureAtlasLayout>) -> Self {
        Self {
            texture,
            texture_atlas_layout,
            tile_size: Vec2::new(64.0, 32.0),
            texture_size: Vec2::new(64.0, 63.0),
            height_shift: Vec2::new(0.0, -8.0),
        }
    }

    /// Create a config from asset server and texture atlas layouts
    pub fn from_assets(
        asset_server: &AssetServer,
        texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
        texture_path: &str,
        texture_tile_size: UVec2,
        atlas_size: (u32, u32),
    ) -> Self {
        let texture = asset_server.load(texture_path);

        let layout = TextureAtlasLayout::from_grid(
            texture_tile_size,
            atlas_size.0,
            atlas_size.1,
            None,
            None,
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        Self::new(texture, texture_atlas_layout)
    }

    /// Create default grass config
    pub fn default_grass(
        asset_server: &AssetServer,
        texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    ) -> Self {
        Self::from_assets(
            asset_server,
            texture_atlas_layouts,
            "grass_sheet.png",
            UVec2::new(64, 63),
            (19, 1),
        )
    }

    /// Builder pattern methods for customization
    pub fn with_tile_size(mut self, tile_size: Vec2) -> Self {
        self.tile_size = tile_size;
        self
    }

    pub fn with_texture_size(mut self, texture_size: Vec2) -> Self {
        self.texture_size = texture_size;
        self
    }

    pub fn with_height_shift(mut self, height_shift: Vec2) -> Self {
        self.height_shift = height_shift;
        self
    }
}

/// OpenTTD-style terrain renderer that works with OpenTTDMapper
pub struct OpenTTDRenderer {
    pub config: OpenTTDRendererConfig,
}

impl OpenTTDRenderer {
    /// Create a new OpenTTDRenderer with custom configuration
    pub fn new(config: OpenTTDRendererConfig) -> Self {
        Self { config }
    }
}

impl TerrainRenderer for OpenTTDRenderer {
    type Mapper = OpenTTDMapper;

    fn spawn_terrain(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
        heights: &HeightMap,
        tile_heights: &TileHeights,
        config: &HeightMapConfig,
        mapper: &Self::Mapper,
        map_entity: Entity,
        map_transform: &Transform,
    ) {
        let max_height = config.max_height;

        // Use the texture and layout from our config
        let grass_texture = self.config.texture.clone();
        let texture_atlas_layout = self.config.texture_atlas_layout.clone();

        // Group tiles by their height (like Godot's tile_maps array)
        let mut tiles_by_height: HashMap<i32, Vec<(usize, usize, usize)>> = HashMap::new();

        for x in 0..tile_heights.len() {
            for y in 0..tile_heights[x].len() {
                let tile_height = tile_heights[x][y]; // This is the lowest corner height
                let sprite_index = self.get_sprite_index(x, y, heights, mapper);

                tiles_by_height
                    .entry(tile_height)
                    .or_insert_with(Vec::new)
                    .push((x, y, sprite_index));
            }
        }

        // Debug: Print how many tiles are at each height
        println!("\nTiles per height layer for map {:?}:", map_entity);
        for height in 0..max_height {
            if let Some(tiles) = tiles_by_height.get(&height) {
                println!("Height {}: {} tiles", height, tiles.len());
            } else {
                println!("Height {}: 0 tiles", height);
            }
        }

        // Now render each height layer with its own transform (like Godot's tile_maps)
        let mut total_tiles_spawned = 0;

        for height in 0..max_height {
            if let Some(tiles) = tiles_by_height.get(&height) {
                // Apply the height-based transform shift (like Godot's tile_map.set_transform)
                let layer_shift = self.config.height_shift * height as f32;

                println!(
                    "Rendering height {} with shift {:?} for map {:?} - {} tiles",
                    height,
                    layer_shift,
                    map_entity,
                    tiles.len()
                );

                for &(x, y, sprite_index) in tiles {
                    // Isometric transformation (like Godot's isometric TileSet)
                    // Convert grid coordinates to isometric coordinates
                    let iso_x = (x as f32 - y as f32) * self.config.tile_size.x / 2.0;
                    let iso_y = (x as f32 + y as f32) * self.config.tile_size.y / 2.0;

                    // Apply layer transform shift
                    let final_pos = Vec2::new(iso_x, iso_y) + layer_shift;

                    // Apply map entity transform as offset
                    let world_pos = final_pos + map_transform.translation.xy();

                    commands.spawn((
                        Sprite {
                            image: grass_texture.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: texture_atlas_layout.clone(),
                                index: sprite_index,
                            }),
                            custom_size: Some(self.config.texture_size),
                            ..default()
                        },
                        Transform::from_xyz(
                            world_pos.x,
                            -world_pos.y, // Y inverted for Bevy coordinate system
                            height as f32 + map_transform.translation.z, // Z-index with map offset
                        ),
                        TerrainTile { map_entity }, // Reference to parent map
                    ));

                    total_tiles_spawned += 1;
                }
            }
        }

        println!(
            "🎨 Total tiles spawned for map {:?}: {}",
            map_entity, total_tiles_spawned
        );

        // Analyze slope distribution (sample only)
        let mut slope_counts = HashMap::new();
        let sample_size = 50.min(heights.len() - 1);

        for x in 0..sample_size {
            for y in 0..sample_size.min(heights[x].len() - 1) {
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

                *slope_counts.entry(bitmask).or_insert(0) += 1;
            }
        }

        println!("\nSlope distribution (sample) for map {:?}:", map_entity);
        for (bitmask, count) in slope_counts {
            println!("Slope {:05b}: {} tiles", bitmask, count);
        }

        println!(
            "\nOpenTTD-style height tile map generated successfully for map {:?}!",
            map_entity
        );
        println!(
            "Size: {}x{}, Roughness: {}",
            config.size - 1,
            config.size - 1,
            config.roughness
        );
        println!("Camera positioned with proper tile layering!");
    }

    fn get_sprite_index(
        &self,
        x: usize,
        y: usize,
        heights: &HeightMap,
        mapper: &Self::Mapper,
    ) -> usize {
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
            return 0; // Flat tile
        }

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

        // Map bitmask to sprite index (simplified mapping)
        match bitmask & 0b01111 {
            0b0000 => 0,  // Flat
            0b0001 => 1,  // W raised
            0b0010 => 2,  // S raised
            0b0011 => 3,  // SW raised
            0b0100 => 4,  // E raised
            0b0101 => 5,  // EW raised
            0b0110 => 6,  // ES raised
            0b0111 => 7,  // ESW raised
            0b1000 => 8,  // N raised
            0b1001 => 9,  // NW raised
            0b1010 => 10, // NS raised
            0b1011 => 11, // NSW raised
            0b1100 => 12, // NE raised
            0b1101 => 13, // NEW raised
            0b1110 => 14, // NES raised
            0b1111 => 15, // All raised
            _ => 0,       // Default to flat
        }
    }
}
