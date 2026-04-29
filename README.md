# City Simulator in Rust

A high-performance city simulation engine written in Rust, designed for WebAssembly (WASM) deployment with ~30,000 concurrently simulated entities.

## Architecture

This project follows **Data-Oriented Design (DOD)** principles using:

- **Bevy ECS** - Entity Component System for efficient parallel processing
- **glam** - SIMD-optimized math library for vector calculations
- **bytemuck** - Zero-copy memory layout for GPU interop
- **petgraph** - Graph algorithms for pathfinding and network analysis

## Features

### 🏗️ Buildings & Infrastructure
- Residential, Commercial, Industrial, Service, and Utility buildings
- Power generation and distribution networks
- Water supply and sewage systems
- Service access (health, safety, education)
- Tax generation and economic simulation

### 🚗 Roads & Transport
- Bezier curve-based road splines with smooth curves
- Multi-lane roads with lane closure support
- Traffic jam simulation with agent frustration
- Public transit system (buses, trains)
- Pathfinding for autonomous agents

### 👥 Agents & Population
- Cars, buses, trains, and emergency vehicles
- Multi-modal journey planning
- Bus stops and waiting queues
- Transit line dispatching

### 🔧 Maintenance & Disasters
- Infrastructure health decay based on budget
- Repair tools (manual and passive)
- Disasters: fires, tornadoes, earthquakes, floods
- Construction zones

### 🌳 Nature & Ecology
- Vegetation growth and ecological succession
- Fire spreading with wind influence
- Soil health and compaction
- Particle emitters for visual effects

### 💰 Economy
- City budget management
- Maintenance fund allocation
- Tax revenue calculation
- Building abandonment mechanics

## Project Structure

```
src/
├── lib.rs              # Main library entry point
├── main.rs             # Native test application
├── components/         # ECS Components
│   ├── agent.rs        # Agent types (cars, trains, etc.)
│   ├── building.rs     # Buildings and power/water
│   ├── grid.rs         # Grid-based spatial indexing
│   ├── maintenance.rs  # Repair tools and maintenance
│   ├── nature.rs       # Vegetation and fires
│   ├── road.rs         # Road splines and geometry
│   ├── transport.rs    # Transit lines and stations
│   └── utilities.rs    # Utility networks
├── systems/            # ECS Systems (game logic)
│   ├── agent_systems.rs
│   ├── maintenance_systems.rs
│   ├── nature_systems.rs
│   ├── rendering_sync.rs
│   ├── road_systems.rs
│   └── utility_systems.rs
├── resources/          # Global Resources
│   └── mod.rs        # Time, budget, grid, networks
└── utils/              # Utilities
    ├── math.rs         # Math helpers
    └── spatial.rs      # Spatial data structures
```

## Building

### Native Build
```bash
cargo build --release
```

### WebAssembly Build
```bash
cargo build --release --target wasm32-unknown-unknown --features wasm
```

## Testing

Run the native test application:
```bash
cargo run
```

## License

MIT License - See LICENSE for details
