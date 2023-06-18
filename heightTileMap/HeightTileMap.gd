extends Node2D
class_name HeightTileMap

@export var force_seed: int = 0
@export var size: int = 16
@export var max_height: int = 16
@export var roughness: float = 30
@export var shift: Vector2 = Vector2(0, -8)
@export var base_tile_map: TileMap
@export var mapper_script: GDScript
@export var map_generator_script: GDScript

@onready var mapper = mapper_script.new()
@onready var map_generator = map_generator_script.new()

var heights: Array[Array]

var tile_maps = []
var main_map: TileMap

# Called when the node enters the scene tree for the first time.
func _ready():
	var rand = RandomNumberGenerator.new()
	if force_seed != 0:
		rand.set_seed(force_seed)
	
	# TODO: this seems a bit broken... if re-using a automatically created seed, it generates differently...
	print("Seed:")
	print(rand.seed)
	
	heights = map_generator.generate(size+1, roughness, max_height-1, rand)
		
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

func _process(_delta):
	# Get the mouse position, and map it to the map and back.
	var mouse_pos = get_local_mouse_position()
	var tile_pos = self.local_to_map(mouse_pos)
	var back_to_local = self.map_to_local(tile_pos)
	print("mouse: ", mouse_pos)
	print("tile:  ", tile_pos)
	print("reverse:  ", back_to_local)
	self.erase_cell(0, tile_pos)

func height(at: Vector2i) -> int:
	if at.x < 0 || at.x >= heights.size():
		return 0
	if at.y < 0 || at.y >= heights[at.x].size():
		return 0
	
	return heights[at.x][at.y]

#void _tile_data_runtime_update(layer: int, coords: Vector2i, tile_data: TileData) 
#virtual bool _use_tile_data_runtime_update(layer: int, coords: Vector2i) 

func _apply_to_all(fn: Callable):
	fn.call(main_map)
	for tile_map in tile_maps:
		fn.call(tile_map)

func add_layer(to_position: int):
	_apply_to_all(func(tile_map: TileMap): tile_map.add_layer(to_position))

func clear():
	_apply_to_all(func(tile_map: TileMap): tile_map.clear())

func clear_layer(layer: int):
	_apply_to_all(func(tile_map: TileMap): tile_map.clear_layer(layer))

func erase_cell(layer: int, coords: Vector2i):
	_apply_to_all(func(tile_map: TileMap): tile_map.erase_cell(layer, coords))
	
func fix_invalid_tiles():
	_apply_to_all(func(tile_map: TileMap): tile_map.fix_invalid_tiles())

func force_update(layer: int = -1):
	_apply_to_all(func(tile_map: TileMap): tile_map.force_update(layer))

func get_cell_alternative_tile(layer: int, coords: Vector2i, use_proxies: bool = false) -> int:
	return main_map.get_cell(layer, coords, use_proxies)

func get_cell_atlas_coords(layer: int, coords: Vector2i, use_proxies: bool = false) -> Vector2i:
	return main_map.get_cell_atlas_coords(layer, coords, use_proxies)

func get_cell_source_id(layer: int, coords: Vector2i, use_proxies: bool = false) -> int:
	return main_map.get_cell_source_id(layer, coords, use_proxies)

func get_cell_tile_data(layer: int, coords: Vector2i, use_proxies: bool = false) -> TileData:
	return main_map.get_cell_tile_data(layer, coords, use_proxies)

func get_coords_forbody_rid(body: RID) -> Vector2i:
	return main_map.get_coords_for(body)

func get_layer_modulate(layer: int) -> Color:
	return main_map.get_layer_modulate(layer)

func get_layer_name(layer: int) -> String:
	return main_map.get_layer_name(layer)

func get_layer_y_sort_origin(layer: int) -> int:
	return main_map.get_layer_y_sort_origin(layer)

func get_layer_z_index(layer: int) -> int:
	return main_map.get_layer_z_index(layer)

func get_layers_count() -> int:
	return main_map.get_layers_count()

func get_navigation_map(layer: int) -> RID:
	return main_map.get_navigation_map(layer)

func get_neighbor_cell(coords: Vector2i, neighbor: TileSet.CellNeighbor) -> Vector2i:
	return main_map.get_neighbor_cell(coords, neighbor)

func get_pattern(layer: int, coords_array: Array[Vector2i]) -> TileMapPattern:
	return main_map.get_pattern(layer, coords_array)

