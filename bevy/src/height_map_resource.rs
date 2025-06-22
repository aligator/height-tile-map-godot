use bevy::prelude::*;

use crate::{
    diamond_square::{DiamondSquareGenerator, HeightMap},
    open_ttd_mapper::{OpenTTDMapper, TileHeights},
    open_ttd_renderer::OpenTTDRenderer,
};

/// Configuration for height map generation
#[derive(Debug, Clone, Copy, Component, Resource)]
pub struct HeightMapConfig {
    pub size: usize,
    pub roughness: f32,
    pub max_height: i32,
    pub seed: u64,
}

impl Default for HeightMapConfig {
    fn default() -> Self {
        Self {
            size: 129, // 2^7 + 1
            roughness: 10.0,
            max_height: 16,
            seed: 42,
        }
    }
}

/// Trait for generating height maps from terrain data
pub trait HeightGenerator: Send + Sync + 'static {
    /// Generate a height map with the given configuration
    fn generate(&self, config: &HeightMapConfig) -> HeightMap;
}

/// Trait for mapping height data to tile information  
pub trait TileMapper: Send + Sync + 'static {
    /// Build tile height information from corner heights
    fn build(&self, heights: &HeightMap) -> TileHeights;

    /// Get heights at a specific tile position
    #[allow(dead_code)]
    fn get_heights(&self, heights: &HeightMap, x: usize, y: usize) -> [i32; 4];

    /// Get the lowest height from an array of heights
    #[allow(dead_code)]
    fn get_lowest(&self, heights: &[i32; 4]) -> i32;
}

/// Trait for rendering terrain tiles visually
pub trait TerrainRenderer: Send + Sync + 'static {
    /// The type of mapper this renderer works with
    type Mapper: TileMapper;

    /// Spawn terrain entities for the given height map
    fn spawn_terrain(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
        heights: &HeightMap,
        tile_heights: &TileHeights,
        config: &HeightMapConfig,
        mapper: &Self::Mapper,
    );

    /// Get the sprite index for a tile at given coordinates
    fn get_sprite_index(
        &self,
        x: usize,
        y: usize,
        heights: &HeightMap,
        mapper: &Self::Mapper,
    ) -> usize;
}

/// Bevy Resource containing generated terrain data with generic types
#[derive(Resource)]
pub struct HeightMapResource<G, M>
where
    G: HeightGenerator,
    M: TileMapper,
{
    pub config: HeightMapConfig,
    pub heights: HeightMap,
    pub tile_heights: TileHeights,
    generator: G,
    mapper: M,
}

impl<G, M> HeightMapResource<G, M>
where
    G: HeightGenerator,
    M: TileMapper,
{
    /// Create a new HeightMapResource with specified generators
    pub fn new(generator: G, mapper: M, config: HeightMapConfig) -> Self {
        let heights = generator.generate(&config);
        let tile_heights = mapper.build(&heights);

        Self {
            config,
            heights,
            tile_heights,
            generator,
            mapper,
        }
    }

    /// Internal method to regenerate terrain with new configuration
    fn regenerate_internal(&mut self, config: HeightMapConfig) {
        self.config = config;
        self.heights = self.generator.generate(&config);
        self.tile_heights = self.mapper.build(&self.heights);
    }

    /// Regenerate terrain with new configuration
    #[allow(dead_code)]
    pub fn regenerate(&mut self, config: HeightMapConfig) {
        self.regenerate_internal(config);
    }

    /// Regenerate with new seed only
    #[allow(dead_code)]
    pub fn regenerate_with_seed(&mut self, seed: u64) {
        let mut config = self.config;
        config.seed = seed;
        self.regenerate_internal(config);
    }

    /// Get reference to the mapper
    pub fn mapper(&self) -> &M {
        &self.mapper
    }
}

