use bevy_ecs::component::Component;
use glam::Vec3;

/// Grid cell size for spatial hashing
pub const CELL_SIZE: f32 = 1.0;

/// Maximum number of entities in a city
pub const MAX_ENTITIES: usize = 30_000;

/// Grid dimensions
pub const GRID_WIDTH: usize = 256;
pub const GRID_HEIGHT: usize = 256;

/// Component to track an entity's position on the grid
#[derive(Component, Debug, Clone, Copy)]
pub struct GridPosition {
 pub x: usize,
 pub y: usize,
 pub z: f32,
}

impl GridPosition {
 pub fn new(x: usize, y: usize) -> Self {
 Self { x, y, z: 0.0 }
 }

 pub fn with_elevation(x: usize, y: usize, z: f32) -> Self {
 Self { x, y, z }
 }

 pub fn from_world(pos: Vec3) -> (usize, usize) {
 (
 (pos.x / CELL_SIZE).floor() as usize,
 (pos.y / CELL_SIZE).floor() as usize,
 )
 }

 pub fn to_world(&self) -> Vec3 {
 Vec3::new(
 self.x as f32 * CELL_SIZE,
 self.y as f32 * CELL_SIZE,
 self.z,
 )
 }

 pub fn to_index(&self, width: usize) -> usize {
 self.y * width + self.x
 }

 pub fn index_from_xy(x: usize, y: usize, width: usize) -> usize {
 y * width + x
 }
}

/// Spatial hash for fast neighbor queries
#[derive(Component)]
pub struct SpatialHash {
 pub cell_entities: Vec<Vec<bevy_ecs::entity::Entity>>,
 pub cell_size: f32,
 pub width: usize,
 pub height: usize,
}

impl SpatialHash {
 pub fn new(width: usize, height: usize, cell_size: f32) -> Self {
 let grid_size = ((width as f32 / cell_size).ceil() as usize)
 * ((height as f32 / cell_size).ceil() as usize);
 Self {
 cell_entities: vec![Vec::new(); grid_size.max(1024)],
 cell_size,
 width,
 height,
 }
 }

 pub fn get_cell_index(&self, x: f32, y: f32) -> usize {
 let cx = (x / self.cell_size).floor() as usize;
 let cy = (y / self.cell_size).floor() as usize;
 cy * (self.width / self.cell_size as usize) + cx
 }

 pub fn insert(&mut self, entity: bevy_ecs::entity::Entity, x: f32, y: f32) {
 let idx = self.get_cell_index(x, y);
 if idx < self.cell_entities.len() {
 self.cell_entities[idx].push(entity);
 }
 }

 pub fn clear(&mut self) {
 for cell in &mut self.cell_entities {
 cell.clear();
 }
 }

 pub fn query_radius(&self, x: f32, y: f32, radius: f32) -> Vec<bevy_ecs::entity::Entity> {
 let mut result = Vec::new();
 let r_cells = (radius / self.cell_size).ceil() as isize;
 let cx = (x / self.cell_size).floor() as isize;
 let cy = (y / self.cell_size).floor() as isize;

 for dy in -r_cells..=r_cells {
 for dx in -r_cells..=r_cells {
 let nx = cx + dx;
 let ny = cy + dy;
 if nx >= 0 && ny >= 0 {
 let idx = (ny as usize) * (self.width / self.cell_size as usize) + (nx as usize);
 if idx < self.cell_entities.len() {
 result.extend(&self.cell_entities[idx]);
 }
 }
 }
 }
 result
 }
}

/// Component for tracking which grid cells a building/road occupies
#[derive(Component, Debug, Clone)]
pub struct GridFootprint {
 pub cells: Vec<(usize, usize)>,
}

impl GridFootprint {
 pub fn new() -> Self {
 Self { cells: Vec::new() }
 }

 pub fn from_rect(x: usize, y: usize, width: usize, height: usize) -> Self {
 let mut cells = Vec::with_capacity(width * height);
 for dy in 0..height {
 for dx in 0..width {
 cells.push((x + dx, y + dy));
 }
 }
 Self { cells }
 }
}
