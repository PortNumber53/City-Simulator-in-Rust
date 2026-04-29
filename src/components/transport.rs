use bevy_ecs::component::Component;

/// Transit mode classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitMode {
 Bus,
 Train,
}

/// Custom transit line defined by player
#[derive(Component, Debug, Clone)]
pub struct TransitLine {
 pub id: u32,
 pub mode: TransitMode,
 pub color: [f32; 3],
 pub frequency: f32,
 pub waypoints: Vec<bevy_ecs::entity::Entity>,
 pub path_segments: Vec<bevy_ecs::entity::Entity>,
 pub last_dispatch: f32,
}

impl TransitLine {
 pub fn new(id: u32, mode: TransitMode) -> Self {
 Self {
 id,
 mode,
 color: match mode {
 TransitMode::Bus => [1.0, 0.5, 0.0],
 TransitMode::Train => [0.0, 0.5, 1.0],
 },
 frequency: 10.0,
 waypoints: Vec::new(),
 path_segments: Vec::new(),
 last_dispatch: 0.0,
 }
 }

 pub fn with_color(mut self, r: f32, g: f32, b: f32) -> Self {
 self.color = [r, g, b];
 self
 }

 pub fn with_frequency(mut self, freq: f32) -> Self {
 self.frequency = freq.max(1.0);
 self
 }

 pub fn should_dispatch(&self, current_time: f32) -> bool {
 current_time - self.last_dispatch >= self.frequency
 }

 pub fn mark_dispatched(&mut self, time: f32) {
 self.last_dispatch = time;
 }
}

/// Stop/Station entity component
#[derive(Component, Debug, Clone, Copy)]
pub struct TransitStop {
 pub stop_id: u32,
 pub line_ids: [u32; 4],
 pub line_count: u8,
 pub stop_position: f32,
 pub has_bus_present: bool,
 pub is_train_station: bool,
}

impl TransitStop {
 pub fn new(stop_id: u32) -> Self {
 Self {
 stop_id,
 line_ids: [0; 4],
 line_count: 0,
 stop_position: 0.0,
 has_bus_present: false,
 is_train_station: false,
 }
 }

 pub fn add_line(&mut self, line_id: u32) -> bool {
 if self.line_count >= 4 {
 return false;
 }
 self.line_ids[self.line_count as usize] = line_id;
 self.line_count += 1;
 true
 }

 pub fn has_line(&self, line_id: u32) -> bool {
 self.line_ids.iter().take(self.line_count as usize).any(|&id| id == line_id)
 }
}

/// Fixed path script for transit vehicles
#[derive(Component, Debug, Clone)]
pub struct FixedPath {
 pub line_id: u32,
 pub current_stop_index: usize,
 pub direction: i8,
 pub route_complete: bool,
}

impl FixedPath {
 pub fn new(line_id: u32) -> Self {
 Self {
 line_id,
 current_stop_index: 0,
 direction: 1,
 route_complete: false,
 }
 }

 pub fn advance_stop(&mut self, num_stops: usize) {
 self.current_stop_index = (self.current_stop_index as i8 + self.direction) as usize;
 
 if self.current_stop_index >= num_stops {
 if self.direction > 0 {
 self.direction = -1;
 self.current_stop_index = num_stops.saturating_sub(2);
 } else {
 self.current_stop_index = 0;
 self.direction = 1;
 }
 }
 }

 pub fn current_stop(&self) -> usize {
 self.current_stop_index
 }
}

/// Waiting queue for bus/train stops
#[derive(Component, Debug, Clone)]
pub struct WaitingQueue {
 pub agents: Vec<bevy_ecs::entity::Entity>,
 pub max_capacity: u16,
}

impl WaitingQueue {
 pub fn new(max_capacity: u16) -> Self {
 Self {
 agents: Vec::new(),
 max_capacity,
 }
 }

 pub fn add_agent(&mut self, agent: bevy_ecs::entity::Entity) -> bool {
 if self.agents.len() >= self.max_capacity as usize {
 return false;
 }
 self.agents.push(agent);
 true
 }

 pub fn remove_agent(&mut self, count: usize) -> Vec<bevy_ecs::entity::Entity> {
 let to_remove = count.min(self.agents.len());
 self.agents.drain(0..to_remove).collect()
 }

 pub fn total_wait(&self) -> f32 {
 self.agents.len() as f32 * 0.5
 }

 pub fn is_full(&self) -> bool {
 self.agents.len() >= self.max_capacity as usize
 }
}

/// Rail segment component
#[derive(Component, Debug, Clone)]
pub struct RailSegment {
 pub is_tunnel: bool,
 pub is_bridge: bool,
}

/// Block signaling for trains
#[derive(Component, Debug, Clone, Copy)]
pub struct BlockSignal {
 pub block_id: u32,
 pub is_occupied: bool,
 pub next_signal: Option<bevy_ecs::entity::Entity>,
}

impl BlockSignal {
 pub fn new(block_id: u32) -> Self {
 Self {
 block_id,
 is_occupied: false,
 next_signal: None,
 }
 }

 pub fn is_clear(&self) -> bool {
 !self.is_occupied
 }

 pub fn occupy(&mut self) {
 self.is_occupied = true;
 }

 pub fn clear(&mut self) {
 self.is_occupied = false;
 }
}

/// Transit service statistics
#[derive(Component, Debug, Clone, Copy)]
pub struct TransitStats {
 pub ridership: u32,
 pub fare_revenue: f32,
 pub operating_cost: f32,
 pub efficiency: f32,
}

impl Default for TransitStats {
 fn default() -> Self {
 Self {
 ridership: 0,
 fare_revenue: 0.0,
 operating_cost: 0.0,
 efficiency: 1.0,
 }
 }
}

impl TransitStats {
 pub fn profit(&self) -> f32 {
 self.fare_revenue - self.operating_cost
 }

 pub fn update_efficiency(&mut self) {
 if self.operating_cost > 0.0 {
 self.efficiency = (self.fare_revenue / self.operating_cost).min(1.0);
 }
 }
}

/// Transfer node marker for multi-modal connections
#[derive(Component, Debug, Clone, Copy)]
pub struct TransferNode {
 pub connects_walking: bool,
 pub connects_transit: bool,
 pub connects_driving: bool,
}

/// Station influence map value
#[derive(Component, Debug, Clone, Copy)]
pub struct StationInfluence {
 pub service_type: ServiceType,
 pub radius: f32,
 pub strength: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceType {
 Bus,
 Train,
 Mixed,
}
