use bevy::prelude::*;

use crate::{
    diamond_square::{DiamondSquareGenerator, HeightMap},
    open_ttd_mapper::{OpenTTDMapper, TileHeights},
    open_ttd_renderer::{OpenTTDRenderer, OpenTTDRendererConfig},
};

/// Configuration for height map generation - now a Component instead of Resource
#[derive(Debug, Clone, Copy, Component)]
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

/// Component containing the generated height map data
#[derive(Component, Clone)]
pub struct HeightMapData {
    pub heights: HeightMap,
    pub tile_heights: TileHeights,
}

/// Marker component to identify height map entities
#[derive(Component)]
pub struct HeightMapEntity;

/// Component to mark that a height map needs regeneration
#[derive(Component)]
pub struct RegenerateHeightMap;

/// Component to specify which generator to use for this map
#[derive(Component)]
pub struct GeneratorComponent<G>(pub G);

/// Component to specify which mapper to use for this map
#[derive(Component)]
pub struct MapperComponent<M>(pub M);

/// Component to specify which renderer to use for this map
#[derive(Component)]
pub struct RendererComponent<R>(pub R);

/// Marker component for terrain tiles that need to be updated when height map changes
#[derive(Component)]
pub struct TerrainTile {
    pub map_entity: Entity, // Reference to the parent map entity
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
        map_entity: Entity,        // Add map entity for parenting
        map_transform: &Transform, // Add map transform for positioning
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

/// Bundle for creating a height map entity
#[derive(Bundle)]
pub struct HeightMapBundle<G, M, R>
where
    G: HeightGenerator,
    M: TileMapper,
    R: TerrainRenderer<Mapper = M>,
{
    pub config: HeightMapConfig,
    pub marker: HeightMapEntity,
    pub generator: GeneratorComponent<G>,
    pub mapper: MapperComponent<M>,
    pub renderer: RendererComponent<R>,
    pub transform: Transform,
}

impl<G, M, R> HeightMapBundle<G, M, R>
where
    G: HeightGenerator,
    M: TileMapper,
    R: TerrainRenderer<Mapper = M>,
{
    pub fn new(config: HeightMapConfig, generator: G, mapper: M, renderer: R) -> Self {
        Self {
            config,
            marker: HeightMapEntity,
            generator: GeneratorComponent(generator),
            mapper: MapperComponent(mapper),
            renderer: RendererComponent(renderer),
            transform: Transform::default(),
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}

/// Type alias for the default height map bundle
pub type DefaultHeightMapBundle =
    HeightMapBundle<DiamondSquareGenerator, OpenTTDMapper, OpenTTDRenderer>;

/// Plugin to add height map generation functionality
///
/// This plugin uses entity-based height maps. You can spawn multiple maps by
/// creating entities with HeightMapBundle.
///
/// # Usage:
///
/// ```rust
/// // Add the plugin
/// app.add_plugins(HeightMapPlugin::default());
///
/// // Spawn maps in systems
/// fn spawn_maps(
///     mut commands: Commands,
///     asset_server: Res<AssetServer>,
///     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
/// ) {
///     // Create renderer configs
///     let grass_config = OpenTTDRendererConfig::default_grass(&asset_server, &mut texture_atlas_layouts);
///     let stone_config = OpenTTDRendererConfig::from_assets(
///         &asset_server,
///         &mut texture_atlas_layouts,
///         "stone_sheet.png",
///         UVec2::new(64, 63),
///         (19, 1),
///     );
///     
///     // Main map
///     commands.spawn(DefaultHeightMapBundle::new(
///         HeightMapConfig {
///             size: 257,
///             roughness: 20.0,
///             max_height: 32,
///             seed: 12345,
///         },
///         DiamondSquareGenerator::default(),
///         OpenTTDMapper::default(),
///         OpenTTDRenderer::new(grass_config),
///     ));
///     
///     // Mini-map with different texture
///     commands.spawn(DefaultHeightMapBundle::new(
///         HeightMapConfig {
///             size: 65,
///             roughness: 5.0,
///             max_height: 8,
///             seed: 54321,
///         },
///         DiamondSquareGenerator::default(),
///         OpenTTDMapper::default(),
///         OpenTTDRenderer::new(stone_config),
///     ).with_transform(Transform::from_xyz(1000.0, 0.0, 0.0)));
/// }
/// ```
pub struct HeightMapPlugin;

impl Default for HeightMapPlugin {
    fn default() -> Self {
        Self
    }
}

impl Plugin for HeightMapPlugin {
    fn build(&self, app: &mut App) {
        // Single unified system that handles everything
        app.add_systems(
            Update,
            render_height_map_system::<DiamondSquareGenerator, OpenTTDMapper, OpenTTDRenderer>,
        );
    }
}

/// Type alias for the default plugin
pub type DefaultHeightMapPlugin = HeightMapPlugin;

/// Unified system that handles both initial generation and regeneration
fn render_height_map_system<G, M, R>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    // All entities that need terrain generation (initial or regeneration)
    height_map_query: Query<
        (
            Entity,
            &HeightMapConfig,
            &GeneratorComponent<G>,
            &MapperComponent<M>,
            &RendererComponent<R>,
            &Transform,
        ),
        (
            With<HeightMapEntity>,
            Or<(Without<HeightMapData>, Changed<HeightMapConfig>)>,
        ),
    >,
    // Existing terrain tiles for cleanup
    terrain_query: Query<(Entity, &TerrainTile)>,
) where
    G: HeightGenerator,
    M: TileMapper,
    R: TerrainRenderer<Mapper = M>,
{
    let entity_count = height_map_query.iter().count();
    if entity_count > 0 {
        println!("🔄 Processing {} height map entities", entity_count);

        for (entity, config, generator, mapper, renderer, transform) in height_map_query.iter() {
            println!("⚙️  Processing terrain for entity {:?}", entity);

            // Always clean up existing terrain tiles (prevents memory leaks)
            let mut despawned_count = 0;
            for (terrain_entity, terrain_tile) in terrain_query.iter() {
                if terrain_tile.map_entity == entity {
                    commands.entity(terrain_entity).despawn();
                    despawned_count += 1;
                }
            }

            if despawned_count > 0 {
                println!(
                    "🗑️  Cleaned up {} old terrain tiles for entity {:?}",
                    despawned_count, entity
                );
            }

            // Generate new terrain data
            let heights = generator.0.generate(config);
            let tile_heights = mapper.0.build(&heights);
            let height_map_data = HeightMapData {
                heights,
                tile_heights,
            };

            // Insert/update data component
            commands.entity(entity).insert(height_map_data.clone());

            // Spawn new terrain
            renderer.0.spawn_terrain(
                &mut commands,
                &asset_server,
                &mut texture_atlas_layouts,
                &height_map_data.heights,
                &height_map_data.tile_heights,
                config,
                &mapper.0,
                entity,
                transform,
            );

            println!("✅ Terrain processed for entity {:?}", entity);
        }
    }
}
