use bevy_ecs::component::Component;

/// Component for tracking which network a building belongs to
#[derive(Component, Debug, Clone, Copy)]
pub struct NetworkMember {
 pub network_id: u32,
 pub network_type: NetworkType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkType {
 Power,
 Water,
}

/// State of a repair site
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairState {
 Stable,
 PendingRedirection,
 DrainingTraffic,
 RepairActive,
 Reopening,
}

impl RepairState {
 pub fn is_phase1(&self) -> bool {
 matches!(self, Self::PendingRedirection | Self::DrainingTraffic)
 }

 pub fn is_phase2(&self) -> bool {
 matches!(self, Self::RepairActive)
 }

 pub fn allows_emergency(&self) -> bool {
 self.is_phase1()
 }
}

/// Construction depot for spawning vehicles
#[derive(Component, Debug, Clone, Copy)]
pub struct UtilityDepot {
 pub depot_type: UtilityDepotType,
 pub dispatch_interval: f32,
 pub last_dispatch: f32,
 pub vehicle_pool: VehiclePool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UtilityDepotType {
 BusDepot,
 TrainYard,
}

#[derive(Debug, Clone, Copy)]
pub struct VehiclePool {
 pub available_vehicles: u16,
 pub total_vehicles: u16,
}

impl UtilityDepot {
 pub fn bus_depot() -> Self {
 Self {
 depot_type: UtilityDepotType::BusDepot,
 dispatch_interval: 10.0,
 last_dispatch: 0.0,
 vehicle_pool: VehiclePool {
 available_vehicles: 10,
 total_vehicles: 10,
 },
 }
 }

 pub fn train_yard() -> Self {
 Self {
 depot_type: UtilityDepotType::TrainYard,
 dispatch_interval: 5.0,
 last_dispatch: 0.0,
 vehicle_pool: VehiclePool {
 available_vehicles: 5,
 total_vehicles: 5,
 },
 }
 }

 pub fn can_dispatch(&self, current_time: f32) -> bool {
 current_time - self.last_dispatch >= self.dispatch_interval
 && self.vehicle_pool.available_vehicles > 0
 }

 pub fn dispatch(&mut self, current_time: f32) -> bool {
 if !self.can_dispatch(current_time) {
 return false;
 }
 self.last_dispatch = current_time;
 self.vehicle_pool.available_vehicles -= 1;
 true
 }

 pub fn return_vehicle(&mut self) {
 if self.vehicle_pool.available_vehicles < self.vehicle_pool.total_vehicles {
 self.vehicle_pool.available_vehicles += 1;
 }
 }
}

/// Utility fire component
#[derive(Component, Debug, Clone, Copy)]
pub struct UtilityFire {
 pub intensity: f32,
 pub spread_radius: f32,
 pub unique_id: u32,
}

impl UtilityFire {
 pub fn new(intensity: f32) -> Self {
 Self {
 intensity,
 spread_radius: 3.0,
 unique_id: 0,
 }
 }

 pub fn is_active(&self) -> bool {
 self.intensity > 0.0
 }

 pub fn spread_rate(&self) -> f32 {
 self.intensity * 0.5
 }
}

/// Active fire list for performance optimization
#[derive(Debug, Default)]
pub struct ActiveFireList {
 pub fires: Vec<bevy_ecs::entity::Entity>,
}

/// Terrain heightmap resource
#[derive(Debug, Clone)]
pub struct TerrainHeightmap {
 pub heights: Vec<f32>,
 pub width: usize,
 pub height: usize,
}

impl TerrainHeightmap {
 pub fn new(width: usize, height: usize) -> Self {
 Self {
 heights: vec![0.0; width * height],
 width,
 height,
 }
 }

 pub fn get(&self, x: usize, y: usize) -> f32 {
 let idx = y * self.width + x;
 if idx < self.heights.len() {
 self.heights[idx]
 } else {
 0.0
 }
 }

 pub fn set(&mut self, x: usize, y: usize, value: f32) {
 let idx = y * self.width + x;
 if idx < self.heights.len() {
 self.heights[idx] = value;
 }
 }

 pub fn smooth(&mut self, x: usize, y: usize, radius: usize) {
 let mut sum = 0.0;
 let mut count = 0;

 for dy in 0..=radius * 2 {
 for dx in 0..=radius * 2 {
 let nx = x.saturating_add(dx).saturating_sub(radius);
 let ny = y.saturating_add(dy).saturating_sub(radius);
 if nx < self.width && ny < self.height {
 sum += self.get(nx, ny);
 count += 1;
 }
 }
 }

 if count > 0 {
 self.set(x, y, sum / count as f32);
 }
 }
}

/// Terraforming (leveling) tool parameters
#[derive(Debug, Clone, Copy)]
pub struct TerraformTool {
 pub radius: f32,
 pub strength: f32,
 pub target_height: f32,
}

impl Default for TerraformTool {
 fn default() -> Self {
 Self {
 radius: 5.0,
 strength: 0.5,
 target_height: 0.0,
 }
 }
}

/// Clip map for tunnel entrances
#[derive(Debug, Clone)]
pub struct ClipMap {
 pub holes: Vec<bool>,
 pub width: usize,
 pub height: usize,
}

impl ClipMap {
 pub fn new(width: usize, height: usize) -> Self {
 Self {
 holes: vec![false; width * height],
 width,
 height,
 }
 }

 pub fn has_hole(&self, x: usize, y: usize) -> bool {
 let idx = y * self.width + x;
 if idx < self.holes.len() {
 self.holes[idx]
 } else {
 false
 }
 }

 pub fn set_hole(&mut self, x: usize, y: usize, has_hole: bool) {
 let idx = y * self.width + x;
 if idx < self.holes.len() {
 self.holes[idx] = has_hole;
 }
 }
}
