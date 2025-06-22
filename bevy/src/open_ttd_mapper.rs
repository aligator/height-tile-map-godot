//! This is the reference implementation of a mapper script.
//! It implements the OpenTTD tile system.
//!
//! Reference: https://newgrf-specs.tt-wiki.net/wiki/NML:List_of_tile_slopes
//!
//! The bitboards to map to the specific tile indices is built like in
//! OpenTTD: https://newgrf-specs.tt-wiki.net/wiki/NML:List_of_tile_slopes
//! From the lsb to msb:
//! - CORNER_W: west corner is above the lowest corner.
//! - CORNER_S: south corner is above the lowest corner.
//! - CORNER_E: east corner is above the lowest corner.
//! - CORNER_N: north corner is above the lowest corner.
//! - IS_STEEP_SLOPE: this tile is a steep slope (the corner opposite to the lowest corner is 2 units higher).

/// Bit flags for corner elevation and slope steepness.
///
/// These flags are combined to create the bitmask that determines
/// the appropriate tile sprite for a given terrain configuration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flag {
    /// West corner is above the lowest corner
    CornerW = 0b00001,
    /// South corner is above the lowest corner
    CornerS = 0b00010,
    /// East corner is above the lowest corner
    CornerE = 0b00100,
    /// North corner is above the lowest corner
    CornerN = 0b01000,
    /// This tile is a steep slope (opposite corner is 2 units higher)
    IsSteepSlope = 0b10000,
}

/// Represents the different slope types in the OpenTTD tile system.
///
/// Each variant corresponds to a specific bit pattern that indicates
/// which corners are elevated relative to the lowest corner of the tile.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlopeType {
    /// Flat terrain - all corners at same height
    Flat = 0b00000,
    /// West corner elevated
    W = 0b00001,
    /// South corner elevated
    S = 0b00010,
    /// East corner elevated
    E = 0b00100,
    /// North corner elevated
    N = 0b01000,
    /// North-West corners elevated
    NW = 0b01001,
    /// South-West corners elevated
    SW = 0b00011,
    /// South-East corners elevated
    SE = 0b00110,
    /// North-East corners elevated
    NE = 0b01100,
    /// East-West corners elevated (valley)
    EW = 0b00101,
    /// North-South corners elevated (valley)
    NS = 0b01010,
    /// North-West-South corners elevated
    NWS = 0b01011,
    /// West-South-East corners elevated
    WSE = 0b00111,
    /// South-East-North corners elevated
    SEN = 0b01110,
    /// East-North-West corners elevated
    ENW = 0b01101,
    /// Steep slope with west corner lowest
    SteepW = 0b10001,
    /// Steep slope with south corner lowest
    SteepS = 0b10010,
    /// Steep slope with east corner lowest
    SteepE = 0b10100,
    /// Steep slope with north corner lowest
    SteepN = 0b11000,
    /// Invalid slope configuration
    Invalid = -1,
}

impl From<i32> for SlopeType {
    /// Convert from bitmask integer to SlopeType enum.
    ///
    /// # Arguments
    ///
    /// * `value` - The bitmask representing the slope configuration
    ///
    /// # Returns
    ///
    /// The corresponding SlopeType, or Invalid if no match is found
    fn from(value: i32) -> Self {
        match value {
            0b00000 => SlopeType::Flat,
            0b00001 => SlopeType::W,
            0b00010 => SlopeType::S,
            0b00100 => SlopeType::E,
            0b01000 => SlopeType::N,
            0b01001 => SlopeType::NW,
            0b00011 => SlopeType::SW,
            0b00110 => SlopeType::SE,
            0b01100 => SlopeType::NE,
            0b00101 => SlopeType::EW,
            0b01010 => SlopeType::NS,
            0b01011 => SlopeType::NWS,
            0b00111 => SlopeType::WSE,
            0b01110 => SlopeType::SEN,
            0b01101 => SlopeType::ENW,
            0b10001 => SlopeType::SteepW,
            0b10010 => SlopeType::SteepS,
            0b10100 => SlopeType::SteepE,
            0b11000 => SlopeType::SteepN,
            _ => SlopeType::Invalid,
        }
    }
}

