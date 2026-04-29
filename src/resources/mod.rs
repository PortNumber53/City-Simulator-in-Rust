// Resources for City Simulator
// These are global data structures that systems can read/write

use bevy_ecs::prelude::*;
use glam::Vec2;
use rand::SeedableRng;

/// Time resource for tracking simulation time
#[derive(Resource)]
pub struct Time {
 elapsed_seconds: f32,
 delta_seconds: f32,
}

impl Default for Time {
 fn default() -> Self {
 Self {
 elapsed_seconds: 0.0,
 delta_seconds: 0.016,
 }
 }
}

impl Time {
 pub fn delta_seconds(&self) -> f32 {
 self.delta_seconds
 }
 
 pub fn elapsed_seconds(&self) -> f32 {
 self.elapsed_seconds
 }
 
 pub fn advance(&mut self, dt: f32) {
 self.delta_seconds = dt;
 self.elapsed_seconds += dt;
 }
}

/// Current tick counter
#[derive(Resource, Default)]
pub struct CurrentTick(pub u64);

/// Random number generator for simulation
#[derive(Resource)]
pub struct RandomGenerator {
 pub rng: rand::rngs::StdRng,
}

impl Default for RandomGenerator {
 fn default() -> Self {
 Self {
 rng: rand::rngs::StdRng::seed_from_u64(42),
 }
 }
}

/// Budget resource for city finances
#[derive(Resource)]
pub struct CityBudget {
 pub total_funds: f32,
 pub maintenance_fund: f32,
 pub required_maintenance: f32,
 pub tax_rate: f32,
}

impl Default for CityBudget {
 fn default() -> Self {
 Self {
 total_funds: 100_000.0,
 maintenance_fund: 50_000.0,
 required_maintenance: 50_000.0,
 tax_rate: 0.1,
 }
 }
}

impl CityBudget {
 /// Calculate maintenance ratio
 pub fn maintenance_ratio(&self) -> f32 {
 if self.required_maintenance <= 0.0 {
 1.0
 } else {
 (self.maintenance_fund / self.required_maintenance).min(1.0)
 }
 }

 /// Check if budget is in surplus
 pub fn has_surplus(&self) -> bool {
 self.maintenance_fund > self.required_maintenance
 }

 /// Allocate funds for maintenance
 pub fn allocate_maintenance(&mut self, amount: f32) {
 self.maintenance_fund = (self.maintenance_fund + amount).max(0.0);
 }

 /// Spend from maintenance budget
 pub fn spend_maintenance(&mut self, amount: f32) -> bool {
 if self.maintenance_fund >= amount {
 self.maintenance_fund -= amount;
 true
 } else {
 false
 }
 }

 /// Add tax revenue
 pub fn add_taxes(&mut self, amount: f32) {
 self.total_funds += amount;
 }
}

/// Camera resource for rendering
#[derive(Resource)]
pub struct Camera {
 pub position: glam::Vec3,
 pub target: glam::Vec3,
 pub fov: f32,
 pub aspect_ratio: f32,
}

impl Default for Camera {
 fn default() -> Self {
 Self {
 position: glam::Vec3::new(0.0, 0.0, 50.0),
 target: glam::Vec3::ZERO,
 fov: 45.0,
 aspect_ratio: 16.0 / 9.0,
 }
 }
}

/// Global overlay mode
#[derive(Resource, Default)]
pub enum OverlayMode {
 #[default]
 Normal,
 Power,
 Water,
 Maintenance,
 Nature,
}

/// Input state
#[derive(Resource)]
pub struct InputState {
 pub world_x: f32,
 pub world_y: f32,
 pub is_left_mouse_down: bool,
 pub current_tool: crate::components::ToolMode,
}

impl Default for InputState {
 fn default() -> Self {
 Self {
 world_x: 0.0,
 world_y: 0.0,
 is_left_mouse_down: false,
 current_tool: crate::components::ToolMode::None,
 }
 }
}

/// Grid cell component using bit-packing
/// Layout:
/// - bit 0: Occupied
/// - bit 1: Conducts Power
/// - bit 2: Conducts Water
/// - bit 3: Has Water (hydrant for fire fighting)
/// - bit 4: Has Power
/// - bit 5: Is Road
/// - bit 6: Is Building
/// - bit 7: Is Transit
/// - bits 8-31: Entity ID (index into ECS)
#[derive(Resource)]
pub struct CityGrid {
 pub cells: Vec<u32>, // Bit-packed cell data
 pub width: usize,
 pub height: usize,
}

// Bitmask constants
pub const OCCUPIED_BIT: u32 = 1 << 0;
pub const CONDUCTS_POWER_BIT: u32 = 1 << 1;
pub const CONDUCTS_WATER_BIT: u32 = 1 << 2;
pub const HAS_WATER_BIT: u32 = 1 << 3;
pub const HAS_POWER_BIT: u32 = 1 << 4;
pub const IS_ROAD_BIT: u32 = 1 << 5;
pub const IS_BUILDING_BIT: u32 = 1 << 6;
pub const IS_TRANSIT_BIT: u32 = 1 << 7;
pub const ENTITY_ID_MASK: u32 = !0xFF;
pub const ENTITY_ID_SHIFT: u32 = 8;

impl CityGrid {
 pub fn new(width: usize, height: usize) -> Self {
 Self {
 cells: vec![0; width * height],
 width,
 height,
 }
 }

 #[inline(always)]
 pub fn get(&self, x: usize, y: usize) -> u32 {
 let idx = y * self.width + x;
 if idx < self.cells.len() {
 self.cells[idx]
 } else {
 0
 }
 }

 #[inline(always)]
 pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut u32> {
 let idx = y * self.width + x;
 self.cells.get_mut(idx)
 }

