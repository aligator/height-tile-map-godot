#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlopeType {
    Flat = 0b00000,
    W = 0b00001,
    S = 0b00010,
    E = 0b00100,
    N = 0b01000,
    NW = 0b01001,
    SW = 0b00011,
    SE = 0b00110,
    NE = 0b01100,
    EW = 0b00101,
    NS = 0b01010,
    NWS = 0b01011,
    WSE = 0b00111,
    SEN = 0b01110,
    ENW = 0b01101,
    SteepW = 0b10001,
    SteepS = 0b10010,
    SteepE = 0b10100,
    SteepN = 0b11000,
    Invalid = -1,
}

impl From<i32> for SlopeType {
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

pub struct OpenTTDMapper;

impl OpenTTDMapper {
    pub fn new() -> Self {
        Self
    }

    pub fn build(&self, corner_heights: &[Vec<i32>]) -> Vec<Vec<i32>> {
        let mut tile_heights = vec![vec![0; corner_heights[0].len() - 1]; corner_heights.len() - 1];

        for x in 0..corner_heights.len() - 1 {
            for y in 0..corner_heights[x].len() - 1 {
                let heights_at = self.get_heights(corner_heights, x, y);
                let lowest = self.get_lowest(&heights_at);

                // Calculate slope bitmask
                let mut bitmask = 0;

                // Check for steep slopes
                for i in 0..heights_at.len() {
                    if heights_at[i] == lowest && heights_at[(i + 2) % 4] == lowest + 2 {
                        bitmask |= 0b10000; // IS_STEEP_SLOPE flag
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

                // Get sprite index from bitmask
                let _sprite_index = self.bitmask_to_sprite_index(bitmask);

                // Store the tile height (lowest corner height)
                tile_heights[x][y] = lowest;
            }
        }

        tile_heights
    }

    pub fn get_heights(&self, heights: &[Vec<i32>], x: usize, y: usize) -> [i32; 4] {
        [
            heights[x][y],         // top-left
            heights[x + 1][y],     // top-right
            heights[x + 1][y + 1], // bottom-right
            heights[x][y + 1],     // bottom-left
        ]
    }

    pub fn get_lowest(&self, heights: &[i32; 4]) -> i32 {
        *heights.iter().min().unwrap()
    }

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
            _ => 0,        // Default to flat
        }
    }
}