/// OpenTTD-style terrain mapper that converts corner heights to tile configurations.
///
/// This mapper takes corner height data and generates appropriate tile sprites
/// and height information for rendering OpenTTD-style terrain.
pub struct OpenTTDMapper;

impl OpenTTDMapper {
    pub fn new() -> Self {
        Self
    }

    /// Build takes the corner_heights and generates tile height information.
    ///
    /// It calculates which tile from the tile set each tile should have based on
    /// the corner height differences. It returns a new height map (tile_heights)
    /// which maps each tile to an actual height. This information is later used
    /// to shift each tile to the correct position.
    ///
    /// # Arguments
    ///
    /// * `corner_heights` - 2D array of corner heights for the terrain
    ///
    /// # Returns
    ///
    /// A 2D vector containing the tile heights (height of lowest corner for each tile)
    pub fn build(&self, corner_heights: &[Vec<i32>]) -> Vec<Vec<i32>> {
        let mut tile_heights = vec![vec![0; corner_heights[0].len() - 1]; corner_heights.len() - 1];

        // Read the HeightMap and generate the tilemaps
        for x in 0..corner_heights.len() - 1 {
            for y in 0..corner_heights[x].len() - 1 {
                // Get all relevant heights
                let heights_at = self.get_heights(corner_heights, x, y);
                let lowest = self.get_lowest(&heights_at);

                // Get the bitboard number
                let mut bitmask = 0;

                // Find steep tiles - check if opposite corners differ by 2 units
                for i in 0..heights_at.len() {
                    if heights_at[i] == lowest && heights_at[(i + 2) % 4] == lowest + 2 {
                        bitmask |= Flag::IsSteepSlope as i32;
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
                    // Set corner flags based on which corners are above lowest
                    if heights_at[0] > lowest {
                        bitmask |= Flag::CornerN as i32;
                    }
                    if heights_at[1] > lowest {
                        bitmask |= Flag::CornerE as i32;
                    }
                    if heights_at[2] > lowest {
                        bitmask |= Flag::CornerS as i32;
                    }
                    if heights_at[3] > lowest {
                        bitmask |= Flag::CornerW as i32;
                    }
                }

                // Get sprite index from bitmask
                let _sprite_index = self.bitmask_to_sprite_index(bitmask);

                // Store the tile height (lowest corner height)
                // Note: We ignore the actual height here for sprite selection.
                //       The actual height is mapped by the TileMap.
                //       Instead set the tile_heights to the height we want our tile at.
                tile_heights[x][y] = lowest;
            }
        }

        tile_heights
    }

    /// Get the heights of a tile at the given position.
    ///
    /// Returns the corner heights in the order: [north, east, south, west]
    /// which corresponds to [top-left, top-right, bottom-right, bottom-left]
    ///
    /// # Arguments
    ///
    /// * `heights` - 2D array of corner heights
    /// * `x` - X coordinate of the tile
    /// * `y` - Y coordinate of the tile
    ///
    /// # Returns
    ///
    /// Array of 4 heights: [north, east, south, west]
    pub fn get_heights(&self, heights: &[Vec<i32>], x: usize, y: usize) -> [i32; 4] {
        [
            heights[x][y],         // north (top-left)
            heights[x + 1][y],     // east (top-right)
            heights[x + 1][y + 1], // south (bottom-right)
            heights[x][y + 1],     // west (bottom-left)
        ]
    }

    /// Get the lowest height from an array of corner heights.
    ///
    /// # Arguments
    ///
    /// * `heights` - Array of 4 corner heights
    ///
    /// # Returns
    ///
    /// The minimum height value
    pub fn get_lowest(&self, heights: &[i32; 4]) -> i32 {
        *heights.iter().min().unwrap()
    }

    /// Convert bitmask to sprite index using OpenTTD mapping.
    ///
    /// The reverse mapping from the bitboard number to the sprite index.
    /// Note that some values would be invalid in a complete implementation.
    ///
    /// # Arguments
    ///
    /// * `bitmask` - The slope bitmask
    ///
    /// # Returns
    ///
    /// The sprite index for rendering, defaults to 0 (flat) for invalid inputs
    fn bitmask_to_sprite_index(&self, bitmask: i32) -> i32 {
        // Mapping from bitmask to sprite index (OpenTTD style)
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
            _ => 0,        // Default to flat for invalid configurations
        }
    }
}
