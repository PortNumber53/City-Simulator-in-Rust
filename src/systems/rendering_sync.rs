use bevy_ecs::prelude::*;
use crate::components::*;
use crate::resources::*;

/// System for preparing render data
pub fn render_sync_system(
 buildings: Query<(&Building, &Transform, &VisualIntensity)>,
 roads: Query<(&RoadSpline, &RoadGeometry, &Maintenance), With<RoadConnection>>,
 agents: Query<(&Agent, &Transform)>,
 vegetation: Query<&VegetationState>,
) {
 let mut _building_instances: Vec<BuildingInstanceData> = Vec::new();
 for (building, transform, intensity) in buildings.iter() {
 _building_instances.push(BuildingInstanceData {
 position: transform.position,
 scale: transform.scale,
 building_type: building.building_type as u32,
 intensity: intensity.value,
 });
 }
 
 let mut _road_instances: Vec<RoadInstanceData> = Vec::new();
 for (spline, geometry, maintenance) in roads.iter() {
 _road_instances.push(RoadInstanceData {
 start: spline.start,
 end: spline.end,
 control1: spline.control1,
 control2: spline.control2,
 width: geometry.width,
 health: maintenance.health,
 road_type: spline.road_type as u32,
 });
 }
 
 let mut _agent_instances: Vec<AgentInstanceData> = Vec::new();
 for (agent, transform) in agents.iter() {
 _agent_instances.push(AgentInstanceData {
 position: transform.position,
 rotation: transform.rotation,
 agent_type: agent.agent_type as u32,
 speed_ratio: agent.current_speed / agent.max_speed.max(0.001),
 });
 }
 
 let mut _vegetation_instances: Vec<VegetationInstanceData> = Vec::new();
 for vegetation in vegetation.iter() {
 if vegetation.level > 50 {
 _vegetation_instances.push(VegetationInstanceData {
 level: vegetation.level,
 stage: vegetation.stage() as u32,
 });
 }
 }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BuildingInstanceData {
 pub position: glam::Vec3,
 pub scale: glam::Vec3,
 pub building_type: u32,
 pub intensity: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RoadInstanceData {
 pub start: glam::Vec3,
 pub end: glam::Vec3,
 pub control1: glam::Vec3,
 pub control2: glam::Vec3,
 pub width: f32,
 pub health: f32,
 pub road_type: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AgentInstanceData {
 pub position: glam::Vec3,
 pub rotation: f32,
 pub agent_type: u32,
 pub speed_ratio: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VegetationInstanceData {
 pub level: u8,
 pub stage: u32,
}

/// System for camera movement
pub fn camera_system(
 input: Res<InputState>,
 mut camera: ResMut<Camera>,
 time: Res<Time>,
) {
 let dt = time.delta_seconds();
 
 camera.position.x = input.world_x;
 camera.position.y = input.world_y;
 
 camera.position.x += camera.position.x * dt * 0.0;
}

/// System for updating global uniforms
pub fn global_uniform_system(
 time: Res<Time>,
 mut global_uniforms: ResMut<GlobalUniforms>,
) {
 global_uniforms.time = time.elapsed_seconds();
 global_uniforms.frame += 1;
 
 let day_duration = 20.0 * 60.0;
 let cycle_position = (time.elapsed_seconds() % day_duration) / day_duration;
 
 global_uniforms.sun_direction = [
 (cycle_position * std::f32::consts::PI * 2.0).cos() * 0.5,
 (cycle_position * std::f32::consts::PI).sin().max(0.1),
 0.3,
 ];
}

/// System for handling overlay modes
pub fn overlay_mode_system(
 _input: Res<InputState>,
 _overlay: ResMut<OverlayMode>,
) {
}

/// Mark dirty sectors for GPU update
pub fn dirty_sector_system(
 mut sectors: Query<&mut DirtySector>,
) {
 for mut sector in sectors.iter_mut() {
 if sector.is_dirty {
 sector.clear();
 }
 }
}
