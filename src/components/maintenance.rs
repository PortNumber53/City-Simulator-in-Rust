use bevy_ecs::component::Component;

/// Component for tracking which sector of the grid needs updating
#[derive(Component, Debug, Clone, Copy)]
pub struct DirtySector {
 pub sector_x: usize,
 pub sector_y: usize,
 pub is_dirty: bool,
}

impl DirtySector {
 pub fn new(sector_x: usize, sector_y: usize) -> Self {
 Self {
 sector_x,
 sector_y,
 is_dirty: false,
 }
 }

 pub fn mark_dirty(&mut self) {
 self.is_dirty = true;
 }

 pub fn clear(&mut self) {
 self.is_dirty = false;
 }
}

/// Free list for entity recycling
#[derive(Component)]
pub struct FreeList {
 pub available_ids: Vec<u32>,
}

impl FreeList {
 pub fn new() -> Self {
 Self {
 available_ids: Vec::new(),
 }
 }

 pub fn acquire(&mut self) -> Option<u32> {
 self.available_ids.pop()
 }

 pub fn release(&mut self, id: u32) {
 self.available_ids.push(id);
 }

 pub fn has_available(&self) -> bool {
 !self.available_ids.is_empty()
 }
}

/// Generation-based entity versioning for safe recycling
#[derive(Component, Debug, Clone, Copy)]
pub struct EntityVersion {
 pub index: u32,
 pub generation: u32,
}

impl EntityVersion {
 pub fn new(index: u32) -> Self {
 Self { index, generation: 1 }
 }

 pub fn recycle(&mut self) {
 self.generation += 1;
 }

 pub fn matches(&self, other: &EntityVersion) -> bool {
 self.index == other.index && self.generation == other.generation
 }
}

/// Event log for tracking city history
#[derive(Component, Debug, Clone)]
pub struct EventLog {
 pub events: Vec<CityEvent>,
 pub max_events: usize,
}

impl EventLog {
 pub fn new(max_events: usize) -> Self {
 Self {
 events: Vec::with_capacity(max_events),
 max_events,
 }
 }

 pub fn log(&mut self, event: CityEvent) {
 if self.events.len() >= self.max_events {
 self.events.remove(0);
 }
 self.events.push(event);
 }
}

#[derive(Debug, Clone, Copy)]
pub struct CityEvent {
 pub timestamp: u64,
 pub event_type: EventType,
 pub x: f32,
 pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
 BuildingPlaced,
 RoadPlaced,
 BuildingDestroyed,
 FireStarted,
 FireExtinguished,
 Disaster,
 BudgetUpdated,
}

/// Request queue for asynchronous operations
#[derive(Component, Debug, Clone)]
pub struct RequestQueue<T> {
 pub requests: Vec<T>,
}

impl<T> RequestQueue<T> {
 pub fn new() -> Self {
 Self {
 requests: Vec::new(),
 }
 }

 pub fn push(&mut self, request: T) {
 self.requests.push(request);
 }

 pub fn drain(&mut self) -> Vec<T> {
 std::mem::take(&mut self.requests)
 }

 pub fn is_empty(&self) -> bool {
 self.requests.is_empty()
 }
}

/// Command buffer for deferred entity operations
#[derive(Component, Debug, Clone)]
pub struct CommandBuffer {
 pub commands: Vec<EntityCommand>,
}

impl CommandBuffer {
 pub fn new() -> Self {
 Self {
 commands: Vec::new(),
 }
 }

 pub fn push(&mut self, command: EntityCommand) {
 self.commands.push(command);
 }

 pub fn clear(&mut self) {
 self.commands.clear();
 }
}

