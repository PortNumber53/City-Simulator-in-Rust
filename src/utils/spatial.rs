// Spatial data structures for City Simulator

use glam::{Vec2, Vec3};

/// 2D bounding box for spatial queries
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
 pub min: Vec2,
 pub max: Vec2,
}

impl BoundingBox {
 pub fn new(min: Vec2, max: Vec2) -> Self {
 Self { min, max }
 }

 pub fn from_center(center: Vec2, size: f32) -> Self {
 let half = size * 0.5;
 Self {
 min: Vec2::new(center.x - half, center.y - half),
 max: Vec2::new(center.x + half, center.y + half),
 }
 }

 pub fn contains(&self, point: Vec2) -> bool {
 point.x >= self.min.x && point.x <= self.max.x &&
 point.y >= self.min.y && point.y <= self.max.y
 }

 pub fn intersects(&self, other: &BoundingBox) -> bool {
 self.min.x <= other.max.x && self.max.x >= other.min.x &&
 self.min.y <= other.max.y && self.max.y >= other.min.y
 }

 pub fn center(&self) -> Vec2 {
 (self.min + self.max) * 0.5
 }

 pub fn size(&self) -> Vec2 {
 self.max - self.min
 }

 pub fn area(&self) -> f32 {
 let size = self.size();
 size.x * size.y
 }
}

/// Simple quadtree for spatial partitioning
pub struct Quadtree<T> {
 bounds: BoundingBox,
 capacity: usize,
 points: Vec<(Vec2, T)>,
 divided: bool,
 northeast: Option<Box<Quadtree<T>>>,
 northwest: Option<Box<Quadtree<T>>>,
 southeast: Option<Box<Quadtree<T>>>,
 southwest: Option<Box<Quadtree<T>>>,
}

impl<T> Quadtree<T> 
where
 T: Copy + PartialEq
{
 pub fn new(bounds: BoundingBox, capacity: usize) -> Self {
 Self {
 bounds,
 capacity,
 points: Vec::with_capacity(capacity),
 divided: false,
 northeast: None,
 northwest: None,
 southeast: None,
 southwest: None,
 }
 }

 pub fn insert(&mut self, point: Vec2, data: T) -> bool {
 if !self.bounds.contains(point) {
 return false;
 }

 if self.points.len() < self.capacity && !self.divided {
 self.points.push((point, data));
 return true;
 }

 if !self.divided {
 self.subdivide();
 }

 if let Some(ref mut ne) = self.northeast {
 if ne.insert(point, data) { return true; }
 }
 if let Some(ref mut nw) = self.northwest {
 if nw.insert(point, data) { return true; }
 }
 if let Some(ref mut se) = self.southeast {
 if se.insert(point, data) { return true; }
 }
 if let Some(ref mut sw) = self.southwest {
 if sw.insert(point, data) { return true; }
 }

 false
 }

 fn subdivide(&mut self) {
 let center = self.bounds.center();
 let half_size = self.bounds.size() * 0.5;

 let ne_bounds = BoundingBox::new(center, self.bounds.max);
 let nw_bounds = BoundingBox::new(
 Vec2::new(self.bounds.min.x, center.y),
 Vec2::new(center.x, self.bounds.max.y)
 );
 let se_bounds = BoundingBox::new(
 Vec2::new(center.x, self.bounds.min.y),
 Vec2::new(self.bounds.max.x, center.y)
 );
 let sw_bounds = BoundingBox::new(self.bounds.min, center);

 self.northeast = Some(Box::new(Quadtree::new(ne_bounds, self.capacity)));
 self.northwest = Some(Box::new(Quadtree::new(nw_bounds, self.capacity)));
 self.southeast = Some(Box::new(Quadtree::new(se_bounds, self.capacity)));
 self.southwest = Some(Box::new(Quadtree::new(sw_bounds, self.capacity)));

 self.divided = true;

 let points = std::mem::take(&mut self.points);
 for (point, data) in points {
 self.insert(point, data);
 }
 }

 pub fn query(&self, range: BoundingBox, found: &mut Vec<T>) {
 if !self.bounds.intersects(&range) {
 return;
 }

 for (point, data) in &self.points {
 if range.contains(*point) {
 found.push(*data);
 }
 }

 if self.divided {
 if let Some(ref ne) = self.northeast { ne.query(range, found); }
 if let Some(ref nw) = self.northwest { nw.query(range, found); }
 if let Some(ref se) = self.southeast { se.query(range, found); }
 if let Some(ref sw) = self.southwest { sw.query(range, found); }
 }
 }

 pub fn clear(&mut self) {
 self.points.clear();
 self.divided = false;
 self.northeast = None;
 self.northwest = None;
 self.southeast = None;
 self.southwest = None;
 }
}

/// Grid-based spatial hash
pub struct SpatialHashGrid {
 cell_size: f32,
 inv_cell_size: f32,
 width: usize,
 height: usize,
 cells: Vec<Vec<u32>>,
}

impl SpatialHashGrid {
 pub fn new(cell_size: f32, width: usize, height: usize) -> Self {
 let grid_width = (width as f32 / cell_size).ceil() as usize;
 let grid_height = (height as f32 / cell_size).ceil() as usize;
 let cell_count = grid_width * grid_height;
 
 Self {
 cell_size,
 inv_cell_size: 1.0 / cell_size,
 width: grid_width,
 height: grid_height,
 cells: vec![Vec::new(); cell_count],
 }
 }

 fn cell_index(&self, x: f32, y: f32) -> (usize, usize) {
 let cx = (x * self.inv_cell_size).floor() as usize;
 let cy = (y * self.inv_cell_size).floor() as usize;
 (cx.min(self.width - 1), cy.min(self.height - 1))
 }

 fn flat_index(&self, cx: usize, cy: usize) -> usize {
 cy * self.width + cx
 }

 pub fn insert(&mut self, x: f32, y: f32, entity_id: u32) {
 let (cx, cy) = self.cell_index(x, y);
 let idx = self.flat_index(cx, cy);
 if idx < self.cells.len() {
 self.cells[idx].push(entity_id);
 }
 }

 pub fn clear(&mut self) {
 for cell in &mut self.cells {
 cell.clear();
 }
 }

 pub fn query_radius(&self, x: f32, y: f32, radius: f32) -> Vec<u32> {
 let mut result = Vec::new();
 let radius_cells = (radius * self.inv_cell_size).ceil() as i32;
 let (center_cx, center_cy) = self.cell_index(x, y);

 for dy in -radius_cells..=radius_cells {
 for dx in -radius_cells..=radius_cells {
 let cx = (center_cx as i32 + dx).max(0) as usize;
 let cy = (center_cy as i32 + dy).max(0) as usize;
 
 if cx < self.width && cy < self.height {
 let idx = self.flat_index(cx, cy);
 result.extend(&self.cells[idx]);
 }
 }
 }
 
 result
 }
}

/// Ray for raycasting
#[derive(Debug, Clone, Copy)]
pub struct Ray {
 pub origin: Vec3,
 pub direction: Vec3,
}

impl Ray {
 pub fn new(origin: Vec3, direction: Vec3) -> Self {
 Self {
 origin,
 direction: direction.normalize(),
 }
 }

 pub fn at(&self, t: f32) -> Vec3 {
 self.origin + self.direction * t
 }
}
