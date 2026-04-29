use bevy_ecs::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::resources::*;

/// System for decaying infrastructure health based on budget
pub fn infrastructure_decay_system(
 time: Res<Time>,
 budget: Res<CityBudget>,
 mut maintenance_query: Query<&mut Maintenance>,
) {
 let maintenance_ratio = budget.maintenance_ratio();
 let dt = time.delta_seconds();

 for mut maintenance in maintenance_query.iter_mut() {
 let decay = maintenance.calculate_decay(maintenance_ratio, dt);
 maintenance.health -= decay;
 maintenance.health = maintenance.health.max(0.0);
 }
}

/// System for passive repairs from surplus budget
pub fn passive_repair_system(
 time: Res<Time>,
 budget: Res<CityBudget>,
 mut maintenance_query: Query<&mut Maintenance>,
 current_tick: Res<CurrentTick>,
) {
 if !budget.has_surplus() {
 return;
 }

 let delta = budget.maintenance_fund / budget.required_maintenance - 1.0;
 let repair_rate = delta * 0.001;
 let dt = time.delta_seconds();

 for mut maintenance in maintenance_query.iter_mut() {
 if maintenance.health < 1.0 {
 let repair_amount = repair_rate * dt;
 maintenance.repair(repair_amount, current_tick.0);
 }
 }
}

/// System for manual repairs using the repair tool
pub fn manual_repair_system(
 mut commands: Commands,
 mut maintenance_query: Query<(Entity, &mut Maintenance, &GridPosition)>,
 input: Res<InputState>,
 mut budget: ResMut<CityBudget>,
 current_tick: Res<CurrentTick>,
) {
 if input.current_tool != ToolMode::Repair || !input.is_left_mouse_down {
 return;
 }

 let tool_radius = 3.0;
 let tool_x = input.world_x;
 let tool_y = input.world_y;

 for (entity, mut maintenance, position) in maintenance_query.iter_mut() {
 let pos = position.to_world();
 let dx = pos.x - tool_x;
 let dy = pos.y - tool_y;
 let dist_sq = dx * dx + dy * dy;

 if dist_sq < tool_radius * tool_radius {
 let health_missing = 1.0 - maintenance.health;
 let base_cost = 100.0 * health_missing;
 
 if budget.spend_maintenance(base_cost) {
 maintenance.repair(health_missing, current_tick.0);
 
 commands.entity(entity).insert(UnderConstruction {
 remaining_ticks: 60,
 });
 }
 }
 }
}

/// System for handling construction sites
pub fn construction_site_system(
 mut commands: Commands,
 time: Res<Time>,
 mut sites: Query<(Entity, &mut UnderConstruction)>,
) {
 let dt = time.delta_seconds();
 
 for (entity, mut construction) in sites.iter_mut() {
 let ticks_to_remove = (60.0 * dt) as u32;
 
 if construction.remaining_ticks <= ticks_to_remove {
 commands.entity(entity).remove::<UnderConstruction>();
 } else {
 construction.remaining_ticks -= ticks_to_remove;
 }
 }
}

/// System for updating required maintenance calculations
pub fn update_maintenance_requirements(
 maintenance_query: Query<&Maintenance>,
 mut budget: ResMut<CityBudget>,
) {
 let mut required = 0.0f32;
 
 for maintenance in maintenance_query.iter() {
 required += 1.0;
 required += maintenance.wear_rate * 100.0;
 }
 
 budget.required_maintenance = required;
}

/// System for disaster-organization
pub fn disaster_system(
 mut commands: Commands,
 mut disasters: Query<(Entity, &mut Disaster)>,
 mut buildings: Query<(Entity, &GridPosition, &mut Flammability)>,
 time: Res<Time>,
) {
 let dt = time.delta_seconds();
 
 for (disaster_entity, mut disaster) in disasters.iter_mut() {
 disaster.remaining_ticks = disaster.remaining_ticks.saturating_sub((dt * 60.0) as u32);
 
 match disaster.disaster_type {
 crate::components::DisasterType::Fire => {
 let radius = disaster.radius;
 let cx = disaster.center_x;
 let cy = disaster.center_y;
 
 for (_building_entity, position, mut flammability) in buildings.iter_mut() {
 let pos = position.to_world();
 let dx = pos.x - cx;
 let dy = pos.y - cy;
 if (dx * dx + dy * dy).sqrt() < radius {
 if flammability.current_heat < flammability.ignition_temp {
 flammability.current_heat += 10.0 * dt;
 }
 }
 }
 }
 crate::components::DisasterType::Tornado => {
 disaster.center_x += 1.0 * dt;
 disaster.center_y += 0.5 * dt;
 }
 crate::components::DisasterType::Earthquake => {}
 crate::components::DisasterType::Flood => {
 disaster.radius += 0.1 * dt;
 }
 }
 
 if disaster.remaining_ticks == 0 {
 commands.entity(disaster_entity).despawn();
 }
 }
}

/// Spawns disasters randomly
pub fn spawn_disaster_system(
 mut commands: Commands,
 time: Res<Time>,
) {
 let mut rng = rand::thread_rng();
 
 if rng.gen::<f32>() < 0.00001 {
 let x = (rng.gen::<f32>() * 256.0) - 128.0;
 let y = (rng.gen::<f32>() * 256.0) - 128.0;
 
 let disaster_type = match rng.gen::<u32>() % 4 {
 0 => DisasterType::Fire,
 1 => DisasterType::Tornado,
 2 => DisasterType::Earthquake,
 _ => DisasterType::Flood,
 };
 
 commands.spawn((
 Disaster::new(disaster_type, x, y, 10.0, 1.0),
 ));
 }
}
