extends GutTest

var DiamondSquare = load('res://scripts/diamondSquare.gd')

var rand = RandomNumberGenerator.new()

func before_each():
	gut.p("Reset Seed")
	rand.set_seed(42)

# With this setup, you can use `params.p1`, `params.p2`, and
# `params.result` in the test below.
var diamond_params = ParameterFactory.named_parameters(
	# map: Array[Array], x: int, y: int, reach: int, size: int, roughness: float, max_height: int
	['name', 'map', 'x', 'y', 'reach', 'size', 'roughness', 'max_height', "result"], # names
	[# values
	["normal calculation",
		[[   10, 0, 0, 0, 10],
			[0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0],
			[10, 0, 0, 0, 10]] as Array[Array], # map
		0, 0, # x, y
		2,    # reach
		5,    # size
		0,    # roughness
		20,    # max_height
		[[   10, 0, 0, 0, 10],
			[0, 0, 0, 0, 0],
			[0, 0, 10, 0, 0],
			[0, 0, 0, 0, 0],
			[10, 0, 0, 0, 10]] as Array[Array], # result
	],
	["normal calculation with roughness",
		[[   10, 0, 0, 0, 10],
			[0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0],
			[10, 0, 0, 0, 10]] as Array[Array], # map
		0, 0, # x, y
		2,    # reach
		5,    # size
		1,    # roughness
		20,    # max_height
		[[   10, 0, 0, 0, 10],
			[0, 0, 0, 0, 0],
			[0, 0, 9, 0, 0],
			[0, 0, 0, 0, 0],
			[10, 0, 0, 0, 10]] as Array[Array], # result
	],
	["max_height kicks in",
		[[   10, 0, 0, 0, 10],
			[0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0],
			[10, 0, 0, 0, 10]] as Array[Array], # map
		0, 0, # x, y
		2,    # reach
		5,    # size
		0,    # roughness
		5,    # max_height
		[[   10, 0, 0, 0, 10],
			[0, 0, 0, 0, 0],
			[0, 0, 5, 0, 0], # In this test the center is 10
			[0, 0, 0, 0, 0],
			[10, 0, 0, 0, 10]] as Array[Array], # result
	],
	["adjacent points in reach are out of map-size and therefore not considered",
		[[   10, 0, 0, 0, 10],
			[0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0],
			[10, 0, 0, 0, 10]] as Array[Array], # map
		0, 0, # x, y
		2,    # reach
		5,    # size
		0,    # roughness
		20,    # max_height
		[[   10, 0, 0, 0, 10],
			[0, 0, 0, 0, 0],
			[0, 0, 10, 0, 0], # In this test the center is 10
			[0, 0, 0, 0, 0],
			[10, 0, 0, 0, 10]] as Array[Array], # result
	],
	["adjacent points in reach forces new min",
	# Despite this test using the same values as the previouse one, 
	# the center is now 11 as a lower one is not allowed due to the adjacent values (15)
		[[   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 10, 0, 0, 0, 10, 0, 0, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 15, 0, 0, 0, 0, 0, 0, 0, 15, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 10, 0, 0, 0, 10, 0, 0, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]] as Array[Array], # map
		3, 3, # x, y
		2,    # reach
		5,    # size
		0,    # roughness
		20,    # max_height
		[[   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 10, 0, 0, 0, 10, 0, 0, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 15, 0, 0, 0, 11, 0, 0, 0, 15, 0], 
			[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 10, 0, 0, 0, 10, 0, 0, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]] as Array[Array], # result
	],
	])
func test_diamond_step(params = use_parameters(diamond_params)):
	gut.p(params.name)
	
	var ds = DiamondSquare.new()
	
	# deep copy the map as it will be modified
	var got: Array[Array] = []
	for x in range(params.map.size()):
		got.append([])
		for y in range(params.map[x].size()):
			got[x].append(params.map[x][y])
	
	ds.diamond_step(got, params.x, params.y, params.reach, params.size-1, params.roughness, params.max_height, rand)
	
	for x in range(got.size()):
		for y in range(got[x].size()):
			assert_eq(got[x][y], params.result[x][y], "x: " + str(x) + ", y: " + str(y))
