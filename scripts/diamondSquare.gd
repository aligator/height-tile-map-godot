# This script implements the diamond square algorithm.
# It is built using the following ressources:
# https://medium.com/@nickobrien/diamond-square-algorithm-explanation-and-c-implementation-5efa891e486f
# https://peterellisjones.com/posts/generating-transport-tycoon-terrain/
# https://craftofcoding.wordpress.com/tag/diamond-square-algorithm/

func print_map(map):
	# Print the map line by line
	for x in range(0, map.size()):
		var line = ""
		for y in range(0, map[x].size()):
			line += str(map[x][y]) + " "
		print(line)
	print("_______________")

func diamondStep(map: Array[Array], x: int, y: int, reach: int, size: int, roughness: float, max_height: int, rand: RandomNumberGenerator):
	var count = 0
	var avg = 0.0
	if x - reach >= 0:
		avg += map[x-reach][y-reach]
		count+=1
	if x + reach < size:
		avg += map[x+reach][y]
		count+=1
	if y - reach >= 0:
		avg += map[x][y-reach]
		count+=1
	if y + reach < size:
		avg += map[x][y+reach]
		count+=1
		
	avg += rand.randf_range(-roughness, roughness)
	avg /= count
	map[x][y] = clampi(roundi(avg), 0, max_height)


# generate a height map as a two dimensional array out of integers.
# Size must be 2^n + 1 -> 3, 5, 9, 17, ... 257
func generate(size: int, roughness: float, max_height: int, rand: RandomNumberGenerator) -> Array[Array]:
	var map: Array[Array] = []
	# Prefill the map with zeros
	for x in range(0, size):
		var row: Array = []
		for y in range(0, size):
			row.append(0)
		map.append(row)
	
	# Set the corners to random values
	map[0][0] = rand.randi_range(0, max_height)
	map[size-1][0] = rand.randi_range(0, max_height)
	map[0][size-1] = rand.randi_range(0, max_height)
	map[size-1][size-1] = rand.randi_range(0, max_height)
	
	var side_length: int = size - 1
	while side_length >= 2:
		var half_length = floori(side_length / 2.0);
		# Caclulate the ssquare values
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
				avg = clampi(roundi(avg), 0, max_height)

				map[x + half_length][y + half_length] = avg
		
		print_map(map)
		
		# Calculate the diamondSteps 
		var col = 0
		for x in range(0, size+half_length, half_length):
			col += 1
			# If this is an odd column
			if col % 2 == 1:
				for y in range(half_length, size, side_length):
					diamondStep(map, x % size, y % size, half_length, size, roughness, max_height, rand)
			else:
				for y in range(0, size, side_length):
					diamondStep(map, x % size, y % size, half_length, size, roughness, max_height, rand)
				
		print_map(map)
		
		roughness /= 2.0
		side_length /= 2

	print_map(map)

	# Return the map
	return map

