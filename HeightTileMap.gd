extends TileMap
class_name HeightTileMap

@export var size_min = Vector2(0, 0)
@export var size_max = Vector2(10, 10)

# TODO: generate tilemaps automatically later.
@onready var tile_maps = [
	$TileMap0, 
	$TileMap1,
	$TileMap2, 
	$TileMap3,
	$TileMap4,
	$TileMap5,
]
@onready var height_map: HeightTileMap = self

enum Flag {
	CORNER_W = 0b00001,
	CORNER_S = 0b00010,
	CORNER_E = 0b00100,
	CORNER_N = 0b01000,
	IS_STEEP_SLOPE = 0b10000,
}

# The bitboards to map to the specific tile inices is built like in
# OpenTTD: https://newgrf-specs.tt-wiki.net/wiki/NML:List_of_tile_slopes
# From the lsb to msb:
# CORNER_W			west corner is above the lowest corner.
# CORNER_S			south corner is above the lowest corner.
# CORNER_E			east corner is above the lowest corner.
# CORNER_N			north corner is above the lowest corner.
# IS_STEEP_SLOPE	this tile is a steep slope (the corner opposite to the lowest corner is 2 units higher). 
enum SlopeTypes {
	SLOPE_FLAT	  = 0b00000,
	SLOPE_W 	  = 0b00001,
	SLOPE_S 	  = 0b00010,
	SLOPE_E 	  = 0b00100,
	SLOPE_N		  = 0b01000,
	SLOPE_NW 	  = 0b01001,
	SLOPE_SW 	  = 0b00011,
	SLOPE_SE 	  = 0b00110,
	SLOPE_NE 	  = 0b01100,
	SLOPE_EW 	  = 0b00101,
	SLOPE_NS 	  = 0b01010,
	SLOPE_NWS 	  = 0b01011,
	SLOPE_WSE 	  = 0b00111,
	SLOPE_SEN 	  = 0b01110,
	SLOPE_ENW 	  = 0b01101,
	SLOPE_STEEP_W = 0b10001,
	SLOPE_STEEP_S = 0b10010,
	SLOPE_STEEP_E = 0b10100,
	SLOPE_STEEP_N = 0b11000,

	SLOPE_INVALID = -1,
}

# The bitboards are mapped to the sprite index.
var bitboards = [
	SlopeTypes.SLOPE_FLAT,
	SlopeTypes.SLOPE_W,
	SlopeTypes.SLOPE_S,
	SlopeTypes.SLOPE_SW,
	SlopeTypes.SLOPE_E,
	SlopeTypes.SLOPE_EW,
	SlopeTypes.SLOPE_SE,
	SlopeTypes.SLOPE_WSE,
	SlopeTypes.SLOPE_N,
	SlopeTypes.SLOPE_NW,
	SlopeTypes.SLOPE_NS,
	SlopeTypes.SLOPE_NWS,
	SlopeTypes.SLOPE_NE,
	SlopeTypes.SLOPE_ENW,
	SlopeTypes.SLOPE_SEN,
	SlopeTypes.SLOPE_STEEP_N,	
	SlopeTypes.SLOPE_STEEP_S,
	SlopeTypes.SLOPE_STEEP_W,
	SlopeTypes.SLOPE_STEEP_E,
]

