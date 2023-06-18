# This script implements the diamond square algorithm.
# It is built using the following ressources:
# https://medium.com/@nickobrien/diamond-square-algorithm-explanation-and-c-implementation-5efa891e486f
# https://peterellisjones.com/posts/generating-transport-tycoon-terrain/
# https://craftofcoding.wordpress.com/tag/diamond-square-algorithm/
#
# Note that some ressources swap the naming between the diamond_step and the square_step.
# I name it like in wikipedia https://upload.wikimedia.org/wikipedia/commons/b/bf/Diamond_Square.svg

func print_map(map):
	# Print the map line by line
	for x in range(0, map.size()):
		var line = "  | "
		if x == map.size()/2:
			line = "Y | "
			
		for y in range(0, map[x].size()):
			line += str("%0*d " % [2, map[y][x]])
		print(line)
	print("  ____________________________")
	print("               X\n")

# Sets the center based on the height of four edge points
#
# 7  0  0  0  1 
# 0  0  0  0  0 
# 0  0  15 0  0 
# 0  0  0  0  0 
# 14 0  0  0  13
func diamond_step(map: Array[Array], x: int, y: int, reach: int, size: int, roughness: float, max_height: int, rand: RandomNumberGenerator):
	# avg over the 4 corners of the square
	var top_left = map[x][y]
	var top_right = map[x + size][y]
	var bottom_left = map[x][y + size]
	var bottom_right = map[x + size][y + size]

	var avg = (top_left + top_right + bottom_left + bottom_right) / 4.0
	avg += rand.randf_range(-roughness, roughness)

	var local_max_height = min(
		max_height, 
		top_left + reach, 
		top_right + reach, 
		bottom_left + reach, 
		bottom_right + reach,
	)
	
	var local_min_height = max(
		0, 
		top_left - reach, 
		top_right - reach, 
		bottom_left - reach, 
		bottom_right - reach,
	)

	var center_x = x + reach
	var center_y = y + reach
	if center_x - reach*2 >= 0:
		var pos = map[center_x-reach*2][center_y]
		if pos != -1:
			local_min_height = max(local_min_height, pos - reach*2)
			local_max_height = min(local_max_height, pos + reach*2)
	if center_x + reach*2 < map.size():
		var pos = map[center_x+reach*2][center_y]
		if pos != -1:
			local_min_height = max(local_min_height, pos - reach*2)
			local_max_height = min(local_max_height, pos + reach*2)
	if center_y - reach*2 >= 0:
		var pos = map[center_x][center_y-reach*2]
		if pos != -1:
			local_min_height = max(local_min_height, pos - reach*2)
			local_max_height = min(local_max_height, pos + reach*2)
	if center_y + reach*2 < map.size():
		var pos = map[center_x][center_y+reach*2]
		if pos != -1:
			local_min_height = max(local_min_height, pos - reach*2)
			local_max_height = min(local_max_height, pos + reach*2)
	
	# Set the center to the average of the corners + random value
	avg = clampi(roundi(avg), max(0, min(local_min_height, local_max_height)), max(0, local_max_height))

	map[x + reach][y + reach] = avg

# Sets the center of the sides of the square.
# Each call only sets one side based on the reach. But together al 
# 'X' in the drawing below get set.
#
# 7  0  X  0  1 
# 0  0  0  0  0 
# X  0  15 0  X 
# 0  0  0  0  0 
# 14 0  X  0  13
func square_step(map: Array[Array], x: int, y: int, reach: int, size: int, roughness: float, max_height: int, rand: RandomNumberGenerator):
	var count = 0
	var avg = 0.0
	var local_max_height = max_height
	var local_min_height = 0
	
	if x - reach >= 0:
		avg += map[x-reach][y]
		count+=1
		local_min_height = max(local_min_height, map[x-reach][y] - reach)
		local_max_height = min(local_max_height, map[x-reach][y] + reach)
	if x + reach < size:
		avg += map[x+reach][y]
		count+=1
		local_min_height = max(local_min_height, map[x+reach][y] - reach)
		local_max_height = min(local_max_height, map[x+reach][y] + reach)
	if y - reach >= 0:
		avg += map[x][y-reach]
		count+=1
		local_min_height = max(local_min_height, map[x][y-reach] - reach)
		local_max_height = min(local_max_height, map[x][y-reach] + reach)
	if y + reach < size:
		avg += map[x][y+reach]
		count+=1
		local_min_height = max(local_min_height, map[x][y+reach] - reach)
		local_max_height = min(local_max_height, map[x][y+reach] + reach)
		
#	local_min_height = max(local_min_height, map[x][y]-reach)
#	local_max_height = min(local_max_height, map[x][y]+reach)
		
	avg /= count
	avg += rand.randf_range(-roughness, roughness)
	
	map[x][y] = clampi(roundi(avg), max(0, min(local_min_height, local_max_height)), max(0, local_max_height))

# generate a height map as a two dimensional array out of integers.
# Size must be 2^n + 1 -> 3, 5, 9, 17, ... 257
func generate(size: int, roughness: float, max_height: int, rand: RandomNumberGenerator) -> Array[Array]:
	var map: Array[Array] = []
	# Prefill the map with zeros
	for x in range(0, size):
		var row: Array = []
		for y in range(0, size):
			row.append(-1)
		map.append(row)
	
	# Set the corners to random values
#	map[0][0] = rand.randi_range(0, max_height)
#	map[size-1][0] = rand.randi_range(0, max_height)
#	map[0][size-1] = rand.randi_range(0, max_height)
#	map[size-1][size-1] = rand.randi_range(0, max_height)
	map[0][0] = 2
	map[size-1][0] = 1
	map[0][size-1] = 6
	map[size-1][size-1] = 4
	
	var side_length: int = size - 1
	while side_length >= 2:
		var half_length = floori(side_length / 2.0);
		for x in range(0, size - 1, side_length):
			for y in range(0, size - 1, side_length):
				diamond_step(map, x, y, half_length, side_length, roughness, max_height, rand)
				
		if size <= 16:
			print("diamond")
			print_map(map)
		
		var col = 0
		for x in range(0, size+1, half_length):
			col += 1
			# If this is an odd column
			if col % 2 == 1:
				for y in range(half_length, size, side_length):
					square_step(map, x % size, y % size, half_length, size, roughness, max_height, rand)
			else:
				for y in range(0, size, side_length):
					square_step(map, x % size, y % size, half_length, size, roughness, max_height, rand)
		
		if size <= 16:
			print("square")
			print_map(map)
		
		roughness /= 2.0
		side_length /= 2

	if size <= 16:
		print_map(map)
	
	# Return the map
	return map