/// Plugin to add height map generation functionality
///
/// This plugin uses the config-as-resource pattern. If no HeightMapConfig resource
/// exists, it will insert a default one automatically.
///
/// # Usage:
///
/// ```rust
/// // Simple usage with default config
/// app.add_plugins(DefaultHeightMapPlugin::default());
///
/// // Custom config - insert resource first, then add plugin
/// app.insert_resource(HeightMapConfig {
///     size: 257,
///     roughness: 20.0,
///     max_height: 32,
///     seed: 12345,
/// })
/// .add_plugins(HeightMapPlugin::<DiamondSquareGenerator, OpenTTDMapper, OpenTTDRenderer>::default());
///
/// // Runtime regeneration - just change the resource!
/// fn regenerate_map(mut config: ResMut<HeightMapConfig>) {
///     config.seed = rand::thread_rng().gen();
/// }
/// ```
pub struct HeightMapPlugin<G, M, R>
where
    G: HeightGenerator + Default,
    M: TileMapper + Default,
    R: TerrainRenderer<Mapper = M> + Default,
{
    _phantom_g: std::marker::PhantomData<G>,
    _phantom_m: std::marker::PhantomData<M>,
    _phantom_r: std::marker::PhantomData<R>,
}

impl<G, M, R> Default for HeightMapPlugin<G, M, R>
where
    G: HeightGenerator + Default,
    M: TileMapper + Default,
    R: TerrainRenderer<Mapper = M> + Default,
{
    fn default() -> Self {
        Self {
            _phantom_g: std::marker::PhantomData,
            _phantom_m: std::marker::PhantomData,
            _phantom_r: std::marker::PhantomData,
        }
    }
}

impl<G, M, R> Plugin for HeightMapPlugin<G, M, R>
where
    G: HeightGenerator + Default,
    M: TileMapper + Default,
    R: TerrainRenderer<Mapper = M> + Default,
{
    fn build(&self, app: &mut App) {
        // Insert default config if none exists
        app.init_resource::<HeightMapConfig>()
            .add_systems(PreStartup, init_height_map_system::<G, M>)
            .add_systems(Update, auto_regenerate_height_map_system::<G, M>)
            .add_systems(Update, update_terrain_visuals_system::<G, M, R>);
    }
}

/// Type alias for the default plugin (without where clause - not supported yet)
pub type DefaultHeightMapPlugin =
    HeightMapPlugin<DiamondSquareGenerator, OpenTTDMapper, OpenTTDRenderer>;

/// System to initialize the height map resource with generic types
fn init_height_map_system<G, M>(mut commands: Commands, config: Res<HeightMapConfig>)
where
    G: HeightGenerator + Default,
    M: TileMapper + Default,
{
    let height_map_resource = HeightMapResource::new(G::default(), M::default(), *config);

    println!("Initialized HeightMapResource with config: {:?}", *config);
    println!("{}", height_map_resource.heights);
    println!("{}", height_map_resource.tile_heights);

    commands.insert_resource(height_map_resource);
}

/// System that automatically regenerates the height map when config changes
fn auto_regenerate_height_map_system<G, M>(
    config: Res<HeightMapConfig>,
    mut height_map_res: ResMut<HeightMapResource<G, M>>,
) where
    G: HeightGenerator + Default,
    M: TileMapper + Default,
{
    // Bevy's change detection - only runs when config actually changes!
    if config.is_changed() {
        println!("Config changed! Auto-regenerating height map...");
        println!("New config: {:?}", *config);

        height_map_res.regenerate_internal(*config);

        println!("Height map regenerated successfully!");
        println!("{}", height_map_res.heights);
    }
}

/// Marker component for terrain tiles that need to be updated when height map changes
#[derive(Component)]
pub struct TerrainTile;

/// System that updates the visual terrain when the height map resource changes
fn update_terrain_visuals_system<G, M, R>(
    height_map_res: Res<HeightMapResource<G, M>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    // Query all existing terrain tiles to despawn them
    terrain_query: Query<Entity, With<TerrainTile>>,
) where
    G: HeightGenerator + Default,
    M: TileMapper + Default,
    R: TerrainRenderer<Mapper = M> + Default,
{
    // Only run when the height map actually changed
    if height_map_res.is_changed() {
        println!("🎨 Updating terrain visuals...");

        // Despawn all existing terrain tiles
        for entity in terrain_query.iter() {
            commands.entity(entity).despawn();
        }

        // Create renderer and spawn terrain
        let renderer = R::default();
        renderer.spawn_terrain(
            &mut commands,
            &asset_server,
            &mut texture_atlas_layouts,
            &height_map_res.heights,
            &height_map_res.tile_heights,
            &height_map_res.config,
            height_map_res.mapper(),
        );

        println!("✨ Terrain visuals updated!");
    }
}
