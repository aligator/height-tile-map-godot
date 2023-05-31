extends Node

class_name DiamondSquare

# generate a height map as a two dimensional array out of integers.
static func generate(size: int, roughness: float, max_height: float, rand: RandomNumberGenerator) -> Array[Array]:
	var map: Array[Array] = []
	# Prefill the map with zeros
	for x in range(0, size):
		var row: Array = []
		for y in range(0, size):
			row.append(0)
		map.append(row)
	
	# Set the corners to random values
	map[0][0] = rand.randf_range(0, max_height)
	map[size-1][0] = rand.randf_range(0, max_height)
	map[0][size-1] = rand.randf_range(0, max_height)
	map[size-1][size-1] = rand.randf_range(0, max_height)

	# Error if not Pow of 2
	
	assert(size % 2 != 0, "size must not be a power of 2")
	
	var side_length: int = size - 1
	while side_length >= 2:
		var half_length = roundi(side_length / 2.0);
		# Caclulate the diamond values
		for x in range(0, size - 1, side_length):
			for y in range(0, size - 1, side_length):

				# avg over the 4 corners of the square
				var top_left = map[x][y]
				var top_right = map[x + side_length][y]
				var bottom_left = map[x][y + side_length]
				var bottom_right = map[x + side_length][y + side_length]

				var avg = (top_left + top_right + bottom_left + bottom_right) / 4.0
				avg += rand.randf_range(-roughness, roughness)
				
				# Set the center to the average of the corners + random value
				map[x + half_length][y + half_length] = avg

		# Calculate the square values
		for x in range(0, size, half_length):
			for y in range((x + half_length) % side_length, size, side_length):
				# avg over the 4 corners of the diamond
				var left = map[(x - half_length + size) % size][y]
				var right = map[(x + half_length) % size][y]
				var top = map[x][(y - half_length + size) % size]
				var bottom = map[x][(y + half_length) % size]

				var avg = (left + right + top + bottom) / 4.0
				avg += rand.randf_range(-roughness, roughness)
				
				# Set the center to the average of the corners + random value
				map[x][y] = avg

				# Special case for the left edge
				if x == 0:
					map[size - 1][y] = avg
				# Special case for the top edge
				if y == 0:
					map[x][size - 1] = avg

		#roughness /= 2.0
		side_length /= 2
		
	# Normalize the map between 0 and max_height
	var lowest = 0
	var highest = 0
	for x in range(0, size):
		for y in range(0, size):
			if map[x][y] < lowest:
				lowest = map[x][y]
			if map[x][y] > highest:
				highest = map[x][y]
	for x in range(0, size):
		for y in range(0, size):
			map[x][y] = roundi((map[x][y] - lowest) / (highest - lowest) * max_height)

	# Return the map
	return map