 pub fn is_occupied(&self, x: usize, y: usize) -> bool {
 (self.get(x, y) & OCCUPIED_BIT) != 0
 }

 pub fn conducts_power(&self, x: usize, y: usize) -> bool {
 (self.get(x, y) & CONDUCTS_POWER_BIT) != 0
 }

 pub fn get_entity_id(&self, x: usize, y: usize) -> Option<u32> {
 let id = (self.get(x, y) & ENTITY_ID_MASK) >> ENTITY_ID_SHIFT;
 if id == 0 { None } else { Some(id - 1) }
 }

 pub fn set_entity_id(&mut self, x: usize, y: usize, id: Option<u32>) {
 if let Some(cell) = self.get_mut(x, y) {
 *cell &= !ENTITY_ID_MASK;
 if let Some(id) = id {
 *cell |= ((id + 1) << ENTITY_ID_SHIFT) as u32;
 }
 }
 }

 pub fn rebuild_networks(&mut self) {
 // BFS/DSU algorithm to group connected buildings
 // Called when connectivity changes
 }

 pub fn mark_dirty(&mut self) {
 // Trigger utility rebuild
 }
}

impl Default for CityGrid {
 fn default() -> Self {
 Self::new(256, 256)
 }
}

/// Network metadata for power/water grids
#[derive(Debug, Clone, Copy)]
pub struct NetworkMetadata {
 pub network_id: u32,
 pub total_supply: f32,
 pub total_demand: f32,
 pub status: NetworkStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkStatus {
 Sufficient,
 Critical,
 Failure,
}

impl NetworkMetadata {
 pub fn new(id: u32) -> Self {
 Self {
 network_id: id,
 total_supply: 0.0,
 total_demand: 0.0,
 status: NetworkStatus::Failure,
 }
 }

 pub fn load_factor(&self) -> f32 {
 if self.total_demand <= 0.0 {
 1.0
 } else {
 (self.total_supply / self.total_demand).min(1.0)
 }
 }

 pub fn update_status(&mut self) {
 self.status = if self.total_supply == 0.0 {
 NetworkStatus::Failure
 } else if self.total_supply < self.total_demand {
 NetworkStatus::Critical
 } else {
 NetworkStatus::Sufficient
 };
 }
}

/// Network map resource
#[derive(Resource)]
pub struct NetworkMap {
 pub networks: Vec<NetworkMetadata>,
}

impl Default for NetworkMap {
 fn default() -> Self {
 Self {
 networks: Vec::with_capacity(100),
 }
 }
}

impl NetworkMap {
 pub fn reset_all(&mut self) {
 for network in &mut self.networks {
 network.total_supply = 0.0;
 network.total_demand = 0.0;
 }
 }

 pub fn add_supply(&mut self, network_id: u32, supply: f32) {
 if let Some(network) = self.networks.get_mut(network_id as usize) {
 network.total_supply += supply;
 }
 }

 pub fn add_demand(&mut self, network_id: u32, demand: f32) {
 if let Some(network) = self.networks.get_mut(network_id as usize) {
 network.total_demand += demand;
 }
 }

 pub fn get(&self, network_id: u32) -> Option<&NetworkMetadata> {
 self.networks.get(network_id as usize)
 }

 pub fn update_statuses(&mut self) {
 for network in &mut self.networks {
 network.update_status();
 }
 }
}

/// Navigation graph resource
#[derive(Resource, Default)]
pub struct NavigationGraph {
 pub nodes: Vec<NavNode>,
 pub edges: Vec<NavEdge>,
 pub adjacency: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, Copy)]
pub struct NavNode {
 pub position: glam::Vec3,
 pub entity: bevy_ecs::entity::Entity,
}

#[derive(Debug, Clone, Copy)]
pub struct NavEdge {
 pub from: usize,
 pub to: usize,
 pub weight: f32,
 pub road_entity: bevy_ecs::entity::Entity,
}

/// Transit versioning for invalidation
#[derive(Resource, Default)]
pub struct TransitVersion(pub u64);

/// Nature system resource
#[derive(Resource)]
pub struct NatureSystem {
 pub tick_rate: u32,
 pub base_growth: u8,
 pub random_tick_count: usize,
}

impl Default for NatureSystem {
 fn default() -> Self {
 Self {
 tick_rate: 500,
 base_growth: 1,
 random_tick_count: 500,
 }
 }
}

/// Wind direction and speed
#[derive(Resource)]
pub struct Wind {
 pub direction: f32,
 pub velocity: f32,
}

impl Default for Wind {
 fn default() -> Self {
 Self {
 direction: 0.0,
 velocity: 5.0,
 }
 }
}

impl Wind {
 /// Calculate wind factor for a given direction
 pub fn dot_with_direction(&self, dx: f32, dy: f32) -> f32 {
 let wind_dir = Vec2::from_angle(self.direction);
 let spread_dir = Vec2::new(dx, dy).normalize_or_zero();
 wind_dir.dot(spread_dir)
 }
}

/// Active fires list
#[derive(Resource, Default)]
pub struct ActiveFires {
 pub fires: Vec<bevy_ecs::entity::Entity>,
}

/// To-ignite buffer for fire spreading
#[derive(Resource, Default)]
pub struct ToIgniteBuffer {
 pub targets: Vec<(bevy_ecs::entity::Entity, f32)>,
}

/// Global uniforms for rendering
#[derive(Resource, Debug, Clone)]
pub struct GlobalUniforms {
 pub time: f32,
 pub frame: u32,
 pub sun_direction: [f32; 3],
}

impl Default for GlobalUniforms {
 fn default() -> Self {
 Self {
 time: 0.0,
 frame: 0,
 sun_direction: [0.0, 1.0, 0.3],
 }
 }
}
