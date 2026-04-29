use bevy_ecs::prelude::*;
use glam::Vec3;
use crate::{components::*, resources::*};

/// System for moving agents along road splines
pub fn agent_movement_system(
 time: Res<Time>,
 mut agents: Query<(&mut Agent, &mut PathProgress, &Priority)>,
 roads: Query<(&RoadSpline, &RoadGeometry, &Maintenance), With<RoadConnection>>,
) {
 for (mut agent, mut progress, priority) in agents.iter_mut() {
 if let Ok((spline, geometry, maintenance)) = roads.get(progress.current_road) {
 let health_multiplier = maintenance.speed_multiplier();
 let lane_mult = if geometry.has_passable_lane() { 1.0 } else { 0.0 };
 
 let target_speed = agent.max_speed * health_multiplier * lane_mult;
 agent.update_speed(target_speed, time.delta_seconds());

 if priority.is_emergency() && maintenance.health > 0.0 {
 let emergency_max = agent.max_speed;
 agent.update_speed_with_max(emergency_max, emergency_max, time.delta_seconds());
 }

 let spline_length = (spline.end - spline.start).length();
 progress.advance(agent.current_speed, spline_length);

 if agent.current_speed < agent.max_speed * 0.1 {
 agent.add_frustration(0.01 * time.delta_seconds());
 } else {
 agent.reduce_frustration(0.001 * time.delta_seconds());
 }
 }
 }
}

/// System for updating agent transforms for rendering
pub fn agent_transform_system(
 mut query: Query<(&PathProgress, &Agent, &mut Transform)>,
 roads: Query<&RoadSpline, With<RoadConnection>>,
) {
 for (progress, agent, mut transform) in query.iter_mut() {
 if let Ok(spline) = roads.get(progress.current_road) {
 let t = progress.t.clamp(0.0, 1.0);
 let position = spline.get_point(t);
 let tangent = spline.get_tangent(t);

 let jitter = if agent.agent_type == AgentType::Car {
 let speed_factor = agent.current_speed / agent.max_speed;
 (1.0 - agent.frustration) * 0.05 * speed_factor
 } else {
 0.0
 };

 transform.position = position + Vec3::new(0.0, 0.0, jitter);
 transform.rotation = tangent.y.atan2(tangent.x);
 }
 }
}

/// System for transit line dispatching
pub fn transit_dispatch_system(
 time: Res<Time>,
 mut lines: Query<&mut TransitLine>,
 mut depots: Query<&mut UtilityDepot>,
 mut commands: Commands,
) {
 for mut line in lines.iter_mut() {
 if line.should_dispatch(time.elapsed_seconds()) {
 for mut depot in depots.iter_mut() {
 let can_dispatch = match line.mode {
 TransitMode::Bus => depot.depot_type == UtilityDepotType::BusDepot,
 TransitMode::Train => depot.depot_type == UtilityDepotType::TrainYard,
 };

 if can_dispatch && depot.dispatch(time.elapsed_seconds()) {
 spawn_transit_vehicle(&mut commands, &line, time.elapsed_seconds());
 line.mark_dispatched(time.elapsed_seconds());
 break;
 }
 }
 }
 }
}

fn spawn_transit_vehicle(
 commands: &mut Commands,
 line: &TransitLine,
 _current_time: f32,
) {
 let capacity = match line.mode {
 TransitMode::Bus => TransitCapacity::bus(),
 TransitMode::Train => TransitCapacity::train(),
 };

 let agent_type = match line.mode {
 TransitMode::Bus => AgentType::Bus,
 TransitMode::Train => AgentType::Train,
 };

 commands.spawn((
 Agent::new(agent_type),
 PathProgress::new(line.path_segments.first().copied().unwrap_or(bevy_ecs::entity::Entity::PLACEHOLDER), 1),
 capacity,
 FixedPath::new(line.id),
 AgentVisual::default(),
 ));
}

/// System for handling bus stop logic
pub fn bus_stop_system(
 mut stops: Query<(&mut BusStop, &mut WaitingQueue, &GridPosition)>,
 mut vehicles: Query<(&mut Agent, &mut PathProgress, &mut TransitCapacity), With<FixedPath>>,
) {
 for (stop, mut queue, _position) in stops.iter_mut() {
 for (_vehicle, progress, mut capacity) in vehicles.iter_mut() {
 if progress.is_complete() {
 let _boarding = (capacity.available_seats() as f32 * stop.stop_position) as u16;
 let alighting = (capacity.current_passengers as f32 * 0.3) as u16;
 capacity.current_passengers = capacity.current_passengers.saturating_sub(alighting);

 let to_board = (capacity.available_seats() as usize).min(queue.agents.len());
 let _picked_up = queue.remove_agent(to_board);
 capacity.current_passengers += to_board as u16;
 }
 }
 }
}

/// System for updating train block signaling
pub fn train_signaling_system(
 mut blocks: Query<&mut BlockSignal>,
 trains: Query<(&PathProgress, &Agent), With<BlockSignal>>,
) {
 for mut block in blocks.iter_mut() {
 block.clear();
 }

 for (progress, _agent) in trains.iter() {
 let _ = progress.current_road;
 }
}

/// Multi-modal pathfinding decision system
pub fn pathfinding_decision_system(
 mut query: Query<(Entity, &mut Itinerary, &Commuter), Without<Agent>>,
 _stations: Query<(&TransitStop, &StationInfluence)>,
 _transit_version: Res<TransitVersion>,
) {
 for (_entity, itinerary, _commuter) in query.iter_mut() {
 if let Some(JourneyLeg::Transit { .. }) = itinerary.current_leg() {
 // In a real implementation, check if line still exists
 // If not, trigger re-route
 }
 }
}

/// System for updating train positions
pub fn train_movement_system(
 time: Res<Time>,
 mut trains: Query<(&mut Agent, &mut PathProgress), With<BlockSignal>>,
 blocks: Query<&BlockSignal>,
) {
 for (mut train, mut _progress) in trains.iter_mut() {
 let target_speed = if blocks.iter().any(|b| !b.is_clear()) {
 0.0
 } else {
 train.max_speed
 };

 train.update_speed(target_speed, time.delta_seconds());
 }
}