func get_surrounding_cells(coords: Vector2i) -> Array[Vector2i]:
	return main_map.get_surrounding_cells(coords)

func get_used_cells(layer: int) -> Array[Vector2i]:
	return main_map.get_used_cells(layer)

func get_used_cells_by_id(layer: int, source_id: int = -1, atlas_coords: Vector2i = Vector2i(-1, -1), alternative_tile: int = -1) -> Array[Vector2i]:
	return main_map.get_used_cells_by_id(layer, source_id, atlas_coords, alternative_tile)

func get_used_rect() -> Rect2:
	return main_map.get_used_rect()

func is_layer_enabled(layer: int) -> bool:
	return main_map.is_layer_enabled(layer)

func is_layer_y_sort_enabled(layer: int) -> bool:
	return main_map.is_layer_y_sort_enabled(layer)

func local_to_map(local_position: Vector2) -> Vector2i:
	# Go through each layer, and check if the tile at the given position is the respective height.
	# If so, return its position.
	for tile_height in range(tile_maps.size()):
		var tile_map: TileMap = tile_maps[tile_height]
		
		var at = tile_map.local_to_map(local_position - shift * tile_height)
		if height(at) == tile_height:
			return at
	return Vector2i(0, 0)

func map_pattern(position_in_tilemap: Vector2i, coords_in_pattern: Vector2i, pattern: TileMapPattern) -> Vector2i:
	return main_map.map_pattern(position_in_tilemap, coords_in_pattern, pattern)

func map_to_local(map_position: Vector2i) -> Vector2:
	var local_pos = tile_maps[height(map_position)].map_to_local(map_position)
	return local_pos + shift * height(map_position)
	
func move_layer(layer: int, to_position: int):
	_apply_to_all(func(tile_map: TileMap): tile_map.move_layer(layer, to_position))

func remove_layer(layer: int):
	_apply_to_all(func(tile_map: TileMap): tile_map.remove_layer(layer))

func set_cell(layer: int, coords: Vector2i, source_id: int = -1, atlas_coords: Vector2i = Vector2i(-1, -1), alternative_tile: int = 0):
	_apply_to_all(func(tile_map: TileMap): tile_map.set_cell(layer, coords, source_id, atlas_coords, alternative_tile))

func set_cells_terrain_connect(layer: int, cells: Array[Vector2i], terrain_set: int, terrain: int, ignore_empty_terrains: bool = true):
	_apply_to_all(func(tile_map: TileMap): tile_map.set_cells_terrain_connect(layer, cells, terrain_set, terrain, ignore_empty_terrains))

func set_cells_terrain_path(layer: int, path: Array[Vector2i], terrain_set: int, terrain: int, ignore_empty_terrains: bool = true):
	_apply_to_all(func(tile_map: TileMap): tile_map.set_cells_terrain_path(layer, path, terrain_set, terrain, ignore_empty_terrains))

func set_layer_enabled(layer: int, enabled: bool):
	_apply_to_all(func(tile_map: TileMap): tile_map.set_layer_enabled(layer, enabled))

func set_layer_modulate(layer: int, modulate: Color):
	_apply_to_all(func(tile_map: TileMap): tile_map.set_layer_modulate(layer, modulate))

func set_layer_name(layer: int, name: String):
	_apply_to_all(func(tile_map: TileMap): tile_map.set_layer_name(layer, name))

func set_layer_y_sort_enabled(layer: int, y_sort_enabled: bool):
	_apply_to_all(func(tile_map: TileMap): tile_map.set_layer_y_sort_enabled(layer, y_sort_enabled))

func set_layer_y_sort_origin(layer: int, y_sort_origin: int):
	_apply_to_all(func(tile_map: TileMap): tile_map.set_layer_y_sort_origin(layer, y_sort_origin))

func set_layer_z_index(layer: int, z_index: int):
	_apply_to_all(func(tile_map: TileMap): tile_map.set_layer_z_index(layer, z_index))

func set_navigation_map(layer: int, map: RID):
	_apply_to_all(func(tile_map: TileMap): tile_map.set_navigation_map(layer, map))

func set_pattern(layer: int, position: Vector2i, pattern: TileMapPattern):
	_apply_to_all(func(tile_map: TileMap): tile_map.set_pattern(layer, position, pattern))
	
