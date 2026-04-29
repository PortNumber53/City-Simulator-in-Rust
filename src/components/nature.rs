use bevy_ecs::component::Component;

/// Vegetation state for ecological succession
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct VegetationState {
 pub level: u8,
}

impl VegetationState {
 pub fn new() -> Self {
 Self { level: 0 }
 }

 pub fn fully_naturalized() -> Self {
 Self { level: 255 }
 }

 pub fn stage(&self) -> VegetationStage {
 match self.level {
 0 => VegetationStage::Dead,
 1..=10 => VegetationStage::Pioneer,
 11..=100 => VegetationStage::Grass,
 101..=200 => VegetationStage::Shrubs,
 _ => VegetationStage::Forest,
 }
 }

 pub fn is_mature(&self) -> bool {
 self.level >= 200
 }

 pub fn grow(&mut self, amount: u8) {
 self.level = self.level.saturating_add(amount);
 }

 pub fn clear(&mut self) {
 self.level = 0;
 }

 pub fn decay(&mut self, amount: u8) {
 self.level = self.level.saturating_sub(amount);
 }

 pub fn calculate_growth_bonus(&self, neighbor_levels: &[u8]) -> u8 {
 let avg_neighbor: u8 = if neighbor_levels.is_empty() {
 0
 } else {
 (neighbor_levels.iter().sum::<u8>() / neighbor_levels.len() as u8).min(255)
 };
 avg_neighbor / 10
 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VegetationStage {
 Dead,
 Pioneer,
 Grass,
 Shrubs,
 Forest,
}

/// Soil compaction affects recovery rate
#[derive(Component, Debug, Clone, Copy)]
pub struct SoilHealth {
 pub compaction: f32,
 pub pollution: f32,
 pub recovery_penalty: f32,
}

impl Default for SoilHealth {
 fn default() -> Self {
 Self {
 compaction: 0.0,
 pollution: 0.0,
 recovery_penalty: 0.0,
 }
 }
}

impl SoilHealth {
 pub fn new() -> Self {
 Self::default()
 }

 pub fn compact(&mut self, amount: f32) {
 self.compaction = (self.compaction + amount).min(1.0);
 self.recovery_penalty = self.compaction * 0.5;
 }

 pub fn decompress(&mut self, amount: f32) {
 self.compaction = (self.compaction - amount).max(0.0);
 self.recovery_penalty = self.compaction * 0.5;
 }

 pub fn growth_modifier(&self) -> f32 {
 1.0 - self.recovery_penalty - self.pollution * 0.3
 }
}

/// Fire component for burning entities
#[derive(Component, Debug, Clone, Copy)]
pub struct NatureFire {
 pub intensity: f32,
 pub spread_rate: f32,
 pub fuel_remaining: f32,
}

impl NatureFire {
 pub fn new(intensity: f32) -> Self {
 Self {
 intensity,
 spread_rate: intensity * 0.1,
 fuel_remaining: 100.0,
 }
 }

 pub fn is_active(&self) -> bool {
 self.intensity > 0.0 && self.fuel_remaining > 0.0
 }

 pub fn burn(&mut self, dt: f32) {
 self.fuel_remaining -= 5.0 * dt;
 self.intensity = self.intensity * 0.99;

 if self.fuel_remaining <= 0.0 {
 self.intensity = 0.0;
 self.fuel_remaining = 0.0;
 }
 }

 pub fn spread_heat(&self) -> f32 {
 self.intensity * 0.3
 }
}

/// Disaster component for active disasters
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisasterType {
 Fire,
 Tornado,
 Earthquake,
 Flood,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Disaster {
 pub disaster_type: DisasterType,
 pub center_x: f32,
 pub center_y: f32,
 pub radius: f32,
 pub intensity: f32,
 pub remaining_ticks: u32,
}

impl Disaster {
 pub fn new(disaster_type: DisasterType, x: f32, y: f32, radius: f32, intensity: f32) -> Self {
 Self {
 disaster_type,
 center_x: x,
 center_y: y,
 radius,
 intensity,
 remaining_ticks: 100,
 }
 }

 pub fn is_in_range(&self, x: f32, y: f32) -> bool {
 let dx = x - self.center_x;
 let dy = y - self.center_y;
 (dx * dx + dy * dy).sqrt() <= self.radius
 }

 pub fn damage_at(&self, x: f32, y: f32) -> f32 {
 if !self.is_in_range(x, y) {
 return 0.0;
 }
 let dx = x - self.center_x;
 let dy = y - self.center_y;
 let dist = (dx * dx + dy * dy).sqrt();
 self.intensity * (1.0 - dist / self.radius)
 }
}

/// Particle spawner for fire effects
#[derive(Component, Debug, Clone, Copy)]
pub struct ParticleEmitter {
 pub emission_rate: f32,
 pub lifetime: f32,
 pub remaining: f32,
}

impl ParticleEmitter {
 pub fn fire() -> Self {
 Self {
 emission_rate: 50.0,
 lifetime: 2.0,
 remaining: 10.0,
 }
 }

 pub fn smoke() -> Self {
 Self {
 emission_rate: 20.0,
 lifetime: 5.0,
 remaining: f32::INFINITY,
 }
 }

 pub fn update(&mut self, dt: f32) {
 if self.remaining.is_finite() {
 self.remaining -= dt;
 }
 }

 pub fn is_active(&self) -> bool {
 self.remaining > 0.0
 }

 pub fn should_emit(&self, time: f32) -> bool {
 let interval = 1.0 / self.emission_rate;
 (time / interval).fract() < 0.1
 }
}

/// Terrain modification record
#[derive(Component, Debug, Clone, Copy)]
pub struct TerrainModification {
 pub x: usize,
 pub y: usize,
 pub original_height: f32,
 pub modified_height: f32,
 pub tick: u64,
}

/// Visual marker for mature vegetation
#[derive(Component, Debug, Clone, Copy)]
pub struct VegetationMature {
 pub tree_type: u32,
}

/// Particle component for visual effects
#[derive(Component, Debug, Clone, Copy)]
pub struct Particle {
 pub position: glam::Vec3,
 pub velocity: [f32; 3],
 pub lifetime: f32,
 pub remaining: f32,
}
