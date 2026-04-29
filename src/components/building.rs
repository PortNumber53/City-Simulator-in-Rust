use bevy_ecs::component::Component;
use glam::Vec3;

/// Building type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingType {
 Residential,
 Commercial,
 Industrial,
 Service,
 Utility,
 Abandoned,
 Ruin,
}

impl BuildingType {
 pub fn is_zoned(&self) -> bool {
 matches!(self, BuildingType::Residential | BuildingType::Commercial | BuildingType::Industrial)
 }

 pub fn requires_power(&self) -> bool {
 !matches!(self, BuildingType::Abandoned | BuildingType::Ruin)
 }

 pub fn requires_water(&self) -> bool {
 !matches!(self, BuildingType::Abandoned | BuildingType::Ruin)
 }
}

/// Component identifying a building entity
#[derive(Component, Debug, Clone, Copy)]
pub struct Building {
 pub building_type: BuildingType,
 pub level: u8,
 pub population: u32,
 pub max_population: u32,
}

impl Building {
 pub fn new(building_type: BuildingType) -> Self {
 Self {
 building_type,
 level: 1,
 population: 0,
 max_population: Self::get_max_pop_for_type(building_type),
 }
 }

 pub fn residential(level: u8) -> Self {
 let mut b = Self::new(BuildingType::Residential);
 b.level = level;
 b.max_population = level as u32 * 50;
 b
 }

 pub fn commercial(level: u8) -> Self {
 let mut b = Self::new(BuildingType::Commercial);
 b.level = level;
 b.max_population = level as u32 * 30;
 b
 }

 fn get_max_pop_for_type(building_type: BuildingType) -> u32 {
 match building_type {
 BuildingType::Residential => 50,
 BuildingType::Commercial => 30,
 BuildingType::Industrial => 100,
 BuildingType::Service => 20,
 BuildingType::Utility => 10,
 BuildingType::Abandoned | BuildingType::Ruin => 0,
 }
 }
}

/// Power consumer component
#[derive(Component, Debug, Clone, Copy)]
pub struct PowerConsumer {
 pub demand: f32,
 pub is_powered: bool,
 pub load_factor: f32,
}

impl PowerConsumer {
 pub fn new(demand: f32) -> Self {
 Self {
 demand,
 is_powered: false,
 load_factor: 0.0,
 }
 }
}

/// Power producer component
#[derive(Component, Debug, Clone, Copy)]
pub struct PowerProducer {
 pub supply: f32,
 pub current_output: f32,
}

impl PowerProducer {
 pub fn new(supply: f32) -> Self {
 Self {
 supply,
 current_output: supply,
 }
 }
}

/// Water consumer component
#[derive(Component, Debug, Clone, Copy)]
pub struct WaterConsumer {
 pub demand: f32,
 pub has_water: bool,
}

impl WaterConsumer {
 pub fn new(demand: f32) -> Self {
 Self {
 demand,
 has_water: false,
 }
 }
}

/// Water producer component
#[derive(Component, Debug, Clone, Copy)]
pub struct WaterProducer {
 pub supply: f32,
}

impl WaterProducer {
 pub fn new(supply: f32) -> Self {
 Self { supply }
 }
}

/// Grid connection - links building to utility network
#[derive(Component, Debug, Clone, Copy)]
pub struct GridConnection {
 pub network_id: Option<u32>,
}

impl Default for GridConnection {
 fn default() -> Self {
 Self { network_id: None }
 }
}

/// Visual intensity for dimming effects
#[derive(Component, Debug, Clone, Copy)]
pub struct VisualIntensity {
 pub value: f32,
 pub target: f32,
}

impl Default for VisualIntensity {
 fn default() -> Self {
 Self { value: 1.0, target: 1.0 }
 }
}

impl VisualIntensity {
 pub fn new(value: f32) -> Self {
 Self { value, target: value }
 }

 pub fn update(&mut self, dt: f32, speed: f32) {
 let diff = self.target - self.value;
 self.value += diff * speed * dt;
 }

