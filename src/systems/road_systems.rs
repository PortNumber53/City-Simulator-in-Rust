use bevy_ecs::prelude::*;
use crate::{components::*, resources::*};

/// System for updating road connectivity
pub fn road_connectivity_system(
 mut _road_query: Query<(&mut RoadSpline, &mut GridConnection, &GridPosition)>,
 _grid: Res<CityGrid>,
) {
}

/// System for updating traffic weights on navigation graph
pub fn navigation_weight_system(
 roads: Query<(&RoadGeometry, &Maintenance, &RepairState), With<RoadSpline>>,
 mut graph: ResMut<NavigationGraph>,
) {
 graph.nodes.clear();
 graph.edges.clear();

 for (_geometry, _maintenance, _state) in roads.iter() {
 }
}

/// System for managing the two-phase repair workflow
pub fn repair_orchestrator_system(
 mut commands: Commands,
 mut roads: Query<(Entity, &mut RepairState, &TrafficCounter, &RoadGeometry, &GridPosition)>,
 mut depots: Query<&mut UtilityDepot>,
 time: Res<Time>,
 current_tick: Res<CurrentTick>,
) {
 for (entity, mut state, counter, _geometry, _position) in roads.iter_mut() {
 match *state {
 RepairState::PendingRedirection => {
 for mut depot in depots.iter_mut() {
 if depot.can_dispatch(time.elapsed_seconds()) {
 depot.dispatch(time.elapsed_seconds());
 *state = RepairState::DrainingTraffic;
 commands.entity(entity).insert(ConstructionZone::new(current_tick.0));
 break;
 }
 }
 }
 RepairState::DrainingTraffic => {
 if counter.count == 0 {
 *state = RepairState::RepairActive;
 }
 }
 RepairState::RepairActive => {}
 RepairState::Reopening => {
 *state = RepairState::Stable;
 commands.entity(entity).remove::<ConstructionZone>();
 }
 _ => {}
 }
 }
}

/// Current tick counter resource
#[derive(Resource)]
pub struct CurrentTick(pub u64);

impl Default for CurrentTick {
 fn default() -> Self {
 Self(0)
 }
}

/// Path request queue
#[derive(Resource, Default)]
pub struct PathRequestQueue {
 pub requests: Vec<PathRequest>,
}

#[derive(Debug, Clone, Copy)]
pub struct PathRequest {
 pub entity: Entity,
 pub start: glam::Vec3,
 pub destination: glam::Vec3,
 pub agent_type: AgentType,
}
