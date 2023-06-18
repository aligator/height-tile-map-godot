# This script just generates a flat map at height max_height
func generate(size: int, roughness: float, max_height: int, rand: RandomNumberGenerator) -> Array[Array]:
	var map: Array[Array] = []
	# Prefill the map with zeros
	for x in range(0, size):
		var row: Array = []
		for y in range(0, size):
			row.append(max_height)
		map.append(row)
	
	# Return the map
	return map
