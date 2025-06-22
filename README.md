# OpenTTD-Style Height Tile Map in Bevy 0.16

This project implements an OpenTTD-style height tile map system in Bevy 0.16, demonstrating how to generate terrain using the diamond-square algorithm and classify tiles using the OpenTTD slope system.

## Features

- **Diamond-Square Algorithm**: Generates realistic terrain height maps
- **OpenTTD Slope System**: Classifies tiles based on corner heights using bitmasks
- **Height Layering**: Supports multiple height layers for 3D-like terrain representation
- **Bevy 0.16 Compatible**: Uses modern Bevy APIs without deprecated bundles

## How It Works

### 1. Diamond-Square Algorithm

The diamond-square algorithm generates height data by:
- Starting with random corner values
- Performing diamond steps (averaging corners + random noise)
- Performing square steps (averaging edges + random noise)
- Repeating with decreasing roughness

### 2. OpenTTD Slope System

Each tile is classified based on the heights of its four corners:
- **Flat tiles**: All corners at the same height
- **Sloped tiles**: One or more corners higher than the lowest
- **Steep slopes**: Opposite corners differ by 2 units

The system uses bitmasks to encode slope information:
- Bit 0: West corner higher
- Bit 1: South corner higher  
- Bit 2: East corner higher
- Bit 3: North corner higher
- Bit 4: Steep slope flag

### 3. Height Layering

Tiles are rendered at different Z-positions based on their height, creating a layered 3D effect similar to the original Godot implementation.

## Project Structure

```
src/
├── main.rs              # Main application and setup
├── diamond_square.rs    # Diamond-square terrain generation
└── open_ttd_mapper.rs   # OpenTTD slope classification
```

## Usage

```bash
cargo run
```

The application will:
1. Generate a 17x17 height map using diamond-square algorithm
2. Classify each tile using the OpenTTD slope system
3. Print the height map and slope distribution to console
4. Display the terrain in a Bevy window

## Key Differences from Godot Version

- **No Bundles**: Uses Bevy 0.16's new component system
- **Simplified Rendering**: Focuses on core algorithm demonstration
- **Rust Implementation**: Leverages Rust's type safety and performance

## Algorithm Details

### Diamond Step
```
7  0  0  0  1 
0  0  0  0  0 
0  0 15 0  0 
0  0  0  0  0 
14 0  0  0  13
```
Sets center based on average of four corners + random noise.

### Square Step
```
7  0  X  0  1 
0  0  0  0  0 
X  0  15 0  X 
0  0  0  0  0 
14 0  X  0  13
```
Sets edge centers based on adjacent points + random noise.

## Slope Types

The system supports 19 different slope types:
- Flat (00000)
- Single slopes: W, S, E, N
- Double slopes: SW, SE, EW, NS, NW, NE
- Triple slopes: NWS, WSE, SEN, ENW
- Steep slopes: SteepW, SteepS, SteepE, SteepN

## Future Enhancements

- Add texture support for different terrain types
- Implement tile sprites for visual representation
- Add camera controls for terrain exploration
- Support for larger map sizes
- Real-time terrain modification

## References

- [OpenTTD Slope System](https://newgrf-specs.tt-wiki.net/wiki/NML:List_of_tile_slopes)
- [Diamond-Square Algorithm](https://en.wikipedia.org/wiki/Diamond-square_algorithm)
- [Bevy 0.16 Migration Guide](https://bevy.org/learn/migration-guides/0-15-to-0-16/)