# The reverse mapping from the bitboard number to the sprite index.
# Note that some values contain -1, which means they are invalid.
var bitboards_reverse = [
# spriteIndex, slopeType,           bitboard number
	0,  # SlopeTypes.SLOPE_FLAT,    # 0
	1,  # SlopeTypes.SLOPE_W,       # 1
	2,  # SlopeTypes.SLOPE_S,       # 2
	3,  # SlopeTypes.SLOPE_SW,      # 3
	4,  # SlopeTypes.SLOPE_E,       # 4
	5,  # SlopeTypes.SLOPE_EW,      # 5
	6,  # SlopeTypes.SLOPE_SE,      # 6
	7,  # SlopeTypes.SLOPE_WSE,     # 7
	8,  # SlopeTypes.SLOPE_N,       # 8
	9,  # SlopeTypes.SLOPE_NW,      # 9
	10, # SlopeTypes.SLOPE_NS,      # 10
	11, # SlopeTypes.SLOPE_NWS,     # 11
	12, # SlopeTypes.SLOPE_NE,      # 12
	13, # SlopeTypes.SLOPE_ENW,     # 13
	14, # SlopeTypes.SLOPE_SEN,     # 14
	-1, # SlopeTypes.SLOPE_INVALID, # 15
	-1, # SlopeTypes.SLOPE_INVALID, # 16
	-1, # SlopeTypes.SLOPE_INVALID, # 17
	-1, # SlopeTypes.SLOPE_INVALID, # 18
	-1, # SlopeTypes.SLOPE_INVALID, # 19
	-1, # SlopeTypes.SLOPE_INVALID, # 20
	-1, # SlopeTypes.SLOPE_INVALID, # 21
	-1, # SlopeTypes.SLOPE_INVALID, # 22
	16, # SlopeTypes.SLOPE_STEEP_S, # 23
	-1, # SlopeTypes.SLOPE_INVALID, # 24
	-1, # SlopeTypes.SLOPE_INVALID, # 25
	-1, # SlopeTypes.SLOPE_INVALID, # 26
	17, # SlopeTypes.SLOPE_STEEP_W, # 27
	-1, # SlopeTypes.SLOPE_INVALID, # 28
	15, # SlopeTypes.SLOPE_STEEP_N, # 29	
	18, # SlopeTypes.SLOPE_STEEP_E, # 30
]

enum Direction {
	NORTH,
	EAST,
	SOUTH,
	WEST,
}

# Get the hights of a tile at the given position.
# It is returned as an array of [top, right, bottom, left]
func _get_heights(x, y):
	return [
		height_map.get_cell_atlas_coords(0, Vector2i(x, y)).x,
		height_map.get_cell_atlas_coords(0, Vector2i(x+1, y)).x,
		height_map.get_cell_atlas_coords(0, Vector2i(x+1, y+1)).x,
		height_map.get_cell_atlas_coords(0, Vector2i(x, y+1)).x,
	]

func _get_lowest(heights: Array) -> int:
	var lowest = heights[0]
	for i in range(heights.size()):
		var height = heights[i]
		if height < lowest:
			lowest = height
	
	return lowest

func map_heights_to_tile_maps():
	# Read the HeightMap and generate the tilemaps.
	for x in range(size_min.x, size_max.x):
		for y in range(size_min.y, size_max.y):
			var at = Vector2(x, y)

			# Get all relevant heights.
			var heights = _get_heights(x, y)
			var lowest = _get_lowest(heights)

			# Get the bitboard number.
			var bitmask = 0
			
			# Find steep tiles.
			for i in range(heights.size()):
				if heights[i] == lowest && heights[(i+2)%4] == lowest+2:
					bitmask |= Flag.IS_STEEP_SLOPE
					break
			
			if heights[0] == heights[1] && heights[0] == heights[2] && heights[0] == heights[3]:
				bitmask = 0 
			if heights[0] > lowest:
				bitmask |= Flag.CORNER_N
			if heights[1] > lowest:
				bitmask |= Flag.CORNER_E
			if heights[2] > lowest:
				bitmask |= Flag.CORNER_S
			if heights[3] > lowest:
				bitmask |= Flag.CORNER_W

			# Get the sprite index.
			var sprite_index = bitboards_reverse[bitmask]

			# Use the lowest height as the base layer.
			for i in range(tile_maps.size()):
				var tile_map: TileMap = tile_maps[i]
				if i == lowest:
					tile_map.set_cell(0, at, 0, Vector2i(sprite_index, 0))
				else:
					tile_map.erase_cell(0, at)

# Called when the node enters the scene tree for the first time.
func _ready():
	var rand = RandomNumberGenerator.new()
	var map = DiamondSquare.generate(11, 1.0, 5, rand)
	
	# print the map line by line
	for y in range(map.size()):
		var line = ""
		for x in range(map[y].size()):
			line += str(map[y][x]) + " "
		print(line)
	
	for tile_map in tile_maps:
		tile_map.clear()
		
	height_map.clear()
	for x in range(map.size()):
		for y in range(map[x].size()):
			height_map.set_cell(0, Vector2i(x, y), 0, Vector2i(map[x][y], 0))
	
	
	height_map.map_heights_to_tile_maps()

func _on_changed():
	if height_map == null: 
		return
		
	print("changed")
	height_map.map_heights_to_tile_maps()
