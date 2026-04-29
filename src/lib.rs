// City Simulator - A WebAssembly-based city simulation game

use bevy_ecs::prelude::*;
use glam::Vec3;

pub mod components;
pub mod resources;
pub mod systems;
pub mod utils;

pub use components::*;
pub use resources::*;
pub use systems::*;
pub use utils::*;

/// Main simulation state
pub struct CitySimulator {
 pub world: World,
 pub schedule: Schedule,
}

impl Default for CitySimulator {
 fn default() -> Self {
 let mut world = World::new();
 let mut schedule = Schedule::default();
 
 // Initialize resources
 world.insert_resource(Time::default());
 world.insert_resource(CurrentTick::default());
 world.insert_resource(CityGrid::default());
 world.insert_resource(NetworkMap::default());
 world.insert_resource(CityBudget::default());
 world.insert_resource(InputState::default());
 world.insert_resource(Camera::default());
 world.insert_resource(OverlayMode::default());
 world.insert_resource(Wind::default());
 world.insert_resource(NatureSystem::default());
 world.insert_resource(ActiveFires::default());
 world.insert_resource(ToIgniteBuffer::default());
 world.insert_resource(NavigationGraph::default());
 world.insert_resource(TransitVersion::default());
 world.insert_resource(RandomGenerator::default());
 world.insert_resource(GlobalUniforms::default());

 Self { world, schedule }
 }
}

impl CitySimulator {
 /// Create a new city simulator
 pub fn new() -> Self {
 Self::default()
 }
 
 /// Update the simulation by one frame
 pub fn update(&mut self, dt: f32) {
 if let Some(mut time) = self.world.get_resource_mut::<Time>() {
 time.advance(dt);
 }
 
 self.schedule.run(&mut self.world);
 }
 
 /// Get the ECS world
 pub fn world(&self) -> &World {
 &self.world
 }
 
 /// Get mutable access to the ECS world
 pub fn world_mut(&mut self) -> &mut World {
 &mut self.world
 }
 
 /// Spawn a building
 pub fn spawn_building(&mut self, building_type: BuildingType, position: (usize, usize)) -> Entity {
 let entity = self.world.spawn((
 Building::new(building_type),
 GridPosition::new(position.0, position.1),
 Transform::new(Vec3::new(position.0 as f32, position.1 as f32, 0.0)),
 VisualIntensity::default(),
 Maintenance::default(),
 VegetationState::default(),
 )).id();
 
 entity
 }
 
 /// Spawn a road
 pub fn spawn_road(&mut self, start: Vec3, end: Vec3, lanes: u8) -> Entity {
 let entity = self.world.spawn((
 RoadSpline::new(start, end),
 RoadGeometry::new(lanes),
 Maintenance::new(0.001),
 )).id();
 
 entity
 }
 
 /// Get simulation time
 pub fn time(&self) -> f32 {
 self.world.get_resource::<Time>()
 .map(|t| t.elapsed_seconds())
 .unwrap_or(0.0)
 }
 
 /// Get city budget
 pub fn budget(&self) -> &CityBudget {
 self.world.get_resource::<CityBudget>()
 .expect("CityBudget resource not found")
 }
 
 /// Modify budget
 pub fn modify_budget(&mut self, amount: f32) {
 if let Some(mut budget) = self.world.get_resource_mut::<CityBudget>() {
 budget.total_funds += amount;
 }
 }
}

// WASM Bindings
#[cfg(target_arch = "wasm32")]
mod wasm {
 use super::*;
 use wasm_bindgen::prelude::*;
 
 #[wasm_bindgen]
 pub struct WasmSimulator {
 simulator: CitySimulator,
 }
 
 #[wasm_bindgen]
 impl WasmSimulator {
 #[wasm_bindgen(constructor)]
 pub fn new() -> Self {
 Self {
 simulator: CitySimulator::new(),
 }
 }
 
 #[wasm_bindgen]
 pub fn update(&mut self, dt: f32) {
 self.simulator.update(dt);
 }
 
 #[wasm_bindgen]
 pub fn time(&self) -> f32 {
 self.simulator.time()
 }
 
 #[wasm_bindgen]
 pub fn spawn_building(&mut self, building_type: u32, x: usize, y: usize) {
 let bt = match building_type {
 0 => BuildingType::Residential,
 1 => BuildingType::Commercial,
 2 => BuildingType::Industrial,
 _ => BuildingType::Residential,
 };
 self.simulator.spawn_building(bt, (x, y));
 }
 }
}
