// Math utilities for City Simulator

use glam::{Vec2, Vec3};

/// Linear interpolation between two values
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Smoothstep interpolation (smoother than lerp)
#[inline]
pub fn smoothstep(a: f32, b: f32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let t = t * t * (3.0 - 2.0 * t);
    a + (b - a) * t
}

/// 2D distance squared (faster than sqrt)
#[inline]
pub fn distance_sq_2d(a: Vec2, b: Vec2) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx * dx + dy * dy
}

/// 3D distance squared
#[inline]
pub fn distance_sq_3d(a: Vec3, b: Vec3) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    dx * dx + dy * dy + dz * dz
}

/// Calculate slope between two points
pub fn slope(p1: Vec3, p2: Vec3) -> f32 {
    let horizontal = ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt();
    let vertical = (p2.z - p1.z).abs();
    
    if horizontal < 0.001 {
        return if vertical < 0.1 { 0.0 } else { f32::INFINITY };
    }
    
    vertical / horizontal
}

/// Check if slope is valid (below maximum grade)
pub fn is_slope_valid(p1: Vec3, p2: Vec3, max_grade: f32) -> bool {
    slope(p1, p2) <= max_grade
}

/// Catmull-Rom spline interpolation
pub fn catmull_rom(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
    let t = t.clamp(0.0, 1.0);
    let t2 = t * t;
    let t3 = t2 * t;
    
    let a = 0.5 * (2.0 * p1);
    let b = 0.5 * (p2 - p0);
    let c = 0.5 * (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3);
    let d = 0.5 * (-p0 + 3.0 * p1 - 3.0 * p2 + p3);
    
    a + b * t + c * t2 + d * t3
}

/// Wrap value to range [0, max)
#[inline]
pub fn wrap(value: f32, max: f32) -> f32 {
    ((value % max) + max) % max
}

/// Clamp value to range [min, max]
#[inline]
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.clamp(min, max)
}

/// Fast approximation of square root
#[inline]
pub fn fast_sqrt(x: f32) -> f32 {
    x.sqrt()
}

/// Pseudo-random hash for spatial queries
pub fn spatial_hash(x: i32, y: i32, seed: i32) -> u32 {
    let mut hash = seed as u32;
    hash = hash.wrapping_mul(0x9e3779b9);
    hash = hash.wrapping_add(x as u32);
    hash = hash.wrapping_mul(0x9e3779b9);
    hash = hash.wrapping_add(y as u32);
    hash = hash.wrapping_mul(0x9e3779b9);
    hash
}

/// Sample noise at position (simple value noise)
pub fn noise(x: f32, y: f32, seed: u32) -> f32 {
    let floor_x = x.floor() as i32;
    let floor_y = y.floor() as i32;
    
    let corners = [
        spatial_hash(floor_x, floor_y, seed as i32) as f32 / u32::MAX as f32,
        spatial_hash(floor_x + 1, floor_y, seed as i32) as f32 / u32::MAX as f32,
        spatial_hash(floor_x, floor_y + 1, seed as i32) as f32 / u32::MAX as f32,
        spatial_hash(floor_x + 1, floor_y + 1, seed as i32) as f32 / u32::MAX as f32,
    ];
    
    let fx = x.fract();
    let fy = y.fract();
    
    let i1 = lerp(corners[0], corners[1], fx);
    let i2 = lerp(corners[2], corners[3], fx);
    
    lerp(i1, i2, fy)
}

/// Map a value from one range to another
pub fn map_range(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    let normalized = (value - from_min) / (from_max - from_min);
    to_min + normalized * (to_max - to_min)
}

/// Rotation matrix for 2D
pub fn rotation_2d(angle: f32) -> [[f32; 2]; 2] {
    let c = angle.cos();
    let s = angle.sin();
    [[c, -s], [s, c]]
}
