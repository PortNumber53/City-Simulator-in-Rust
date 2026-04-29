use bevy_ecs::component::Component;
use glam::Vec3;

/// Road type determining visual and behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoadType {
 Surface,
 Embankment,
 Bridge,
 Tunnel,
}

impl Default for RoadType {
 fn default() -> Self {
 RoadType::Surface
 }
}

/// 4-bit road mask for auto-tiling
pub struct RoadMask;

impl RoadMask {
 pub const NORTH: u8 = 1;
 pub const EAST: u8 = 2;
 pub const SOUTH: u8 = 4;
 pub const WEST: u8 = 8;

 pub fn from_neighbors(n: bool, e: bool, s: bool, w: bool) -> u8 {
 (if n { Self::NORTH } else { 0 })
 | (if e { Self::EAST } else { 0 })
 | (if s { Self::SOUTH } else { 0 })
 | (if w { Self::WEST } else { 0 })
 }

 pub fn get_rotation(mask: u8) -> f32 {
 match mask {
 3 => 0.0,
 6 => 90.0,
 12 => 180.0,
 9 => 270.0,
 _ => 0.0,
 }
 }
}

/// Cubic Bezier curve control points for road segment
#[derive(Component, Debug, Clone)]
pub struct RoadSpline {
 pub start: Vec3,
 pub end: Vec3,
 pub control1: Vec3,
 pub control2: Vec3,
 pub road_type: RoadType,
}

impl RoadSpline {
 pub fn new(start: Vec3, end: Vec3) -> Self {
 let mid = (start + end) * 0.5;
 Self {
 start,
 end,
 control1: mid,
 control2: mid,
 road_type: RoadType::Surface,
 }
 }

 pub fn with_curvature(start: Vec3, end: Vec3, curvature: f32) -> Self {
 let mid = (start + end) * 0.5;
 let offset = (end - start).cross(Vec3::Z).normalize() * curvature;
 Self {
 start,
 end,
 control1: mid + offset,
 control2: mid + offset,
 road_type: RoadType::Surface,
 }
 }

 pub fn get_point(&self, t: f32) -> Vec3 {
 let t2 = t * t;
 let t3 = t2 * t;
 let inv_t = 1.0 - t;
 let inv_t2 = inv_t * inv_t;
 let inv_t3 = inv_t2 * inv_t;

 self.start * inv_t3
 + self.control1 * 3.0 * inv_t2 * t
 + self.control2 * 3.0 * inv_t * t2
 + self.end * t3
 }

 pub fn get_tangent(&self, t: f32) -> Vec3 {
 let t2 = t * t;
 let inv_t = 1.0 - t;
 let inv_t2 = inv_t * inv_t;

 let tangent = self.start * (-3.0 * inv_t2)
 + self.control1 * (3.0 * inv_t2 - 6.0 * inv_t * t)
 + self.control2 * (6.0 * inv_t * t - 3.0 * t2)
 + self.end * (3.0 * t2);

 tangent.normalize()
 }

 pub fn get_normal(&self, t: f32) -> Vec3 {
 let tangent = self.get_tangent(t);
 Vec3::new(-tangent.y, tangent.x, 0.0).normalize()
 }

 pub fn get_elevation(&self, t: f32) -> f32 {
 self.start.z * (1.0 - t) + self.end.z * t
 }

 pub fn sample(&self, segments: usize) -> Vec<Vec3> {
 (0..=segments)
 .map(|i| {
 let t = i as f32 / segments as f32;
 self.get_point(t)
 })
 .collect()
 }

 pub fn get_entry_exit(&self) -> (Vec3, Vec3) {
 (self.start, self.end)
 }

 pub fn is_slope_valid(&self, max_grade: f32) -> bool {
 let horizontal = ((self.end.x - self.start.x).powi(2)
 + (self.end.y - self.start.y).powi(2))
 .sqrt();
 let vertical = (self.end.z - self.start.z).abs();
 if horizontal < 0.001 {
 return vertical < 0.1;
 }
 (vertical / horizontal) <= max_grade
 }
}

/// Road geometry for rendering/traffic
#[derive(Component, Debug, Clone)]
pub struct RoadGeometry {
 pub total_lanes: u8,
 pub closed_lanes: u8,
 pub is_one_way: bool,
 pub width: f32,
}

impl RoadGeometry {
 pub fn new(lanes: u8) -> Self {
 Self {
 total_lanes: lanes,
 closed_lanes: 0,
 is_one_way: false,
 width: lanes as f32 * 3.5,
 }
 }

 pub fn one_way(mut self) -> Self {
 self.is_one_way = true;
 self
 }

 pub fn with_width(mut self, width: f32) -> Self {
 self.width = width;
 self
 }

 #[inline(always)]
 pub fn has_passable_lane(&self) -> bool {
 (self.total_lanes as u8 - self.closed_lanes) > 0
 }

 #[inline(always)]
 pub fn passable_lanes(&self) -> u8 {
 self.total_lanes.saturating_sub(self.closed_lanes)
 }

 #[inline(always)]
 pub fn lane_bitmask(&self) -> u8 {
 let open = self.passable_lanes();
 (1u8 << open).wrapping_sub(1)
 }
}

/// Start and end node entities for graph connectivity
#[derive(Component, Debug, Clone, Copy)]
pub struct RoadConnection {
 pub start_node: bevy_ecs::entity::Entity,
 pub end_node: bevy_ecs::entity::Entity,
}

/// Intersection/Node in the road graph
#[derive(Component, Debug, Clone, Copy)]
pub struct RoadNode {
 pub position: Vec3,
 pub is_intersection: bool,
}

impl RoadNode {
 pub fn new(pos: Vec3) -> Self {
 Self {
 position: pos,
 is_intersection: false,
 }
 }

 pub fn intersection(pos: Vec3) -> Self {
 Self {
 position: pos,
 is_intersection: true,
 }
 }
}

/// Traffic counter for tracking occupancy
#[derive(Component, Debug, Clone, Copy)]
pub struct TrafficCounter {
 pub count: u32,
}

impl Default for TrafficCounter {
 fn default() -> Self {
 Self { count: 0 }
 }
}

/// Repaired state for visual feedback
#[derive(Component, Debug, Clone, Copy)]
pub struct RepairTint {
 pub duration_ticks: u32,
 pub max_duration: u32,
}

impl RepairTint {
 pub fn new(duration: u32) -> Self {
 Self {
 duration_ticks: duration,
 max_duration: duration,
 }
 }

 pub fn progress(&self) -> f32 {
 1.0 - (self.duration_ticks as f32 / self.max_duration as f32)
 }
}

/// Bus stop attached to a road segment
#[derive(Component, Debug, Clone)]
pub struct BusStop {
 pub stop_position: f32,
 pub line_ids: Vec<u32>,
 pub has_bus_present: bool,
}

/// Constraint checking for road placement
#[derive(Component, Debug, Clone, Copy)]
pub struct SlopeConstraint {
 pub max_grade: f32,
}

impl Default for SlopeConstraint {
 fn default() -> Self {
 Self { max_grade: 0.10 }
 }
}
