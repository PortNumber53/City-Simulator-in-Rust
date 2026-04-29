use bevy_ecs::prelude::*;
use crate::{components::*, resources::*};

/// System for aggregating power network supply and demand
pub fn power_aggregation_system(
 mut networks: ResMut<NetworkMap>,
 producers: Query<(&PowerProducer, &GridConnection)>,
 consumers: Query<(&PowerConsumer, &GridConnection)>,
) {
 networks.reset_all();

 for (producer, connection) in producers.iter() {
 if let Some(network_id) = connection.network_id {
 networks.add_supply(network_id, producer.supply);
 }
 }

 for (consumer, connection) in consumers.iter() {
 if let Some(network_id) = connection.network_id {
 networks.add_demand(network_id, consumer.demand);
 }
 }

 networks.update_statuses();
}

/// System for distributing power and applying effects
pub fn power_distribution_system(
 networks: Res<NetworkMap>,
 time: Res<Time>,
 mut consumers: Query<(&mut PowerConsumer, &GridConnection, &mut VisualIntensity)>,
) {
 for (mut consumer, connection, mut visual) in consumers.iter_mut() {
 if let Some(network_id) = connection.network_id {
 if let Some(network) = networks.get(network_id) {
 let load_factor = network.load_factor();
 
 consumer.is_powered = load_factor > 0.1;
 consumer.load_factor = load_factor;

 if load_factor < 1.0 {
 let time_seconds = time.elapsed_seconds();
 let flicker = (time_seconds * 15.0).sin() * (1.0 - load_factor) * 0.2;
 visual.value = (load_factor + flicker).clamp(0.0, 1.0);
 } else {
 visual.value = 1.0;
 }
 }
 } else {
 consumer.is_powered = false;
 visual.value = 0.0;
 }
 }
}

/// System for maintaining utility network connectivity
pub fn connectivity_update_system(
 mut grid: ResMut<CityGrid>,
 _roads: Query<&GridConnection, Changed<RoadGeometry>>,
) {
 grid.rebuild_networks();
}

/// Disjoint Set Union for efficient network grouping
pub struct DisjointSetUnion {
 parent: Vec<u32>,
 rank: Vec<u32>,
}

impl DisjointSetUnion {
 pub fn new(size: usize) -> Self {
 Self {
 parent: (0..size as u32).collect(),
 rank: vec![0; size],
 }
 }

 pub fn find(&mut self, x: u32) -> u32 {
 let parent = self.parent[x as usize];
 if parent != x {
 self.parent[x as usize] = self.find(parent);
 self.parent[x as usize]
 } else {
 x
 }
 }

 pub fn union(&mut self, x: u32, y: u32) {
 let x_root = self.find(x);
 let y_root = self.find(y);
 
 if x_root == y_root {
 return;
 }

 if self.rank[x_root as usize] < self.rank[y_root as usize] {
 self.parent[x_root as usize] = y_root;
 } else if self.rank[x_root as usize] > self.rank[y_root as usize] {
 self.parent[y_root as usize] = x_root;
 } else {
 self.parent[y_root as usize] = x_root;
 self.rank[x_root as usize] += 1;
 }
 }
}