 pub fn set_from_load_factor(&mut self, load_factor: f32) {
 self.target = load_factor.clamp(0.0, 1.0);
 }

 pub fn apply_flicker(&mut self, time: f32, load_factor: f32) {
 if load_factor < 1.0 {
 let flicker = (time * 20.0).sin() * (1.0 - load_factor) * 0.5;
 self.value = (load_factor + flicker).clamp(0.0, 1.0);
 }
 }
}

/// Service access for hospitals, police, etc.
#[derive(Component, Debug, Clone, Copy)]
pub struct ServiceAccess {
 pub health: u8,
 pub safety: u8,
 pub education: u8,
}

impl Default for ServiceAccess {
 fn default() -> Self {
 Self {
 health: 0,
 safety: 0,
 education: 0,
 }
 }
}

/// Food level for residential buildings
#[derive(Component, Debug, Clone, Copy)]
pub struct FoodLevel {
 pub current: f32,
 pub max: f32,
}

impl Default for FoodLevel {
 fn default() -> Self {
 Self { current: 100.0, max: 100.0 }
 }
}

/// Building abandonment state
#[derive(Component, Debug, Clone, Copy)]
pub struct Abandoned {
 pub ticks_abandoned: u32,
 pub reason: AbandonmentReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbandonmentReason {
 NoPower,
 NoWater,
 NoFood,
 LowMaintenance,
 Disaster,
}

/// Ruin component for destroyed buildings
#[derive(Component, Debug, Clone, Copy)]
pub struct Ruin {
 pub recovery_cost: f32,
 pub months_since_destruction: u32,
}

/// Tax generator component
#[derive(Component, Debug, Clone, Copy)]
pub struct TaxGenerator {
 pub base_rate: f32,
 pub current_revenue: f32,
 pub efficiency: f32,
}

impl TaxGenerator {
 pub fn new(base_rate: f32) -> Self {
 Self {
 base_rate,
 current_revenue: 0.0,
 efficiency: 1.0,
 }
 }

 pub fn calculate_taxes(&self, population: u32) -> f32 {
 self.base_rate * population as f32 * self.efficiency
 }
}

/// Flammability for fire spreading
#[derive(Component, Debug, Clone, Copy)]
pub struct Flammability {
 pub fuel_level: f32,
 pub ignition_temp: f32,
 pub current_heat: f32,
}

impl Default for Flammability {
 fn default() -> Self {
 Self {
 fuel_level: 100.0,
 ignition_temp: 100.0,
 current_heat: 20.0,
 }
 }
}

impl Flammability {
 pub fn is_burning(&self) -> bool {
 self.current_heat >= self.ignition_temp
 }

 pub fn burn(&mut self, heat_transfer: f32, dt: f32) {
 self.current_heat += heat_transfer * dt;
 self.fuel_level -= 10.0 * dt;
 if self.fuel_level <= 0.0 {
 self.fuel_level = 0.0;
 self.current_heat = 0.0;
 }
 }
}

/// Transform component for rendering
#[derive(Component, Debug, Clone, Copy)]
pub struct Transform {
 pub position: Vec3,
 pub rotation: f32,
 pub scale: Vec3,
}

impl Default for Transform {
 fn default() -> Self {
 Self {
 position: Vec3::ZERO,
 rotation: 0.0,
 scale: Vec3::ONE,
 }
 }
}

impl Transform {
 pub fn new(position: Vec3) -> Self {
 Self {
 position,
 rotation: 0.0,
 scale: Vec3::ONE,
 }
 }

 pub fn to_matrix(&self) -> [[f32; 4]; 4] {
 let c = self.rotation.cos();
 let s = self.rotation.sin();
 [
 [c * self.scale.x, -s * self.scale.y, 0.0, 0.0],
 [s * self.scale.x, c * self.scale.y, 0.0, 0.0],
 [0.0, 0.0, self.scale.z, 0.0],
 [self.position.x, self.position.y, self.position.z, 1.0],
 ]
 }
}
