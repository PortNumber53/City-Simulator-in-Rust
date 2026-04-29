use bevy_ecs::component::Component;
use glam::Vec3;

/// Agent type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentType {
 Car,
 Bus,
 Train,
 Emergency,
 Pedestrian,
 FireTruck,
 PoliceCar,
 Ambulance,
 MaintenanceTruck,
}

impl AgentType {
 pub fn is_emergency(&self) -> bool {
 matches!(self, Self::Emergency | Self::FireTruck | Self::PoliceCar | Self::Ambulance)
 }

 pub fn is_public_transit(&self) -> bool {
 matches!(self, Self::Bus | Self::Train)
 }

 pub fn max_speed(&self) -> f32 {
 match self {
 Self::Emergency | Self::FireTruck | Self::Ambulance | Self::PoliceCar => 30.0,
 Self::Car => 20.0,
 Self::Bus => 15.0,
 Self::Train => 40.0,
 Self::Pedestrian => 2.0,
 Self::MaintenanceTruck => 12.0,
 }
 }
}

/// Main agent component
#[derive(Component, Debug, Clone, Copy)]
pub struct Agent {
 pub agent_type: AgentType,
 pub max_speed: f32,
 pub preferred_speed: f32,
 pub current_speed: f32,
 pub acceleration: f32,
 pub frustration: f32,
}

impl Agent {
 pub fn new(agent_type: AgentType) -> Self {
 let max_speed = agent_type.max_speed();
 Self {
 agent_type,
 max_speed,
 preferred_speed: max_speed,
 current_speed: 0.0,
 acceleration: 5.0,
 frustration: 0.0,
 }
 }

 pub fn update_speed(&mut self, target_speed: f32, dt: f32) {
 let max_speed = self.max_speed;
 self.update_speed_with_max(target_speed, max_speed, dt);
 }

 pub fn update_speed_with_max(&mut self, target_speed: f32, max_speed: f32, dt: f32) {
 let diff = target_speed - self.current_speed;
 self.current_speed += diff.signum() * self.acceleration * dt;
 self.current_speed = self.current_speed.clamp(0.0, max_speed);
 }

 pub fn add_frustration(&mut self, amount: f32) {
 self.frustration = (self.frustration + amount).clamp(0.0, 1.0);
 }

 pub fn reduce_frustration(&mut self, amount: f32) {
 self.frustration = (self.frustration - amount).clamp(0.0, 1.0);
 }
}

/// Position along a road spline using parametric coordinate
#[derive(Component, Debug, Clone, Copy)]
pub struct PathProgress {
 pub current_road: bevy_ecs::entity::Entity,
 pub t: f32,
 pub lane: u8,
 pub direction: i8,
}

impl PathProgress {
 pub fn new(road: bevy_ecs::entity::Entity, lane: u8) -> Self {
 Self {
 current_road: road,
 t: 0.0,
 lane,
 direction: 1,
 }
 }

 pub fn advance(&mut self, speed: f32, spline_length: f32) {
 let dt = speed / spline_length.max(0.001);
 self.t += dt * self.direction as f32;

 if self.t > 1.0 {
 self.t = 1.0;
 } else if self.t < 0.0 {
 self.t = 0.0;
 }
 }

 pub fn is_complete(&self) -> bool {
 (self.direction > 0 && self.t >= 1.0) || (self.direction < 0 && self.t <= 0.0)
 }
}

/// Multi-modal journey itinerary
#[derive(Component, Debug, Clone)]
pub struct Itinerary {
 pub current_leg: usize,
 pub legs: Vec<JourneyLeg>,
 pub last_validated_version: u64,
}

impl Itinerary {
 pub fn new(legs: Vec<JourneyLeg>) -> Self {
 Self {
 current_leg: 0,
 legs,
 last_validated_version: 0,
 }
 }

 pub fn current_leg(&self) -> Option<&JourneyLeg> {
 self.legs.get(self.current_leg)
 }

 pub fn advance_leg(&mut self) {
 self.current_leg += 1;
 }

 pub fn is_complete(&self) -> bool {
 self.current_leg >= self.legs.len()
 }

 pub fn invalidate(&mut self) {
 self.last_validated_version = 0;
 }
}

/// A single segment of a journey
#[derive(Debug, Clone, Copy)]
pub enum JourneyLeg {
 Walk { from: Vec3, to: Vec3 },
 Drive { road_segments: bevy_ecs::entity::Entity },
 Transit { line_id: u32, get_on_stop: bevy_ecs::entity::Entity, get_off_stop: bevy_ecs::entity::Entity },
}

/// Pathfinding priority for emergency vehicles
#[derive(Component, Debug, Clone, Copy)]
pub struct Priority {
 pub level: u8,
}

impl Default for Priority {
 fn default() -> Self {
 Self { level: 0 }
 }
}

impl Priority {
 pub fn emergency() -> Self {
 Self { level: 255 }
 }

 pub fn is_emergency(&self) -> bool {
 self.level >= 200
 }
}

/// Vehicle state for lane changing and maneuvering
#[derive(Component, Debug, Clone, Copy)]
pub enum VehicleState {
 Driving,
 Stopped,
 LaneChanging { target_lane: u8, progress: f32 },
 StoppedAtBusStop,
 LoadingUnloading,
 EmergencyBypass,
}

impl Default for VehicleState {
 fn default() -> Self {
 Self::Driving
 }
}

/// Passenger capacity and current occupancy
#[derive(Component, Debug, Clone, Copy)]
pub struct TransitCapacity {
 pub max_passengers: u16,
 pub current_passengers: u16,
 pub boarding_rate: f32,
}

impl TransitCapacity {
 pub fn bus() -> Self {
 Self {
 max_passengers: 50,
 current_passengers: 0,
 boarding_rate: 2.0,
 }
 }

 pub fn train() -> Self {
 Self {
 max_passengers: 500,
 current_passengers: 0,
 boarding_rate: 10.0,
 }
 }

 pub fn available_seats(&self) -> u16 {
 self.max_passengers.saturating_sub(self.current_passengers)
 }

 pub fn occupancy_ratio(&self) -> f32 {
 self.current_passengers as f32 / self.max_passengers.max(1) as f32
 }
}

/// Agent renderer component
#[derive(Component, Debug, Clone, Copy)]
pub struct AgentVisual {
 pub mesh_index: u32,
 pub color: [f32; 4],
 pub emissive: f32,
}

impl Default for AgentVisual {
 fn default() -> Self {
 Self {
 mesh_index: 0,
 color: [1.0, 1.0, 1.0, 1.0],
 emissive: 0.0,
 }
 }
}

/// Commuter stats for citizen agents
#[derive(Component, Debug, Clone, Copy)]
pub struct Commuter {
 pub home: bevy_ecs::entity::Entity,
 pub work: Option<bevy_ecs::entity::Entity>,
 pub current_activity: Activity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Activity {
 AtHome,
 Commuting,
 AtWork,
 Shopping,
 Emergency,
}

/// Maintenance crew component
#[derive(Component, Debug, Clone, Copy)]
pub struct MaintenanceCrew {
 pub crew_type: CrewType,
 pub target_entity: Option<bevy_ecs::entity::Entity>,
 pub work_progress: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrewType {
 TrafficWarden,
 ConstructionCrew,
 TowTruck,
}