#[derive(Debug, Clone, Copy)]
pub enum EntityCommand {
 Spawn {
 entity_type: EntityType,
 position: (usize, usize),
 },
 Despawn {
 entity: bevy_ecs::entity::Entity,
 },
 UpdateComponent {
 entity: bevy_ecs::entity::Entity,
 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
 Residential,
 Commercial,
 Industrial,
 Road,
 PowerPlant,
 WaterStation,
 FireStation,
 PoliceStation,
 BusStop,
 TrainStation,
}

/// Repair tool state for manual repairs
#[derive(Component, Debug, Clone, Copy)]
pub struct RepairTool {
 pub radius: f32,
 pub cost_per_health: f32,
 pub is_active: bool,
}

impl Default for RepairTool {
 fn default() -> Self {
 Self {
 radius: 3.0,
 cost_per_health: 100.0,
 is_active: false,
 }
 }
}

/// Leveling/bulldozer tool state
#[derive(Component, Debug, Clone, Copy)]
pub struct LevelingTool {
 pub radius: f32,
 pub strength: f32,
 pub cost_per_cubic_meter: f32,
}

impl Default for LevelingTool {
 fn default() -> Self {
 Self {
 radius: 5.0,
 strength: 0.5,
 cost_per_cubic_meter: 10.0,
 }
 }
}

/// Placement ghost for hover validation
#[derive(Component, Debug, Clone, Copy)]
pub struct PlacementGhost {
 pub is_valid: bool,
 pub cost: f32,
}

impl Default for PlacementGhost {
 fn default() -> Self {
 Self {
 is_valid: true,
 cost: 0.0,
 }
 }
}

/// Tool mode for player interaction
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolMode {
 None,
 BuildResidential,
 BuildCommercial,
 BuildIndustrial,
 BuildRoad,
 BuildPowerPlant,
 BuildWaterStation,
 Bulldozer,
 LevelTerrain,
 Repair,
 PlaceBusStop,
 PlaceTrainStation,
}

impl Default for ToolMode {
 fn default() -> Self {
 ToolMode::None
 }
}

/// Maintenance score for infrastructure health
#[derive(Component, Debug, Clone, Copy)]
pub struct Maintenance {
 pub health: f32,
 pub wear_rate: f32,
 pub last_repair_tick: u64,
 pub compaction: f32,
}

impl Default for Maintenance {
 fn default() -> Self {
 Self {
 health: 1.0,
 wear_rate: 0.001,
 last_repair_tick: 0,
 compaction: 0.0,
 }
 }
}

impl Maintenance {
 pub fn new(wear_rate: f32) -> Self {
 Self {
 health: 1.0,
 wear_rate,
 last_repair_tick: 0,
 compaction: 0.0,
 }
 }

 pub fn calculate_decay(&self, maintenance_ratio: f32, dt: f32) -> f32 {
 let factor = (1.0 - maintenance_ratio).max(0.0);
 self.wear_rate * factor * dt
 }

 pub fn damaged(&self) -> bool {
 self.health < 0.7
 }

 pub fn critical(&self) -> bool {
 self.health < 0.3
 }

 pub fn destroyed(&self) -> bool {
 self.health <= 0.0
 }

 pub fn repair(&mut self, amount: f32, current_tick: u64) {
 self.health = (self.health + amount).min(1.0);
 self.last_repair_tick = current_tick;
 }

 pub fn speed_multiplier(&self) -> f32 {
 self.health.max(0.1)
 }
}

/// Construction zone component
#[derive(Component, Debug, Clone, Copy)]
pub struct ConstructionZone {
 pub progress: f32,
 pub lane_closure: u8,
 pub crew_present: bool,
 pub phase_start_tick: u64,
}

impl ConstructionZone {
 pub fn new(phase_tick: u64) -> Self {
 Self {
 progress: 0.0,
 lane_closure: 0,
 crew_present: false,
 phase_start_tick: phase_tick,
 }
 }
}

/// Repair cost calculation
#[derive(Component, Debug, Clone, Copy)]
pub struct RepairCost {
 pub base_cost: f32,
 pub material_multiplier: f32,
 pub labor_multiplier: f32,
}

impl RepairCost {
 pub fn calculate(&self, health_missing: f32) -> f32 {
 self.base_cost * health_missing * (self.material_multiplier + self.labor_multiplier)
 }
}

/// Under construction marker
#[derive(Component, Debug, Clone, Copy)]
pub struct UnderConstruction {
 pub remaining_ticks: u32,
}

/// Construction crew component
#[derive(Component, Debug, Clone, Copy)]
pub struct MaintenanceCrew {
 pub crew_type: crate::components::CrewType,
 pub target_entity: Option<bevy_ecs::entity::Entity>,
 pub work_progress: f32,
}
