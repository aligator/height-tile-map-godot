extends Node2D
class_name HeightTileMap

@export var size: int = 10
@export var max_height: int = 16
@export var shift: Vector2i = Vector2i(0, -8)
@export var base_tile_map: TileMap
@export var mapper_script: GDScript

@onready var mapper = mapper_script.new()

var heights: Array[Array]

var tile_maps = []
var main_map: TileMap

# Called when the node enters the scene tree for the first time.
func _ready():
	var rand = RandomNumberGenerator.new()
	heights = DiamondSquare.generate(size+1, 10, max_height-1, rand)
		
	# print the map line by line
	for y in range(heights.size()):
		var line = ""
		for x in range(heights[y].size()):
			line += str(heights[y][x]) + " "
	
	# Create a new main map by using the passed mapper.
	# The mapper should just set the tiles on the main_map like they should look like.
	# It does not show them on the correct layer.
	main_map = base_tile_map.duplicate(DUPLICATE_USE_INSTANTIATION)
	main_map.clear()
	# tile_height is the height the tiles should be rendered at.
	var tile_heights = mapper.build(heights, main_map)
	
	# Now generate the layer-tile-maps and just map the tile from the main_map 
	# to the respective layer.
	for i in range(max_height):
		var tile_map: TileMap = base_tile_map.duplicate(DUPLICATE_USE_INSTANTIATION)
		tile_map.visible = true
		tile_map.clear()
		tile_maps.push_back(tile_map)
		tile_map.name = "TileMap" + str(i)
		var transform = tile_map.get_transform()
		tile_map.set_transform(tile_map.get_transform().translated(shift * i))
		add_child(tile_map)

	for x in range(tile_heights.size()):
		for y in range(tile_heights[x].size()):
			var at = Vector2i(x, y)
			var tile_height = tile_heights[x][y]

			# For each layer of the main map, set the tile on the respective layer.
			for layer in range(main_map.get_layers_count()):
				var source_id = main_map.get_cell_source_id(layer, at)
				var atlas_coords = main_map.get_cell_atlas_coords(layer, at)
				var alternative_tile = main_map.get_cell_alternative_tile(layer, at)
				tile_maps[tile_height].set_cell(layer, at, source_id, atlas_coords, alternative_tile)
	
