use bevy_ecs::prelude::*;
use crate::components::*;
use crate::resources::*;
use rand::Rng;

/// System for ecological succession using random ticks
pub fn nature_healing_system(
 mut commands: Commands,
 nature_system: Res<NatureSystem>,
 mut vegetation_query: Query<(Entity, &GridPosition, &mut VegetationState, Option<&mut Maintenance>)>,
 grid: Res<CityGrid>,
 _time: Res<Time>,
) {
 let base_growth = nature_system.base_growth;
 let tick_count = nature_system.random_tick_count.min(vegetation_query.iter().count());
 let mut rng = rand::thread_rng();
 
 for _ in 0..tick_count {
 let count = vegetation_query.iter().count();
 if count == 0 { break; }
 let index = rng.gen_range(0..count);
 
 let mut vegetation_list: Vec<_> = vegetation_query.iter_mut().collect();
 if let Some((entity, position, vegetation, maintenance)) = vegetation_list.get_mut(index) {
 if let Some(maintenance) = maintenance {
 if maintenance.health > 0.9 {
 vegetation.level = vegetation.level.saturating_sub(50);
 continue;
 }
 }
 
 let neighbors = get_neighbor_levels(&grid, position.x, position.y);
 let bonus = vegetation.calculate_growth_bonus(&neighbors);
 
 let growth_amount = base_growth + bonus;
 vegetation.grow(growth_amount);
 
 if vegetation.level >= 200 && vegetation.level - growth_amount < 200 {
 commands.entity(*entity).insert(VegetationMature {
 tree_type: rng.gen_range(0..3),
 });
 }
 }
 }
}

/// System for soil decompression
pub fn soil_decompression_system(
 time: Res<Time>,
 mut soil_query: Query<&mut SoilHealth>,
) {
 let dt = time.delta_seconds();
 
 for mut soil in soil_query.iter_mut() {
 soil.decompress(0.0001 * dt);
 }
}

/// System for fire spreading
pub fn fire_spread_system(
 mut commands: Commands,
 mut active_fires: ResMut<ActiveFires>,
 mut fire_query: Query<(Entity, &mut NatureFire, &GridPosition)>,
 grid: Res<CityGrid>,
 wind: Res<Wind>,
 time: Res<Time>,
) {
 let dt = time.delta_seconds();
 let to_ignite = ToIgniteBuffer::default();
 
 for (fire_entity, mut fire, position) in fire_query.iter_mut() {
 if !fire.is_active() {
 continue;
 }
 
 fire.burn(dt);
 
 let spread_heat = fire.spread_heat();
 let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
 
 for (dx, dy) in directions.iter() {
 let nx = position.x as i32 + dx;
 let ny = position.y as i32 + dy;
 
 if nx >= 0 && ny >= 0 {
 let wind_factor = wind.dot_with_direction(*dx as f32, *dy as f32);
 let _heat_transfer = spread_heat * (1.0 + wind_factor.max(0.0));
 
 let _cell = grid.get(nx as usize, ny as usize);
 // Would add to ignite buffer
 }
 }
 
 if !fire.is_active() {
 commands.entity(fire_entity).remove::<NatureFire>();
 }
 }
 
 active_fires.fires.retain(|entity| {
 fire_query.get_mut(*entity).map(|(_, fire, _)| fire.is_active()).unwrap_or(false)
 });
 
 let _ = to_ignite;
}

/// System for smoke/particle emission from fires
pub fn particle_system(
 mut commands: Commands,
 mut emitters: Query<(Entity, &mut ParticleEmitter, &GridPosition)>,
 time: Res<Time>,
) {
 let dt = time.delta_seconds();
 
 for (entity, mut emitter, position) in emitters.iter_mut() {
 emitter.update(dt);
 
 if emitter.should_emit(time.elapsed_seconds()) {
 commands.spawn((
 Particle {
 position: position.to_world(),
 velocity: [0.0, 0.5, 0.2],
 lifetime: emitter.lifetime,
 remaining: emitter.lifetime,
 },
 ));
 }
 
 if !emitter.is_active() {
 commands.entity(entity).remove::<ParticleEmitter>();
 }
 }
}

/// System for updating particles
pub fn particle_update_system(
 mut commands: Commands,
 mut particles: Query<(Entity, &mut Particle)>,
 time: Res<Time>,
) {
 let dt = time.delta_seconds();
 
 for (entity, mut particle) in particles.iter_mut() {
 particle.remaining -= dt;
 
 particle.position.x += particle.velocity[0] * dt;
 particle.position.y += particle.velocity[1] * dt;
 particle.position.z += particle.velocity[2] * dt;
 
 if particle.remaining <= 0.0 {
 commands.entity(entity).despawn();
 }
 }
}

fn get_neighbor_levels(_grid: &CityGrid, _x: usize, _y: usize) -> Vec<u8> {
 Vec::new()
}
